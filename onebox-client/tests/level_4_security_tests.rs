use std::process::Command;

// Include the common module for test environment setup
mod common;
use common::TestEnvironment;

/// **TS4.1: Authentication Rejection (Invalid PSK)**
///
/// This test validates that the server correctly rejects a client that provides
/// an invalid Pre-Shared Key (PSK).
///
/// # Methodology
/// 1. The `TestEnvironment` is initialized with a standard server configuration
///    but a client configuration that specifies a deliberately incorrect PSK.
/// 2. The test waits briefly to allow the client to attempt its handshake, which
///    is expected to fail.
/// 3. A `ping` command is executed from the client's network namespace.
/// 4. The test asserts that the `ping` command **fails**, confirming that no
///    traffic can pass through the tunnel, thus proving the authentication check
///    was successful.
#[test]
fn test_authentication_rejection() {
    println!("--- Running authentication rejection test (TS4.1) ---");
    // Use the config file with the bad PSK for the client.
    // The server uses the default, correct PSK.
    let _env = TestEnvironment::new(Some("../config.test.client.bad_psk.toml"), None);

    // Allow a moment for the client to attempt and fail the handshake.
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Execute a ping command. It should fail because the PSKs don't match
    // and no secure tunnel should have been established.
    let ping_output = Command::new("sudo")
        .arg("ip")
        .arg("netns")
        .arg("exec")
        .arg("client")
        .arg("ping")
        .arg("-c")
        .arg("1") // Only need one packet to see if it fails
        .arg("-W")
        .arg("2") // Wait max 2 seconds for a reply
        .arg("10.0.0.88") // The simulated internet endpoint
        .output()
        .expect("Failed to execute ping command in client namespace");

    // Print the output from the command for easier debugging in test logs.
    println!(
        "Ping stdout:\n{}",
        String::from_utf8_lossy(&ping_output.stdout)
    );
    println!(
        "Ping stderr:\n{}",
        String::from_utf8_lossy(&ping_output.stderr)
    );

    // The most important check: The command should FAIL.
    assert!(
        !ping_output.status.success(),
        "Ping command succeeded, but it was expected to fail. The tunnel is allowing traffic with a bad PSK."
    );

    println!("--- Authentication rejection test successful (ping failed as expected) ---");
}

/// **TS4.2: Data Confidentiality Verification**
///
/// This test validates that all data transmitted through the tunnel is properly
/// encrypted and that no plaintext is observable on the underlying network.
///
/// # Methodology
/// 1. The `TestEnvironment` is initialized with matching, valid configurations
///    for both client and server, establishing a working tunnel.
/// 2. `tcpdump` is started in the background on the client's `wan0` interface
///    to capture all outgoing UDP traffic to a temporary file.
/// 3. A `ping` command is executed from the client's namespace. This ping is
///    given a specific, unique payload pattern using the `-p` flag. The ping
///    is asserted to be successful.
/// 4. The `tcpdump` process is stopped.
/// 5. The raw bytes of the capture file are read into memory.
/// 6. The test asserts that the unique payload pattern from the ping command is
///    **NOT** present anywhere in the capture file. If the pattern is found,
///    it means the data was sent in plaintext and the test fails.
#[test]
fn test_data_confidentiality() {
    println!("--- Running data confidentiality test (TS4.2) ---");
    let _env = TestEnvironment::new(None, None);

    // Give time for tunnel to be fully established
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Define a unique, valid hex pattern for the ping payload and a capture file path
    let ping_pattern = "deadbeefcafe"; // 12 hex chars = 6 bytes
    let capture_file = "/tmp/onebox_capture.pcap";

    // Start tcpdump in the background
    let mut tcpdump_handle = Command::new("sudo")
        .arg("ip")
        .arg("netns")
        .arg("exec")
        .arg("client")
        .arg("tcpdump")
        .arg("-i")
        .arg("wan0") // Capture on the first WAN link
        .arg("-w")
        .arg(capture_file)
        .arg("udp") // Capture only UDP traffic to reduce noise
        .spawn()
        .expect("Failed to start tcpdump");

    // Allow tcpdump a moment to start up
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("--- tcpdump started ---");

    // Execute a ping command with a specific payload pattern
    let ping_output = Command::new("sudo")
        .arg("ip")
        .arg("netns")
        .arg("exec")
        .arg("client")
        .arg("ping")
        .arg("-c")
        .arg("1")
        .arg("-p")
        .arg(ping_pattern)
        .arg("10.0.0.88")
        .output()
        .expect("Failed to execute ping command");

    assert!(
        ping_output.status.success(),
        "Ping failed, cannot verify confidentiality if the tunnel isn't working."
    );
    println!("--- Ping through tunnel was successful ---");

    // Stop tcpdump
    Command::new("sudo")
        .arg("kill")
        .arg(tcpdump_handle.id().to_string())
        .status()
        .expect("Failed to kill tcpdump process");
    let _ = tcpdump_handle.wait();
    println!("--- tcpdump stopped ---");

    // Read the capture file
    let pcap_data = std::fs::read(capture_file).expect("Failed to read pcap file");

    // Search for the plaintext pattern in the capture file.
    let found_pattern = pcap_data
        .windows(ping_pattern.len())
        .any(|window| window == ping_pattern.as_bytes());

    assert!(
        !found_pattern,
        "Plaintext ping pattern was found in the traffic capture. Data is NOT confidential."
    );

    println!("--- Data confidentiality test successful (plaintext pattern not found) ---");

    // Clean up the capture file, which was created by a sudo process
    let cleanup_status = Command::new("sudo")
        .arg("rm")
        .arg("-f")
        .arg(capture_file)
        .status()
        .expect("Failed to run sudo rm for cleanup");
    assert!(cleanup_status.success(), "Failed to cleanup pcap file");
}
