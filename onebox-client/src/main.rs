//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
pub mod health;
use chacha20poly1305::Key;
use health::LinkStats;
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
use tokio::net::{UnixListener, UnixStream, UdpSocket};
use tokio::sync::{Mutex, RwLock};
use tokio_tun::TunBuilder;
use tracing::{debug, error, info, warn, Level};

const STATUS_SOCKET_PATH: &str = "/tmp/onebox_status.sock";

async fn perform_handshake(
    socket: &UdpSocket,
    key: &Key,
    client_id: ClientId,
) -> anyhow::Result<()> {
    info!("Performing handshake...");
    let auth_request_header = PacketHeader::new(0, PacketType::AuthRequest, client_id);
    let header_bytes = bincode::serialize(&auth_request_header)?;
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

async fn discover_and_bind_sockets(
    server_addr: SocketAddr,
) -> anyhow::Result<Vec<(String, Arc<UdpSocket>)>> {
    info!("Discovering WAN interfaces and binding sockets...");
    let mut sockets = Vec::new();
    let ifaces = NetworkInterface::show()?;

    for iface in ifaces {
        debug!("Processing interface: {:?}", iface);
        if iface.name.starts_with("lo")
            || iface.name.contains("docker")
            || iface.name.starts_with("onebox")
        {
            debug!("Skipping interface {}: loopback/virtual", iface.name);
            continue;
        }

        for addr in &iface.addr {
            if let std::net::IpAddr::V4(ipv4) = addr.ip() {
                info!(
                    "Found potential WAN interface {} with IP {}",
                    iface.name, ipv4
                );
                let socket = UdpSocket::bind("0.0.0.0:0").await?;
                let device_name = OsString::from(iface.name.as_str());
                if let Err(e) = setsockopt(&socket, BindToDevice, &device_name) {
                    error!("Failed to bind socket to device {}: {}", iface.name, e);
                    continue;
                }
                info!("Successfully bound UDP socket to device {}", iface.name);
                socket.connect(server_addr).await?;
                info!(
                    "Socket for {} connected to server at {}",
                    iface.name, server_addr
                );
                sockets.push((iface.name.clone(), Arc::new(socket)));
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
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,
    Status,
    Config,
}

async fn handle_status_connection(
    mut stream: UnixStream,
    link_stats: Arc<Mutex<HashMap<String, LinkStats>>>,
) -> anyhow::Result<()> {
    let stats = link_stats.lock().await;
    let mut response = String::new();
    response.push_str(&format!(
        "{:<15} {:<10} {:<15} {:<10}\n",
        "Link", "Status", "Latency (ms)", "Loss (%)"
    ));
    response.push_str(&format!(
        "{:-<15} {:-<10} {:-<15} {:-<10}\n",
        "", "", "", ""
    ));

    for (name, stats) in stats.iter() {
        let status_str = format!("{:?}", stats.status);
        let rtt_str = if stats.status == health::LinkStatus::Up {
            format!("{:.2}", stats.rtt.as_secs_f32() * 1000.0)
        } else {
            "-".to_string()
        };
        let loss_str = format!("{:.2}", stats.packet_loss_percent());
        response.push_str(&format!(
            "{:<15} {:<10} {:<15} {:<10}\n",
            name, status_str, rtt_str, loss_str
        ));
    }

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn handle_probe_response(
    header: &PacketHeader,
    iface_name: &str,
    stats_mutex: Arc<Mutex<HashMap<String, LinkStats>>>,
    all_sockets: &Arc<Vec<(String, Arc<UdpSocket>)>>,
    active_sockets: &Arc<RwLock<Vec<(String, Arc<UdpSocket>)>>>,
) {
    let mut should_mark_up = false;
    let mut stats_guard = stats_mutex.lock().await;
    if let Some(stats) = stats_guard.get_mut(iface_name) {
        if let Some(sent_at) = stats.in_flight_probes.remove(&header.sequence_number) {
            stats.probes_received += 1;
            stats.rtt = sent_at.elapsed();
            stats.consecutive_failures = 0;
            if stats.status != health::LinkStatus::Up {
                stats.status = health::LinkStatus::Up;
                should_mark_up = true;
            }
        }
    }
    drop(stats_guard);

    if should_mark_up {
        info!(
            "Link {} has recovered and is now UP. Adding back to active pool.",
            iface_name
        );
        if let Some(socket_to_add) = all_sockets.iter().find(|(name, _)| name == iface_name) {
            let mut active_links_guard = active_sockets.write().await;
            if !active_links_guard.iter().any(|(name, _)| name == iface_name) {
                active_links_guard.push(socket_to_add.clone());
                info!(
                    "Link {} re-added. Active links: {}",
                    iface_name,
                    active_links_guard.len()
                );
            }
        }
    }
}

async fn handle_data_packet(
    header: &PacketHeader,
    packet_buf: &mut [u8],
    len: usize,
    key: &Key,
    tun_writer: &mut tokio::io::WriteHalf<tokio_tun::Tun>,
) -> anyhow::Result<()> {
    let header_size = bincode::serialized_size(header).unwrap_or(0) as usize;
    if len < header_size {
        return Err(anyhow::anyhow!("Packet too small for header"));
    }
    let ciphertext_buf = &mut packet_buf[header_size..len];
    if let Ok(plaintext) = decrypt_in_place(key, ciphertext_buf, header.sequence_number) {
        if tun_writer.write_all(plaintext).await.is_err() {
            return Err(anyhow::anyhow!("Failed to write to TUN device"));
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = match Config::from_file(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {e}");
            return Err(anyhow::anyhow!("Configuration error: {}", e));
        }
    };

    match cli.command {
        Commands::Start => {
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
            info!("Starting onebox client...");

            let server_addr_str = format!(
                "{}:{}",
                config.client.server_address, config.client.server_port
            );
            let server_addr: SocketAddr = server_addr_str.parse()?;
            info!("Will connect to server at {}", server_addr);

            let all_sockets = Arc::new(discover_and_bind_sockets(server_addr).await?);
            if all_sockets.is_empty() {
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

            let active_sockets = Arc::new(RwLock::new(all_sockets.to_vec()));
            let key = Arc::new(derive_key(&config.preshared_key));
            let client_id = ClientId(1);
            let (iface_name, handshake_socket) = all_sockets.first().unwrap();
            info!("Performing handshake over interface '{}'", iface_name);
            perform_handshake(handshake_socket, &key, client_id).await?;
            info!("Handshake complete. Starting data plane...");

            let link_stats = Arc::new(Mutex::new(HashMap::<String, health::LinkStats>::new()));

            let status_listener_stats = link_stats.clone();
            tokio::spawn(async move {
                let _ = tokio::fs::remove_file(STATUS_SOCKET_PATH).await;
                let listener = match UnixListener::bind(STATUS_SOCKET_PATH) {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Failed to bind status socket: {}", e);
                        return;
                    }
                };
                info!("Status socket listening on {}", STATUS_SOCKET_PATH);
                loop {
                    if let Ok((stream, _)) = listener.accept().await {
                        let stats_clone = status_listener_stats.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_status_connection(stream, stats_clone).await {
                                warn!("Error handling status connection: {}", e);
                            }
                        });
                    }
                }
            });

            for (iface_name, socket) in all_sockets.iter() {
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
                    const PROBE_INTERVAL: Duration = Duration::from_millis(500);
                    const PROBE_TIMEOUT: Duration = Duration::from_secs(2);
                    let mut interval = tokio::time::interval(PROBE_INTERVAL);
                    loop {
                        interval.tick().await;
                        let mut should_mark_down = false;
                        let seq;
                        let mut stats_guard = prober_stats.lock().await;
                        if let Some(stats) = stats_guard.get_mut(&prober_iface_name) {
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
                            if stats.consecutive_failures >= health::MAX_CONSECUTIVE_FAILURES
                                && stats.status != health::LinkStatus::Down
                            {
                                stats.status = health::LinkStatus::Down;
                                should_mark_down = true;
                            }
                            seq = stats.next_probe_seq;
                            stats.next_probe_seq = stats.next_probe_seq.wrapping_add(1);
                        } else {
                            error!("Could not find stats for iface {}", prober_iface_name);
                            continue;
                        }
                        drop(stats_guard);
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
                        let probe_header = PacketHeader::new(seq, PacketType::Probe, client_id);
                        let header_bytes = bincode::serialize(&probe_header).unwrap();
                        let encrypted_payload =
                            encrypt(&prober_key, b"", probe_header.sequence_number).unwrap();
                        let probe_packet =
                            [header_bytes.as_slice(), encrypted_payload.as_slice()].concat();
                        let sent_at = std::time::Instant::now();
                        if prober_socket.send(&probe_packet).await.is_ok() {
                            let mut stats_guard = prober_stats.lock().await;
                            if let Some(stats) = stats_guard.get_mut(&prober_iface_name) {
                                stats.probes_sent += 1;
                                stats.in_flight_probes.insert(seq, sent_at);
                            }
                        } else {
                            error!("Failed to send probe on {}", prober_iface_name);
                            let mut stats_guard = prober_stats.lock().await;
                            if let Some(stats) = stats_guard.get_mut(&prober_iface_name) {
                                stats.consecutive_failures += 1;
                                warn!(
                                    "Send failure on {} (seq={}), consecutive failures: {}",
                                    prober_iface_name, seq, stats.consecutive_failures
                                );
                            }
                        }
                    }
                });
            }

            let round_robin_counter = Arc::new(AtomicUsize::new(0));
            let sequence_number = Arc::new(AtomicU64::new(0));
            let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);
            let tun_to_udp_active_sockets = active_sockets.clone();
            let tun_to_udp_counter = round_robin_counter.clone();
            let tun_to_udp_seq = sequence_number.clone();
            let tun_to_udp_key = key.clone();
            let tun_to_udp = tokio::spawn(async move {
                const MTU: usize = 1500;
                const HEADER_SIZE: usize = PacketHeader::size();
                const TAG_SIZE: usize = 16;
                let mut packet_buf = [0u8; HEADER_SIZE + MTU + TAG_SIZE];

                loop {
                    let payload_buf = &mut packet_buf[HEADER_SIZE..];
                    match tun_reader.read(payload_buf).await {
                        Ok(0) | Err(_) => {
                            // End of stream or a read error, either way we can't proceed with this packet.
                            continue;
                        }
                        Ok(plaintext_len) => {
                            let seq = tun_to_udp_seq.fetch_add(1, Ordering::Relaxed);

                            // Encrypt the payload in place
                            let ciphertext_len = match encrypt_in_place(
                                &tun_to_udp_key,
                                payload_buf,
                                plaintext_len,
                                seq,
                            ) {
                                Ok(len) => len,
                                Err(e) => {
                                    warn!("Encryption failed: {}", e);
                                    continue; // Skip this packet
                                }
                            };

                            // Prepare the full packet (Header + Encrypted Payload)
                            let header = PacketHeader::new(seq, PacketType::Data, client_id);
                            bincode::serialize_into(&mut packet_buf[..HEADER_SIZE], &header)
                                .expect("Serialization into a fixed-size buffer should not fail");
                            let packet_to_send = &packet_buf[..HEADER_SIZE + ciphertext_len];

                            // Send the packet over a chosen link using round-robin
                            let active_links_guard = tun_to_udp_active_sockets.read().await;
                            if active_links_guard.is_empty() {
                                drop(active_links_guard);
                                warn!("No active links available to send data. Waiting...");
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }

                            let index = tun_to_udp_counter.fetch_add(1, Ordering::Relaxed)
                                % active_links_guard.len();
                            let (iface_name, socket) = &active_links_guard[index];

                            if let Err(e) = socket.send(packet_to_send).await {
                                warn!("Failed to send packet on {}: {}", iface_name, e);
                            }
                        }
                    }
                }
            });

            // Downstream Data Plane: UDP -> TUN
            // One task per socket to receive, a single task to process and write to TUN
            const DOWNSTREAM_BUF_SIZE: usize = 2048;
            let (tx, mut rx) =
                tokio::sync::mpsc::channel::<(usize, [u8; DOWNSTREAM_BUF_SIZE], String)>(1024);

            for (iface_name, socket) in all_sockets.iter() {
                let tx_clone = tx.clone();
                let iface_name_clone = iface_name.clone();
                let socket_clone = socket.clone();
                tokio::spawn(async move {
                    let mut buf = [0u8; DOWNSTREAM_BUF_SIZE];
                    while let Ok(len) = socket_clone.recv(&mut buf).await {
                        if tx_clone.send((len, buf, iface_name_clone.clone())).await.is_err() {
                            // Channel closed, receiver is gone.
                            break;
                        }
                    }
                });
            }
            drop(tx); // Drop the original sender so the channel closes when all clones are dropped.

            let udp_to_tun_stats = link_stats.clone();
            let downstream_key = key.clone();
            let downstream_active_sockets = active_sockets.clone();
            let downstream_all_sockets = all_sockets.clone();

            let udp_to_tun = tokio::spawn(async move {
                while let Some((len, mut packet_buf, iface_name)) = rx.recv().await {
                    if let Ok(header) = bincode::deserialize::<PacketHeader>(&packet_buf[..len]) {
                        match header.packet_type {
                            PacketType::Probe => {
                                handle_probe_response(
                                    &header,
                                    &iface_name,
                                    udp_to_tun_stats.clone(),
                                    &downstream_all_sockets,
                                    &downstream_active_sockets,
                                )
                                .await;
                            }
                            PacketType::Data => {
                                if let Err(e) = handle_data_packet(
                                    &header,
                                    &mut packet_buf,
                                    len,
                                    &downstream_key,
                                    &mut tun_writer,
                                )
                                .await
                                {
                                    warn!("Error handling data packet: {}. Stopping downstream task.", e);
                                    break; // Exit loop on critical error (e.g., TUN write failure)
                                }
                            }
                            _ => {
                                // Ignore other packet types like AuthRequest, etc.
                            }
                        }
                    }
                }
            });
            tokio::select! {
                _ = tun_to_udp => info!("TUN->UDP task finished."),
                _ = udp_to_tun => info!("UDP->TUN task finished."),
            };
        }
        Commands::Stop => info!("Client stop not yet implemented"),
        Commands::Status => {
            match UnixStream::connect(STATUS_SOCKET_PATH).await {
                Ok(mut stream) => {
                    let mut response = String::new();
                    stream.read_to_string(&mut response).await?;
                    print!("{}", response);
                }
                Err(_) => {
                    eprintln!("Could not get client status. Is the client running?");
                }
            }
        }
        Commands::Config => {
            println!("Configuration loaded from: {}", &cli.config);
            println!("{config:#?}");
        }
    }
    Ok(())
}

fn set_default_route(tun_name: &str) -> anyhow::Result<()> {
    info!("Setting default route to {}", tun_name);
    let commands = [
        vec!["route", "add", "0.0.0.0/1", "dev", tun_name],
        vec!["route", "add", "128.0.0.0/1", "dev", tun_name],
    ];
    for args in &commands {
        let status = Command::new("ip").args(args).status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to set default route"));
        }
    }
    info!("Default route successfully set to {}", tun_name);
    Ok(())
}
