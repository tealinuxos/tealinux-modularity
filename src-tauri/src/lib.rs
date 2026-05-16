#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

use crate::grub::initialization::GrubManager;

mod grub;
mod installer;
mod settings;
mod splash;
mod sysinfo;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // GRUB Manager Struct Setup
    let grub_manager = GrubManager::new();

    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        // System info commands
        sysinfo::computer::computer_info,
        sysinfo::display::display_info,
        sysinfo::audio::audio_info,
        // Package management commands
        installer::backend_runner::install_packages,
        installer::backend_runner::remove_packages,
        installer::backend_runner::update_db,
        installer::backend_runner::check_package_installed,
        // Profile commands (install/uninstall via pacman + systemctl directly)
        installer::backend_runner::install_profile,
        installer::backend_runner::uninstall_profile,
        // Service commands (via pkexec systemctl)
        installer::backend_runner::enable_service,
        installer::backend_runner::disable_service,
        // Profile loader (reads TOML files)
        installer::profiler::list_profiles,
        installer::profiler::get_profile,
        // Splash/Setup
        splash::display_splash::init_configuration_file,
        splash::display_splash::check_configuration_file,
        splash::display_splash::show_main_window,
        // GRUB Commands
        grub::command::get_grub_themes,
        grub::command::set_grub_theme,
        // Settings Commands
        settings::command::get_cache_size,
        settings::command::clean_cache,
        settings::command::refresh_mirror,
        settings::command::switch_dns,
        settings::command::set_cpu_profile,
        settings::command::set_swap_mode,
    ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::BigInt)
                .formatter(specta_typescript::formatter::biome)
                .header("// @ts-nocheck"),
            "../src/lib/commands.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // GRUB Manager State Setup
        .manage(grub_manager)
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            let is_initialized =
                splash::display_splash::check_configuration_file().unwrap_or(false);
            eprintln!("[setup] is_initialized: {}", is_initialized);

            if is_initialized {
                // Already initialized: show main window directly
                if let Some(main_win) = app.get_webview_window("main") {
                    let _ = main_win.show();
                    eprintln!("[setup] Main window shown");
                }
                // Close splashscreen since it's not needed
                if let Some(splash_win) = app.get_webview_window("splashscreen") {
                    let _ = splash_win.close();
                    eprintln!("[setup] Splashscreen closed");
                }
            } else {
                // Not initialized: show splashscreen for onboarding
                if let Some(splash_win) = app.get_webview_window("splashscreen") {
                    let _ = splash_win.show();
                    eprintln!("[setup] Splashscreen shown");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
