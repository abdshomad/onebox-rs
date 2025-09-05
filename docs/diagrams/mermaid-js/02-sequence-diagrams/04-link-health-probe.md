```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    loop For each WAN link
        C->>S: Health Probe
        S-->>C: Probe ACK
    end
```
