## Software Requirements Specification: onebox-rs

| **Product Name:** | onebox-rs | **Version:** | 1.0 |
| :--- | :--- | :--- | :--- |
| **Author:** | Gemini AI | **Date:** | September 2, 2025 |
| **Status:** | Final Draft | | |

### 1. Introduction

#### 1.1 Purpose
This document provides a detailed specification for the `onebox-rs` software system. It describes the system's functional and non-functional requirements, its interfaces, and its operational constraints. This SRS is intended for the development team to guide implementation, for the QA team to create test plans, and for stakeholders to have a definitive understanding of the final product.

#### 1.2 Scope
The `onebox-rs` system is a client-server internet bonding solution. The scope of this document covers two distinct software components:
1.  **`onebox-client`**: A Rust application designed to run on a Linux-based Single-Board Computer (SBC). It captures local network traffic and distributes it across multiple available Wide Area Network (WAN) interfaces.
2.  **`onebox-server`**: A Rust application designed to run on a cloud-based Virtual Private Server (VPS). It receives traffic from the client, reassembles it, and forwards it to the public internet.

This document specifies the behavior of both components and the protocol used for their communication. Management of physical network connections (e.g., connecting to Wi-Fi) is outside the scope of this software and is assumed to be handled by the underlying operating system.

#### 1.3 Definitions, Acronyms, and Abbreviations
*   **AEAD**: Authenticated Encryption with Associated Data
*   **CLI**: Command-Line Interface
*   **LAN**: Local Area Network
*   **NAT**: Network Address Translation
*   **SBC**: Single-Board Computer (e.g., Raspberry Pi)
*   **SRS**: Software Requirements Specification
*   **TCP**: Transmission Control Protocol
*   **TUN**: A virtual network kernel interface that simulates a point-to-point connection.
*   **UDP**: User Datagram Protocol
*   **VPS**: Virtual Private Server
*   **WAN**: Wide Area Network

#### 1.4 References
*   `onebox-rs` Product Requirements Document v1.0

### 2. Overall Description

#### 2.1 Product Perspective
`onebox-rs` is a self-contained, self-hosted system composed of two cooperating executables. It functions as a specialized, high-performance virtual private network (VPN) focused on multipath data transmission. It relies on the Linux kernel for low-level networking tasks, including traffic routing to its virtual TUN interface and management of physical network hardware.

#### 2.2 Product Functions
*   Transparently intercept all outgoing traffic from its host machine (the client).
*   Encapsulate and encrypt IP packets using a custom UDP-based protocol.
*   Distribute encapsulated packets across all available and healthy WAN interfaces.
*   Receive, decrypt, and reassemble packets in the correct order on the server.
*   Forward reassembled packets to the public internet.
*   Perform the reverse process for downstream traffic from the internet back to the client.
*   Monitor link health and perform seamless connection failover.

#### 2.3 User Characteristics
The target user is technically proficient, comfortable with the Linux command line, and capable of provisioning and managing a cloud VPS. Users include network administrators, remote workers, and technology enthusiasts.

#### 2.4 Constraints
*   **C-1**: The software shall be written in the Rust programming language (latest stable toolchain).
*   **C-2**: `onebox-client` shall be compatible with `aarch64` Linux systems (e.g., Raspberry Pi OS 64-bit).
*   **C-3**: `onebox-server` shall be compatible with `x86_64` Linux systems (e.g., Ubuntu 22.04 LTS).
*   **C-4**: The software requires root privileges or the `CAP_NET_ADMIN` capability to create and configure TUN interfaces and manipulate routing tables.
*   **C-5**: The server component requires a publicly accessible, static IP address and an open UDP port.
*   **C-6**: The system architecture is strictly client-server; direct client-to-client communication is not supported.

### 3. Specific Requirements

#### 3.1 External Interface Requirements

##### 3.1.1 User Interfaces
The system shall provide a Command-Line Interface (CLI) for both client and server.
*   **UI-1**: `onebox-client start [--config <path>]`: Starts the client service.
*   **UI-2**: `onebox-client stop`: Stops the client service.
*   **UI-3**: `onebox-client status`: Displays a real-time table of all detected WAN links with their current status (Up/Down), latency (ms), packet loss (%), and throughput (kbps).
*   **UI-4**: `onebox-server start [--config <path>]`: Starts the server service.
*   **UI-5**: `onebox-server status`: Displays a list of connected clients and their aggregated throughput.

##### 3.1.2 Software Interfaces
*   **SI-1 (OS Interface)**: The system shall interface with the Linux kernel via system calls to:
    *   Create, configure, and manage a virtual TUN network interface.
    *   Enumerate available network interfaces.
    *   Create and bind UDP sockets to specific network interfaces (using the `SO_BINDTODEVICE` socket option).
    *   Manipulate the system routing table to direct traffic through the TUN interface.
