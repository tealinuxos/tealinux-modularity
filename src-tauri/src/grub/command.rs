use crate::grub::executor::GrubInstructionExecutor;
use crate::grub::initialization::GrubManager;
use crate::grub::models::ThemeManifest;
use crate::pkexec_args::{pkexec_stderr_local_result, run_pkexec_program};
use crate::settings::ApiResultVoid;
use crate::sysinfo::display_resolution::grub_screen_resolution_px;
use crate::utils::modularitea_path::resolve_on_path;

#[tauri::command]
#[specta::specta]
pub fn get_grub_themes(state: tauri::State<'_, std::sync::Mutex<GrubManager>>) -> Vec<ThemeManifest> {
    let mut g = state.lock().expect("GrubManager mutex poisoned");
    let (w, h) = grub_screen_resolution_px();
    g.instruction.reload_manifest();
    g.instruction = g.instruction.clone_with_resolution(w, h);
    g.instruction.get_all_theme_available()
}

#[tauri::command]
#[specta::specta]
pub fn set_grub_theme(state: tauri::State<'_, std::sync::Mutex<GrubManager>>, theme_name: String) -> ApiResultVoid {
    use crate::settings::{err_void, map_mkerr_void, map_pkexec_void};
    let theme_name = theme_name.trim().to_string();
    if theme_name.is_empty() {
        return err_void("empty theme name", "INVALID_ARGUMENT");
    }

    let (themes_dir, known): (String, bool) = {
        let mut g = match state.lock() {
            Ok(l) => l,
            Err(_) => return err_void("Grub manager lock poisoned", "UNKNOWN_ERROR"),
        };
        let (w, h) = grub_screen_resolution_px();
        g.instruction.reload_manifest();
        g.instruction = g.instruction.clone_with_resolution(w, h);
        let themes_dir = g.instruction.themes_dir.clone();
        let known = g.instruction.manifest.iter().any(|m| m.name == theme_name);
        (themes_dir, known)
    };

    if !known {
        return err_void(format!("unknown theme: {}", theme_name), "INVALID_ARGUMENT");
    }

    let Some(bin) = resolve_on_path("modularitea-grub") else {
        return map_mkerr_void("modularitea-grub binary not found", "COMMAND_FAILED");
    };

    let (w, h) = grub_screen_resolution_px();
    let args = vec![themes_dir, theme_name.clone(), w.to_string(), h.to_string()];

    match run_pkexec_program(&bin, &args) {
        Ok(output) => map_pkexec_void(pkexec_stderr_local_result("modularitea-grub", output)),
        Err(e) => map_pkexec_void(Err(e)),
    }
}
