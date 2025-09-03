//! onebox-server - Server binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::packet::PacketHeader;
use onebox_core::packet::PacketType;
use onebox_core::prelude::*;
use onebox_core::types::ClientId;
use std::collections::{BTreeMap, HashMap};
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio_tun::TunBuilder;
use tracing::{error, info, warn, Level};

#[derive(Parser)]
#[command(name = "onebox-server")]
#[command(about = "Server for onebox-rs internet bonding solution")]
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
    /// Start the server
    Start {
        /// Run in foreground (don't daemonize)
        #[arg(short, long)]
        foreground: bool,

        /// Bind address override
        #[arg(short, long)]
        bind: Option<String>,
    },

    /// Stop the server
    Stop,

    /// Show server status
    Status,

    /// Show server configuration
    Config,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthStatus {
    Pending,
    Authenticated,
}

struct ClientState {
    auth_status: AuthStatus,
    jitter_buffer: BTreeMap<u64, Vec<u8>>,
    next_seq: u64,
    last_seen_addr: SocketAddr,
}

impl ClientState {
    fn new(addr: SocketAddr) -> Self {
        Self {
            auth_status: AuthStatus::Pending,
            jitter_buffer: BTreeMap::new(),
            next_seq: 0,
            last_seen_addr: addr,
        }
    }
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

    info!("onebox-server starting up...");
    info!("Configuration loaded from {}", cli.config);

    match cli.command {
        Commands::Start { foreground, bind } => {
            info!("Starting onebox server...");
            if foreground {
                info!("Running in foreground mode");
            }

            // Determine bind address (override takes precedence)
            let bind_addr_str = bind.unwrap_or_else(|| {
                format!(
                    "{}:{}",
                    config.server.listen_address, config.server.listen_port
                )
            });

            let bind_addr: SocketAddr = match bind_addr_str.parse() {
                Ok(addr) => {
                    info!("Binding to address: {}", addr);
                    addr
                }
                Err(e) => {
                    error!("Invalid bind address '{}': {}", bind_addr_str, e);
                    return Err(anyhow::anyhow!("Invalid bind address"));
                }
            };

            let tun_ip: Ipv4Addr = "10.99.99.1"
                .parse()
                .expect("Failed to parse TUN IP address");
            let tun_netmask: Ipv4Addr = "255.255.255.0"
                .parse()
                .expect("Failed to parse TUN netmask");

            info!("Creating TUN device 'onebox0'...");
            let tun = match TunBuilder::new()
                .name("onebox0")
                .tap(false) // Use TUN mode
                .packet_info(false) // No extra packet info header
                .up() // Bring the interface up
                .address(tun_ip)
                .netmask(tun_netmask)
                .try_build()
            {
                Ok(tun) => {
                    info!("TUN device 'onebox0' created successfully.");
                    info!("IP: 10.99.99.1, Netmask: 255.255.255.0");
                    tun
                }
                Err(e) => {
                    error!("Failed to create TUN device: {}", e);
                    return Err(anyhow::anyhow!("TUN creation failed: {}", e));
                }
            };

            // Enable IP forwarding
            info!("Enabling IP forwarding...");
            let output = Command::new("sysctl")
                .arg("-w")
                .arg("net.ipv4.ip_forward=1")
                .output()
                .map_err(|e| anyhow::anyhow!("Failed to execute sysctl: {}", e))?;

            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr);
                error!("Failed to enable IP forwarding: {}", error_message);
                return Err(anyhow::anyhow!(
                    "Failed to enable IP forwarding: {}",
                    error_message
                ));
            }
            info!("IP forwarding enabled successfully.");

            // Set up NAT masquerading
            let default_iface = match get_default_interface() {
                Ok(iface) => iface,
                Err(e) => {
                    error!("Could not get default network interface: {}", e);
                    return Err(e);
                }
            };

            setup_nat_masquerade(&default_iface, "10.99.99.0/24")?;

            // Bind UDP socket and log incoming datagrams
            let socket = UdpSocket::bind(bind_addr).await.map_err(|e| {
                error!("Failed to bind UDP socket on {}: {}", bind_addr, e);
                anyhow::anyhow!(e)
            })?;

            info!("UDP server listening on {}", bind_addr);

            // Derive the encryption key from the PSK
            let key = derive_key(&config.preshared_key);
            let key = Arc::new(key);

            // Split TUN device into reader and writer
            let (mut tun_reader, tun_writer) = tokio::io::split(tun);