*   **SI-2 (Configuration)**: Both client and server shall be configured via a `config.toml` file.
    *   **Client Config Fields:** `server_address`, `server_port`, `preshared_key`, `log_level`.
    *   **Server Config Fields:** `listen_address`, `listen_port`, `preshared_key`, `log_level`.

#### 3.2 Functional Requirements

##### 3.2.1 FR-CLIENT: `onebox-client`
*   **FR-C-01: Initialization**: Upon startup, the client shall read its configuration file, create and configure a TUN interface with a private IP address, and update the system's routing table to make the TUN device the default gateway.
*   **FR-C-02: Traffic Interception**: The client shall continuously read raw IP packets from the TUN interface's file descriptor.
*   **FR-C-03: WAN Discovery**: The client shall periodically scan for all non-loopback network interfaces that have a public or carrier-grade NAT IP address assigned. Each discovered interface shall be considered a potential WAN link.
*   **FR-C-04: Socket Management**: For each valid WAN link, the client shall create a dedicated UDP socket and bind it explicitly to that network interface.
*   **FR-C-05: Protocol Encapsulation**: Each IP packet read from the TUN interface shall be encapsulated in a custom UDP payload. The header for this payload shall include:
    *   A 64-bit monotonic sequence number.
    *   A client identifier.
*   **FR-C-06: Packet Distribution**: The client shall distribute encapsulated packets across all active WAN sockets. The default v1.0 algorithm shall be round-robin.
*   **FR-C-07: Link Health Probing**: The client shall send small, periodic keep-alive packets over each WAN link. It shall measure the round-trip time and packet loss for each link based on the server's acknowledgments.
*   **FR-C-08: Link Failover**: If a link fails to receive keep-alive acknowledgments for a configurable threshold (e.g., 3 consecutive probes), it shall be marked as "Down" and removed from the packet distribution pool. It shall be marked "Up" again after a successful probe.
*   **FR-C-09: Downstream Traffic**: The client shall listen for incoming traffic on all its WAN sockets, decrypt the payloads, and write the inner IP packet to the TUN interface.

##### 3.2.2 FR-SERVER: `onebox-server`
*   **FR-S-01: Initialization**: Upon startup, the server shall read its configuration file, create and configure a TUN interface, and enable IP forwarding and NAT rules (e.g., via `iptables`) to masquerade traffic from the TUN interface to its primary public interface.
*   **FR-S-02: Client Session Management**: The server shall listen for incoming UDP packets on its public port. It shall maintain a hash map of active client sessions, identified by a unique ID derived from their pre-shared key or source IP.
*   **FR-S-03: Packet Reassembly**: For each client session, the server shall maintain a jitter buffer. Incoming packets shall be placed in the buffer according to their sequence number. The server will release packets in sequence to the TUN interface, handling out-of-order arrivals up to a certain window size.
*   **FR-S-04: Traffic Forwarding**: Decrypted, reassembled IP packets shall be written to the server's TUN interface, where the kernel's networking stack will route them to the public internet.
*   **FR-S-05: Downstream Forwarding**: IP packets arriving at the server's TUN interface from the internet (response traffic) shall be read by the server, encapsulated, encrypted, and sent back to the originating client via the UDP socket from which the client's last packet was received.

#### 3.3 Non-Functional Requirements

*   **NFR-PERF-01 (Throughput)**: The system shall achieve an aggregate throughput of at least 80% of the mathematical sum of all active links' individual throughputs, measured under ideal network conditions.
*   **NFR-PERF-02 (Latency)**: The additional latency introduced by the tunnel (encapsulation and encryption overhead) shall not exceed 10 milliseconds over the latency of the fastest available link.
*   **NFR-PERF-03 (Resource Usage)**: The `onebox-client` process shall consume less than 20% of a single core on a Raspberry Pi 4 while processing 100 Mbps of aggregated traffic.
*   **NFR-SEC-01 (Confidentiality & Integrity)**: All traffic within the tunnel shall be encrypted and authenticated using the ChaCha20-Poly1305 AEAD cipher.
*   **NFR-SEC-02 (Authentication)**: For v1.0, client and server shall authenticate each other using a symmetric Pre-Shared Key (PSK). Packets from unauthenticated sources shall be dropped.
*   **NFR-REL-01 (Failover Time)**: The system shall detect a link failure and cease using it within 2 seconds.
*   **NFR-REL-02 (Stability)**: Both client and server processes must be capable of running continuously for at least 30 days without memory leaks, crashes, or requiring a restart.
*   **NFR-MAINT-01 (Code Quality)**: The entire Rust codebase shall be formatted with `rustfmt`, pass `clippy --deny warnings`, and be accompanied by unit and integration tests with a target code coverage of 70%.