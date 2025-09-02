//! Packet handling and protocol structures

use serde::{Deserialize, Serialize};

/// Packet header for the onebox protocol
/// This header is prepended to all packets sent through the tunnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    /// Monotonic sequence number for packet ordering
    pub sequence_number: u64,

    /// Packet type identifier
    pub packet_type: PacketType,

    /// Timestamp when packet was created (Unix timestamp in milliseconds)
    pub timestamp: u64,

    /// Reserved field for future use
    pub reserved: u32,
}

/// Types of packets in the onebox protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketType {
    /// Data packet containing actual network traffic
    Data = 0x01,

    /// Keep-alive/probe packet for link health monitoring
    Probe = 0x02,

    /// Authentication packet
    Auth = 0x03,

    /// Control packet for session management
    Control = 0x04,
}

impl Default for PacketHeader {
    fn default() -> Self {
        Self {
            sequence_number: 0,
            packet_type: PacketType::Data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            reserved: 0,
        }
    }
}

impl PacketHeader {
    /// Create a new packet header with the given sequence number and type
    pub fn new(sequence_number: u64, packet_type: PacketType) -> Self {
        Self {
            sequence_number,
            packet_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            reserved: 0,
        }
    }

    /// Get the size of the packet header in bytes
    pub fn size() -> usize {
        std::mem::size_of::<u64>() * 2 + std::mem::size_of::<u32>() * 2
    }
}
