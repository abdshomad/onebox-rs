use std::process::{Command, Child};

/// A helper struct to manage the test environment.
/// It sets up the network namespaces and starts the client/server processes.
/// When it goes out of scope, it will clean everything up.
pub struct TestEnvironment {
    pub server_process: Child,
    pub client_process: Child,
}

use std::io::{BufRead, BufReader};
use std::process::Stdio;

impl TestEnvironment {
    pub fn new() -> Self {
        println!("--- Setting up test environment ---");

        // Step 1: Clean and set up network
        Command::new("sudo").arg("../cleanup.sh").status().expect("cleanup failed");
        Command::new("sudo").arg("../setup_net_env.sh").status().expect("setup failed");

        // Step 2: Build workspace
        let build_status = Command::new("cargo").arg("build").arg("--workspace").status().expect("build failed");
        assert!(build_status.success());

        // Step 3: Start the server and wait for it to be ready
        println!("--- Starting onebox-server and waiting for it to be ready... ---");
        let mut server_process = Command::new("sudo")
            .arg("ip").arg("netns").arg("exec").arg("server")
            .arg("../target/debug/onebox-server")
            .arg("--config").arg("../config.test.server.toml")
            .arg("start")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn server");

        let server_stdout = server_process.stdout.take().expect("Failed to get server stdout");
        let reader = BufReader::new(server_stdout);
        let mut server_ready = false;
        // Set a timeout for waiting for the server to be ready
        let timeout = std::time::Duration::from_secs(10);
        let start_time = std::time::Instant::now();

        for line in reader.lines() {
            if start_time.elapsed() > timeout {
                panic!("Timeout waiting for server to become ready.");
            }
            let line = line.expect("Failed to read line from server");
            println!("[SERVER LOG] {}", line);
            if line.contains("UDP server listening") {
                server_ready = true;
                break;
            }
        }

        if !server_ready {
            let stderr = server_process.stderr.take().unwrap();
            let stderr_reader = BufReader::new(stderr);
            let stderr_lines: Vec<String> = stderr_reader.lines().map(|l| l.unwrap()).collect();
            panic!("Server did not become ready and exited. Stderr: {:?}", stderr_lines);
        }
        println!("--- Server is ready. Starting client. ---");

        // Step 4: Start the client process
        let client_process = Command::new("sudo")
            .arg("ip").arg("netns").arg("exec").arg("client")
            .arg("../target/debug/onebox-client")
            .arg("--config").arg("../config.test.client.toml")
            .arg("start")
            .spawn()
            .expect("Failed to spawn client");

        // Give the client a few seconds to perform its handshake
        println!("--- Waiting for client to initialize and connect... ---");
        std::thread::sleep(std::time::Duration::from_secs(4));
        println!("--- Test environment setup complete ---");

        Self {
            server_process,
            client_process,
        }
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        println!("--- Tearing down test environment ---");

        // Kill the child processes using sudo, as they were spawned within a root-owned namespace.
        println!("Stopping client process (PID: {})...", self.client_process.id());
        if let Err(e) = Command::new("sudo").arg("kill").arg(self.client_process.id().to_string()).status() {
            eprintln!("Warning: Failed to kill client process (PID: {}): {}. It might have already exited.", self.client_process.id(), e);
        }

        println!("Stopping server process (PID: {})...", self.server_process.id());
        if let Err(e) = Command::new("sudo").arg("kill").arg(self.server_process.id().to_string()).status() {
            eprintln!("Warning: Failed to kill server process (PID: {}): {}. It might have already exited.", self.server_process.id(), e);
        }

        // Wait for the processes to be fully terminated to prevent them from becoming zombies.
        let _ = self.client_process.wait();
        let _ = self.server_process.wait();
        println!("Client and server processes reaped.");

        // Finally, clean up the network environment.
        println!("Running network cleanup script...");
        let cleanup_status = Command::new("sudo")
            .arg("../cleanup.sh")
            .status()
            .expect("Failed to execute cleanup.sh during teardown");

        if !cleanup_status.success() {
            // It's important not to panic in a drop implementation.
            eprintln!("WARNING: cleanup.sh script failed during teardown. Manual cleanup may be required.");
        }

        println!("--- Test environment teardown complete ---");
    }
}
