//! # onebox-core
//!
//! Core library for the onebox-rs internet bonding solution.
//! This library provides the fundamental data structures, networking primitives,
//! and utilities needed by both the client and server components.

pub mod config;
pub mod error;
pub mod packet;
pub mod types;

pub use error::{OneboxError, OneboxResult};
pub use packet::PacketHeader;
pub use types::*;

/// Re-export commonly used items
pub mod prelude {
    pub use super::{config::*, error::*, packet::*, types::*};
}
