//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
pub mod health;
use chacha20poly1305::Key;
use nix::sys::socket::{setsockopt, sockopt::BindToDevice};
use onebox_core::crypto::{decrypt_in_place, encrypt_in_place};
use onebox_core::packet::{PacketHeader, PacketType};
use onebox_core::prelude::*;
use onebox_core::types::ClientId;
use std::collections::HashMap;
use std::ffi::OsString;
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, RwLock};
use tokio_tun::TunBuilder;
use tracing::{debug, error, info, warn, Level};

async fn perform_handshake(
    socket: &UdpSocket,
    key: &Key,
    client_id: ClientId,
) -> anyhow::Result<()> {
    info!("Performing handshake...");
    let auth_request_header = PacketHeader::new(0, PacketType::AuthRequest, client_id);
    let header_bytes = bincode::serialize(&auth_request_header)?;
    // The payload for an auth request can be minimal.
    let encrypted_payload = encrypt(key, b"AUTH_REQUEST", 0)?;
    let request_packet = [header_bytes.as_slice(), encrypted_payload.as_slice()].concat();

    for i in 0..5 {
        info!("Sending AuthRequest (attempt {})...", i + 1);
        socket.send(&request_packet).await?;

        let mut recv_buf = [0u8; 2048];
        match tokio::time::timeout(
            std::time::Duration::from_secs(2),
            socket.recv(&mut recv_buf),
        )
        .await
        {
            Ok(Ok(len)) => {
                let response_packet = &recv_buf[..len];
                if let Ok(header) = bincode::deserialize::<PacketHeader>(response_packet) {
                    if header.packet_type == PacketType::AuthResponse {
                        info!("Handshake successful: received AuthResponse from server.");
                        return Ok(());
                    }
                }
                warn!("Received unexpected packet during handshake.");
            }
            Ok(Err(e)) => return Err(anyhow::anyhow!("Socket recv error during handshake: {}", e)),
            Err(_) => {
                warn!("Handshake timeout, retrying...");
                continue;
            }
        }
    }

    Err(anyhow::anyhow!("Handshake failed after multiple attempts."))
}

/// Discovers WAN interfaces and binds a UDP socket to each one.
/// Returns a vector containing the interface name and the bound socket.
async fn discover_and_bind_sockets(
    server_addr: SocketAddr,
) -> anyhow::Result<Vec<(String, Arc<UdpSocket>)>> {
    info!("Discovering WAN interfaces and binding sockets...");
    let mut sockets = Vec::new();
    let ifaces = NetworkInterface::show()?;

    for iface in ifaces {
        debug!("Processing interface: {:?}", iface);
        // Patched for testing: Allow veth and skip onebox interfaces.
        if iface.name.starts_with("lo")
            || iface.name.contains("docker")
            || iface.name.starts_with("onebox")
        {
            debug!("Skipping interface {}: loopback/virtual", iface.name);
            continue;
        }

        for addr in &iface.addr {
            if let std::net::IpAddr::V4(ipv4) = addr.ip() {
                // Patched for testing: The original code filtered for public/CGNAT IPs.
                // This version allows any IPv4 address to facilitate testing in simulated environments.
                info!(
                    "Found potential WAN interface {} with IP {}",
                    iface.name, ipv4
                );

                // Bind a socket to 0.0.0.0:0 to let the OS choose the port
                let socket = UdpSocket::bind("0.0.0.0:0").await?;

                // Per SI-1 and FR-C-04, bind this socket to the specific device
                let device_name = OsString::from(iface.name.as_str());
                if let Err(e) = setsockopt(&socket, BindToDevice, &device_name) {
                    error!("Failed to bind socket to device {}: {}", iface.name, e);
                    continue; // Don't add this socket if we couldn't bind it.
                }

                info!("Successfully bound UDP socket to device {}", iface.name);

                // Connect the socket to the server's address for easy sending
                socket.connect(server_addr).await?;
                info!(
                    "Socket for {} connected to server at {}",
                    iface.name, server_addr
                );

                sockets.push((iface.name.clone(), Arc::new(socket)));

                // We only need one socket per interface, so break from the address loop.
                break;
            }
        }
    }

    if sockets.is_empty() {
        error!("No suitable WAN interfaces found. Please check network configuration.");
        return Err(anyhow::anyhow!("No WAN interfaces found."));
    }

    info!(
        "Successfully bound {} sockets to WAN interfaces.",
        sockets.len()
    );
    Ok(sockets)
}

