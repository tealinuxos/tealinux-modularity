use crate::grub::executor::GrubInstructionExecutor;

use crate::grub::initialization::GrubManager;
use crate::grub::models::ThemeManifest;
use crate::utils::error_libs::{LocalCommandOutput, LocalModulariteaError};

#[tauri::command]
#[specta::specta]
pub fn get_grub_themes(state: tauri::State<'_, GrubManager>) -> Vec<ThemeManifest> {
    state.instruction.get_all_theme_available()
}

#[tauri::command]
#[specta::specta]
pub fn set_grub_theme(
    state: tauri::State<'_, GrubManager>,
    theme_name: String,
) -> Result<LocalCommandOutput, LocalModulariteaError> {
    match state.instruction.apply_grub_theme(&theme_name) {
        Ok(output) => Ok(LocalCommandOutput::from(output)),
        Err(e) => Err(LocalModulariteaError::from(e)),
    }
}
