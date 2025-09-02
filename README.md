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

onebox-rs consists of two main components:

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
│   ├── PRD.md             # Product Requirements Document
│   ├── SRS.md             # Software Requirements Specification
│   ├── TEST_PLAN.md       # Test scenarios and validation
│   └── TASK_LIST.md       # Implementation roadmap
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

The project includes comprehensive test scenarios covering:

- **Basic Connectivity**: End-to-end ping tests
- **Failover Scenarios**: Link failure and recovery
- **Performance**: Throughput and latency measurements
- **Security**: Authentication and encryption validation
- **Stress Testing**: High-load scenarios

See `docs/TEST_PLAN.md` for detailed test procedures.

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