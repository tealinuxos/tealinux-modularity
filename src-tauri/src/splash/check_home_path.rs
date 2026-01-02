use std::path::{Path, PathBuf};

use anyhow::Context;

pub fn check_home_path() -> Result<PathBuf, anyhow::Error> {
    let home = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    let config_dir = Path::new(&home).join(".config/tealinux-modularity");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).context("Failed to create configuration directory")?;
    }
    let path = config_dir.join("initialized.toml");

    Ok(path)
}
