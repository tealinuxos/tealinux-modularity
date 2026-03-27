use modularitea_libs::infrastructure::grub::GrubInstructionExecutor;
use std::io::ErrorKind;
use std::process::Command;

use crate::grub::initialization::GrubManager;
use crate::grub::models::LocalThemeManifest;
use crate::utils::error_libs::{LocalCommandOutput, LocalModulariteaError};

const GRUB_THEME_DIR: &str = "/usr/share/modularitea-libs/grub-theme/";

#[tauri::command]
#[specta::specta]
pub fn get_grub_themes(state: tauri::State<'_, GrubManager>) -> Vec<LocalThemeManifest> {
    let themes = state.instruction.get_all_theme_available();
    themes.into_iter().map(LocalThemeManifest::from).collect()
}

#[tauri::command]
#[specta::specta]
pub fn set_grub_theme(
    _state: tauri::State<'_, GrubManager>,
    theme_name: String,
) -> Result<LocalCommandOutput, LocalModulariteaError> {
    let run_result = Command::new("pkexec")
        .arg("modularitea-grub")
        .arg(GRUB_THEME_DIR)
        .arg(&theme_name)
        .output();

    let run_output = match run_result {
        Ok(output) => output,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Err(LocalModulariteaError::RootBinaryNotFound {
                binary: "modularitea-grub".to_string(),
            })
        }
        Err(e) => {
            return Err(LocalModulariteaError::CommandError {
                command: "modularitea-grub".to_string(),
                exit_code: None,
                stderr: format!("Failed to run modularitea-grub: {}", e),
            })
        }
    };

    if !run_output.status.success() {
        if run_output.status.code() == Some(126) {
            return Err(LocalModulariteaError::PolkitCancelled);
        }

        return Err(LocalModulariteaError::CommandError {
            command: "modularitea-grub".to_string(),
            exit_code: run_output.status.code(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
        });
    }

    Ok(LocalCommandOutput {
        success: run_output.status.success(),
        exit_code: run_output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
    })
}
