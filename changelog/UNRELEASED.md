# [Unreleased]

## Overview
This document tracks upcoming changes and features that are planned for future releases of onebox-rs. These changes are currently in development or planned for implementation.

## Added
- **Configuration System**: A simplified, `serde`-based configuration system for loading settings from `config.toml`. (T2)

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

## Deprecated
- N/A

## Removed
- N/A

## Fixed
- N/A

## Security
- N/A

## Development Status

### Phase 1: Project Foundation & Core Infrastructure (Completed)
- **T0**: Project Scaffolding - `Done`
- **T1**: Core Data Structures - `Done`
- **T2**: Configuration System - `Done`
- **T3**: CLI Framework - `To Do`

### Phase 2: Basic Networking & TUN Interface (Planned)
- **T4**: Basic UDP Server - `To Do`
- **T5**: Basic UDP Client - `To Do`
- **T6**: Client TUN & Routing - `To Do`
- **T7**: Server TUN & Forwarding - `To Do`

### Phase 3: Core Bonding Engine (Planned)
- **T8**: E2E Ping Tunnel - `To Do`
- **T9**: Multi-Link Socket Binding - `To Do`
- **T10**: Packet Distribution - `To Do`
- **T11**: Sequencing & Reassembly - `To Do`

### Phase 4: Security & Authentication (Planned)
- **T12**: Authentication & Encryption - `To Do`
- **T13**: Secure Handshake - `To Do`

### Phase 5: Link Health & Failover (Planned)
- **T14**: Link Health Probing - `To Do`
- **T15**: Failover Logic - `To Do`
- **T16**: Link Recovery Logic - `To Do`

### Phase 6: Performance & Optimization (Planned)
- **T17**: Performance Profiling - `To Do`
- **T18**: Memory & CPU Optimization - `To Do`
- **T19**: Concurrency Optimization - `To Do`

### Phase 7: Testing & Quality Assurance (Planned)
- **T20**: Unit Tests - `To Do`
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
