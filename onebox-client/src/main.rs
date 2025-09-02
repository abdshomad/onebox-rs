//! onebox-client - Client binary for the onebox-rs internet bonding solution

use clap::{Parser, Subcommand};
use onebox_core::prelude::*;
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
            // TODO: Implement client startup logic
            info!("Client startup not yet implemented");
        }

        Commands::Stop => {
            info!("Stopping onebox client...");
            // TODO: Implement client stop logic
            info!("Client stop not yet implemented");
        }

        Commands::Status => {
            info!("Showing client status...");
            // TODO: Implement status display logic
            info!("Status display not yet implemented");
        }

        Commands::Config => {
            info!("Showing client configuration...");
            println!("Configuration loaded from: {}", cli.config);
            println!(
                "Client TUN: {} ({})",
                config.client.tun.name, config.client.tun.ip
            );
            println!("Server: {}", config.client.server.address);
        }
    }

    info!("onebox-client operation completed");
    Ok(())
}
