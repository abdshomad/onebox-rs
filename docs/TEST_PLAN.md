## Test Plan & Scenarios: onebox-rs v1.0

### Test Environment Setup

*   **Client Host:** A virtual machine or Raspberry Pi 4/5 running a 64-bit Linux OS (e.g., Debian/Raspberry Pi OS). This host will have at least three virtual network interfaces: `eth0` (for management/SSH), `wan0`, and `wan1`.
*   **Server Host:** A cloud VPS or a separate virtual machine running Ubuntu 22.04 LTS with a publicly accessible IP address.
*   **Network Simulation:** The `wan0` and `wan1` interfaces on the client will connect to separate simulated networks. The `tc` command-line tool with `netem` will be used to introduce artificial latency, packet loss, and bandwidth limits to simulate real-world WAN conditions.
*   **Tooling:** `iperf3` for performance testing, `ping` and `curl` for connectivity testing, `tcpdump`/Wireshark for packet inspection, and `htop`/`ps` for resource monitoring.

---

### Level 0: Sanity & "Hello Onebox" Smoke Tests

This level ensures the application binaries can start, communicate at the most basic level, and set up their required virtual interfaces without errors.

*   **TS0.1: Client Startup & TUN Creation**
    *   **Action:** Run `sudo ./onebox-client start` on the client host.
    *   **Expected Result:**
        1.  The process starts and runs without crashing.
        2.  A new virtual network interface (e.g., `onebox0`) is created.
        3.  `ip addr show onebox0` shows the interface is UP and has a private IP address assigned from the config.
        4.  `ip route` shows a new default route pointing to the `onebox0` interface.

*   **TS0.2: Server Startup & TUN Creation**
    *   **Action:** Run `sudo ./onebox-server start` on the server host.
    *   **Expected Result:**
        1.  The process starts and runs without crashing.
        2.  A new virtual network interface (e.g., `onebox0`) is created.
        3.  The server process is listening for UDP traffic on its configured public port.

*   **TS0.3: Basic Authenticated Handshake ("Hello")**
    *   **Action:** With both client and server running and correctly configured with the same Pre-Shared Key (PSK), observe the client's logs.
    *   **Expected Result:**
        1.  Client logs show that it has discovered `wan0` and `wan1`.
        2.  Client logs indicate it is sending initial handshake or keep-alive packets to the server.
        3.  Server logs show a "New client connected" message.
        4.  Client `status` command shows both links as "Up" with measurable latency. **This is the successful "Hello Onebox"**.

---

### Level 1: Core Functional Tests

This level validates the primary requirements defined in the SRS: routing traffic through the tunnel.

*   **TS1.1: Upstream ICMP Traffic (Ping)**
    *   **Action:** On the client host's command line, run `ping 8.8.8.8`.
    *   **Expected Result:** The ping should succeed. ICMP request packets are routed to the `onebox0` TUN, tunneled to the server, and sent to the internet. The ICMP replies follow the reverse path.

*   **TS1.2: Upstream TCP Traffic (Web Browsing)**
    *   **Action:** On the client host, run `curl https://google.com`.
    *   **Expected Result:** The command successfully downloads the HTML content for Google's homepage.

*   **TS1.3: Upstream UDP Traffic (DNS)**
    *   **Action:** On the client host, run `dig @8.8.8.8 A google.com`.
    *   **Expected Result:** The command successfully resolves the domain name, proving UDP traffic is being tunneled correctly.

*   **TS1.4: Multi-Link Packet Distribution**
    *   **Action:** While running a continuous data transfer (e.g., `iperf3`), use `tcpdump` on both `wan0` and `wan1` on the client.
    *   **Expected Result:** `tcpdump` output should show encrypted UDP traffic flowing out of **both** interfaces, confirming the round-robin distribution is active.

---

### Level 2: Reliability & Failover Tests

This level tests the system's resilience to common network problems.

*   **TS2.1: Hard Link Failure**
    *   **Action:** Start a large file download or a long-running `iperf3` test. While it's running, disable one of the WAN interfaces using `sudo ifdown wan1`.
    *   **Expected Result:**
        1.  The data transfer should pause for no more than 2-3 seconds.
        2.  The transfer must resume automatically over the remaining active link (`wan0`).
        3.  The `onebox-client status` command should show `wan1` as "Down".
        4.  The transfer must complete successfully, albeit at a lower speed.

