---
layout: cover
---

# onebox-rs
### High-performance, secure internet bonding
<p>A presentation by Jules</p>

---
layout: default
---

## The Problem

- Low Bandwidth
- Poor Reliability
- Lack of Redundancy
- High Cost of dedicated lines

---
layout: default
---

## The Solution: onebox-rs

A Rust-based internet bonding solution that aggregates multiple internet connections into a single, resilient virtual connection.

---
layout: default
---

## Features

---
layout: default
---

### Internet Bonding
Combine multiple WAN connections (Wi-Fi, Ethernet, Cellular) for increased bandwidth.

---
layout: default
---

### Seamless Failover
Automatic failover when connections drop, with zero packet loss.

---
layout: default
---

### End-to-End Encryption
ChaCha20-Poly1305 encryption for all tunnel traffic.

---
layout: default
---

### High Performance
Built with Rust and Tokio for minimal CPU overhead.

---
layout: default
---

## Architecture

---
layout: default
---

### Client (`onebox-client`)
- Runs on Linux-based single-board computers (e.g., Raspberry Pi)
- Creates a TUN interface to capture all outgoing traffic
- Distributes packets across multiple connections

---
layout: default
---

### Server (`onebox-server`)
- Runs on a cloud VPS with a public IP
- Receives encrypted packets from clients
- Reassembles packets and forwards them to the internet

---
layout: image-right
image: https://i.imgur.com/9y7B42s.png
---

## Architecture Diagram

A visual representation of the client-server architecture.

---
layout: default
---

## Technical Details

- Language: Rust
- Async Runtime: Tokio
- Protocol: Custom UDP-based protocol
- Encryption: ChaCha20-Poly1305
- Configuration: TOML

---
layout: default
---

## Future Work

---
layout: default
---

### Web Dashboard
A simple web interface (served locally by the client) for real-time monitoring and configuration.

---
layout: default
---

### Advanced Bonding Modes
Implement different policies, such as "reliability first" or "cost-aware".

---
layout: default
---

### WireGuard Integration
Explore using WireGuard as the underlying transport.

---
layout: default
---

### Cross-Platform Client
Expand client support to other operating systems like macOS and Windows.

---
layout: cover
---

# The End
<p>Created with Slidev</p>
