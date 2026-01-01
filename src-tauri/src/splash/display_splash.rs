use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::splash::check_home_path::check_home_path;

#[derive(Serialize, Deserialize, specta::Type)]
pub struct Initialized {
    initialized: bool,
}

#[tauri::command]
#[specta::specta]
pub fn init_configuration_file() -> Result<(), String> {
    init_configuration_file_().map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn check_configuration_file() -> Result<bool, String> {
    check_configuration_file_().map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn show_main_window(window: tauri::Window) {
    let is_initialized = check_configuration_file().unwrap_or(false);

    if is_initialized {
        window.get_webview_window("main").unwrap().show().unwrap();
        if let Some(splashscreen) = window.get_webview_window("splashscreen") {
            splashscreen.close().unwrap();
        }
    } else {
        window
            .get_webview_window("splashscreen")
            .unwrap()
            .show()
            .unwrap();
    }
}

pub fn init_configuration_file_() -> Result<(), anyhow::Error> {
    let path = check_home_path()?;

    let config = Initialized { initialized: true };

    let toml_string = toml::to_string(&config).context("Failed to create TOML string")?;

    std::fs::write(path, toml_string).context("Failed to write configuration to file")?;

    Ok(())
}

pub fn check_configuration_file_() -> Result<bool, anyhow::Error> {
    let path = check_home_path()?;

    let init_file = std::fs::read_to_string(&path).context("Failed to Read Configuration File")?;
    let config: Initialized = toml::from_str(&init_file).context("Failed to parse TOML string")?;

    Ok(config.initialized)
}
