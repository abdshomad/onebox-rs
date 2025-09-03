//! Health monitoring for network links.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Represents the status of a network link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    /// The link is considered up and healthy.
    Up,
    /// The link is considered down due to probe failures.
    Down,
    /// The link is in an unknown state, awaiting first probe result.
    Unknown,
}

/// Holds health statistics for a single network link.
#[derive(Debug, Clone)]
pub struct LinkStats {
    /// The current status of the link.
    pub status: LinkStatus,
    /// The last measured Round-Trip Time (RTT).
    pub rtt: Duration,
    /// The number of probes sent.
    pub probes_sent: u64,
    /// The number of probe echoes received.
    pub probes_received: u64,
    /// A map of sent probe sequence numbers to the time they were sent.
    pub in_flight_probes: HashMap<u64, Instant>,
    /// The next sequence number to use for a probe on this link.
    pub next_probe_seq: u64,
}

impl LinkStats {
    /// Creates a new `LinkStats` with default values.
    pub fn new() -> Self {
        Self {
            status: LinkStatus::Unknown,
            rtt: Duration::default(),
            probes_sent: 0,
            probes_received: 0,
            in_flight_probes: HashMap::new(),
            next_probe_seq: 0,
        }
    }

    /// Calculates the current packet loss percentage.
    pub fn packet_loss_percent(&self) -> f32 {
        if self.probes_sent == 0 {
            0.0
        } else {
            let loss = self.probes_sent.saturating_sub(self.probes_received);
            (loss as f32 / self.probes_sent as f32) * 100.0
        }
    }
}

impl Default for LinkStats {
    fn default() -> Self {
        Self::new()
    }
}
