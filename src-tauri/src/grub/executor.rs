/// Resolve the GRUB themes directory path.
///
/// Priority:
/// 1. Runtime env var `TEALINUX_GRUB_CHANGER_MANIFEST_DIR`
/// 2. Compile-time env var `TEALINUX_GRUB_CHANGER_MANIFEST_DIR`
/// 3. Production path `/usr/share/tealinux/grub-themes`
/// 4. Dev fallback: sibling `tealinux-modularitea-libs/data/grub-theme`
/// 5. Ultimate fallback (empty results)
pub fn grub_themes_dir_path() -> String {
    // 1. Runtime env var
    if let Ok(dir) = std::env::var("TEALINUX_GRUB_CHANGER_MANIFEST_DIR") {
        if std::path::Path::new(&dir).is_dir() {
            eprintln!("[grub] Using env var themes dir: {}", dir);
            return dir;
        }
    }

    // 2. Compile-time env var
    if let Some(dir) = option_env!("TEALINUX_GRUB_CHANGER_MANIFEST_DIR") {
        if std::path::Path::new(dir).is_dir() {
            eprintln!("[grub] Using compile-time env themes dir: {}", dir);
            return dir.to_string();
        }
    }

    // 3. Production path
    let system_path = "/usr/share/modularitea-libs/grub-theme";
    if std::path::Path::new(system_path).is_dir() {
        eprintln!("[grub] Using system themes dir: {}", system_path);
        return system_path.to_string();
    }

    // 4. Dev fallback: sibling tealinux-modularitea-libs/data/grub-theme
    //    CARGO_MANIFEST_DIR is always set by cargo at compile time to the package root (src-tauri/)
    if let Some(manifest_dir) = option_env!("CARGO_MANIFEST_DIR") {
        let from_manifest = format!("{}/../../tealinux-modularitea-libs/data/grub-theme", manifest_dir);
        eprintln!("[grub] Checking dev fallback: {}", from_manifest);
        if std::path::Path::new(&from_manifest).is_dir() {
            if let Ok(abs) = std::fs::canonicalize(&from_manifest) {
                eprintln!("[grub] Using dev fallback themes dir: {}", abs.display());
                return abs.to_string_lossy().to_string();
            }
        }
    }

    // 5. Ultimate fallback (will return empty results)
    eprintln!("[grub] WARNING: No themes dir found, using fallback: {}", system_path);
    system_path.to_string()
}
