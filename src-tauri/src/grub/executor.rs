use duct::cmd;
use std::fs;

use crate::grub::models::{Step, ThemeManifest};
use crate::utils::error_libs::{LocalCommandOutput, LocalModulariteaError};

pub trait GrubInstructionExecutor {
    fn new() -> Self;
    fn set_screen_resolution(self, width: u32, height: u32) -> Self;
    fn get_all_theme_available(&self) -> Vec<ThemeManifest>;
    fn details(&self, theme_name: &str) -> Option<ThemeManifest>;
    fn apply_grub_theme(
        &self,
        theme_name: &str,
    ) -> Result<LocalCommandOutput, LocalModulariteaError>;
}

pub struct GrubInstruction {
    pub manifest: Vec<ThemeManifest>,
    pub screen_resolution: Option<(u32, u32)>,
    pub themes_dir: String,
}

pub fn grub_themes_dir_path() -> String {
    // 1. Runtime env var
    if let Ok(dir) = std::env::var("TEALINUX_GRUB_CHANGER_MANIFEST_DIR") {
        if std::path::Path::new(&dir).is_dir() {
            return dir;
        }
    }

    // 2. Compile-time env var
    if let Some(dir) = option_env!("TEALINUX_GRUB_CHANGER_MANIFEST_DIR") {
        if std::path::Path::new(dir).is_dir() {
            return dir.to_string();
        }
    }

    // 3. Production path
    let system_path = "/usr/share/tealinux/grub-themes";
    if std::path::Path::new(system_path).is_dir() {
        return system_path.to_string();
    }

    // 4. Dev fallback: sibling tealinux-modularitea-libs/data/grub-theme
    let dev_candidates = [
        // Relative to CWD (workspace root)
        "../tealinux-modularitea-libs/data/grub-theme",
        "../../tealinux-modularitea-libs/data/grub-theme",
    ];
    // Also try relative to CARGO_MANIFEST_DIR (src-tauri/)
    if let Some(manifest_dir) = option_env!("CARGO_MANIFEST_DIR") {
        let from_manifest = format!("{}/../../tealinux-modularitea-libs/data/grub-theme", manifest_dir);
        if std::path::Path::new(&from_manifest).is_dir() {
            if let Ok(abs) = std::fs::canonicalize(&from_manifest) {
                eprintln!("[grub] Using dev fallback themes dir: {}", abs.display());
                return abs.to_string_lossy().to_string();
            }
        }
    }
    for candidate in &dev_candidates {
        let p = std::path::Path::new(candidate);
        if p.is_dir() {
            if let Ok(abs) = std::fs::canonicalize(p) {
                eprintln!("[grub] Using dev fallback themes dir: {}", abs.display());
                return abs.to_string_lossy().to_string();
            }
        }
    }

    // 5. Ultimate fallback (will return empty results)
    system_path.to_string()
}

