use std::process::Command;

// Include the common module for test environment setup
mod common;
use common::TestEnvironment;

#[test]
#[ignore = "Fails due to unresolved network issue where data packets are dropped after handshake"]
fn test_ping_e2e() {
    // The '_env' variable's scope controls the setup and teardown.
    // When it is created here, TestEnvironment::new() is called.
    // When it goes out of scope at the end of the test, TestEnvironment::drop() is called.
    let _env = TestEnvironment::new();

    println!("--- Running E2E ping test (TS1.1) ---");

    // Allow a moment for the client to fully establish its routes through the server.
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Execute the ping command inside the 'client' network namespace.
    // This traffic should be captured by the client's TUN device, sent to the server,
    // and then forwarded to the public internet.
    let ping_output = Command::new("sudo")
        .arg("ip")
        .arg("netns")
        .arg("exec")
        .arg("client")
        .arg("ping")
        .arg("-c")
        .arg("4") // Send 4 packets
        .arg("8.8.8.8") // A reliable public IP
        .output()
        .expect("Failed to execute ping command in client namespace");

    // Print the output from the command for easier debugging in test logs.
    println!("Ping stdout:\n{}", String::from_utf8_lossy(&ping_output.stdout));
    println!("Ping stderr:\n{}", String::from_utf8_lossy(&ping_output.stderr));

    // The most important check: Did the command exit with a success code?
    assert!(
        ping_output.status.success(),
        "Ping command failed. The E2E tunnel is not passing ICMP traffic correctly."
    );

    // Optional: A more robust check could be to parse the stdout and ensure packets were received.
    assert!(
        String::from_utf8_lossy(&ping_output.stdout).contains("4 packets received"),
        "Ping output did not indicate that all packets were received."
    );

    println!("--- E2E ping test successful ---");
}
