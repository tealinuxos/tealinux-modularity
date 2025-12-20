use crate::sysinfo::{
    types::{
        display_fastfetch::DisplayFastfetch, gpu_fastfetch::GPUFastfetch, wm_fastfetch::WMFastfetch,
    },
    utils::fetch_module::fetch_module,
};

#[derive(serde::Serialize)]
pub struct Display {
    monitor_name: Vec<String>,
    graphic_cards: Vec<String>,
    display_protocol: String,
    display_windows_manager: String,
}

impl Display {
    pub fn new() -> Self {
        let gpu_vec: GPUFastfetch = fetch_module("GPU");
        let wm_vec: WMFastfetch = fetch_module("WM");
        let display_vec: DisplayFastfetch = fetch_module("Display");

        let gpu_item = gpu_vec
            .into_iter()
            .next()
            .map(|item| item.result.into_iter().map(|g| g.name).collect())
            .unwrap_or_else(|| vec![]);

        let display_item = display_vec
            .into_iter()
            .next()
            .map(|item| {
                item.result
                    .into_iter()
                    .map(|d| {
                        format!(
                            "{}: {}x{}@{}",
                            d.name, d.output.width, d.output.height, d.output.refresh_rate
                        )
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec![]);

        let wm_item = wm_vec
            .into_iter()
            .next()
            .expect("Windows Manager Data is not found!");

        Display {
            monitor_name: display_item,
            graphic_cards: gpu_item,
            display_protocol: wm_item.result.protocol_name,
            display_windows_manager: wm_item.result.pretty_name,
        }
    }
}

#[tauri::command]
pub fn display_info() -> Display {
    Display::new()
}
