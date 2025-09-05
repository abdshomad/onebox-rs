# Mermaid.js Diagrams for onebox-rs

This directory contains a comprehensive set of diagrams for the `onebox-rs` project, created using the [Mermaid.js](https://mermaid-js.github.io/mermaid/#/) syntax.

## 1. Flowcharts

### 1.1. Client Application Logic Flow

This flowchart illustrates the high-level logic of the `onebox-client` application, from startup to the main processing loop.

```mermaid
graph TD
    A[Start onebox-client] --> B{Load config.toml};
    B -- Success --> C[Discover WAN Interfaces & Bind Sockets];
    B -- Failure --> D[Log Config Error];
    D --> E[Stop];
    C --> F{Create Virtual TUN Device};
    F -- Success --> G[Set System Default Route to TUN];
    F -- Failure --> H[Log TUN Device Error];
    H --> E;
    G --> I{Perform Handshake};
    I -- Success --> J[Enter Main Loop];
    I -- Failure --> K[Log Handshake Error];
    K --> E;
    J --> L[Read IP Packet from TUN];
    L --> M[Encrypt & Encapsulate];
    M --> N[Select WAN link];
    N --> O[Send Packet];
    O --> L;
```

### 1.2. Server Application Logic Flow

This flowchart illustrates the high-level logic of the `onebox-server` application.

```mermaid
graph TD
    A[Start onebox-server] --> B{Load config.toml};
    B -- Success --> C{Create TUN Device & Setup NAT};
    B -- Failure --> D[Log Config Error];
    D --> E[Stop];
    C -- Success --> F{Bind Public UDP Socket};
    C -- Failure --> G[Log TUN/NAT Error];
    G --> E;
    F -- Success --> H[Enter Main Loop];
    F -- Failure --> I[Log Socket Bind Error];
    I --> E;
    H --> J[Listen for Packets];
    J --> K{Packet Received};
    K -- Yes --> L[Process Packet];
    L --> J;
    K -- No --> J;
```

## 2. Sequence Diagrams

### 2.1. Authentication Handshake

This sequence diagram shows the interaction between the client and server during the initial authentication process.

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: AuthRequest(PSK)
    S->>S: Verify PSK
    alt Auth Successful
        S-->>C: AuthResponse(Success)
    else Auth Failed
        S-->>C: AuthResponse(Failure)
    end
```

### 2.2. Upstream Data Transfer

This sequence diagram illustrates how a data packet travels from the local network, through the client, to the server, and out to the internet.

```mermaid
sequenceDiagram
    participant LAN
    participant C as onebox-client
    participant S as onebox-server
    participant Internet
    LAN->>C: IP Packet
    C->>C: Encrypt & Encapsulate
    C->>S: Encrypted Packet
    S->>S: Decrypt & Reassemble
    S->>Internet: Original IP Packet
```

### 2.3. Downstream Data Transfer

This sequence diagram shows the reverse path of a data packet, from the internet back to the local network.

```mermaid
sequenceDiagram
    participant Internet
    participant S as onebox-server
    participant C as onebox-client
    participant LAN
    Internet->>S: IP Packet
    S->>S: Encrypt & Encapsulate
    S->>C: Encrypted Packet
    C->>C: Decrypt & Reassemble
    C->>LAN: Original IP Packet
```

### 2.4. Link Health Probe

This sequence diagram shows the keep-alive mechanism used to monitor the health of each WAN link.

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    loop For each WAN link
        C->>S: Health Probe
        S-->>C: Probe ACK
    end
```

## 3. State Diagram

### 3.1. Link Health State Machine

This state diagram models the different states of a WAN link based on the success or failure of health probes.

```mermaid
stateDiagram-v2
    [*] --> Unknown
    Unknown --> Up: Successful Probe
    Unknown --> Down: 4 Consecutive Probe Failures
    Up --> Down: 4 Consecutive Probe Failures
    Down --> Up: Successful Probe
```

## 4. Class Diagram

### 4.1. Configuration Schema

This class diagram shows the structure of the `config.toml` file and the relationship between its sections.

```mermaid
classDiagram
    class Config {
        +String log_level
        +String preshared_key
    }
    class ClientConfig {
        +String server_address
        +int server_port
        +String tun_name
        +String tun_ip
        +String tun_netmask
    }
    class ServerConfig {
        +String listen_address
        +int listen_port
    }
    Config "1" -- "1" ClientConfig
    Config "1" -- "1" ServerConfig
```

## 5. User Journey

### 5.1. CLI Usage Journey

This diagram illustrates the typical journey of a user setting up and running the `onebox-rs` application.

```mermaid
journey
    title onebox-rs CLI User Journey
    section Setup
      Download Binaries: 5: User
      Configure Server: 4: User
      Start Server: 3: User
      Configure Client: 4: User
    section Execution
      Start Client: 5: User
      Monitor Status: 4: User
    section Teardown
      Stop Client: 3: User
      Stop Server: 2: User
```

## 6. Packet Diagram

### 6.1. Packet Structure

This diagram shows the structure of a `onebox-rs` data packet as it is sent over a WAN link.

```mermaid
packet-beta
    "UDP Datagram" {
        "IP Header" {
            "Source IP": 16,
            "Destination IP": 16
        }
        "UDP Header" {
            "Source Port": 8,
            "Destination Port": 8
        }
        "onebox Packet Header" {
            "PacketType": 4,
            "ClientId": 12,
            "SequenceNumber": 16
        }
        "Encrypted Payload" {
            "Original IP Packet": "...",
            "Authentication Tag": 32
        }
    }
```

## 7. Edge Case Scenarios

These sequence diagrams illustrate how the system is expected to handle various edge cases and non-nominal conditions.

### 7.1. Invalid PSK Authentication

This diagram shows the server rejecting a client that provides an invalid Pre-Shared Key (PSK).

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: AuthRequest(Invalid PSK)
    S->>S: Verify PSK
    S-->>C: AuthResponse(Failure)
    note over C,S: Server rejects connection
```

### 7.2. Malformed Packet Handling

This diagram shows the server's behavior when it receives a malformed or undecipherable packet.

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: Malformed Packet
    S->>S: Attempt to Decrypt/Parse
    note over S: Silently drop packet
```

### 7.3. Link Flapping Scenario

This diagram illustrates how the client handles a "flapping" WAN link that is rapidly changing its state between up and down.

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    loop Link is flapping
        C->>S: Health Probe
        S-->>C: Probe ACK
        note over C: Mark Link Up
        C->>S: Health Probe
        note over C: Probe Timeout
        note over C: Mark Link Down
    end
```

### 7.4. Out-of-Order Packet Handling

This diagram shows how the server's jitter buffer handles packets that arrive out of order from different WAN links.

```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: Packet (Seq=3)
    C->>S: Packet (Seq=1)
    C->>S: Packet (Seq=2)
    S->>S: Add packets to Jitter Buffer
    S->>S: Reorder packets
    S->>S: Process Packet (Seq=1)
    S->>S: Process Packet (Seq=2)
    S->>S: Process Packet (Seq=3)
```

## 8. Architecture Diagram

### 8.1. System Context (C4)

This C4 diagram shows the high-level system context for the `onebox-rs` application.

```mermaid
C4Context
  title System Context diagram for onebox-rs
  Enterprise_Boundary(b, "onebox-rs System") {
    System(client, "onebox-client", "Intercepts traffic and distributes it across multiple WAN links")
    System(server, "onebox-server", "Receives traffic, reassembles it, and forwards it to the internet")
    Rel(client, server, "Sends encrypted packets over UDP")
  }
  System_Ext(user, "User", "A user of the onebox-rs system")
  System_Ext(internet, "Internet", "The public internet")

  Rel(user, client, "Uses the bonded internet connection")
  Rel(server, internet, "Forwards traffic to")
```
