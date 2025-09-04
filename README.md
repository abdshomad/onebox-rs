# onebox-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)](https://www.linux.org)

**High-performance, secure internet bonding solution built in Rust**

onebox-rs aggregates multiple internet connections into a single, resilient virtual connection, providing faster speeds and seamless failover for individuals and small businesses.

## 🚀 Features

- **🔄 Internet Bonding**: Combine multiple WAN connections (Wi-Fi, Ethernet, Cellular) for increased bandwidth
- **🛡️ Seamless Failover**: Automatic failover when connections drop, with zero packet loss
- **🔒 End-to-End Encryption**: ChaCha20-Poly1305 encryption for all tunnel traffic
- **📊 Link Health Monitoring**: Real-time monitoring of connection latency, jitter, and packet loss
- **⚡ High Performance**: Built with Rust and Tokio for minimal CPU overhead
- **🖥️ Cross-Platform**: Client runs on ARM64 (Raspberry Pi) and server on x86_64 (VPS)
- **🔧 Easy Configuration**: Simple TOML-based configuration
- **📱 CLI Interface**: Intuitive command-line tools for management and monitoring

## 🏗️ Architecture

onebox-rs consists of two main components: a client and a server. The client captures all network traffic from the local machine, sends it across multiple internet connections to the server, which then decrypts and forwards it to the public internet.

```mermaid
graph TD
    subgraph "User's LAN"
        A[PC / Laptop] -- All Traffic --> B(onebox-client);
    end

    subgraph "onebox-client Device"
        B -- Intercepts & Encapsulates --> C{TUNNEL};
        C -- Round-Robin --> D[WAN 1 <br> (e.g., Ethernet)];
        C -- Round-Robin --> E[WAN 2 <br> (e.g., Cellular)];
    end

    subgraph "Public Internet"
        D -- Encrypted UDP --> F{onebox-server};
        E -- Encrypted UDP --> F;
    end

    subgraph "Cloud VPS"
        F -- Reassembles & Decrypts --> G[TUNNEL];
        G -- Forwards --> H[Internet];
    end

    style B fill:#f9f,stroke:#333,stroke-width:2px
    style F fill:#ccf,stroke:#333,stroke-width:2px

    click F "#-protocol" "Go to Protocol Details"
    click B "#-common-workflows" "Go to Common Workflows"
```

### Client (`onebox-client`)
- Runs on Linux-based single-board computers (e.g., Raspberry Pi)
- Creates a TUN interface to capture all outgoing traffic
- Automatically discovers available WAN interfaces
- Distributes packets across multiple connections using round-robin algorithm
- Monitors link health and performs failover

### Server (`onebox-server`)
- Runs on cloud VPS with public IP
- Receives encrypted packets from clients
- Reassembles packets in correct order using sequence numbers
- Forwards traffic to the internet using NAT
- Handles multiple concurrent client connections

## 📦 Protocol

All traffic is sent as UDP datagrams. The payload of each UDP packet contains a custom `onebox` header followed by an encrypted payload.

```mermaid
graph LR
    subgraph "UDP Datagram sent over a WAN link"
        A[IP Header] --> B[UDP Header];
        B --> C{onebox Packet Header};
        C --> D[Encrypted Payload];
    end

    subgraph "onebox Packet Header (Plaintext, Authenticated)"
        direction TB
        H1[PacketType <br> e.g., Data, Probe, Auth];
        H2[ClientId];
        H3[SequenceNumber];
    end

    subgraph "Encrypted Payload (ChaCha20-Poly1305)"
        direction TB
        P1[Original IP Packet from TUN];
        P2[16-byte Authentication Tag];
    end

    C -- Contains --> H1;
    C -- Contains --> H2;
    C -- Contains --> H3;

    D -- Contains --> P1;
    D -- Contains --> P2;

    style P1 fill:#f99,stroke:#333,stroke-width:2px;
```

## ⚙️ Common Workflows

### Client Authentication

The client authenticates with the server using a simple handshake. This ensures that only clients with the correct Pre-Shared Key (PSK) can connect.

```mermaid
sequenceDiagram
    autonumber
    participant C as onebox-client
    participant S as onebox-server

    Note over C: Client starts up, discovers WAN links.
    C->>S: Sends Packet(Header{Type: AuthRequest})
    activate S

    Note over S: Server receives AuthRequest. <br> Decrypts payload to verify PSK.
    Note over S: If successful, marks client as Authenticated.

    S-->>C: Responds with Packet(Header{Type: AuthResponse})
    deactivate S

    C->>S: Begins sending data packets (PacketType::Data)
    S-->>C: Begins sending data packets (PacketType::Data)

    Note over C,S: Secure tunnel is now active.
```

### Link Failover