            // This HashMap will store the state for each connected client, keyed by ClientId.
            let clients = Arc::new(Mutex::new(HashMap::<ClientId, ClientState>::new()));

            // Create Arc for the socket to share between tasks
            let socket = Arc::new(socket);

            // Task 1: UDP -> TUN (Main logic loop for receiving from clients)
            let udp_to_tun_socket = socket.clone();
            let clients_writer = clients.clone();
            let tun_writer = Arc::new(Mutex::new(tun_writer));
            let decryption_key = key.clone();
            let encryption_key_clone = key.clone(); // For sending AuthResponse

            let udp_to_tun = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                while let Ok((len, peer)) = udp_to_tun_socket.recv_from(&mut buf).await {
                    // We can't get the ClientId without decrypting, which requires the key.
                    // This initial packet must be handled carefully. We'll assume AuthRequest for now.
                    let header = match bincode::deserialize::<PacketHeader>(&buf[..len]) {
                        Ok(h) => h,
                        Err(_) => continue, // Drop malformed packets
                    };

                    let header_size = bincode::serialized_size(&header).unwrap_or(0) as usize;
                    if len < header_size {
                        continue;
                    }
                    let ciphertext = &buf[header_size..len];

                    match decrypt(&decryption_key, ciphertext, header.sequence_number) {
                        Ok(plaintext) => {
                            let mut clients_guard = clients_writer.lock().await;
                            let client_state = clients_guard
                                .entry(header.client_id)
                                .or_insert_with(|| ClientState::new(peer));

                            // Update last seen address
                            client_state.last_seen_addr = peer;

                            match header.packet_type {
                                PacketType::AuthRequest => {
                                    info!(
                                        "Received AuthRequest from client {}, processing.",
                                        header.client_id.0
                                    );
                                    client_state.auth_status = AuthStatus::Authenticated;

                                    // Send AuthResponse
                                    let response_header = PacketHeader::new(
                                        0,
                                        PacketType::AuthResponse,
                                        header.client_id,
                                    );
                                    let response_header_bytes =
                                        bincode::serialize(&response_header).unwrap();
                                    let response_payload =
                                        encrypt(&encryption_key_clone, b"AUTH_OK", 0).unwrap();
                                    let response_packet =
                                        [&response_header_bytes[..], &response_payload[..]]
                                            .concat();

                                    if let Err(e) =
                                        udp_to_tun_socket.send_to(&response_packet, peer).await
                                    {
                                        error!("Failed to send AuthResponse to {}: {}", peer, e);
                                    }
                                }
                                PacketType::Data => {
                                    if client_state.auth_status != AuthStatus::Authenticated {
                                        warn!(
                                            "Dropping data packet from unauthenticated client {}",
                                            header.client_id.0
                                        );
                                        continue;
                                    }

                                    // Jitter buffer logic
                                    if header.sequence_number < client_state.next_seq {
                                        continue; // Old packet
                                    }
                                    if header.sequence_number > client_state.next_seq {
                                        client_state
                                            .jitter_buffer
                                            .insert(header.sequence_number, plaintext);
                                        continue;
                                    }

                                    let mut tun = tun_writer.lock().await;
                                    if tun.write_all(&plaintext).await.is_err() {
                                        break;
                                    }
                                    client_state.next_seq += 1;

                                    while let Some(p) =
                                        client_state.jitter_buffer.remove(&client_state.next_seq)
                                    {
                                        if tun.write_all(&p).await.is_err() {
                                            break;
                                        }
                                        client_state.next_seq += 1;
                                    }
                                }
                                PacketType::Probe => {
                                    if client_state.auth_status != AuthStatus::Authenticated {
                                        warn!(
                                            "Dropping probe packet from unauthenticated client {}",
                                            header.client_id.0
                                        );
                                        continue;
                                    }
                                    info!(
                                        "Received probe from client {}, echoing back.",
                                        header.client_id.0
                                    );
                                    // Echo the exact packet back to the sender for RTT measurement
                                    if let Err(e) =
                                        udp_to_tun_socket.send_to(&buf[..len], peer).await
                                    {
                                        error!("Failed to echo probe to {}: {}", peer, e);
                                    }
                                }
                                _ => warn!("Unhandled packet type: {:?}", header.packet_type),
                            }
                        }
                        Err(_) => {
                            warn!("Decryption failed for packet from {}", peer);
                        }
                    }
                }
            });

            // Task 2: TUN -> UDP (with Encryption)
            let tun_to_udp_socket = socket.clone();
            let clients_reader = clients.clone();
            let downstream_seq = Arc::new(AtomicU64::new(0));
            let encryption_key = key.clone();
            let tun_to_udp = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match tun_reader.read(&mut buf).await {
                        Ok(len) => {
                            if len == 0 {
                                continue;
                            }

                            // This simple routing logic sends to the first authenticated client.
                            // A proper implementation would map TUN IPs to client addresses.
                            let clients_guard = clients_reader.lock().await;
                            let peer_addr = clients_guard.values().find_map(|state| {
                                if state.auth_status == AuthStatus::Authenticated {
                                    Some(state.last_seen_addr)
                                } else {
                                    None
                                }
                            });

                            if let Some(peer_addr) = peer_addr {
                                let seq = downstream_seq.fetch_add(1, Ordering::Relaxed);
                                let header =
                                    PacketHeader::new(seq, PacketType::Data, ClientId::default());

                                let plaintext = &buf[..len];
                                let ciphertext = match encrypt(&encryption_key, plaintext, seq) {
                                    Ok(ct) => ct,
                                    Err(_) => continue,
                                };

                                let header_bytes = bincode::serialize(&header).unwrap();
                                let packet_to_send = [&header_bytes[..], &ciphertext[..]].concat();

                                if tun_to_udp_socket
                                    .send_to(&packet_to_send, peer_addr)
                                    .await
                                    .is_err()
                                {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            error!("TUN read error: {}", e);
                            break;
                        }
                    }
                }
            });

            tokio::select! {
                _ = udp_to_tun => info!("UDP->TUN task finished."),
                _ = tun_to_udp => info!("TUN->UDP task finished."),
            }
        }

        Commands::Stop => {
            info!("Stopping onebox server...");
            // TODO: Implement server stop logic
            info!("Server stop not yet implemented");
        }

        Commands::Status => {
            info!("Showing server status...");
            // Per SRS UI-5, display a list of connected clients.
            // For T3, we will display a static, placeholder list.
            println!("Connected Clients");
            println!("{:-<17}", "");
            println!(
                "{:<17} {:<22} {:<30}",
                "Client ID", "Source IP", "Aggregated Throughput (kbps)"
            );
            println!("{:-<17} {:-<22} {:-<30}", "", "", "");
            println!(
                "{:<17} {:<22} {:<30}",
                "client-1234", "198.51.100.10:12345", "75000"
            );
        }

        Commands::Config => {
            info!("Showing server configuration...");
            println!("Configuration loaded from: {}", &cli.config);
            println!("{config:#?}");
        }
    }

    info!("onebox-server operation completed");
    Ok(())
}

