# onebox-rs Implementation Task List

## Overview
This document contains the complete list of tasks required to implement the onebox-rs internet bonding solution according to the Software Requirements Specification (SRS.md) and Product Requirements Document (PRD.md).

## Task Status Legend
- **To Do**: Task not yet started
- **In Progress**: Task currently being worked on
- **Done**: Task completed and tested
- **Blocked**: Task cannot proceed due to dependencies

## Implementation Tasks

### Phase 1: Project Foundation & Core Infrastructure

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T0** | **Project Scaffolding**: Create a Cargo workspace with `onebox-core` (lib), `onebox-client` (bin), and `onebox-server` (bin). Add initial dependencies (`tokio`, `tracing`, `nix`, `clap`, `serde`, `tokio-tun`). | C-1 | N/A | `Done` | High |
| **T1** | **Core Data Structures**: In `onebox-core`, define the `PacketHeader` (with sequence number placeholder) and other core data structures. | FR-C-05 | N/A | `Done` | High |
| **T2** | **Configuration System**: Implement configuration loading from `config.toml` for both client and server using `serde`. | SI-2 | N/A | `Done` | High |
| **T3** | **CLI Framework**: Implement the basic CLI structure for both client and server using `clap`. | UI-1 to UI-5 | N/A | `Done` | Medium |

### Phase 2: Basic Networking & TUN Interface

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T4** | **Basic UDP Server**: In `onebox-server`, create a basic Tokio UDP server that listens on a port and logs any data it receives. | FR-S-02 | TS0.2 | `Done` | High |
| **T5** | **Basic UDP Client**: In `onebox-client`, create a basic client that sends a "Hello Onebox" message to the server's address. | FR-C-04 | TS0.3 | `Done` | High |
| **T6** | **Client TUN & Routing**: In `onebox-client`, implement TUN interface creation and modify the system routing table to capture traffic. | FR-C-01, FR-C-02, SI-1 | TS0.1 | `Done` | High |
| **T7** | **Server TUN & Forwarding**: In `onebox-server`, implement TUN interface creation and basic NAT/forwarding rules. | FR-S-01, FR-S-04 | N/A | `Done` | High |

### Phase 3: Core Bonding Engine

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T8** | **E2E Ping Tunnel**: Connect the client and server TUNs. Read a packet from the client TUN, send it to the server, and write it to the server TUN. Make `ping 8.8.8.8` succeed. | FR-C-02, FR-S-04 | TS1.1, TS1.2, TS1.3 | `Done` | High |
| **T9** | **Multi-Link Socket Binding**: In `onebox-client`, implement logic to discover all WAN interfaces and bind a dedicated UDP socket to each one. | FR-C-03, FR-C-04 | N/A | `Done` | High |
| **T10** | **Packet Distribution**: Implement a round-robin algorithm in the client to distribute outgoing packets across all active WAN sockets. | FR-C-06 | TS1.4 | `Done` | High |
| **T11** | **Sequencing & Reassembly**: Add a monotonic sequence number to the protocol header. Implement a reordering jitter buffer on the server. | FR-C-05, FR-S-03 | N/A | `Done` | High |

### Phase 4: Security & Authentication

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T12** | **Authentication & Encryption**: Implement PSK authentication and ChaCha20-Poly1305 encryption for all tunnel traffic. | NFR-SEC-01, NFR-SEC-02 | TS4.1, TS4.2 | `Done` | High |
| **T13** | **Secure Handshake**: Implement secure session establishment and key exchange between client and server. | NFR-SEC-01, NFR-SEC-02 | TS4.1 | `Done` | High |

