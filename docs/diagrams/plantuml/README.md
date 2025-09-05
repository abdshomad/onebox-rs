# PlantUML Diagrams

This directory contains the source files for project diagrams created using the PlantUML syntax.

## 1. System Architecture

```plantuml
@startuml
!theme plain

title System Architecture

package "User's LAN" {
  [PC / Laptop] as PC
  [onebox-client] as Client
  PC --> Client : All Traffic
}

package "onebox-client Device" {
  Client --> TUNNEL_Client : Intercepts & Encapsulates
  interface "TUNNEL" as TUNNEL_Client
  TUNNEL_Client --> [WAN 1\n(e.g., Ethernet)] : Round-Robin
  TUNNEL_Client --> [WAN 2\n(e.g., Cellular)] : Round-Robin
}

package "Public Internet" {
  [WAN 1\n(e.g., Ethernet)] --> Server : Encrypted UDP
  [WAN 2\n(e.g., Cellular)] --> Server : Encrypted UDP
}

package "Cloud VPS" {
  [onebox-server] as Server
  Server --> TUNNEL_Server : Reassembles & Decrypts
  interface "TUNNEL" as TUNNEL_Server
  TUNNEL_Server --> [Internet]
}

@enduml
```

## 2. Packet Structure

```plantuml
@startuml
!theme plain

title Packet Structure

rectangle "UDP Datagram sent over a WAN link" {
  object "IP Header" as IPHeader
  object "UDP Header" as UDPHeader
  object "onebox Packet Header" as OneboxHeader
  object "Encrypted Payload" as EncryptedPayload

  IPHeader -> UDPHeader
  UDPHeader -> OneboxHeader
  OneboxHeader -> EncryptedPayload
}

rectangle "onebox Packet Header (Plaintext, Authenticated)" {
  object "PacketType\n(e.g., Data, Probe, Auth)" as PacketType
  object "ClientId" as ClientId
  object "SequenceNumber" as SequenceNumber
}

rectangle "Encrypted Payload (ChaCha20-Poly1305)" {
  object "Original IP Packet from TUN" as OriginalPacket
  object "16-byte Authentication Tag" as AuthTag
}

OneboxHeader *-- PacketType
OneboxHeader *-- ClientId
OneboxHeader *-- SequenceNumber

EncryptedPayload *-- OriginalPacket
EncryptedPayload *-- AuthTag

@enduml
```

## 3. Client Application Logic Flow

```plantuml
@startuml
!theme plain

title Client Application Logic Flow (Enhanced)

start
:Start onebox-client;
:Parse CLI Arguments;
if (Load config.toml) then (Success)
  :Discover WAN Interfaces & Bind Sockets;
  if (Create Virtual TUN Device) then (Success)
    :Set System Default Route to TUN Device;
    if (Perform Handshake with Server) then (Success)
      fork
        partition "Data Plane (Upstream)" {
          while (true)
            :Read IP Packet from TUN;
            :Add to send queue;
            :Encrypt Packet;
            :Select WAN link (Round-Robin);
            :Send Encrypted Packet over chosen WAN Socket;
          endwhile
        }
      fork again
        partition "Data Plane (Downstream)" {
          while (true)
            :Receive Encrypted Packet from any WAN Socket;
            :Decrypt Packet;
            :Write IP Packet to TUN;
          endwhile
        }
      fork again
        partition "Control Plane (Health Probers)" {
          :Periodically send probe packets;
          note right
            See [[client-health-check-subflow.puml]]
          end note
        }
      fork again
        partition "Control Plane (Status Socket)" {
          :Listen for status requests;
        }
      end fork
    else (Failure)
      :Log Handshake Error;
      stop
    endif
  else (Failure)
    :Log TUN Device Error;
    stop
  endif
else (Failure)
  :Log Config Error;
  stop
endif

stop

@enduml
```

## 4. Server Application Logic Flow

```plantuml
@startuml
!theme plain

title Server Application Logic Flow (Enhanced)

start
:Start onebox-server;
if (Parse CLI & Load Config) then (Success)
  if (Create TUN Device, Setup IP Forwarding & NAT) then (Success)
    if (Bind Public UDP Socket) then (Success)
      fork
        partition "Dispatcher" {
          while (true)
            :Listens on Public UDP Socket;
            :Receives all incoming packets;
            :Forwards packet to Worker Pool via channel;
          endwhile
        }
      fork again
        partition "Worker Pool" {
          :See [[server-packet-processing-subflow.puml]];
        }
      fork again
        partition "Downstream (TUN to UDP)" {
          while (true)
            :Read IP packet from TUN;
            :Find corresponding client;
            :Encrypt packet;
            :Send to client over UDP;
          endwhile
        }
      end fork
    else (Failure)
      :Log Socket Bind Error;
      stop
    endif
  else (Failure)
    :Log TUN/NAT Error;
    stop
  endif
else (Failure)
  :Log Config Error;
  stop
endif

stop

@enduml
```

## 5. Link Health State Machine

```plantuml
@startuml
!theme plain

title Link Health State Machine

[*] --> Unknown

Unknown --> Up : Successful Probe
Unknown --> Down : 4 Consecutive Probe Failures

Up --> Down : 4 Consecutive Probe Failures
Down --> Up : Successful Probe

@enduml
```

## 6. Configuration Schema

```plantuml
@startuml
!theme plain

title Configuration Schema (config.toml)

class Config {
  + log_level: String
  + preshared_key: String
}

class ClientConfig {
  + server_address: String
  + server_port: u16
  + tun_name: String
  + tun_ip: IpAddr
  + tun_netmask: IpAddr
}

class ServerConfig {
  + listen_address: String
  + listen_port: u16
}

Config "1" *-- "1" ClientConfig : contains
Config "1" *-- "1" ServerConfig : contains

@enduml
```
