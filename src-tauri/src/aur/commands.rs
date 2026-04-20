use crate::installer::backend_runner::BackendResult;
use modularitea_libs::infrastructure::{AurClient, Paru};
use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashSet;
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

// --- Top-packages cache: valid for 10 minutes ---
const TOP_CACHE_TTL: Duration = Duration::from_secs(10 * 60);

static TOP_CACHE: OnceLock<Mutex<Option<(Vec<AurPackageInfo>, Instant)>>> = OnceLock::new();

fn top_cache() -> &'static Mutex<Option<(Vec<AurPackageInfo>, Instant)>> {
    TOP_CACHE.get_or_init(|| Mutex::new(None))
}

// --- Shared reqwest client for Tauri commands ---
use std::sync::OnceLock as HttpOnceLock;
static HTTP_CLIENT: HttpOnceLock<reqwest::Client> = HttpOnceLock::new();
fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

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
            // Batch check installed status with a single `pacman -Qq` call
            // instead of one `pacman -Qi` per package (old convert_package).
            let installed_set = batch_check_installed();
            let mut results: Vec<AurPackageInfo> = packages
                .into_iter()
                .map(|pkg| convert_package_batch(pkg, &installed_set))
                .collect();
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
    eprintln!("[aur] install_aur_package called for '{}'", name);
    if !is_valid_package_name(&name) {
        return BackendResult::error(&format!("Invalid package name: {}", name));
    }
    if !Paru::is_available() {
        eprintln!("[aur] paru is not installed on this system");
        return BackendResult::error("paru is not installed. Please install paru first.");
    }
    match Paru::install(&name) {
        Ok(output) => {
            eprintln!("[aur] Successfully installed '{}'", name);
            BackendResult {
                success: true,
                stdout: format!("✓ '{}' installed\n{}", name, output.stdout),
                stderr: output.stderr,
                exit_code: output.exit_code,
            }
        }
        Err(e) => {
            eprintln!("[aur] Failed to install '{}': {}", name, e.stderr);
            BackendResult {
                success: false,
                stdout: String::new(),
                stderr: format!("Failed to install '{}': {}", name, e.stderr),
                exit_code: e.exit_code.unwrap_or(-1),
            }
        }
    }
}

/// Remove an AUR package via paru
#[tauri::command]
#[specta::specta]
pub async fn remove_aur_package(name: String) -> BackendResult {
    eprintln!("[aur] remove_aur_package called for '{}'", name);
    if !is_valid_package_name(&name) {
        return BackendResult::error(&format!("Invalid package name: {}", name));
    }
    if !Paru::is_available() {
        eprintln!("[aur] paru is not installed on this system");
        return BackendResult::error("paru is not installed. Please install paru first.");
    }
    match Paru::remove(&name) {
        Ok(output) => {
            eprintln!("[aur] Successfully removed '{}'", name);
            BackendResult {
                success: true,
                stdout: format!("✓ '{}' removed\n{}", name, output.stdout),
                stderr: output.stderr,
                exit_code: output.exit_code,
            }
        }
        Err(e) => {
            eprintln!("[aur] Failed to remove '{}': {}", name, e.stderr);
            BackendResult {
                success: false,
                stdout: String::new(),
                stderr: format!("Failed to remove '{}': {}", name, e.stderr),
                exit_code: e.exit_code.unwrap_or(-1),
            }
        }
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

/// Get all installed package names as a HashSet (single pacman call)
fn batch_check_installed() -> HashSet<String> {
    Command::new("pacman")
        .args(["-Qq"])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(|l| l.to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn convert_package_batch(
    pkg: modularitea_libs::domain::aur_package::AurPackage,
    installed_set: &HashSet<String>,
) -> AurPackageInfo {
    let installed = installed_set.contains(&pkg.name);
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

/// Get top popular AUR packages (scraped + batch-fetched, cached for 10 minutes)
#[tauri::command]
#[specta::specta]
pub async fn get_top_aur_packages() -> Vec<AurPackageInfo> {
    // --- Check cache first ---
    {
        let cache = top_cache().lock().unwrap();
        if let Some((ref cached, ref cached_at)) = *cache {
            if cached_at.elapsed() < TOP_CACHE_TTL {
                eprintln!("[aur] Returning top packages from cache");
                return cached.clone();
            }
        }
    }

    // --- Cache miss: scrape top package names from AUR website ---
    let html = match http_client()
        .get("https://aur.archlinux.org/packages?O=0&SB=p&SO=d&PP=100")
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => {
                eprintln!("[aur] Failed to read AUR page: {}", e);
                return Vec::new();
            }
        },
        Err(e) => {
            eprintln!("[aur] Failed to fetch AUR page: {}", e);
            return Vec::new();
        }
    };

    let re = Regex::new(r#"<a href="/packages/([^/"]+)">"#).unwrap();
    let mut scraped_names: Vec<String> = Vec::new();
    let mut seen = HashSet::new();
    for cap in re.captures_iter(&html) {
        let name = cap[1].to_string();
        if !name.contains('?') && seen.insert(name.clone()) {
            scraped_names.push(name);
        }
    }

    if scraped_names.is_empty() {
        eprintln!("[aur] No package names scraped from AUR page");
        return Vec::new();
    }

    eprintln!("[aur] Scraped {} package names, fetching info in batch...", scraped_names.len());

    // Batch fetch info from AUR RPC API
    let mut all_packages = Vec::new();
    for chunk in scraped_names.chunks(100) {
        let refs: Vec<&str> = chunk.iter().map(|s| s.as_str()).collect();
        match AurClient::info(&refs).await {
            Ok(pkgs) => all_packages.extend(pkgs),
            Err(e) => {
                eprintln!("[aur] Batch info failed for chunk: {}", e);
            }
        }
    }

    // Batch check installed status (single pacman -Qq call)
    let installed_set = batch_check_installed();

    // Convert and sort by popularity
    let mut results: Vec<AurPackageInfo> = all_packages
        .into_iter()
        .map(|pkg| convert_package_batch(pkg, &installed_set))
        .collect();

    results.sort_by(|a, b| {
        b.popularity
            .partial_cmp(&a.popularity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    eprintln!("[aur] Returning {} top packages", results.len());

    // --- Store in cache ---
    {
        let mut cache = top_cache().lock().unwrap();
        *cache = Some((results.clone(), Instant::now()));
    }

    results
}
