---
marp: true
theme: default
---

<!-- Section: Title -->
<!-- _class: lead -->
# onebox-rs
### High-performance, secure internet bonding
<p>A presentation by Jules</p>

---

<!-- Section: Problem -->
<!-- _class: lead -->
## The Problem

---

<div class="columns">
<div>

### Low Bandwidth
A single internet connection is insufficient for demanding tasks like video streaming, large file transfers, or online gaming.

---

### Poor Reliability
Connections frequently drop or become unstable, disrupting critical activities like video conferences and remote work.

</div>
<div>

### Lack of Redundancy
A single point of failure (e.g., the local ISP has an outage) can result in a complete loss of connectivity.

---

### High Cost
A single, dedicated high-speed business line is prohibitively expensive for individuals, nomads, or small businesses.

</div>
</div>

---

<!-- Section: Solution -->
<!-- _class: lead -->
## The Solution: onebox-rs
A Rust-based internet bonding solution that aggregates multiple internet connections into a single, resilient virtual connection.

---
<!--
Speaker Notes:
This is the core value proposition of onebox-rs. Emphasize the words "aggregates" and "resilient".
-->

---

<!-- Section: Features -->
<!-- _class: lead -->
## Features

---

<div class="columns">
<div>

### :zap: Internet Bonding
Combine multiple WAN connections (Wi-Fi, Ethernet, Cellular) for increased bandwidth.

---

### :shield: End-to-End Encryption
ChaCha20-Poly1305 encryption for all tunnel traffic.

</div>
<div>

### :loop: Seamless Failover
Automatic failover when connections drop, with zero packet loss.

---

### :rocket: High Performance
Built with Rust and Tokio for minimal CPU overhead.

</div>
</div>

---

<!-- Section: Architecture -->
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
<!--
Speaker Notes:
Briefly explain the data flow from client to server. Point out the key components on the diagram.
-->

---

<!-- Section: Technical Details -->
<!-- _class: lead -->
## Technical Details

---

- **:crab: Language:** Rust
- **:tokyo_tower: Async Runtime:** Tokio
- **:envelope: Protocol:** Custom UDP-based
- **:lock: Encryption:** ChaCha20-Poly1305
- **:page_facing_up: Configuration:** TOML

---

<!-- Section: Future Work -->
<!-- _class: lead -->
## Future Work

---

<div class="columns">
<div>

### :bar_chart: Web Dashboard
A simple web interface for real-time monitoring and configuration.

---

### :gear: Advanced Bonding Modes
Implement policies like "reliability first" or "cost-aware".

</div>
<div>

### :shield: WireGuard Integration
Explore using WireGuard as the underlying transport.

---

### :computer: Cross-Platform Client
Expand client support to macOS and Windows.

</div>
</div>

---

<!-- Section: End -->
<!-- _class: lead -->
# The End
<p>Created with Marp</p>
<p>Questions?</p>