*   **TS2.2: Link Recovery**
    *   **Action:** Following TS2.1, re-enable the failed interface with `sudo ifup wan1`.
    *   **Expected Result:**
        1.  Within a few seconds, `onebox-client status` should show `wan1` as "Up" again.
        2.  The data transfer's aggregate speed should increase as the client begins distributing packets over the recovered link.

*   **TS2.3: Graceful Degradation (High Latency)**
    *   **Action:** While a transfer is running, use `tc` to add 300ms of latency to `wan1`.
    *   **Expected Result:** The connection must not drop. The overall throughput may decrease, and `onebox-client status` should reflect the higher latency for `wan1`. The system remains stable.

---

### Level 3: Performance & Load Tests

This level validates the non-functional requirements related to speed and resource usage.

*   **TS3.1: Bandwidth Aggregation Throughput**
    *   **Action:**
        1.  On the client, use `tc` to limit both `wan0` and `wan1` to 50 Mbps each.
        2.  Run an `iperf3` test from the client host to a public `iperf3` server.
    *   **Expected Result:** The measured throughput must be greater than 80 Mbps (80% of the 100 Mbps theoretical maximum).

*   **TS3.2: Latency Overhead Measurement**
    *   **Action:**
        1.  Ping the server host's public IP directly from the client's `wan0` interface. Record the average RTT.
        2.  Ping the same IP address through the `onebox-rs` tunnel (e.g., `ping <server_ip>`). Record the average RTT.
    *   **Expected Result:** The difference between the two RTTs (the overhead) must not exceed 10ms.

*   **TS3.3: Client Resource Usage**
    *   **Action:** While running the `iperf3` test from TS3.1, monitor the `onebox-client` process on the Raspberry Pi using `htop`.
    *   **Expected Result:** The process's CPU usage must remain below 20% of a single core, and memory usage should be stable.

*   **TS3.4: Long-Duration Stability Stress Test**
    *   **Action:** Run a continuous `iperf3` test at 80% of maximum capacity for 1 hour.
    *   **Expected Result:** The system must remain stable with no crashes, memory leaks, or significant performance degradation over the test period.

---

### Level 4: Security Tests

This level validates the security and integrity of the tunnel.

*   **TS4.1: Authentication Rejection (Invalid PSK)**
    *   **Action:** Configure the client with a PSK that does not match the server's PSK. Start the client.
    *   **Expected Result:**
        1.  The client should fail to establish a session.
        2.  The server log should show "authentication failure" or "unknown client" messages.
        3.  No traffic should be able to pass through the tunnel. `ping 8.8.8.8` must fail.

*   **TS4.2: Data Confidentiality Verification**
    *   **Action:** Start a file transfer. Use Wireshark to capture traffic on the client's `wan0` interface.
    *   **Expected Result:** Inspecting the UDP packets should show that the payload is fully encrypted and unreadable. There should be no plaintext data from the original IP packets visible.

*   **TS4.3: Malformed Packet Resilience (Fuzzing)**
    *   **Action:** Use a simple script to send random, malformed UDP packets to the server's public port.
    *   **Expected Result:** The `onebox-server` process must not crash. It should silently drop the invalid packets and continue serving the legitimate client.

---

### Level 5: Advanced & Edge Case Scenarios

*   **TS5.1: Flapping Link Instability**
    *   **Action:** Create a script that brings a WAN interface (`wan1`) up and down every 10 seconds for 5 minutes.
    *   **Expected Result:** The `onebox-client` process must handle this instability without crashing. It should correctly mark the link's status in real-time and adjust its packet distribution accordingly.

*   **TS5.2: Concurrent Client Connection**
    *   **Action:** Set up a second `onebox-client` instance on a different machine, configured with the same PSK and pointing to the same server.
    *   **Expected Result:** The server should be able to handle both clients simultaneously, keeping their sessions and traffic isolated. The `onebox-server status` command should list both connected clients.