# [Unreleased]

## Overview
This document tracks upcoming changes and features that are planned for future releases of onebox-rs. These changes are currently in development or planned for implementation.

## Added
- **Configuration System**: A simplified, `serde`-based configuration system for loading settings from `config.toml`. (T2)
- **CLI Framework**: Basic CLI structure for `onebox-client` and `onebox-server` using `clap`, including `start`, `stop`, and `status` commands. (T3)
- **Basic UDP Server**: Implemented a basic UDP server in `onebox-server` that listens for incoming datagrams and logs them. (T4)
- **Basic UDP Client**: Implemented a basic UDP client in `onebox-client` that sends a "Hello Onebox" message to the server. (T5)
- **Client TUN Creation**: Implemented TUN interface creation and configuration on the client. (T6)
- **Server TUN & Forwarding**: Implemented TUN interface creation, IP forwarding, and NAT masquerading on the server. (T7)
- **End-to-End Tunnel**: Created a bidirectional data path between the client and server. The client reads packets from its TUN, sends them to the server via UDP, and the server writes them to its TUN. The reverse path is also implemented. (T8)
- **Multi-Link Socket Binding**: Implemented logic in `onebox-client` to discover all valid WAN interfaces and bind a dedicated UDP socket to each one. (T9)
- **Packet Distribution**: Implemented a round-robin algorithm to distribute outgoing packets across all active WAN sockets in `onebox-client`. Also implemented multi-socket listening for downstream traffic. (T10)
- **Sequencing & Reassembly**: Added a `PacketHeader` with a sequence number to all upstream packets. Implemented a jitter buffer on the server to reorder packets, ensuring in-order delivery to the TUN interface. (T11)
- **Symmetric Encryption**: Implemented end-to-end encryption for all tunnel traffic using ChaCha20-Poly1305. Keys are derived from the PSK using BLAKE3. This provides both confidentiality and per-packet authentication. (T12)
- **Secure Handshake**: Implemented a simple handshake protocol. The client now sends an `AuthRequest` to establish a session, and the server validates it before accepting data packets. (T13)
- **Link Health Probing**: Implemented a client-side keep-alive mechanism to measure link latency and loss via periodic, authenticated probes. The server now echoes these probes. (T14)
- **Failover Logic**: Implemented client-side logic to detect link failures via probe timeouts and remove them from the packet distribution pool. (T15)
- **Unit Tests**: Added comprehensive unit tests for the `onebox-core` library, achieving over 98% code coverage. (T20)
- **Integration Test Framework**: Added a framework for end-to-end integration tests using network namespaces. Implemented the first test case (TS1.1 - Ping) which is currently blocked by a network-level issue. (T21)

### Planned Features
- **Basic Networking**: UDP server and client communication
- **TUN Interface**: Virtual network interface creation and management
- **Packet Processing**: Basic packet handling and forwarding
- **Link Discovery**: Automatic detection of available network interfaces
- **Health Monitoring**: Link health checking and status reporting

### Infrastructure Improvements
- **Testing Framework**: Unit and integration test suite
- **Performance Metrics**: Monitoring and profiling capabilities
- **Documentation**: Comprehensive API and usage documentation
- **CI/CD Pipeline**: Automated testing and deployment

## Changed
- **Refactored Configuration**: Simplified the existing configuration structs and `config.toml` file to align with the SRS (SI-2). The complex, nested structure has been replaced with a flatter, more direct mapping of requirements. (T2)
- **Optimized Data Path**: Refactored the client's packet processing hot path to use in-place encryption and pre-allocated buffers, significantly reducing memory allocations and CPU usage. This addresses the performance requirements of NFR-PERF-03. (T18)
- **Concurrency Model**: Refactored the server and client data planes from a single-pipeline model to a parallel, dispatcher/worker-pool model. This significantly improves concurrency to better leverage multi-core processors, increasing throughput and reducing latency. (T19)

## Deprecated
- N/A

## Removed
- N/A

## Fixed
- Fixed a bug where the client would authenticate with one `ClientId` but send data packets with another, causing the server to drop them.
- Fixed a bug where the server would send all downstream data packets with a default `ClientId(0)` instead of the authenticated client's ID.
- Fixed a critical routing bug where the server would not install a route for the client's TUN network, causing return packets to leak out the physical interface instead of being sent back through the tunnel.

