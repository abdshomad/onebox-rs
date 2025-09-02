## Product Requirements Document: onebox-rs

| **Product Name:** | onebox-rs | **Version:** | 1.0 |
| :--- | :--- | :--- | :--- |
| **Author:** | Gemini AI | **Date:** | September 2, 2025 |
| **Status:** | Draft | **Target Release:** | v1.0 |

### 1. Introduction & Vision

**onebox-rs** is a high-performance, secure internet bonding solution built entirely in Rust. Its mission is to provide individuals and small businesses with a faster and more reliable internet connection by aggregating the bandwidth of multiple, disparate internet sources (e.g., Wi-Fi, Ethernet, Cellular).

By leveraging Rust's strengths in performance, memory safety, and concurrency, onebox-rs will offer a robust and efficient alternative to existing solutions. The project consists of two core components: a lightweight **client** application (`onebox-client`) that runs on commodity hardware like a Raspberry Pi, and a powerful **server** application (`onebox-server`) that runs on a cloud VPS.

### 2. The Problem

In today's digitally connected world, a stable and fast internet connection is no longer a luxury, but a necessity. However, many users face significant challenges:
*   **Low Bandwidth:** A single internet connection is insufficient for demanding tasks like video streaming, large file transfers, or online gaming.
*   **Poor Reliability:** Connections frequently drop or become unstable, disrupting critical activities like video conferences and remote work.
*   **Lack of Redundancy:** A single point of failure (e.g., the local ISP has an outage) can result in a complete loss of connectivity.
*   **High Cost:** A single, dedicated high-speed business line is prohibitively expensive for individuals, nomads, or small businesses.

### 3. Goals & Objectives

The primary goal of onebox-rs is to combine multiple internet connections into a single, cohesive, and resilient virtual connection.

*   **Aggregate Bandwidth:** Combine the upload and download speeds of all connected links to achieve significantly higher throughput.
*   **Provide Seamless Failover:** If one connection fails, the session should continue uninterrupted over the remaining active links, without the user noticing a drop.
*   **Ensure Security:** All traffic passing through the bonded tunnel must be protected with modern, high-performance, end-to-end encryption.
*   **Achieve High Performance:** The solution must have low CPU and memory overhead, making it suitable for resource-constrained devices like a Raspberry Pi.
*   **Be Accessible:** The software should be easy to configure and deploy for technically-inclined users through clear documentation and a command-line interface.

### 4. Target Audience

*   **Prosumers & Power Users:** Tech-savvy individuals, content creators, and gamers who need maximum speed and reliability for their home network.
*   **Remote Workers & Digital Nomads:** Professionals working from locations with unreliable internet (e.g., RVs, rural areas) who need to combine sources like campsite Wi-Fi and cellular data.
*   **Small Businesses:** Small offices or retail locations that need an affordable way to ensure their internet connection is always online by combining a primary line with a backup (e.g., DSL + 5G).
*   **Rust & Networking Enthusiasts:** Developers interested in a practical, open-source project that showcases the power of Rust for systems programming.

### 5. Features & Requirements (v1.0)

#### **Epic: Core Bonding Engine**
*   **FR1.1: Multipath Tunneling:** The client and server will establish a persistent tunnel. All traffic will be encapsulated and transported over UDP for performance and flexibility.
*   **FR1.2: Packet Sequencing & Reassembly:** The protocol will add a sequence number to each packet. The server will maintain a reordering buffer to handle out-of-order packet arrival from different links.
*   **FR1.3: End-to-End Encryption:** All payloads will be encrypted using a modern, fast AEAD cipher (e.g., ChaCha20-Poly1305). A secure handshake will be used for session key exchange.
*   **FR1.4: Link Health Monitoring:** The client will continuously monitor each link for latency, jitter, and packet loss to make intelligent routing decisions.
*   **FR1.5: Seamless Failover:** If a link is determined to be down, the client will instantly stop sending data over it. The server will handle the gap in sequence numbers and continue the session using the remaining links.

