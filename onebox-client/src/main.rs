//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::prelude::*;
use std::net::Ipv4Addr;
use std::process::Command;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio_tun::TunBuilder;
use tracing::{debug, error, info, Level};

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
            eprintln!("Failed to load configuration: {}", e);
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
            let server_addr = format!(
                "{}:{}",
                config.client.server_address, config.client.server_port
            );
            info!("Will connect to server at {}", server_addr);

            // Bind to a local UDP port.
            let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|e| {
                error!("Failed to bind local UDP socket: {}", e);
                anyhow::anyhow!(e)
            })?;
            // Connect the socket to the server's address. This allows `send` and `recv` syscalls.
            socket.connect(&server_addr).await?;
            info!("UDP socket connected to server at {}", server_addr);

            let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);
            let socket = Arc::new(socket);

            // Task 1: Read from TUN and send to UDP
            let tun_to_udp_socket = socket.clone();
            let tun_to_udp = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match tun_reader.read(&mut buf).await {
                        Ok(len) => {
                            debug!("Read {} bytes from TUN", len);
                            if let Err(e) = tun_to_udp_socket.send(&buf[..len]).await {
                                error!("Error sending to UDP: {}", e);
                                break;
                            }
                            debug!("Sent {} bytes to server", len);
                        }
                        Err(e) => {
                            error!("TUN read error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Task 2: Read from UDP and send to TUN
            let udp_to_tun_socket = socket.clone();
            let udp_to_tun = tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                loop {
                    match udp_to_tun_socket.recv(&mut buf).await {
                        Ok(len) => {
                            debug!("Received {} bytes from server", len);
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
