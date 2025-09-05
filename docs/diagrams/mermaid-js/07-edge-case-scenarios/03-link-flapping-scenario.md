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
