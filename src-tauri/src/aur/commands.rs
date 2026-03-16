use crate::installer::backend_runner::BackendResult;
use modularitea_libs::infrastructure::{AurClient, Paru};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AurPackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: String,
    pub num_votes: u32,
    pub popularity: f64,
    pub out_of_date: bool,
    pub installed: bool,
    pub url: String,
    pub aur_url: String,
    pub first_submitted: Option<u64>,
    pub last_modified: Option<u64>,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub make_depends: Vec<String>,
    pub opt_depends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstalledAurInfo {
    pub name: String,
    pub version: String,
}

fn is_valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 256
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || "@._+-".contains(c))
}

fn check_installed(name: &str) -> bool {
    Command::new("pacman")
        .args(["-Qi", name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn convert_package(pkg: modularitea_libs::domain::aur_package::AurPackage) -> AurPackageInfo {
    let installed = check_installed(&pkg.name);
    AurPackageInfo {
        aur_url: format!(
            "https://aur.archlinux.org/packages/{}",
            pkg.package_base.as_deref().unwrap_or(&pkg.name)
        ),
        name: pkg.name,
        version: pkg.version,
        description: pkg.description.unwrap_or_default(),
        maintainer: pkg.maintainer.unwrap_or_else(|| "Orphan".to_string()),
        num_votes: pkg.num_votes,
        popularity: pkg.popularity,
        out_of_date: pkg.out_of_date.is_some(),
        installed,
        url: pkg.url.unwrap_or_default(),
        first_submitted: pkg.first_submitted,
        last_modified: pkg.last_modified,
        license: pkg.license.unwrap_or_default(),
        depends: pkg.depends,
        make_depends: pkg.make_depends,
        opt_depends: pkg.opt_depends,
    }
}

/// Search AUR packages by query string
#[tauri::command]
#[specta::specta]
pub async fn search_aur_packages(query: String) -> Vec<AurPackageInfo> {
    let query = query.trim();
    if query.is_empty() || query.len() < 2 {
        return Vec::new();
    }

    match AurClient::search(query).await {
        Ok(packages) => {
            let mut results: Vec<AurPackageInfo> =
                packages.into_iter().map(convert_package).collect();
            results.sort_by(|a, b| {
                b.popularity
                    .partial_cmp(&a.popularity)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            results
        }
        Err(e) => {
            eprintln!("[aur] Search failed: {}", e);
            Vec::new()
        }
    }
}

/// Get detailed info for a single AUR package
#[tauri::command]
#[specta::specta]
pub async fn get_aur_package_info(name: String) -> Option<AurPackageInfo> {
    if !is_valid_package_name(&name) {
        return None;
    }
    match AurClient::get_package(&name).await {
        Ok(Some(pkg)) => Some(convert_package(pkg)),
        _ => None,
    }
}

/// Install an AUR package via paru
#[tauri::command]
#[specta::specta]
pub async fn install_aur_package(name: String) -> BackendResult {
    if !is_valid_package_name(&name) {
        return BackendResult::error(&format!("Invalid package name: {}", name));
    }
    if !Paru::is_available() {
        return BackendResult::error("paru is not installed.");
    }
    match Paru::install(&name) {
        Ok(output) => BackendResult {
            success: true,
            stdout: format!("✓ '{}' installed\n{}", name, output.stdout),
            stderr: output.stderr,
            exit_code: output.exit_code,
        },
        Err(e) => BackendResult {
            success: false,
            stdout: String::new(),
            stderr: format!("Failed to install '{}': {}", name, e.stderr),
            exit_code: e.exit_code.unwrap_or(-1),
        },
    }
}

/// Remove an AUR package via paru
#[tauri::command]
#[specta::specta]
pub async fn remove_aur_package(name: String) -> BackendResult {
    if !is_valid_package_name(&name) {
        return BackendResult::error(&format!("Invalid package name: {}", name));
    }
    if !Paru::is_available() {
        return BackendResult::error("paru is not installed.");
    }
    match Paru::remove(&name) {
        Ok(output) => BackendResult {
            success: true,
            stdout: format!("✓ '{}' removed\n{}", name, output.stdout),
            stderr: output.stderr,
            exit_code: output.exit_code,
        },
        Err(e) => BackendResult {
            success: false,
            stdout: String::new(),
            stderr: format!("Failed to remove '{}': {}", name, e.stderr),
            exit_code: e.exit_code.unwrap_or(-1),
        },
    }
}

/// List all installed AUR (foreign) packages
#[tauri::command]
#[specta::specta]
pub async fn list_installed_aur() -> Vec<InstalledAurInfo> {
    match Paru::list_installed() {
        Ok(packages) => packages
            .into_iter()
            .map(|pkg| InstalledAurInfo {
                name: pkg.name,
                version: pkg.version,
            })
            .collect(),
        Err(_) => match Command::new("pacman").args(["-Qm"]).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            Some(InstalledAurInfo {
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            Err(_) => Vec::new(),
        },
    }
}
