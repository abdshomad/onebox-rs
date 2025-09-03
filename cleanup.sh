#!/bin/bash
echo "--- Forceful Cleanup ---"
# Kill processes
sudo killall -9 onebox-server onebox-client 2>/dev/null || true

# Delete namespaces
sudo ip netns del client 2>/dev/null || true
sudo ip netns del server 2>/dev/null || true

# Delete bridges
sudo ip link set br-wan0 down 2>/dev/null || true
sudo ip link set br-wan1 down 2>/dev/null || true
sudo ip link set br-public down 2>/dev/null || true
sudo ip link del br-wan0 2>/dev/null || true
sudo ip link del br-wan1 2>/dev/null || true
sudo ip link del br-public 2>/dev/null || true

# Delete veth pairs that might be lingering on the host
sudo ip link del v-peer-client0 2>/dev/null || true
sudo ip link del v-peer-client1 2>/dev/null || true
sudo ip link del v-peer-server 2>/dev/null || true

echo "Cleanup finished."
