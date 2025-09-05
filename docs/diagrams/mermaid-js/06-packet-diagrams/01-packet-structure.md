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