impl GrubInstruction {
    fn load_manifests() -> Result<(Vec<ThemeManifest>, String), LocalModulariteaError> {
        let manifest_dir = grub_themes_dir_path();

        let mut manifests = Vec::new();

        if let Ok(read_dir) = fs::read_dir(&manifest_dir) {
            for entry in read_dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let candidate = entry.path().join("manifest.json");
                        if candidate.is_file() {
                            if let Ok(content) = fs::read_to_string(&candidate) {
                                if let Ok(manifest) = serde_json::from_str(&content) {
                                    manifests.push(manifest);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok((manifests, manifest_dir))
    }

    pub fn reload_manifest(&mut self) {
        match Self::load_manifests() {
            Ok((m, dir)) => {
                self.manifest = m;
                self.themes_dir = dir;
            }
            Err(_) => {}
        };
    }

    pub fn clone_with_resolution(&self, width: u32, height: u32) -> Self {
        Self {
            manifest: self.manifest.clone(),
            themes_dir: self.themes_dir.clone(),
            screen_resolution: Some((width, height)),
        }
    }

    fn reset_grub_config() -> Result<(), LocalModulariteaError> {
        Ok(())
    }

    fn set_grub_var(key: &str, value: &str) -> Result<(), LocalModulariteaError> {
        let val_escaped = value.replace('"', "\\\"");
        let sed_cmd = format!(
            "sudo sed -i -E 's|^[[:space:]]*#?[[:space:]]*{}=.*|{}={}|' /etc/default/grub",
            key, key, val_escaped
        );
        let _ = cmd("sh", ["-c", &sed_cmd]).run();

        let ensure_cmd = format!(
            "sudo grep -q '^[[:space:]]*{}=' /etc/default/grub || sudo sh -c 'echo \"{}={}\" >> /etc/default/grub'",
            key, key, val_escaped
        );
        let _ = cmd("sh", ["-c", &ensure_cmd]).run();
        Ok(())
    }
}

impl GrubInstructionExecutor for GrubInstruction {
    fn new() -> Self {
        let (manifest, themes_dir) = Self::load_manifests().unwrap_or_else(|_| (vec![], grub_themes_dir_path()));
        Self {
            manifest,
            themes_dir,
            screen_resolution: None,
        }
    }

    fn set_screen_resolution(mut self, width: u32, height: u32) -> Self {
        self.screen_resolution = Some((width, height));
        self
    }

    fn get_all_theme_available(&self) -> Vec<ThemeManifest> {
        self.manifest.clone()
    }

    fn details(&self, theme_name: &str) -> Option<ThemeManifest> {
        self.manifest.iter().find(|m| m.name == theme_name).cloned()
    }

    fn apply_grub_theme(
        &self,
        theme_name: &str,
    ) -> Result<LocalCommandOutput, LocalModulariteaError> {
        let manifest = self
            .manifest
            .iter()
            .find(|m| m.name == theme_name)
            .cloned()
            .ok_or(LocalModulariteaError::InternalError(format!(
                "theme not found: {}",
                theme_name
            )))?;

        Self::reset_grub_config()?;

        if let Some((width, height)) = self.screen_resolution {
            Self::set_grub_var("GRUB_GFXMODE", &format!("{}x{}", width, height))?;
        }

        let mut cmds: Vec<String> = Vec::new();

        let expand = |s: &str| -> String {
            s.replace(
                "${MANIFEST_DIR}",
                option_env!("TEALINUX_GRUB_CHANGER_MANIFEST_DIR")
                    .unwrap_or("/usr/share/modularitea-libs/grub-theme"),
            )
        };

        for step in manifest.steps {
            match step {
                Step::CopyDir { from, to } => {
                    cmds.push(format!("sudo cp -r -u -v '{}' '{}'", expand(&from), to))
                }
                Step::CopyFile { from, to } => {
                    cmds.push(format!("sudo cp '{}' '{}'", expand(&from), to))
                }
                Step::SetGrubVar { key, value } => {
                    let val_escaped = value.replace('"', "\\\"");
                    cmds.push(format!("sudo sed -i -E 's|^[[:space:]]*#?[[:space:]]*{}=.*|{}={}|' /etc/default/grub", key, key, val_escaped));
                    cmds.push(format!("sudo grep -q '^[[:space:]]*{}=' /etc/default/grub || sudo sh -c 'echo \"{}={}\" >> /etc/default/grub'", key, key, val_escaped));
                }
                Step::ReplaceInFile {
                    file,
                    search,
                    replace,
                } => cmds.push(format!("sudo sed -i 's|{}|{}|g' {}", search, replace, file)),
            }
        }

        cmds.push("sudo grub-mkconfig -o /boot/grub/grub.cfg".to_string());

        for command in cmds {
            let command = command
                .strip_prefix("sudo ")
                .unwrap_or(&command)
                .to_string();
            let _ = cmd("sudo", ["sh", "-c", command.as_str()])
                .run()
                .map_err(|e| LocalModulariteaError::GrubError {
                    operation: command,
                    reason: e.to_string(),
                })?;
        }

        Ok(LocalCommandOutput {
            exit_code: 0,
            stdout: "OK".to_string(),
            stderr: String::new(),
            success: true,
        })
    }
}
