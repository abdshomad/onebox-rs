//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use nix::sys::socket::{setsockopt, sockopt::BindToDevice};
use onebox_core::packet::{PacketHeader, PacketType};
use onebox_core::prelude::*;
use onebox_core::types::ClientId;
use std::ffi::OsString;
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio_tun::TunBuilder;
use chacha20poly1305::Key;
use tracing::{debug, error, info, Level, warn};

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
        match tokio::time::timeout(std::time::Duration::from_secs(2), socket.recv(&mut recv_buf)).await {
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
        // Skip loopback, virtual interfaces, and interfaces that are down.
        if iface.name.starts_with("lo")
            || iface.name.contains("docker")
            || iface.name.contains("veth")
        {
            debug!("Skipping interface {}: loopback/virtual", iface.name);
            continue;
        }

        for addr in &iface.addr {
            let ip_addr = addr.ip();
            if ip_addr.is_ipv4() {
                let ipv4 = match ip_addr {
                    std::net::IpAddr::V4(ip) => ip,
                    _ => continue,
                };

                // Per FR-C-03, we need public or CGNAT addresses.
                // We will filter out private and link-local addresses.
                let is_private = ipv4.is_private();
                let is_link_local = ipv4.is_link_local();
                // CGNAT range is 100.64.0.0/10
                let is_cgnat =
                    ipv4.octets()[0] == 100 && (ipv4.octets()[1] >= 64 && ipv4.octets()[1] <= 127);

                if !is_private && !is_link_local || is_cgnat {
                    info!(
                        "Found potential WAN interface {} with IP {}",
                        iface.name, ipv4
                    );

                    // Bind a socket to 0.0.0.0:0 to let the OS choose the port
                    let socket = UdpSocket::bind("0.0.0.0:0").await?;

                    // Per SI-1 and FR-C-04, bind this socket to the specific device
                    let device_name_str = iface.name.as_str();
                    let device_name = OsString::from(device_name_str);
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

                    // We only need one socket per interface, so we can break inner loop
                    break;
                }
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

            // The server address to connect to.
            let server_addr_str = format!(
                "{}:{}",
                config.client.server_address, config.client.server_port
            );
            let server_addr: SocketAddr = server_addr_str.parse()?;
            info!("Will connect to server at {}", server_addr);

            // Discover and bind sockets to all available WAN interfaces.
            let all_sockets = Arc::new(discover_and_bind_sockets(server_addr).await?);

            if all_sockets.is_empty() {
                // discover_and_bind_sockets already logs an error, but we should exit.
                return Err(anyhow::anyhow!("No sockets bound, cannot proceed."));
            }

            // Derive the encryption key from the PSK
            let key = derive_key(&config.preshared_key);
            let key = Arc::new(key);

            // Perform handshake on the first socket to establish the session
            let (iface_name, handshake_socket) = all_sockets.first().unwrap();
            info!("Performing handshake over interface '{}'", iface_name);
            perform_handshake(handshake_socket, &key, ClientId(1)).await?; // Using ClientId(1) for now

            info!("Handshake complete. Starting data plane...");

            let round_robin_counter = Arc::new(AtomicUsize::new(0));
            let sequence_number = Arc::new(AtomicU64::new(0)); // Upstream sequence number

            let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);

            // Task 1: Read from TUN, encrypt, prepend header, and send
            let tun_to_udp_sockets = all_sockets.clone();
            let tun_to_udp_counter = round_robin_counter.clone();
            let tun_to_udp_seq = sequence_number.clone();
            let tun_to_udp_key = key.clone();
            let tun_to_udp = tokio::spawn(async move {
                let mut tun_buf = [0u8; 2048];
                loop {
                    match tun_reader.read(&mut tun_buf).await {
                        Ok(len) => {
                            if len == 0 {
                                continue;
                            }

                            let seq = tun_to_udp_seq.fetch_add(1, Ordering::Relaxed);
                            let header =
                                PacketHeader::new(seq, PacketType::Data, ClientId::default());

                            // Encrypt the payload
                            let plaintext = &tun_buf[..len];
                            let ciphertext = match encrypt(&tun_to_udp_key, plaintext, seq) {
                                Ok(ct) => ct,
                                Err(e) => {
                                    error!("Encryption failed: {}", e);
                                    continue;
                                }
                            };

                            let header_bytes = bincode::serialize(&header).unwrap();
                            let packet_with_header = [&header_bytes[..], &ciphertext[..]].concat();

                            let index = tun_to_udp_counter.fetch_add(1, Ordering::Relaxed)
                                % tun_to_udp_sockets.len();
                            let (iface_name, socket) = &tun_to_udp_sockets[index];

                            debug!(
                                "Seq {}, sending {} encrypted bytes via {}",
                                seq,
                                packet_with_header.len(),
                                iface_name
                            );
                            if let Err(e) = socket.send(&packet_with_header).await {
                                error!("Error sending to UDP via {}: {}", iface_name, e);
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
            let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1024);
            let downstream_key = key.clone();

            for (iface_name, socket) in all_sockets.iter() {
                let tx_clone = tx.clone();
                let iface_name_clone = iface_name.clone();
                let socket_clone = socket.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 2048];
                    loop {
                        match socket_clone.recv(&mut buf).await {
                            Ok(len) => {
                                if tx_clone.send(buf[..len].to_vec()).await.is_err() {
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
            let udp_to_tun = tokio::spawn(async move {
                while let Some(packet_with_header) = rx.recv().await {
                    match bincode::deserialize::<PacketHeader>(&packet_with_header) {
                        Ok(header) => {
                            if header.packet_type != PacketType::Data {
                                // This is not a data packet, so we ignore it in the data plane.
                                // It might be a probe response, etc., to be handled elsewhere later.
                                continue;
                            }

                            let header_size =
                                bincode::serialized_size(&header).unwrap_or(0) as usize;
                            if packet_with_header.len() < header_size {
                                error!("Runt packet received, smaller than header size.");
                                continue;
                            }
                            let ciphertext = &packet_with_header[header_size..];

                            match decrypt(&downstream_key, ciphertext, header.sequence_number) {
                                Ok(plaintext) => {
                                    if let Err(e) = tun_writer.write_all(&plaintext).await {
                                        error!("Error writing to TUN: {}", e);
                                        break;
                                    }
                                    debug!(
                                        "Decrypted and wrote {} bytes to TUN",
                                        plaintext.len()
                                    );
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
