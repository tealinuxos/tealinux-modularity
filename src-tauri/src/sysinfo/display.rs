use crate::sysinfo::{
    types::{
        display_fastfetch::DisplayFastfetch, gpu_fastfetch::GPUFastfetch, wm_fastfetch::WMFastfetch,
    },
    utils::fetch_module::fetch_module,
};

#[derive(serde::Serialize, specta::Type)]
pub struct Display {
    monitor_name: Vec<String>,
    graphic_cards: Vec<String>,
    display_protocol: String,
    display_windows_manager: String,
    errors: Vec<String>,
}

impl Display {
    pub fn new() -> Self {
        let mut errors: Vec<String> = Vec::new();

        let gpu_vec: GPUFastfetch = match fetch_module("GPU") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let wm_vec: WMFastfetch = match fetch_module("WM") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let display_vec: DisplayFastfetch = match fetch_module("Display") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };

        let gpu_item = gpu_vec
            .into_iter()
            .next()
            .map(|item| item.result.into_iter().map(|g| g.name).collect())
            .unwrap_or_default();

        let display_item = display_vec
            .into_iter()
            .next()
            .map(|item| {
                item.result
                    .into_iter()
                    .map(|d| {
                        format!(
                            "{}: {}x{}@{}hz",
                            d.name, d.output.width, d.output.height, d.output.refresh_rate
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();

        let (display_protocol, display_windows_manager) = wm_vec
            .into_iter()
            .next()
            .map(|wm_item| (wm_item.result.protocol_name, wm_item.result.pretty_name))
            .unwrap_or_else(|| ("Unknown Protocol".to_string(), "Unknown WM".to_string()));

        Display {
            monitor_name: display_item,
            graphic_cards: gpu_item,
            display_protocol,
            display_windows_manager,
            errors,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn display_info() -> Display {
    Display::new()
}
