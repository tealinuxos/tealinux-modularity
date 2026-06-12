use std::process::Command;

use duct::cmd;

use modularitea_libs::infrastructure::{
    tools_utils::{CpuBooster, MirrorUtils},
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
    let output = cmd("pkexec", ["modularitea-dns-changer", provider.as_str()])
        .stdout_capture()
        .stderr_capture()
        .run()
        .map_err(|e| e.to_string())?;

    let exit_code = output.status.code().unwrap_or(-1);

    Ok(LocalCommandOutput {
        exit_code,
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn is_swap_enabled() -> Result<bool, String> {
    let zram_conf = std::path::Path::new("/etc/systemd/zram-generator.conf");
    Ok(zram_conf.exists())
}

#[tauri::command]
#[specta::specta]
pub fn get_current_dns_provider() -> Result<Option<DnsProvider>, String> {
    let contents = std::fs::read_to_string("/etc/resolv.conf").unwrap_or_default();
    let provider = ["cloudflare", "google", "quad9", "opendns", "adguard"]
        .iter()
        .find(|&&name| {
            let nameservers: &[&str] = match name {
                "cloudflare" => &["1.1.1.1", "1.0.0.1"],
                "google" => &["8.8.8.8", "8.8.4.4"],
                "quad9" => &["9.9.9.9", "149.112.112.112"],
                "opendns" => &["208.67.222.222", "208.67.220.220"],
                "adguard" => &["94.140.14.14", "94.140.15.15"],
                _ => &[],
            };
            nameservers.iter().all(|ip| contents.contains(ip))
        })
        .map(|s| s.to_string())
        .unwrap_or_default();
    Ok(DnsProvider::from_lib_str(&provider))
}

#[tauri::command]
#[specta::specta]
pub fn get_cpu_governor_state() -> Result<Option<CpuProfile>, String> {
    let output = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .output()
        .map_err(|e| e.to_string())?;
    let governor = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(CpuProfile::from_lib_str(&governor))
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
    let state = match mode {
        SwapMode::Enable => "on",
        SwapMode::Disable => "off",
    };

    let output = cmd("pkexec", ["modularitea-swap", state])
        .stdout_capture()
        .stderr_capture()
        .run()
        .map_err(|e| e.to_string())?;

    let exit_code = output.status.code().unwrap_or(-1);

    Ok(LocalCommandOutput {
        exit_code,
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}
