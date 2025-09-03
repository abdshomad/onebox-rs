//! Packet handling and protocol structures

use crate::types::ClientId;
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

    /// Client identifier
    pub client_id: ClientId,

    /// Reserved field for future use
    pub reserved: u32,
}

/// Types of packets in the onebox protocol
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketType {
    /// Data packet containing actual network traffic
    Data = 0x01,

    /// Keep-alive/probe packet for link health monitoring
    Probe = 0x02,

    /// Authentication request packet
    AuthRequest = 0x03,

    /// Authentication response packet
    AuthResponse = 0x04,

    /// Control packet for session management
    Control = 0x05,
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
            client_id: ClientId::default(),
            reserved: 0,
        }
    }
}

impl PacketHeader {
    /// Create a new packet header with the given sequence number and type
    pub fn new(sequence_number: u64, packet_type: PacketType, client_id: ClientId) -> Self {
        Self {
            sequence_number,
            packet_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            client_id,
            reserved: 0,
        }
    }

    /// Get the size of the packet header in bytes
    pub const fn size() -> usize {
        // sequence_number (u64) + timestamp (u64) + client_id (u128) + packet_type (u32 as repr) + reserved (u32)
        std::mem::size_of::<u64>()
            + std::mem::size_of::<u64>()
            + std::mem::size_of::<u128>()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn packet_type_is_four_bytes() {
        assert_eq!(
            std::mem::size_of::<PacketType>(),
            std::mem::size_of::<u32>()
        );
    }

    #[test]
    fn packet_header_default_values() {
        let header = PacketHeader::default();
        assert_eq!(header.sequence_number, 0);
        assert_eq!(header.packet_type, PacketType::Data);
        assert!(header.timestamp > 0);
        assert_eq!(header.client_id.0, 0);
        assert_eq!(header.reserved, 0);
    }

    #[test]
    fn packet_header_new_sets_fields() {
        let client_id = ClientId(42);
        let header = PacketHeader::new(123, PacketType::Probe, client_id);
        assert_eq!(header.sequence_number, 123);
        assert_eq!(header.packet_type, PacketType::Probe);
        assert_eq!(header.client_id.0, 42);
        assert!(header.timestamp > 0);
    }

    #[test]
    fn packet_header_size_matches_layout() {
        assert_eq!(PacketHeader::size(), 8 + 8 + 16 + 4 + 4);
    }

    #[test]
    fn serde_roundtrip_packet_header() {
        let mut header = PacketHeader::new(7, PacketType::AuthRequest, ClientId(0xABCD));
        header.timestamp = 1234567890;
        header.reserved = 1;
        let json = serde_json::to_string(&header).unwrap();
        let de: PacketHeader = serde_json::from_str(&json).unwrap();
        assert_eq!(de.sequence_number, 7);
        assert_eq!(de.packet_type, PacketType::AuthRequest);
        assert_eq!(de.timestamp, 1234567890);
        assert_eq!(de.client_id.0, 0xABCD);
        assert_eq!(de.reserved, 1);
    }
}