/// Finds the default network interface of the system.
fn get_default_interface() -> anyhow::Result<String> {
    info!("Querying for default network interface...");
    let output = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute 'ip route': {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Failed to get default route: {}",
            error_message
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    // Example output: "8.8.8.8 via 192.168.1.1 dev enp0s3 src 192.168.1.100 uid 0"
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    if let Some(dev_idx) = parts.iter().position(|&r| r == "dev") {
        if let Some(iface) = parts.get(dev_idx + 1) {
            info!("Found default interface: {}", iface);
            return Ok(iface.to_string());
        }
    }

    Err(anyhow::anyhow!(
        "Could not parse default interface from 'ip route' output"
    ))
}

/// Sets up NAT masquerading using iptables.
fn setup_nat_masquerade(interface: &str, source_net: &str) -> anyhow::Result<()> {
    info!(
        "Setting up NAT masquerade on {} for source {}",
        interface, source_net
    );
    // Flush all existing NAT rules to ensure a clean state.
    let flush_status = Command::new("iptables")
        .args(["-t", "nat", "-F"])
        .status()?;
    if !flush_status.success() {
        return Err(anyhow::anyhow!("Failed to flush iptables NAT table"));
    }
    info!("Flushed iptables NAT table.");

    let add_rule_args = [
        "-t",
        "nat",
        "-A",
        "POSTROUTING",
        "-s",
        source_net,
        "-o",
        interface,
        "-j",
        "MASQUERADE",
    ];

    let add_output = Command::new("iptables")
        .args(add_rule_args)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute iptables: {}", e))?;

    if !add_output.status.success() {
        let error_message = String::from_utf8_lossy(&add_output.stderr);
        error!("Failed to add iptables MASQUERADE rule: {}", error_message);
        return Err(anyhow::anyhow!(
            "Failed to add iptables rule: {}",
            error_message
        ));
    }

    info!("iptables MASQUERADE rule set successfully.");
    Ok(())
}
