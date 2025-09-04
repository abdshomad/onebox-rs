use std::process::{Child, Command};

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
    pub fn new(client_config: Option<&str>, server_config: Option<&str>) -> Self {
        println!("--- Setting up test environment ---");

        // Step 1: Clean and set up network
        Command::new("sudo")
            .arg("../cleanup.sh")
            .status()
            .expect("cleanup failed");
        Command::new("sudo")
            .arg("../setup_net_env.sh")
            .status()
            .expect("setup failed");

        // Step 2: Build workspace
        let build_status = Command::new("cargo")
            .arg("build")
            .arg("--workspace")
            .status()
            .expect("build failed");
        assert!(build_status.success());

        // Determine config paths
        let server_config_path = server_config.unwrap_or("../config.test.server.toml");
        let client_config_path = client_config.unwrap_or("../config.test.client.toml");

        // Step 3: Start the server and wait for it to be ready
        println!("--- Starting onebox-server and waiting for it to be ready... ---");
        let mut server_process = Command::new("sudo")
            .arg("ip")
            .arg("netns")
            .arg("exec")
            .arg("server")
            .arg("../target/debug/onebox-server")
            .arg("--config")
            .arg(server_config_path)
            .arg("start")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn server");

        // Asynchronously drain stdout and stderr to prevent the child process from blocking.
        // Also, use a channel to signal when the server is ready.
        let (tx, rx) = std::sync::mpsc::channel();

        let server_stdout = server_process
            .stdout
            .take()
            .expect("Failed to get server stdout");
        let stdout_tx = tx.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(server_stdout);
            for line in reader.lines() {
                let line = line.expect("Failed to read line from server stdout");
                if line.contains("UDP server listening") {
                    // It's okay if this fails, the receiver might have already hung up.
                    let _ = stdout_tx.send(());
                }
                println!("[SERVER STDOUT] {}", line);
            }
        });

        let server_stderr = server_process
            .stderr
            .take()
            .expect("Failed to get server stderr");
        std::thread::spawn(move || {
            let reader = BufReader::new(server_stderr);
            for line in reader.lines() {
                let line = line.expect("Failed to read line from server stderr");
                println!("[SERVER STDERR] {}", line);
            }
        });

        // Wait for the ready signal from the stdout-draining thread.
        match rx.recv_timeout(std::time::Duration::from_secs(10)) {
            Ok(_) => {
                // Server is ready
            }
            Err(_) => {
                panic!("Timeout waiting for server to become ready. Check logs.");
            }
        }
        println!("--- Server is ready. Starting client. ---");

        // Step 4: Start the client process
        let client_process = Command::new("sudo")
            .arg("ip")
            .arg("netns")
            .arg("exec")
            .arg("client")
            .arg("../target/debug/onebox-client")
            .arg("--config")
            .arg(client_config_path)
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
        println!(
            "Stopping client process (PID: {})...",
            self.client_process.id()
        );
        if let Err(e) = Command::new("sudo")
            .arg("kill")
            .arg("-9")
            .arg(self.client_process.id().to_string())
            .status()
        {
            eprintln!("Warning: Failed to kill client process (PID: {}): {}. It might have already exited.", self.client_process.id(), e);
        }

        println!(
            "Stopping server process (PID: {})...",
            self.server_process.id()
        );
        if let Err(e) = Command::new("sudo")
            .arg("kill")
            .arg("-9")
            .arg(self.server_process.id().to_string())
            .status()
        {
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
