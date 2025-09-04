---
marp: true
theme: default
---

# onebox-rs
### High-performance, secure internet bonding
A presentation by Jules

---

## The Problem

- Low Bandwidth
- Poor Reliability
- Lack of Redundancy
- High Cost of dedicated lines

---

## The Solution: onebox-rs

A Rust-based internet bonding solution that aggregates multiple internet connections into a single, resilient virtual connection.

---

## Features

---

### Internet Bonding
Combine multiple WAN connections (Wi-Fi, Ethernet, Cellular) for increased bandwidth.

---

### Seamless Failover
Automatic failover when connections drop, with zero packet loss.

---

### End-to-End Encryption
ChaCha20-Poly1305 encryption for all tunnel traffic.

---

### High Performance
Built with Rust and Tokio for minimal CPU overhead.

---

## Architecture

---

### Client (`onebox-client`)
- Runs on Linux-based single-board computers (e.g., Raspberry Pi)
- Creates a TUN interface to capture all outgoing traffic
- Distributes packets across multiple connections

---

### Server (`onebox-server`)
- Runs on a cloud VPS with a public IP
- Receives encrypted packets from clients
- Reassembles packets and forwards them to the internet

---

![bg](https://i.imgur.com/9y7B42s.png)

---

## Technical Details

- Language: Rust
- Async Runtime: Tokio
- Protocol: Custom UDP-based protocol
- Encryption: ChaCha20-Poly1305
- Configuration: TOML

---

## Future Work

---

### Web Dashboard
A simple web interface (served locally by the client) for real-time monitoring and configuration.

---

### Advanced Bonding Modes
Implement different policies, such as "reliability first" or "cost-aware".

---

### WireGuard Integration
Explore using WireGuard as the underlying transport.

---

### Cross-Platform Client
Expand client support to other operating systems like macOS and Windows.

---

# The End
Created with Marp
