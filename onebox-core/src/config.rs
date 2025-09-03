//! Configuration management for the onebox system.
//!
//! This module defines the structures for loading configuration from a TOML
//! file, adhering to the specification in `docs/SRS.md (SI-2)`.

use crate::error::{OneboxError, OneboxResult};
use serde::Deserialize;
use std::path::Path;

/// Represents the entire configuration loaded from `config.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub log_level: String,
    pub preshared_key: String,
    #[serde(default)]
    pub client: ClientConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

/// Contains client-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientConfig {
    pub server_address: String,
    pub server_port: u16,
    pub tun_name: String,
    pub tun_ip: String,
    pub tun_netmask: String,
}

/// Contains server-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub listen_address: String,
    pub listen_port: u16,
}

impl Config {
    /// Loads configuration from a specified TOML file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the `config.toml` file.
    ///
    /// # Returns
    ///
    /// A `OneboxResult` containing the populated `Config` struct or a `OneboxError`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> OneboxResult<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            OneboxError::Config(format!(
                "Failed to read config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| OneboxError::Config(format!("Failed to parse TOML: {e}")))?;

        Ok(config)
    }
}

// Default implementations for cases where a section might be missing in the TOML.

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1".to_string(),
            server_port: 51820,
            tun_name: "onebox0".to_string(),
            tun_ip: "10.8.0.1".to_string(),
            tun_netmask: "255.255.255.0".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            listen_port: 51820,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_valid_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"
            log_level = "debug"
            preshared_key = "my-test-psk"

            [client]
            server_address = "1.2.3.4"
            server_port = 12345
            tun_name = "test_tun"
            tun_ip = "10.0.0.1"
            tun_netmask = "255.255.0.0"

            [server]
            listen_address = "0.0.0.0"
            listen_port = 54321
            "#
        )
        .unwrap();

        let config = Config::from_file(&file_path).unwrap();

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.preshared_key, "my-test-psk");
        assert_eq!(config.client.server_address, "1.2.3.4");
        assert_eq!(config.client.server_port, 12345);
        assert_eq!(config.client.tun_name, "test_tun");
        assert_eq!(config.client.tun_ip, "10.0.0.1");
        assert_eq!(config.client.tun_netmask, "255.255.0.0");
        assert_eq!(config.server.listen_address, "0.0.0.0");
        assert_eq!(config.server.listen_port, 54321);
    }

    #[test]
    fn test_load_missing_config_file() {
        let result = Config::from_file("a-path-that-does-not-exist.toml");
        assert!(result.is_err());
        if let Err(OneboxError::Config(msg)) = result {
            assert!(msg.contains("Failed to read config file"));
        } else {
            panic!("Expected a Config error");
        }
    }

    #[test]
    fn test_load_malformed_config_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("malformed.toml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "this is not valid toml").unwrap();

        let result = Config::from_file(&file_path);
        assert!(result.is_err());
        if let Err(OneboxError::Config(msg)) = result {
            assert!(msg.contains("Failed to parse TOML"));
        } else {
            panic!("Expected a Config error");
        }
    }

    #[test]
    fn test_default_client_config() {
        let config = ClientConfig::default();
        assert_eq!(config.server_address, "127.0.0.1");
        assert_eq!(config.server_port, 51820);
    }

    #[test]
    fn test_default_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.listen_address, "0.0.0.0");
        assert_eq!(config.listen_port, 51820);
    }
}