The client constantly monitors the health of each WAN link by sending probes. If a link becomes unresponsive, it is quickly removed from the pool of active links.

```mermaid
sequenceDiagram
    autonumber
    participant Client as onebox-client
    participant Server as onebox-server

    loop Health Probing (every 500ms)
        Client->>Server: Sends Packet(Header{Type: Probe}) for Link 1
        Server-->>Client: Echoes Probe Packet for Link 1

        Client->>Server: Sends Packet(Header{Type: Probe}) for Link 2
        Server-->>Client: Echoes Probe Packet for Link 2
    end

    Note over Client, Server: Suddenly, Link 2 becomes unresponsive.

    Client->>Server: Sends Packet(Header{Type: Probe}) for Link 2
    Note over Client: Probe times out (no echo from server)
    Note over Client: consecutive_failures = 1

    Client->>Server: Sends Packet(Header{Type: Probe}) for Link 2
    Note over Client: Probe times out
    Note over Client: consecutive_failures = 2

    Client->>Server: Sends Packet(Header{Type: Probe}) for Link 2
    Note over Client: Probe times out
    Note over Client: consecutive_failures = 3

    activate Client
    Note over Client: Failure threshold reached for Link 2.
    Client->>Client: Mark Link 2 as "Down"
    Client->>Client: Remove Link 2 from active packet distribution pool.
    deactivate Client

    Note over Client, Server: Client now only sends data over Link 1.
```

## 📋 Requirements

### Client Requirements
- Linux-based system (ARM64 or x86_64)
- Root privileges or `CAP_NET_ADMIN` capability
- Multiple network interfaces (Wi-Fi, Ethernet, Cellular, etc.)
- Rust 1.70+ toolchain

### Server Requirements
- Linux VPS with public IP address
- Root privileges
- Open UDP port (configurable)
- Rust 1.70+ toolchain

## 🛠️ Installation

### Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install system dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install build-essential pkg-config libssl-dev
   
   # CentOS/RHEL
   sudo yum groupinstall "Development Tools"
   sudo yum install openssl-devel
   ```

### Building from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/onebox-rs.git
   cd onebox-rs
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Install binaries** (optional):
   ```bash
   cargo install --path .
   ```

## ⚙️ Configuration

### Client Configuration (`config.toml`)

Create a `config.toml` file in the client's working directory:

```toml
[client]
server_address = "your-server-ip.com"
server_port = 8080
preshared_key = "your-secure-preshared-key"
log_level = "info"

[client.tun]
name = "onebox0"
ip = "10.0.0.2"
netmask = "255.255.255.0"
```

### Server Configuration (`config.toml`)

Create a `config.toml` file in the server's working directory:

```toml
[server]
listen_address = "0.0.0.0"
listen_port = 8080
preshared_key = "your-secure-preshared-key"
log_level = "info"

[server.tun]
name = "onebox0"
ip = "10.0.0.1"
netmask = "255.255.255.0"
```

## 🚀 Usage

### Starting the Server

1. **On your VPS**:
   ```bash
   sudo ./target/release/onebox-server start
   ```

2. **Check server status**:
   ```bash
   ./target/release/onebox-server status
   ```

### Starting the Client

1. **On your local machine/Raspberry Pi**:
   ```bash
   sudo ./target/release/onebox-client start
   ```

2. **Check client status**:
   ```bash
   ./target/release/onebox-client status
   ```

3. **Stop the client**:
   ```bash
   sudo ./target/release/onebox-client stop
   ```

### Verifying the Connection

Test your bonded connection:

```bash
# Test basic connectivity
ping 8.8.8.8

# Test bandwidth
curl -o /dev/null http://speedtest.tele2.net/100MB.zip

# Check routing
ip route show
```

## 🔧 Development

### Project Structure

```
onebox-rs/
├── Cargo.toml              # Workspace configuration
├── onebox-core/            # Core library (shared types, protocol)
├── onebox-client/          # Client binary
├── onebox-server/          # Server binary
├── docs/                   # Documentation
│   ├── diagrams/
│   │   ├── 01-overview/
│   │   ├── 02-protocol/
│   │   ├── 03-workflows/
│   │   └── 04-testing/
│   ├── PRD.md
│   ├── SRS.md
│   ├── TEST_PLAN.md
│   └── TASK_LIST.md
└── README.md               # This file
```

### Building for Development

```bash
# Development build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## 🧪 Testing

The project includes a comprehensive integration test suite that runs in an isolated network environment created with network namespaces.

### Test Environment Topology

The `setup_net_env.sh` script creates a virtual network of bridges and namespaces to simulate the client, the server, and the public internet. This allows for end-to-end testing without requiring a real cloud VPS or multiple physical network connections.

