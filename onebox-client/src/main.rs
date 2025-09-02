//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::prelude::*;
use tokio::net::UdpSocket;
use tracing::{error, info, Level};

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

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
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

    info!("onebox-client starting up...");

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
        Commands::Start { foreground } => {
            info!("Starting onebox client...");
            if foreground {
                info!("Running in foreground mode");
            }
            // Basic UDP client: send a Hello Onebox message to server
            let server_addr = format!(
                "{}:{}",
                config.client.server_address, config.client.server_port
            );
            info!("Sending test datagram to server at {}", server_addr);

            // Bind to an ephemeral local UDP port
            let local_socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|e| {
                error!("Failed to bind local UDP socket: {}", e);
                anyhow::anyhow!(e)
            })?;

            let message = b"Hello Onebox";
            match local_socket.send_to(message, &server_addr).await {
                Ok(bytes) => info!("Sent {} bytes to {}", bytes, server_addr),
                Err(e) => error!("Failed to send UDP datagram: {}", e),
            }
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
