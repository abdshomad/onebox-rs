```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: Malformed Packet
    S->>S: Attempt to Decrypt/Parse
    note over S: Silently drop packet
```