#[derive(Parser)]
#[command(name = "onebox-client")]
#[command(about = "Client for onebox-rs internet bonding solution")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the client
    Start {
        /// Run in foreground (don't daemonize)
        #[arg(short, long)]
        foreground: bool,
    },

    /// Stop the client
    Stop,

    /// Show client status
    Status,

    /// Show client configuration
    Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = match Config::from_file(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            // Can't use tracing here because it's not initialized yet.
            eprintln!("Failed to load configuration: {e}");
            return Err(anyhow::anyhow!("Configuration error: {}", e));
        }
    };

    // Initialize logging
    let log_level = match config.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    info!("onebox-client starting up...");
    info!("Configuration loaded from {}", cli.config);

    match cli.command {
        Commands::Start { foreground } => {
            info!("Starting onebox client...");
            if foreground {
                info!("Running in foreground mode");
            }

            // The server address to connect to.
            let server_addr_str = format!(
                "{}:{}",
                config.client.server_address, config.client.server_port
            );
            let server_addr: SocketAddr = server_addr_str.parse()?;
            info!("Will connect to server at {}", server_addr);

            // Discover and bind sockets to all available WAN interfaces FIRST, while routing is normal.
            let all_sockets = Arc::new(discover_and_bind_sockets(server_addr).await?);

            if all_sockets.is_empty() {
                // discover_and_bind_sockets already logs an error, but we should exit.
                return Err(anyhow::anyhow!("No sockets bound, cannot proceed."));
            }

            let tun_ip: Ipv4Addr = config.client.tun_ip.parse()?;
            let tun_netmask: Ipv4Addr = config.client.tun_netmask.parse()?;
            let tun_name = &config.client.tun_name;

            info!("Creating TUN device '{}'...", tun_name);
            let tun = TunBuilder::new()
                .name(tun_name)
                .tap(false)
                .packet_info(false)
                .up()
                .address(tun_ip)
                .netmask(tun_netmask)
                .try_build()
                .map_err(|e| anyhow::anyhow!(e))?;

            info!("TUN device created. Setting as default route...");
            set_default_route(tun_name)?;

            if all_sockets.is_empty() {
                // discover_and_bind_sockets already logs an error, but we should exit.
                return Err(anyhow::anyhow!("No sockets bound, cannot proceed."));
            }

            // Create a new dynamically managed list of active sockets, initially containing all sockets.
            let active_sockets = Arc::new(RwLock::new(all_sockets.to_vec()));

            // Derive the encryption key from the PSK
            let key = derive_key(&config.preshared_key);
            let key = Arc::new(key);

            // Perform handshake on the first socket to establish the session
            let client_id = ClientId(1); // Define a consistent client ID
            let (iface_name, handshake_socket) = all_sockets.first().unwrap();
            info!("Performing handshake over interface '{}'", iface_name);
            perform_handshake(handshake_socket, &key, client_id).await?;

            info!("Handshake complete. Starting data plane...");

            // Create a shared state for link health statistics
            let link_stats = Arc::new(Mutex::new(HashMap::<String, health::LinkStats>::new()));

            // Spawn a health checker for each link
            for (iface_name, socket) in all_sockets.iter() {
                // Initialize stats for this link
                link_stats
                    .lock()
                    .await
                    .insert(iface_name.clone(), health::LinkStats::new());

                let prober_socket = socket.clone();
                let prober_key = key.clone();
                let prober_stats = link_stats.clone();
                let prober_iface_name = iface_name.clone();
                let prober_active_sockets = active_sockets.clone();

                tokio::spawn(async move {
                    const PROBE_INTERVAL: Duration = Duration::from_secs(2);
                    const PROBE_TIMEOUT: Duration = Duration::from_secs(2);
                    let mut interval = tokio::time::interval(PROBE_INTERVAL);

                    loop {
                        interval.tick().await;

                        let mut should_mark_down = false;
                        let seq;

                        // --- Start of stats lock ---
                        let mut stats_guard = prober_stats.lock().await;
                        if let Some(stats) = stats_guard.get_mut(&prober_iface_name) {
                            // 1. Check for timeouts
                            let now = std::time::Instant::now();
                            let timed_out_probes: Vec<u64> = stats
                                .in_flight_probes
                                .iter()
                                .filter(|(_, &sent_at)| now.duration_since(sent_at) > PROBE_TIMEOUT)
                                .map(|(&seq, _)| seq)
                                .collect();

                            for probe_seq in timed_out_probes {
                                stats.in_flight_probes.remove(&probe_seq);
                                stats.consecutive_failures += 1;
                                warn!(
                                    "Probe timeout on {} (seq={}), consecutive failures: {}",
                                    prober_iface_name, probe_seq, stats.consecutive_failures
                                );
                            }

                            // 2. Check if link should be marked as Down
                            if stats.consecutive_failures >= health::MAX_CONSECUTIVE_FAILURES
                                && stats.status != health::LinkStatus::Down
                            {
                                stats.status = health::LinkStatus::Down;
                                should_mark_down = true; // Signal the change
                            }

                            // 3. Prepare for next probe
                            seq = stats.next_probe_seq;
                            stats.next_probe_seq = stats.next_probe_seq.wrapping_add(1);
                        } else {
                            error!("Could not find stats for iface {}", prober_iface_name);
                            continue;
                        }
                        drop(stats_guard);
                        // --- End of stats lock ---

                        // 4. Update active links list if necessary
                        if should_mark_down {
                            warn!(
                                "Link {} marked as DOWN. Removing from active pool.",
                                prober_iface_name
                            );
                            let mut active_links_guard = prober_active_sockets.write().await;
                            active_links_guard.retain(|(name, _)| name != &prober_iface_name);
                            info!(
                                "Link {} removed. Active links: {}",
                                prober_iface_name,
                                active_links_guard.len()
                            );
                        }

                        // 5. Construct and send the probe packet
                        let probe_header = PacketHeader::new(seq, PacketType::Probe, client_id);
                        let header_bytes = bincode::serialize(&probe_header).unwrap();
                        let encrypted_payload =
                            encrypt(&prober_key, b"", probe_header.sequence_number).unwrap();
                        let probe_packet =
                            [header_bytes.as_slice(), encrypted_payload.as_slice()].concat();
                        let sent_at = std::time::Instant::now();

                        if prober_socket.send(&probe_packet).await.is_ok() {
                            // Re-acquire lock briefly to update sent stats
                            let mut stats_guard = prober_stats.lock().await;
                            if let Some(stats) = stats_guard.get_mut(&prober_iface_name) {
                                stats.probes_sent += 1;
                                stats.in_flight_probes.insert(seq, sent_at);
                            }
                        } else {
                            error!("Failed to send probe on {}", prober_iface_name);
                        }
                    }
                });
            }

            let round_robin_counter = Arc::new(AtomicUsize::new(0));
            let sequence_number = Arc::new(AtomicU64::new(0)); // Upstream sequence number

            let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);

            // Task 1: Read from TUN, encrypt, prepend header, and send using the dynamic active links list
            let tun_to_udp_active_sockets = active_sockets.clone();
            let tun_to_udp_counter = round_robin_counter.clone();
            let tun_to_udp_seq = sequence_number.clone();
            let tun_to_udp_key = key.clone();
            let tun_to_udp = tokio::spawn(async move {
                // Optimized buffer handling to avoid allocations in the hot path.
                const MTU: usize = 1500;
                const HEADER_SIZE: usize = PacketHeader::size();
                const TAG_SIZE: usize = 16; // ChaCha20-Poly1305 tag size
                let mut packet_buf = [0u8; HEADER_SIZE + MTU + TAG_SIZE];

                loop {
                    // The payload part of the buffer where TUN data will be read into.
                    let payload_buf = &mut packet_buf[HEADER_SIZE..];

                    match tun_reader.read(payload_buf).await {
                        Ok(plaintext_len) => {
                            if plaintext_len == 0 {
                                continue;
                            }

                            let seq = tun_to_udp_seq.fetch_add(1, Ordering::Relaxed);

                            // Encrypt the payload in-place.
                            let ciphertext_len = match encrypt_in_place(
                                &tun_to_udp_key,
                                payload_buf,
                                plaintext_len,
                                seq,
                            ) {
                                Ok(len) => len,
                                Err(e) => {
                                    error!("In-place encryption failed: {}", e);
                                    continue;
                                }
                            };

                            // Create the header and serialize it into the start of the buffer.
                            let header =
                                PacketHeader::new(seq, PacketType::Data, client_id);
                            if let Err(e) =
                                bincode::serialize_into(&mut packet_buf[..HEADER_SIZE], &header)
                            {
                                error!("Header serialization failed: {}", e);
                                continue;
                            }

                            // The final packet to send includes the header and the encrypted payload.
                            let packet_to_send = &packet_buf[..HEADER_SIZE + ciphertext_len];

                            // Get the current list of active links
                            let active_links_guard = tun_to_udp_active_sockets.read().await;

                            if active_links_guard.is_empty() {
                                warn!("No active links available to send packet. Waiting...");
                                drop(active_links_guard);
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }

                            let index = tun_to_udp_counter.fetch_add(1, Ordering::Relaxed)
                                % active_links_guard.len();
                            let (iface_name, socket) = &active_links_guard[index];

                            // Clone the socket to move it out of the lock guard before await
                            let socket_clone = socket.clone();
                            let iface_name_clone = iface_name.clone();
                            drop(active_links_guard);

                            if let Err(e) = socket_clone.send(packet_to_send).await {
                                error!("Error sending to UDP via {}: {}", iface_name_clone, e);
                            }
                        }
                        Err(e) => {
                            error!("TUN read error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Downstream path: Read from all sockets, decrypt, and write to a single TUN writer
            const DOWNSTREAM_BUF_SIZE: usize = 2048;
            let (tx, mut rx) =
                tokio::sync::mpsc::channel::<(usize, [u8; DOWNSTREAM_BUF_SIZE], String)>(1024);
            let downstream_key = key.clone();
            let downstream_active_sockets = active_sockets.clone();
            let downstream_all_sockets = all_sockets.clone();

            for (iface_name, socket) in all_sockets.iter() {
                let tx_clone = tx.clone();
                let iface_name_clone = iface_name.clone();
                let socket_clone = socket.clone();
                tokio::spawn(async move {
                    let mut buf = [0u8; DOWNSTREAM_BUF_SIZE];
                    loop {
                        match socket_clone.recv(&mut buf).await {
                            Ok(len) => {
                                if tx_clone.send((len, buf, iface_name_clone.clone())).await.is_err() {
                                    error!(
                                        "MPSC channel closed, cannot send packet from {}",
                                        iface_name_clone
                                    );
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("UDP receive error on {}: {}", iface_name_clone, e);
                                break;
                            }
                        }
                    }
                });
            }
            drop(tx);

            // Task 2: Read from MPSC, parse header, decrypt payload, write to TUN
            let udp_to_tun_stats = link_stats.clone();
            let udp_to_tun = tokio::spawn(async move {
                while let Some((len, mut packet_buf, iface_name)) = rx.recv().await {
                    let packet_with_header = &mut packet_buf[..len];

                    match bincode::deserialize::<PacketHeader>(packet_with_header) {
                        Ok(header) => {
                            // Handle Probe packets (echoes from the server)
                            if header.packet_type == PacketType::Probe {
                                let mut should_mark_up = false;

                                // --- Start of stats lock ---
                                let mut stats_guard = udp_to_tun_stats.lock().await;
                                if let Some(stats) = stats_guard.get_mut(&iface_name) {
                                    if let Some(sent_at) =
                                        stats.in_flight_probes.remove(&header.sequence_number)
                                    {
                                        stats.probes_received += 1;
                                        stats.rtt = sent_at.elapsed();
                                        stats.consecutive_failures = 0; // Reset on success

                                        if stats.status != health::LinkStatus::Up {
                                            stats.status = health::LinkStatus::Up;
                                            should_mark_up = true; // Signal recovery
                                        }
                                    } else {
                                        warn!("Received unexpected probe echo from {} (seq={}), no matching sent probe found.", iface_name, header.sequence_number);
                                    }
                                }
                                drop(stats_guard);
                                // --- End of stats lock ---

                                // Update active links list if the link has recovered
                                if should_mark_up {
                                    info!(
                                        "Link {} has recovered and is now UP. Adding back to active pool.",
                                        iface_name
                                    );
                                    if let Some(socket_to_add) = downstream_all_sockets
                                        .iter()
                                        .find(|(name, _)| name == &iface_name)
                                    {
                                        let mut active_links_guard =
                                            downstream_active_sockets.write().await;
                                        if !active_links_guard
                                            .iter()
                                            .any(|(name, _)| name == &iface_name)
                                        {
                                            active_links_guard.push(socket_to_add.clone());
                                            info!(
                                                "Link {} re-added. Active links: {}",
                                                iface_name,
                                                active_links_guard.len()
                                            );
                                        }
                                    }
                                }
                                continue; // Probes are not forwarded to TUN
                            }

                            if header.packet_type != PacketType::Data {
                                warn!(
                                    "Ignoring non-data, non-probe packet: {:?}",
                                    header.packet_type
                                );
                                continue;
                            }

                            // Handle Data packets
                            let header_size =
                                bincode::serialized_size(&header).unwrap_or(0) as usize;
                            if packet_with_header.len() < header_size {
                                error!("Runt packet received, smaller than header size.");
                                continue;
                            }
                            let ciphertext_buf = &mut packet_with_header[header_size..];

                            match decrypt_in_place(&downstream_key, ciphertext_buf, header.sequence_number) {
                                Ok(plaintext) => {
                                    if let Err(e) = tun_writer.write_all(plaintext).await {
                                        error!("Error writing to TUN: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    warn!("Packet decryption failed (likely auth error): {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize packet header: {}", e);
                        }
                    }
                }
                info!("MPSC channel closed. UDP->TUN task finished.");
            });

            tokio::select! {
                _ = tun_to_udp => info!("TUN->UDP task finished."),
                _ = udp_to_tun => info!("UDP->TUN task finished."),
            };
        }

        Commands::Stop => {
            info!("Stopping onebox client...");
            // TODO: Implement client stop logic
            info!("Client stop not yet implemented");
        }

        Commands::Status => {
            info!("Showing client status...");
            // Per SRS UI-3, display a real-time table of all detected WAN links.
            // For T3, we will display a static, placeholder table.
            println!(
                "{:<15} {:<10} {:<15} {:<10} {:<20}",
                "Link", "Status", "Latency (ms)", "Loss (%)", "Throughput (kbps)"
            );
            println!(
                "{:-<15} {:-<10} {:-<15} {:-<10} {:-<20}",
                "", "", "", "", ""
            );
            println!(
                "{:<15} {:<10} {:<15} {:<10} {:<20}",
                "WAN1 (eth0)", "Up", "25", "0.1", "50000"
            );
            println!(
                "{:<15} {:<10} {:<15} {:<10} {:<20}",
                "WAN2 (wlan0)", "Up", "42", "0.3", "25000"
            );
            println!(
                "{:<15} {:<10} {:<15} {:<10} {:<20}",
                "WAN3 (wwan0)", "Down", "-", "-", "-"
            );
        }

        Commands::Config => {
            info!("Showing client configuration...");
            println!("Configuration loaded from: {}", &cli.config);
            println!("{config:#?}");
        }
    }

    info!("onebox-client operation completed");
    Ok(())
}

/// Sets the specified TUN device as the default route for all traffic.
/// This is a critical step to ensure that network traffic is intercepted by our application.
fn set_default_route(tun_name: &str) -> anyhow::Result<()> {
    info!("Setting default route to {}", tun_name);
    // These commands are equivalent to:
    // 1. `ip route add 0.0.0.0/1 dev <tun_name>`
    // 2. `ip route add 128.0.0.0/1 dev <tun_name>`
    // This overrides the existing default route by creating two more specific routes
    // that cover the entire IPv4 address space.
    let commands = [
        vec!["route", "add", "0.0.0.0/1", "dev", tun_name],
        vec!["route", "add", "128.0.0.0/1", "dev", tun_name],
    ];

    for args in &commands {
        let status = Command::new("ip")
            .args(args)
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to execute 'ip route' command: {}", e))?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Failed to set default route part: 'ip {}'",
                args.join(" ")
            ));
        }
    }

    info!("Default route successfully set to {}", tun_name);
    Ok(())
}
