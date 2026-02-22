use modularitea_libs::loader::TomlLoader;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Profile metadata exposed to the frontend via Tauri commands.
/// This is a DTO (Data Transfer Object) that wraps the libs' domain model
/// with `specta::Type` for TypeScript binding generation.
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
    pub packages_remove: Vec<String>,
    pub services_enable: Vec<String>,
    pub services_disable: Vec<String>,
    pub package_count: u32,
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

/// Load all profiles from the profiles directory using TomlLoader from libs
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
            match TomlLoader::load(&path) {
                Ok(profile) => {
                    let id = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    let pkg_count =
                        (profile.packages.install.len() + profile.packages.aur.len()) as u32;

                    profiles.push(ProfileInfo {
                        id,
                        name: profile.meta.name,
                        description: profile.meta.description,
                        version: profile.meta.version,
                        author: profile.meta.author,
                        category: profile.meta.category,
                        packages_install: profile.packages.install,
                        packages_aur: profile.packages.aur,
                        packages_remove: profile.packages.remove,
                        services_enable: profile.services.enable,
                        services_disable: profile.services.disable,
                        package_count: pkg_count,
                    });
                }
                Err(e) => {
                    eprintln!("[profiler] Failed to parse {:?}: {}", path, e);
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