### Phase 5: Link Health & Failover

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T14** | **Link Health Probing**: Implement the client-side keep-alive mechanism to measure link latency and loss. | FR-C-07 | N/A | `Done` | Medium |
| **T15** | **Failover Logic**: Implement the client-side logic to mark links as "Down" based on probe failures and remove them from the distribution pool. | FR-C-08 | TS2.1, TS2.3, TS5.1 | `Done` | Medium |
| **T16** | **Link Recovery Logic**: Implement the logic to probe "Down" links and mark them as "Up" upon successful recovery. | FR-C-08 | TS2.2 | `Done` | Medium |

### Phase 6: Performance & Optimization

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T17** | **Performance Profiling**: Profile the application under load and identify bottlenecks. | NFR-PERF-01, 02, 03 | TS3.1, TS3.2, TS3.3 | `Blocked` | Medium |
| **T18** | **Memory & CPU Optimization**: Optimize memory usage and CPU consumption to meet performance NFRs. | NFR-PERF-03 | TS3.3 | `Done` | Medium |
| **T19** | **Concurrency Optimization**: Optimize the async task model for better throughput and lower latency. | NFR-PERF-01, 02 | TS3.1, TS3.2 | `Done` | Medium |

### Phase 7: Testing & Quality Assurance

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T20** | **Unit Tests**: Implement comprehensive unit tests for all core modules with target coverage of 70%. | NFR-MAINT-01 | N/A | `Done` | Medium |
| **T21** | **Integration Tests**: Implement end-to-end integration tests covering all major functionality. | NFR-MAINT-01 | All | `Blocked` | Medium |
| **T22** | **Performance Tests**: Implement stress tests to validate performance requirements under load. | NFR-PERF-01, 02, 03 | TS3.1, TS3.2, TS3.3 | `Blocked` | Medium |
| **T23** | **Security Tests**: Implement tests to validate encryption and authentication requirements. | NFR-SEC-01, 02 | TS4.1, TS4.2 | `Done` | Medium |
| **T24** | **Failover Tests**: Implement tests to validate link failover and recovery scenarios. | NFR-REL-01, 02 | TS2.1, TS2.2, TS2.3, TS5.1 | `Done` | Medium |

### Phase 8: Documentation & Deployment

| ID | Task Description | Related SRS | Related Tests | Status | Priority |
|----|------------------|--------------|---------------|---------|----------|
| **T25** | **User Documentation**: Create comprehensive README.md with build, installation, and usage instructions. | N/A | N/A | `Done` | Low |
| **T26** | **Developer Documentation**: Document the codebase with clear doc comments and architecture overview. | NFR-MAINT-01 | N/A | `To Do` | Low |
| **T27** | **Deployment Scripts**: Create deployment scripts and Docker configurations for easy setup. | N/A | N/A | `To Do` | Low |
| **T28** | **Final Testing**: Perform a full regression test, including all stress, security, and long-duration stability tests. | All | All | `To Do` | High |

## Dependencies

### Critical Path Dependencies
- T0 → T1, T2, T3 (Project scaffolding must be done first)
- T1 → T11 (Core data structures needed for packet handling)
- T4, T5 → T8 (Basic networking must work before E2E tunnel)
- T6, T7 → T8 (TUN interfaces must be created before tunnel)
- T8 → T9, T10, T11 (Basic tunnel must work before advanced features)
- T12 → T13 (Encryption must be implemented before secure handshake)

### Testing Dependencies
- All implementation tasks → T20-T24 (Tests depend on implementation)
- T20-T24 → T28 (Final testing depends on all other tests)

## Success Criteria

A task is considered **Done** when:
1. ✅ Implementation is complete according to SRS requirements
2. ✅ Code passes `cargo fmt` and `clippy --deny warnings`
3. ✅ All related tests pass
4. ✅ Documentation is updated
5. ✅ Code review is completed (if applicable)

## Notes

- **Priority Levels**: High = Critical path, Medium = Important features, Low = Nice-to-have
- **Testing**: Each task should include corresponding test implementation
- **Code Quality**: All code must meet Rust best practices and project standards
- **Documentation**: Maintain clear doc comments and update relevant documentation
