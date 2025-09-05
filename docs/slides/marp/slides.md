---
marp: true
theme: default
---

<!-- _class: lead -->
# onebox-rs
### High-performance, secure internet bonding
<p>A presentation by Jules</p>

---

<!-- _class: lead -->
## The Problem

---

### Low Bandwidth
A single internet connection is insufficient for demanding tasks like video streaming, large file transfers, or online gaming.

---

### Poor Reliability
Connections frequently drop or become unstable, disrupting critical activities like video conferences and remote work.

---

### Lack of Redundancy
A single point of failure (e.g., the local ISP has an outage) can result in a complete loss of connectivity.

---

### High Cost
A single, dedicated high-speed business line is prohibitively expensive for individuals, nomads, or small businesses.

---

<!-- _class: lead -->
## The Solution: onebox-rs
A Rust-based internet bonding solution that aggregates multiple internet connections into a single, resilient virtual connection.

---

<!-- _class: lead -->
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

<!-- _class: lead -->
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

![bg right:40%](https://i.imgur.com/9y7B42s.png)

## Architecture Diagram

---

<!-- _class: lead -->
## Technical Details

---

### Language: Rust
Chosen for its performance, memory safety, and concurrency features, which are critical for a long-running networking service.

---

### Async Runtime: Tokio
Used as the asynchronous runtime for managing all I/O operations (network sockets, TUN interfaces) efficiently.

---

### Protocol: Custom UDP-based
A custom, lightweight UDP-based protocol designed for low overhead and the essential features of sequencing and authentication.

---

### Encryption: ChaCha20-Poly1305
All traffic within the tunnel is encrypted and authenticated using the ChaCha20-Poly1305 AEAD cipher.

---

### Configuration: TOML
Simple, human-readable TOML files are used for configuration on both the client and server.

---

<!-- _class: lead -->
## Future Work

---

### Web Dashboard
A simple web interface (served locally by the client) for real-time monitoring and configuration.

---

### Advanced Bonding Modes
Implement different policies, such as "reliability first" (duplicates critical packets over multiple links) or "cost-aware" (prioritizes unmetered connections over cellular).

---

### WireGuard Integration
Explore using WireGuard as the underlying transport for its proven security and performance, while still managing the multipath logic.

---

### Cross-Platform Client
Expand client support to other operating systems like macOS and Windows.

---

<!-- _class: lead -->
# The End
<p>Created with Marp</p>