#### **Epic: Client Application (`onebox-client`)**
*   **FR2.1: Platform Support:** Must be deployable on Linux-based single-board computers (target: Raspberry Pi 4/5, `aarch64`).
*   **FR2.2: Traffic Interception:** Will use a kernel **TUN** interface to transparently capture all outgoing IP traffic from the device.
*   **FR2.3: Interface Management:** Must automatically detect all available network interfaces and bind sockets to them for sending data.
*   **FR2.4: Configuration:** Configuration will be managed through a simple, human-readable file (e.g., `config.toml`) specifying the server address and credentials.
*   **FR2.5: Command-Line Interface (CLI):** A CLI will be provided to start/stop the service and display real-time statistics (total throughput, per-link stats, etc.).

#### **Epic: Server Application (`onebox-server`)**
*   **FR3.1: Platform Support:** Must be deployable on common Linux cloud VPS distributions (target: Ubuntu/Debian, `x86_64`).
*   **FR3.2: Multi-Client Architecture:** Must be able to handle connections from multiple clients concurrently and securely.
*   **FR3.3: Traffic Forwarding:** Will use a server-side TUN interface to inject reassembled, decrypted packets back into the kernel, which then forwards them to the public internet using standard NAT.
*   **FR3.4: High-Performance Networking:** Must be built on a high-performance asynchronous runtime to handle thousands of incoming packets per second with minimal overhead.

### 6. Technical Architecture & Implementation (The Rust Approach)

*   **Language:** **Rust (Latest Stable)** for its performance, concurrency features, and memory safety guarantees, which are critical for a long-running networking service.
*   **Core Runtime:** **Tokio** will be used as the asynchronous runtime for managing all I/O operations (network sockets, TUN interfaces) efficiently.
*   **Key Crates:**
    *   **Network Interception:** `tokio-tun` / `tappers` for safe, async interaction with TUN/TAP devices.
    *   **System Calls:** `nix` for low-level socket options required to bind sockets to specific network interfaces.
    *   **Cryptography:** `ring` or a pure-Rust alternative for the cryptographic primitives (key exchange, encryption).
    *   **CLI:** `clap` for creating a user-friendly and feature-rich command-line interface.
    *   **Configuration:** `serde` and `toml` for robust and easy parsing of configuration files.
    *   **Logging:** `tracing` for structured, asynchronous-aware logging.
*   **Protocol:** A custom, lightweight UDP-based protocol will be designed for the tunnel, focusing on low overhead and the essential features of sequencing and authentication.
*   **Concurrency Model:** The architecture will be heavily task-based. The client will spawn async tasks for reading from the TUN device, monitoring each network path, and sending/receiving data on each socket. The server will use a similar model to manage client sessions in parallel.

### 7. Success Metrics

*   **Performance:** The bonded tunnel achieves at least 80% of the theoretical combined bandwidth of all links under ideal conditions.
*   **Reliability:** A running `ping -t` or video stream does not drop a single packet when a secondary link is unplugged.
*   **Efficiency:** The `onebox-client` process consumes less than 20% of a single CPU core on a Raspberry Pi 4 when saturating a 100 Mbps connection.
*   **Adoption:** The project gains traction on GitHub (stars, forks) and sees community contributions in the form of issues and pull requests.

### 8. Future Work & Roadmap (Post v1.0)

*   **Web Dashboard:** A simple web interface (served locally by the client) for real-time monitoring and configuration.
*   **Advanced Bonding Modes:** Implement different policies, such as "reliability first" (duplicates critical packets over multiple links) or "cost-aware" (prioritizes unmetered connections over cellular).
*   **WireGuard Integration:** Explore using WireGuard as the underlying transport for its proven security and performance, while still managing the multipath logic.
*   **Cross-Platform Client:** Expand client support to other operating systems like macOS and Windows.

### 9. Out of Scope (for v1.0)

*   **Graphical User Interface (GUI):** v1.0 will be CLI-only.
*   **Managed "Turnkey" Service:** Users are responsible for providing their own VPS for the server component.
*   **Complex QoS / Traffic Shaping:** The initial version will not support intricate rules for prioritizing specific types of traffic.
