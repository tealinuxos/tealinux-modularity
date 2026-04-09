use serde::de::DeserializeOwned;
use std::process::Command;

pub fn fetch_module<T: DeserializeOwned>(structure: &str) -> Result<T, String> {
    let output = Command::new("fastfetch")
        .args(["--format", "json", "--structure", structure])
        .output()
        .map_err(|err| format!("Failed to run fastfetch for {}: {}", structure, err))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "fastfetch exited with {} for {}: {}",
            output
                .status
                .code()
                .map_or_else(|| "unknown status".to_string(), |code| code.to_string()),
            structure,
            stderr.trim()
        ));
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|err| format!("Failed to parse JSON for {}: {}", structure, err))?;

    if let Some(module_error) = json
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|obj| obj.get("error"))
        .and_then(|value| value.as_str())
    {
        return Err(format!("{}: {}", structure, module_error));
    }

    serde_json::from_value(json)
        .map_err(|err| format!("Failed to deserialize {} payload: {}", structure, err))
}
