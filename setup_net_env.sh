#!/bin/bash

# Exit on any error
set -e

# --- Cleanup previous setup ---
echo "Cleaning up previous network namespaces and interfaces..."
sudo ip netns del client 2>/dev/null || true
sudo ip netns del server 2>/dev/null || true
# Bring down bridges before deleting to release interfaces
sudo ip link set br-wan0 down 2>/dev/null || true
sudo ip link set br-wan1 down 2>/dev/null || true
sudo ip link set br-public down 2>/dev/null || true
# Delete bridges
sudo ip link del br-wan0 2>/dev/null || true
sudo ip link del br-wan1 2>/dev/null || true
sudo ip link del br-public 2>/dev/null || true
# The veth pairs should be deleted when the namespaces/bridges are, but we can be explicit
sudo ip link del v-peer-client0 2>/dev/null || true
sudo ip link del v-peer-client1 2>/dev/null || true
sudo ip link del v-peer-server 2>/dev/null || true
echo "Cleanup complete."

# --- Assuming tools are pre-installed ---
echo "Skipping tool installation. Assuming iperf3 and iproute2 are available."

# --- Namespace Creation ---
echo "Creating client and server network namespaces..."
sudo ip netns add client
sudo ip netns add server
echo "Namespaces created."

# --- Bridge Creation (Simulating Networks) ---
echo "Creating network bridges..."
# Bridge for WAN1
sudo ip link add br-wan0 type bridge
sudo ip link set br-wan0 up
# Bridge for WAN2
sudo ip link add br-wan1 type bridge
sudo ip link set br-wan1 up
# Bridge for Public Internet side
sudo ip link add br-public type bridge
sudo ip link set br-public up
echo "Bridges created."

# --- Client-side veth pairs (wan0 and wan1) ---
echo "Setting up client WAN interfaces..."
# wan0
sudo ip link add v-eth-client0 type veth peer name v-peer-client0
sudo ip link set v-eth-client0 netns client
sudo ip link set v-peer-client0 master br-wan0
sudo ip link set v-peer-client0 up
sudo ip netns exec client ip link set v-eth-client0 name wan0
sudo ip netns exec client ip addr add 192.168.10.2/24 dev wan0
sudo ip netns exec client ip link set wan0 up

# wan1
sudo ip link add v-eth-client1 type veth peer name v-peer-client1
sudo ip link set v-eth-client1 netns client
sudo ip link set v-peer-client1 master br-wan1
sudo ip link set v-peer-client1 up
sudo ip netns exec client ip link set v-eth-client1 name wan1
sudo ip netns exec client ip addr add 192.168.20.2/24 dev wan1
sudo ip netns exec client ip link set wan1 up
echo "Client WAN interfaces created."

# --- Server-side veth pair ---
echo "Setting up server public interface..."
sudo ip link add v-eth-server type veth peer name v-peer-server
sudo ip link set v-eth-server netns server
sudo ip link set v-peer-server master br-public
sudo ip link set v-peer-server up
sudo ip netns exec server ip link set v-eth-server name eth0
sudo ip netns exec server ip addr add 10.0.0.2/24 dev eth0
sudo ip netns exec server ip link set eth0 up
echo "Server public interface created."

# --- Connect bridges to simulate "internet" ---
echo "Configuring host-side bridge interfaces..."
sudo ip addr add 192.168.10.1/24 dev br-wan0
sudo ip addr add 192.168.20.1/24 dev br-wan1
sudo ip addr add 10.0.0.1/24 dev br-public
echo "Host-side interfaces configured."

# --- Set up routes ---
echo "Configuring routing..."
# Client default routes (one for each WAN)
sudo ip netns exec client ip route add default via 192.168.10.1 dev wan0 metric 100
sudo ip netns exec client ip route add default via 192.168.20.1 dev wan1 metric 200

# Server default route
sudo ip netns exec server ip route add default via 10.0.0.1

# Enable IP forwarding on the host to route between bridges
sudo sysctl -w net.ipv4.ip_forward=1

# --- Host forwarding setup ---
echo "Configuring host firewall to allow forwarding..."
sudo iptables -P FORWARD ACCEPT
echo "Host firewall configured."

echo "Network environment setup is complete!"
echo
echo "--- Verification ---"
echo "Pinging server from client on wan0..."
sudo ip netns exec client ping -c 2 -I wan0 10.0.0.2
echo "Pinging server from client on wan1..."
sudo ip netns exec client ping -c 2 -I wan1 10.0.0.2
echo "--------------------"
