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
