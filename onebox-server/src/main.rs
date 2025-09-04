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
use tracing::{debug, error, info, warn, Level};

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
    next_seq: Option<u64>,
    last_seen_addr: SocketAddr,
}

impl ClientState {
    fn new(addr: SocketAddr) -> Self {
        Self {
            auth_status: AuthStatus::Pending,
            jitter_buffer: BTreeMap::new(),
            next_seq: None,
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

            info!("Ensuring old TUN device 'onebox0' is cleaned up...");
            let _ = std::process::Command::new("ip").args(["link", "delete", "onebox0"]).status(); // Ignore result
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

            // Add a route for the client's TUN network to go via our TUN device
            info!("Adding route for client TUN network (10.8.0.0/24)...");
            let route_add_output = Command::new("ip")
                .args(["route", "add", "10.8.0.0/24", "dev", "onebox0"])
                .output()
                .map_err(|e| anyhow::anyhow!("Failed to execute 'ip route add': {}", e))?;
            if !route_add_output.status.success() {
                error!(
                    "Failed to add route for client TUN: {}",
                    String::from_utf8_lossy(&route_add_output.stderr)
                );
                return Err(anyhow::anyhow!("Failed to add route for client TUN"));
            }

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
            match get_default_interface() {
                Ok(iface) => {
                    if let Err(e) = setup_nat_masquerade(&iface, "10.8.0.0/24") {
                        warn!("Failed to set up NAT masquerading: {}. This may be expected in some test environments.", e);
                    }
                }
                Err(e) => {
                    warn!("Could not get default network interface: {}. Skipping NAT setup. This is expected in CI/test environments without a default route.", e);
                }
            };

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

            // Task 1: UDP -> TUN (Worker Pool Model)
            let tun_writer = Arc::new(Mutex::new(tun_writer));
            let decryption_key = key.clone();
            let num_workers = num_cpus::get();
            info!("Spawning {} UDP->TUN worker tasks...", num_workers);

            let (tx, rx) = tokio::sync::mpsc::channel::<(Vec<u8>, SocketAddr)>(1024);
            let shared_rx = Arc::new(Mutex::new(rx));

            for i in 0..num_workers {
                let worker_rx = shared_rx.clone();
                let worker_clients = clients.clone();
                let worker_tun = tun_writer.clone();
                let worker_key = decryption_key.clone();
                let worker_socket = socket.clone();

                tokio::spawn(async move {
                    info!("Worker {} started", i);
                    loop {
                        // Lock the mutex to receive a packet, then immediately unlock
                        let packet_data = {
                            let mut rx_guard = worker_rx.lock().await;
                            rx_guard.recv().await
                        };

                        if let Some((buf, peer)) = packet_data {
                            debug!("[Worker {}] Received {} bytes from {}", i, buf.len(), peer);
                            let header = match bincode::deserialize::<PacketHeader>(&buf) {
                                Ok(h) => h,
                                Err(e) => {
                                    warn!("[Worker {}] Header deserialize failed from {}: {}. Size: {}. Dropping.", i, peer, e, buf.len());
                                    continue;
                                }
                            };

                            let header_size =
                                bincode::serialized_size(&header).unwrap_or(0) as usize;
                            if buf.len() < header_size {
                                continue;
                            }
                            let ciphertext = &buf[header_size..];

                            match decrypt(&worker_key, ciphertext, header.sequence_number) {
                                Ok(plaintext) => {
                                    let mut clients_guard = worker_clients.lock().await;
                                    let client_state = clients_guard
                                        .entry(header.client_id)
                                        .or_insert_with(|| ClientState::new(peer));

                                    client_state.last_seen_addr = peer;

                                    match header.packet_type {
                                        PacketType::AuthRequest => {
                                            info!(
                                                "[Worker {}] AuthRequest from client {}",
                                                i, header.client_id.0
                                            );
                                            client_state.auth_status = AuthStatus::Authenticated;

                                            let resp_header = PacketHeader::new(
                                                0,
                                                PacketType::AuthResponse,
                                                header.client_id,
                                            );
                                            let resp_header_bytes =
                                                bincode::serialize(&resp_header).unwrap();
                                            let resp_payload =
                                                encrypt(&worker_key, b"AUTH_OK", 0).unwrap();
                                            let resp_packet =
                                                [&resp_header_bytes[..], &resp_payload[..]]
                                                    .concat();

                                            if let Err(e) =
                                                worker_socket.send_to(&resp_packet, peer).await
                                            {
                                                error!(
                                                    "[Worker {}] Failed to send AuthResponse: {}",
                                                    i, e
                                                );
                                            }
                                        }
                                        PacketType::Data => {
                                            if client_state.auth_status != AuthStatus::Authenticated
                                            {
                                                continue; // Ignore data from unauthenticated clients
                                            }

                                            // Insert the packet into the jitter buffer.
                                            client_state
                                                .jitter_buffer
                                                .insert(header.sequence_number, plaintext);

                                            // If this is the first data packet, initialize the sequence number.
                                            if client_state.next_seq.is_none() {
                                                if let Some((&first_seq, _)) =
                                                    client_state.jitter_buffer.iter().next()
                                                {
                                                    client_state.next_seq = Some(first_seq);
                                                }
                                            }

                                            // Try to drain the jitter buffer.
                                            if let Some(mut current_seq) = client_state.next_seq {
                                                let mut tun = worker_tun.lock().await;
                                                while let Some(p) =
                                                    client_state.jitter_buffer.remove(&current_seq)
                                                {
                                                    if tun.write_all(&p).await.is_err() {
                                                        // If TUN write fails, stop and put the packet back.
                                                        client_state
                                                            .jitter_buffer
                                                            .insert(current_seq, p);
                                                        break;
                                                    }
                                                    current_seq += 1;
                                                }
                                                // Update the next expected sequence number.
                                                client_state.next_seq = Some(current_seq);
                                            }
                                        }
                                        PacketType::Probe => {
                                            if client_state.auth_status == AuthStatus::Authenticated
                                            {
                                                if let Err(e) =
                                                    worker_socket.send_to(&buf, peer).await
                                                {
                                                    error!(
                                                        "[Worker {}] Failed to echo probe: {}",
                                                        i, e
                                                    );
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    // Using warn level because this could be noisy if an attacker is sending junk packets,
                                    // but it's critical for debugging authentication/encryption issues.
                                    warn!("[Worker {}] Packet decryption failed from peer {}: {}. Dropping packet.", i, peer, e);
                                }
                            }
                        } else {
                            // Channel closed
                            break;
                        }
                    }
                    info!("Worker {} finished", i);
                });
            }

            // The Dispatcher Task
            let dispatcher_socket = socket.clone();
            let dispatcher = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match dispatcher_socket.recv_from(&mut buf).await {
                        Ok((len, peer)) => {
                            if tx.send((buf[..len].to_vec(), peer)).await.is_err() {
                                error!("Worker channel closed, dispatcher shutting down.");
                                break;
                            }
                        }
                        Err(e) => {
                            error!("UDP socket error, dispatcher shutting down: {}", e);
                            break;
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

                            // Find the first authenticated client to send the packet to.
                            // Note: A proper implementation would map TUN IPs to client addresses.
                            let clients_guard = clients_reader.lock().await;
                            let client_info = clients_guard.iter().find_map(|(id, state)| {
                                if state.auth_status == AuthStatus::Authenticated {
                                    Some((*id, state.last_seen_addr))
                                } else {
                                    None
                                }
                            });

                            if let Some((client_id, peer_addr)) = client_info {
                                let seq = downstream_seq.fetch_add(1, Ordering::Relaxed);
                                let header = PacketHeader::new(seq, PacketType::Data, client_id);

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
                _ = dispatcher => info!("Dispatcher task finished."),
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
