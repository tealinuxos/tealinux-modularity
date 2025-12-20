use crate::sysinfo::{types::sound_fastfetch::SoundFastfetch, utils::fetch_module::fetch_module};

#[derive(serde::Serialize)]
pub struct Audio {
    devices: Vec<String>,
}

impl Audio {
    pub fn new() -> Self {
        let audio_vec: SoundFastfetch = fetch_module("Sound");

        let audio_item = audio_vec
            .into_iter()
            .next()
            .map(|item| {
                item.result
                    .into_iter()
                    .map(|a| format!("{} with {}", a.name, a.platform_api))
                    .collect()
            })
            .unwrap_or_else(|| vec![]);

        Audio {
            devices: audio_item,
        }
    }
}

#[tauri::command]
pub fn audio_info() -> Audio {
    Audio::new()
}
