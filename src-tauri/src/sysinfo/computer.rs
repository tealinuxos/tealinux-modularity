use crate::sysinfo::{
    types::{
        cpu_fastfetch::CpuFastfetch, kernel_fastfetch::KernelFastfetch,
        memory_fastfetch::MemoryFastfetch, os_fastfetch::OSFastfetch,
        users_fastfetch::UsersFastfetch,
    },
    utils::fetch_module::fetch_module,
};

#[derive(serde::Serialize, specta::Type)]
pub struct Computer {
    pub processor: String,
    pub memory: i64, // on bytes
    pub operating_system: String,
    pub kernel_version: String,
    pub username: Vec<String>,
    pub errors: Vec<String>,
}

impl Computer {
    pub fn new() -> Self {
        let mut errors: Vec<String> = Vec::new();

        let cpu_vec: CpuFastfetch = match fetch_module("CPU") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let mem_vec: MemoryFastfetch = match fetch_module("Memory") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let user_vec: UsersFastfetch = match fetch_module("Users") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let os_vec: OSFastfetch = match fetch_module("OS") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };
        let kernel_vec: KernelFastfetch = match fetch_module("Kernel") {
            Ok(data) => data,
            Err(err) => {
                errors.push(err);
                Vec::new()
            }
        };

        let processor = cpu_vec
            .into_iter()
            .next()
            .map(|cpu_item| {
                format!(
                    "{} {}x{} Core",
                    cpu_item.result.cpu, cpu_item.result.cores.physical, cpu_item.result.cores.logical
                )
            })
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let kernel_version = kernel_vec
            .into_iter()
            .next()
            .map(|kernel_item| {
                format!(
                    "{} {}",
                    kernel_item.result.release, kernel_item.result.architecture
                )
            })
            .unwrap_or_else(|| "Unknown Kernel".to_string());

        let username = user_vec
            .into_iter()
            .next()
            .map(|item| item.result.into_iter().map(|u| u.name).collect())
            .unwrap_or_default();

        let memory = mem_vec.into_iter().next().map(|item| item.result.total).unwrap_or(0);
        let operating_system = os_vec
            .into_iter()
            .next()
            .map(|item| item.result.pretty_name)
            .unwrap_or_else(|| "Unknown OS".to_string());

        Computer {
            processor,
            memory,
            operating_system,
            kernel_version,
            username,
            errors,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn computer_info() -> Computer {
    Computer::new()
}