```mermaid
graph TD
    subgraph "Host Machine"
        direction LR
        br_wan0["Bridge: br-wan0 <br> (Gateway: 192.168.10.1)"]
        br_wan1["Bridge: br-wan1 <br> (Gateway: 192.168.20.1)"]
        br_public["Bridge: br-public <br> (Gateway: 10.0.0.1)"]
    end

    subgraph "client Namespace"
        direction TB
        subgraph "Virtual Interfaces"
            client_wan0["veth: wan0 <br> (192.168.10.2)"]
            client_wan1["veth: wan1 <br> (192.168.20.2)"]
        end
        client_app["onebox-client Process"]
    end

    subgraph "server Namespace"
        direction TB
        server_eth0["veth: eth0 <br> (10.0.0.3)"]
        server_app["onebox-server Process"]
    end

    subgraph "internet_endpoint Namespace"
        direction TB
        inet_eth0["veth: eth0 <br> (10.0.0.88)"]
        inet_sim["Simulated Web Server"]
    end

    client_wan0 -- "Virtual Cable" --- br_wan0
    client_wan1 -- "Virtual Cable" --- br_wan1
    server_eth0 -- "Virtual Cable" --- br_public
    inet_eth0 -- "Virtual Cable" --- br_public

    style client_app fill:#f9f
    style server_app fill:#ccf
    style inet_sim fill:#9f9
```

### Running Tests

To run all tests, including the integration tests, use the following command. The `--test-threads=1` flag is required to run the integration tests sequentially, as they manipulate shared network resources.

```bash
cargo test -- --test-threads=1
```

For more detailed test procedures, see `docs/TEST_PLAN.md`.

## 📚 Documentation

- **[Product Requirements Document](docs/PRD.md)**: High-level vision and goals
- **[Software Requirements Specification](docs/SRS.md)**: Detailed technical specifications
- **[Test Plan](docs/TEST_PLAN.md)**: Testing scenarios and validation procedures
- **[Task List](docs/TASK_LIST.md)**: Implementation roadmap and progress

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding standards
- Ensure all tests pass
- Update documentation as needed
- Use conventional commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software requires root privileges to create TUN interfaces and modify routing tables. Use at your own risk and ensure you understand the security implications of running network-level software with elevated privileges.

## 🆘 Support

- **Issues**: Report bugs and feature requests on [GitHub Issues](https://github.com/yourusername/onebox-rs/issues)
- **Discussions**: Join community discussions on [GitHub Discussions](https://github.com/yourusername/onebox-rs/discussions)
- **Wiki**: Check our [Wiki](https://github.com/yourusername/onebox-rs/wiki) for additional documentation

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org) and [Tokio](https://tokio.rs)
- Inspired by existing internet bonding solutions
- Community contributors and testers

---

**Made with ❤️ in Rust**

## 🐳 Docker Compose Simulation

You can simulate client-server communication locally using Docker Compose.

1. Build the image:
```bash
docker compose build
```

2. Start the server and client on an isolated bridge network:
```bash
docker compose up -d
```

3. Inspect logs:
```bash
docker compose logs -f server
docker compose logs -f client
```

4. Send another test datagram from the client container:
```bash
docker compose exec client /usr/local/bin/onebox-client --config /home/onebox/config.docker.client.toml start --foreground
```

Notes:
- The compose file uses a custom bridge network with static IPs:
  - Server: `172.28.0.2:8080/udp`
  - Client: `172.28.0.3`
- Client and server load their configs from `config.docker.*.toml` mounted read-only.
- The current client sends a single UDP datagram ("Hello Onebox") to the server. The server logs received datagrams.

## 🔬 Manual Testing

### Option A: With Docker Compose (Recommended)

Prereqs: Docker and Docker Compose plugin installed.

1) Build images
```bash
docker compose build
```

2) Start services
```bash
docker compose up -d
```

3) View logs
```bash
# Server logs (should show "UDP server listening on ..." and "Received ... bytes from ...")
docker compose logs -f server

# Client logs (should show it sent a datagram)
docker compose logs -f client
```

4) Trigger an additional client send
```bash
docker compose exec client /usr/local/bin/onebox-client \
  --config /home/onebox/config.docker.client.toml start --foreground
```

5) Stop services
```bash
docker compose down -v
```

### Option B: Local Binaries (No Docker)

Prereqs: Rust toolchain installed (rustup), Linux environment.

1) Build
```bash
cargo build
```

2) Start server (foreground)
```bash
RUST_LOG=info ./target/debug/onebox-server start --foreground
```

3) In another terminal, run client once
```bash
RUST_LOG=info ./target/debug/onebox-client --config ./config.toml start --foreground
```

Expected:
- Server prints that it is listening and logs a received datagram of 12 bytes from the client.
- Client prints that it sent 12 bytes and exits.