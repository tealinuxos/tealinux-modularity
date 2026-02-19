use serde::{Deserialize, Serialize};
use specta::Type;

/// Profile metadata from TOML files
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProfileInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub category: String,
    pub packages_install: Vec<String>,
    pub packages_aur: Vec<String>,
    pub services_enable: Vec<String>,
    pub package_count: u32,
}

/// Meta section from profile TOML
#[derive(Debug, Deserialize)]
struct TomlMeta {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    category: String,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Deserialize)]
struct TomlPackages {
    #[serde(default)]
    install: Vec<String>,
    #[serde(default)]
    aur: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlServices {
    #[serde(default)]
    enable: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlProfile {
    meta: TomlMeta,
    #[serde(default)]
    packages: Option<TomlPackages>,
    #[serde(default)]
    services: Option<TomlServices>,
}

/// Get the profiles directory path
fn profiles_dir() -> std::path::PathBuf {
    // In development, profiles are relative to the project root
    // In production, they would be in /usr/share/tealinux-modularity/profiles/
    let dev_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("profiles");

    if dev_path.exists() {
        return dev_path;
    }

    // Production path
    std::path::PathBuf::from("/usr/share/tealinux-modularity/profiles")
}

/// Load all profiles from the profiles directory
#[tauri::command]
#[specta::specta]
pub async fn list_profiles() -> Vec<ProfileInfo> {
    let dir = profiles_dir();
    let mut profiles = Vec::new();

    if !dir.exists() {
        eprintln!("[profiler] Profiles directory does not exist: {:?}", dir);
        return profiles;
    }

    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("[profiler] Failed to read profiles dir: {}", e);
            return profiles;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<TomlProfile>(&content) {
                    Ok(parsed) => {
                        let pkgs = parsed.packages.unwrap_or(TomlPackages {
                            install: Vec::new(),
                            aur: Vec::new(),
                        });
                        let svcs = parsed.services.unwrap_or(TomlServices {
                            enable: Vec::new(),
                        });

                        let id = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        let pkg_count = (pkgs.install.len() + pkgs.aur.len()) as u32;

                        profiles.push(ProfileInfo {
                            id,
                            name: parsed.meta.name,
                            description: parsed.meta.description,
                            version: parsed.meta.version,
                            author: parsed.meta.author,
                            category: parsed.meta.category,
                            packages_install: pkgs.install,
                            packages_aur: pkgs.aur,
                            services_enable: svcs.enable,
                            package_count: pkg_count,
                        });
                    }
                    Err(e) => {
                        eprintln!("[profiler] Failed to parse {:?}: {}", path, e);
                    }
                },
                Err(e) => {
                    eprintln!("[profiler] Failed to read {:?}: {}", path, e);
                }
            }
        }
    }

    // Sort by name
    profiles.sort_by(|a, b| a.name.cmp(&b.name));
    profiles
}

/// Get a single profile by ID
#[tauri::command]
#[specta::specta]
pub async fn get_profile(profile_id: String) -> Option<ProfileInfo> {
    let profiles = list_profiles().await;
    profiles.into_iter().find(|p| p.id == profile_id)
}
