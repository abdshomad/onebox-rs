//! Common type definitions for the onebox system

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name (e.g., "eth0", "wlan0")
    pub name: String,

    /// Interface IP address
    pub ip: IpAddr,

    /// Interface subnet mask
    pub netmask: IpAddr,

    /// Gateway IP address
    pub gateway: IpAddr,

    /// Interface status (up/down)
    pub is_up: bool,

    /// Interface type (ethernet, wifi, etc.)
    pub interface_type: InterfaceType,
}

/// Types of network interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceType {
    Ethernet,
    WiFi,
    Cellular,
    Virtual,
    Unknown,
}

/// Link status for health monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkStatus {
    /// Link is healthy and operational
    Up,

    /// Link is down or unreachable
    Down,

    /// Link is being tested/probed
    Testing,
}

/// Link health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkHealth {
    /// Current link status
    pub status: LinkStatus,

    /// Round-trip time in milliseconds
    pub rtt_ms: Option<u32>,

    /// Packet loss percentage (0-100)
    pub packet_loss: Option<f32>,

    /// Last successful probe timestamp
    pub last_success: Option<u64>,

    /// Number of consecutive failures
    pub consecutive_failures: u32,
}

/// Configuration for a network link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    /// Local socket address to bind to
    pub local_addr: SocketAddr,

    /// Remote server address
    pub remote_addr: SocketAddr,

    /// Link priority (lower number = higher priority)
    pub priority: u8,

    /// Whether this link is enabled
    pub enabled: bool,
}

impl Default for LinkHealth {
    fn default() -> Self {
        Self {
            status: LinkStatus::Up,
            rtt_ms: None,
            packet_loss: None,
            last_success: None,
            consecutive_failures: 0,
        }
    }
}

impl Default for LinkConfig {
    fn default() -> Self {
        Self {
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
            remote_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
            priority: 100,
            enabled: true,
        }
    }
}
