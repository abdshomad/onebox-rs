//! onebox-server - Server binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::prelude::*;
use tokio::net::UdpSocket;
use tracing::{error, info, Level};
use std::net::SocketAddr;

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
            let bind_addr: SocketAddr = if let Some(bind_str) = bind {
                match bind_str.parse() {
                    Ok(addr) => {
                        info!("Binding to override address: {}", addr);
                        addr
                    }
                    Err(e) => {
                        error!("Invalid bind address '{}': {}", bind_str, e);
                        return Err(anyhow::anyhow!("Invalid bind address"));
                    }
                }
            } else {
                let addr = config.server.network.bind_address;
                info!("Binding to configured address: {}", addr);
                addr
            };

            // Bind UDP socket and log incoming datagrams
            let socket = UdpSocket::bind(bind_addr).await.map_err(|e| {
                error!("Failed to bind UDP socket on {}: {}", bind_addr, e);
                anyhow::anyhow!(e)
            })?;

            info!("UDP server listening on {}", bind_addr);

            let mut buffer = vec![0u8; 2048];
            loop {
                match socket.recv_from(&mut buffer).await {
                    Ok((len, peer)) => {
                        info!("Received {} bytes from {}", len, peer);
                        if len > 0 {
                            let preview = String::from_utf8_lossy(&buffer[..len]);
                            info!("Data (utf8-lossy preview): {}", preview);
                        }
                    }
                    Err(e) => {
                        error!("UDP receive error: {}", e);
                        break;
                    }
                }
            }
        }

        Commands::Stop => {
            info!("Stopping onebox server...");
            // TODO: Implement server stop logic
            info!("Server stop not yet implemented");
        }

        Commands::Status => {
            info!("Showing server status...");
            // TODO: Implement status display logic
            info!("Status display not yet implemented");
        }

        Commands::Config => {
            info!("Showing server configuration...");
            println!("Configuration loaded from: {}", cli.config);
            println!(
                "Server TUN: {} ({})",
                config.server.tun.name, config.server.tun.ip
            );
            println!("Bind address: {}", config.server.network.bind_address);
            println!("Max connections: {}", config.server.network.max_connections);
        }
    }

    info!("onebox-server operation completed");
    Ok(())
}
