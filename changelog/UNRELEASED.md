# [Unreleased]

## Overview
This document tracks upcoming changes and features that are planned for future releases of onebox-rs. These changes are currently in development or planned for implementation.

## Added

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
- N/A

## Deprecated
- N/A

## Removed
- N/A

## Fixed
- N/A

## Security
- N/A

## Development Status

### Phase 2: Basic Networking & TUN Interface (In Progress)
- **T4**: Basic UDP Server - Not started
- **T5**: Basic UDP Client - Not started  
- **T6**: Client TUN & Routing - Not started
- **T7**: Server TUN & Forwarding - Not started

### Phase 3: Core Bonding Engine (Planned)
- **T8**: E2E Ping Tunnel - Not started
- **T9**: Multi-Link Socket Binding - Not started
- **T10**: Packet Distribution - Not started
- **T11**: Sequencing & Reassembly - Not started

### Phase 4: Security & Authentication (Planned)
- **T12**: Authentication & Encryption - Not started
- **T13**: Secure Handshake - Not started

### Phase 5: Link Health & Failover (Planned)
- **T14**: Link Health Probing - Not started
- **T15**: Failover Logic - Not started
- **T16**: Link Recovery Logic - Not started

### Phase 6: Performance & Optimization (Planned)
- **T17**: Performance Profiling - Not started
- **T18**: Memory & CPU Optimization - Not started
- **T19**: Concurrency Optimization - Not started

### Phase 7: Testing & Quality Assurance (Planned)
- **T20**: Unit Tests - Not started
- **T21**: Integration Tests - Not started
- **T22**: Performance Tests - Not started
- **T23**: Security Tests - Not started
- **T24**: Failover Tests - Not started

### Phase 8: Documentation & Deployment (Planned)
- **T25**: User Documentation - Not started
- **T26**: Developer Documentation - Not started
- **T27**: Deployment Scripts - Not started
- **T28**: Final Testing - Not started

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
