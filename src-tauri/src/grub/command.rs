use modularitea_libs::infrastructure::grub::GrubInstructionExecutor;

use crate::grub::{models::LocalThemeManifest, initialization::GrubManager};

#[tauri::command]
#[specta::specta]
pub fn get_grub_themes(state: tauri::State<'_, GrubManager>) -> Vec<LocalThemeManifest> {
    let themes = state.instruction.get_all_theme_available();

    themes.into_iter().map(LocalThemeManifest::from).collect()
}

// sabar, proses
// #[tauri::command]
// pub fn set_grub_theme(state: tauri::State<'_, GrubManager>, theme: String) {
//     let _a = state.instruction.apply_grub_theme(&theme);
// }
