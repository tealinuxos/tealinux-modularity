#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

mod installer;
mod splash;
mod sysinfo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        sysinfo::computer::computer_info,
        sysinfo::display::display_info,
        sysinfo::audio::audio_info,
        splash::display_splash::init_configuration_file,
        splash::display_splash::check_configuration_file,
        splash::display_splash::show_main_window,
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
