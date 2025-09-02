//! onebox-server - Server binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio_tun::TunBuilder;
use tracing::{debug, error, info, Level};

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

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = match cli.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    info!("onebox-server starting up...");

    // Load configuration
    let config = match Config::from_file(&cli.config) {
        Ok(config) => {
            info!("Configuration loaded from {}", cli.config);
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(anyhow::anyhow!("Configuration error: {}", e));
        }
    };

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

            // Split TUN device into reader and writer
            let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);

            // Create a shared state for the client address
            let client_addr = Arc::new(Mutex::new(None::<SocketAddr>));

            // Create Arc for the socket to share between tasks
            let socket = Arc::new(socket);

            // Task 1: UDP -> TUN
            let udp_to_tun_socket = socket.clone();
            let client_addr_writer = client_addr.clone();
            let udp_to_tun = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match udp_to_tun_socket.recv_from(&mut buf).await {
                        Ok((len, peer)) => {
                            debug!("Received {} bytes from {}", len, peer);
                            // Update the client address
                            *client_addr_writer.lock().await = Some(peer);
                            // Write the packet to the TUN interface
                            if let Err(e) = tun_writer.write_all(&buf[..len]).await {
                                error!("Error writing to TUN: {}", e);
                                break;
                            }
                            debug!("Wrote {} bytes to TUN", len);
                        }
                        Err(e) => {
                            error!("UDP receive error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Task 2: TUN -> UDP
            let tun_to_udp_socket = socket.clone();
            let client_addr_reader = client_addr.clone();
            let tun_to_udp = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match tun_reader.read(&mut buf).await {
                        Ok(len) => {
                            debug!("Read {} bytes from TUN", len);
                            if let Some(peer) = *client_addr_reader.lock().await {
                                // Send the packet to the client
                                if let Err(e) = tun_to_udp_socket.send_to(&buf[..len], peer).await {
                                    error!("Error sending to UDP: {}", e);
                                    break;
                                }
                                debug!("Sent {} bytes to {}", len, peer);
                            }
                            // If client_addr is None, we just drop the packet
                        }
                        Err(e) => {
                            error!("TUN read error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Wait for either task to complete
            tokio::select! {
                _ = udp_to_tun => {
                    info!("UDP->TUN task finished.");
                },
                _ = tun_to_udp => {
                    info!("TUN->UDP task finished.");
                },
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
    let commands = [
        // Delete any existing rule to avoid duplicates
        vec![
            "-t",
            "nat",
            "-D",
            "POSTROUTING",
            "-s",
            source_net,
            "-o",
            interface,
            "-j",
            "MASQUERADE",
        ],
        // Add the new rule
        vec![
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
        ],
    ];

    // The first command (delete) is allowed to fail if the rule doesn't exist.
    Command::new("iptables")
        .args(commands[0].clone())
        .output()?;

    // The second command (add) must succeed.
    let add_output = Command::new("iptables")
        .args(commands[1].clone())
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
