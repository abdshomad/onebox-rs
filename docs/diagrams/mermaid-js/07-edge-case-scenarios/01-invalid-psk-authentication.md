```mermaid
sequenceDiagram
    participant C as onebox-client
    participant S as onebox-server
    C->>S: AuthRequest(Invalid PSK)
    S->>S: Verify PSK
    S-->>C: AuthResponse(Failure)
    note over C,S: Server rejects connection
```
