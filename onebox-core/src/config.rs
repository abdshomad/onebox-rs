//! Configuration management for the onebox system

use crate::error::{OneboxError, OneboxResult};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

/// Main configuration structure for the onebox system
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Client-specific configuration
    #[serde(default)]
    pub client: ClientConfig,

    /// Server-specific configuration
    #[serde(default)]
    pub server: ServerConfig,

    /// Global settings
    #[serde(default)]
    pub global: GlobalConfig,
}

/// Client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// TUN interface configuration
    pub tun: TunConfig,

    /// Server connection settings
    pub server: ServerConnectionConfig,

    /// Link health monitoring settings
    #[serde(default)]
    pub health: HealthConfig,

    /// Authentication settings
    #[serde(default)]
    pub auth: AuthConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// TUN interface configuration
    pub tun: TunConfig,

    /// Network binding settings
    pub network: NetworkConfig,

    /// Authentication settings
    #[serde(default)]
    pub auth: AuthConfig,
}

/// TUN interface configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunConfig {
    /// TUN interface name
    pub name: String,

    /// TUN interface IP address
    pub ip: IpAddr,

    /// TUN interface netmask
    pub netmask: IpAddr,

    /// TUN interface MTU
    #[serde(default = "default_mtu")]
    pub mtu: u16,
}

/// Server connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConnectionConfig {
    /// Server address and port
    pub address: SocketAddr,

    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Keep-alive interval in seconds
    #[serde(default = "default_keepalive")]
    pub keepalive: u64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bind address for the server
    pub bind_address: SocketAddr,

    /// Maximum number of concurrent connections
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    /// Probe interval in seconds
    #[serde(default = "default_probe_interval")]
    pub probe_interval: u64,

    /// Probe timeout in seconds
    #[serde(default = "default_probe_timeout")]
    pub probe_timeout: u64,

    /// Number of consecutive failures before marking link as down
    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Pre-shared key for authentication
    pub psk: String,

    /// Encryption algorithm to use
    #[serde(default = "default_encryption")]
    pub encryption: EncryptionAlgorithm,
}

/// Global configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Configuration file path
    #[serde(default = "default_config_path")]
    pub config_path: String,
}

/// Encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    ChaCha20Poly1305,
    AES256GCM,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> OneboxResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| OneboxError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| OneboxError::Config(format!("Failed to parse TOML: {}", e)))?;

        Ok(config)
    }

    /// Load configuration from the default path
    pub fn load() -> OneboxResult<Self> {
        let default_paths = ["config.toml", "/etc/onebox/config.toml", "./config.toml"];

        for path in &default_paths {
            if Path::new(path).exists() {
                return Self::from_file(path);
            }
        }

        Err(OneboxError::Config(
            "No configuration file found".to_string(),
        ))
    }
}

// Default value functions
fn default_mtu() -> u16 {
    1500
}
fn default_timeout() -> u64 {
    30
}
fn default_keepalive() -> u64 {
    60
}
fn default_max_connections() -> usize {
    1000
}
fn default_probe_interval() -> u64 {
    10
}
fn default_probe_timeout() -> u64 {
    5
}
fn default_failure_threshold() -> u32 {
    3
}
fn default_encryption() -> EncryptionAlgorithm {
    EncryptionAlgorithm::ChaCha20Poly1305
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_config_path() -> String {
    "config.toml".to_string()
}



impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            tun: TunConfig {
                name: "onebox0".to_string(),
                ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
                netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                mtu: 1500,
            },
            server: ServerConnectionConfig {
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                timeout: 30,
                keepalive: 60,
            },
            health: HealthConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            tun: TunConfig {
                name: "onebox0".to_string(),
                ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                mtu: 1500,
            },
            network: NetworkConfig {
                bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
                max_connections: 1000,
            },
            auth: AuthConfig::default(),
        }
    }
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            probe_interval: 10,
            probe_timeout: 5,
            failure_threshold: 3,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            psk: "default-psk-change-me".to_string(),
            encryption: EncryptionAlgorithm::ChaCha20Poly1305,
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            config_path: "config.toml".to_string(),
        }
    }
}
