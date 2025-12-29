use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Initialized {
    initialized: bool,
}

#[tauri::command]
#[specta::specta]
pub fn init_configuration_file() -> Result<(), String> {
    init_configuration_file_().map_err(|err| err.to_string())
}

pub fn init_configuration_file_() -> Result<(), anyhow::Error> {
    let home = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    let config_dir = Path::new(&home).join(".config/tealinux-modularity");
    
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).context("Failed to create configuration directory")?;
    }

    let path = config_dir.join("initialized.toml");

    match path.exists() {
        true => {
            let config = Initialized { initialized: true };
            let toml_string = toml::to_string(&config).context("Failed to create TOML string")?;
            std::fs::write(path, toml_string).context("Failed to write configuration to file")?;
            Ok(())
        }
        false => {
            let config = Initialized { initialized: false };

            let toml_string = toml::to_string(&config).context("Failed to create TOML string")?;

            std::fs::write(path, toml_string).context("Failed to write configuration to file")?;

            Ok(())
        }
    }
}