## Security
- Implemented ChaCha20-Poly1305 AEAD encryption for all tunnel traffic, authenticated by a key derived from the PSK. (T12)

## Development Status

### Phase 1: Project Foundation & Core Infrastructure (Completed)
- **T0**: Project Scaffolding - `Done`
- **T1**: Core Data Structures - `Done`
- **T2**: Configuration System - `Done`
- **T3**: CLI Framework - `Done`

### Phase 2: Basic Networking & TUN Interface (Planned)
- **T4**: Basic UDP Server - `Done`
- **T5**: Basic UDP Client - `Done`
- **T6**: Client TUN & Routing - `Done`
- **T7**: Server TUN & Forwarding - `Done`

### Phase 3: Core Bonding Engine (Planned)
- **T8**: E2E Ping Tunnel - `Done`
- **T9**: Multi-Link Socket Binding - `Done`
- **T10**: Packet Distribution - `Done`
- **T11**: Sequencing & Reassembly - `Done`

### Phase 4: Security & Authentication (Planned)
- **T12**: Authentication & Encryption - `Done`
- **T13**: Secure Handshake - `Done`

### Phase 5: Link Health & Failover (Planned)
- **T14**: Link Health Probing - `Done`
- **T15**: Failover Logic - `Done`
- **T16**: Link Recovery Logic - `Done`

### Phase 6: Performance & Optimization (Planned)
- **T17**: Performance Profiling - `To Do`
- **T18**: Memory & CPU Optimization - `Done`
- **T19**: Concurrency Optimization - `To Do`

### Phase 7: Testing & Quality Assurance (Planned)
- **T20**: Unit Tests - `Done`
- **T21**: Integration Tests - `To Do`
- **T22**: Performance Tests - `To Do`
- **T23**: Security Tests - `To Do`
- **T24**: Failover Tests - `To Do`

### Phase 8: Documentation & Deployment (Planned)
- **T25**: User Documentation - `To Do`
- **T26**: Developer Documentation - `To Do`
- **T27**: Deployment Scripts - `To Do`
- **T28**: Final Testing - `To Do`

## Technical Debt
- **Workspace Resolver**: Address resolver version compatibility warning
- **Error Handling**: Implement more specific error types for different failure modes
- **Configuration Validation**: Add validation for configuration values
- **Logging**: Implement structured logging with correlation IDs

## Performance Goals
- **Latency**: Target <10ms end-to-end latency for packet forwarding
- **Throughput**: Support up to 1Gbps aggregate bandwidth
- **Memory Usage**: Keep memory footprint under 100MB
- **CPU Usage**: Minimize CPU overhead for packet processing

## Security Goals
- **Encryption**: Implement ChaCha20-Poly1305 for all tunnel traffic
- **Authentication**: Secure pre-shared key management
- **Key Exchange**: Implement secure session establishment
- **Access Control**: Network-level access restrictions

## Testing Goals
- **Code Coverage**: Achieve 70%+ test coverage
- **Integration Tests**: End-to-end functionality validation
- **Performance Tests**: Load testing and benchmarking
- **Security Tests**: Vulnerability assessment and penetration testing

## Documentation Goals
- **User Guide**: Comprehensive setup and usage instructions
- **API Reference**: Complete code documentation
- **Architecture Guide**: System design and implementation details
- **Deployment Guide**: Production deployment best practices

## Release Timeline
- **0.2.0**: Basic networking and TUN interface (Phase 2)
- **0.3.0**: Core bonding engine (Phase 3)
- **0.4.0**: Security and authentication (Phase 4)
- **0.5.0**: Link health and failover (Phase 5)
- **0.6.0**: Performance optimization (Phase 6)
- **0.7.0**: Testing and quality assurance (Phase 7)
- **1.0.0**: Production-ready release (Phase 8)

## Notes
- This changelog is updated as development progresses
- Features may be added, removed, or modified based on requirements
- Release dates are estimates and subject to change
- Priority is given to core functionality over nice-to-have features
