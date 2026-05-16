use std::process::Command;

use modularitea_libs::infrastructure::{
    tools_utils::{CpuBooster, DnsSwitcher, MirrorUtils, Swap},
    PackageCacheCleaner,
};

use crate::utils::error_libs::LocalCommandOutput;

use super::models::{CpuProfile, DnsProvider, SwapMode};

const PACMAN_CACHE_DIR: &str = "/var/cache/pacman/pkg/";

#[tauri::command]
#[specta::specta]
pub fn get_cache_size() -> Result<u64, String> {
    let output = Command::new("du")
        .args(["-sb", PACMAN_CACHE_DIR])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| format!("Failed to parse du output: {}", stdout))
}

#[tauri::command]
#[specta::specta]
pub fn clean_cache() -> Result<LocalCommandOutput, String> {
    PackageCacheCleaner::clean()
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn refresh_mirror(country: Option<String>) -> Result<LocalCommandOutput, String> {
    let mirror = MirrorUtils::set_country(country);
    mirror
        .refresh_fastest_mirror()
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn switch_dns(provider: DnsProvider) -> Result<LocalCommandOutput, String> {
    DnsSwitcher::switch(provider.as_str())
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn set_cpu_profile(profile: CpuProfile) -> Result<LocalCommandOutput, String> {
    CpuBooster::set_profile(profile.as_str())
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn set_swap_mode(mode: SwapMode) -> Result<LocalCommandOutput, String> {
    Swap::set(mode.into())
        .map(Into::into)
        .map_err(|e| e.to_string())
}
