use serde::de::DeserializeOwned;
use std::process::Command;

pub fn fetch_module<T: DeserializeOwned>(structure: &str) -> T {
    let output = Command::new("fastfetch")
        .args(["--format", "json", "--structure", structure])
        .output()
        .unwrap_or_else(|_| panic!("Failed to run fastfetch for {}", structure));

    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|_| panic!("Failed to parse JSON for {}", structure))
}
