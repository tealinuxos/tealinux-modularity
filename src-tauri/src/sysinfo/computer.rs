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
}

impl Computer {
    pub fn new() -> Self {
        let cpu_vec: CpuFastfetch = fetch_module("CPU");
        let mem_vec: MemoryFastfetch = fetch_module("Memory");
        let user_vec: UsersFastfetch = fetch_module("Users");
        let os_vec: OSFastfetch = fetch_module("OS");
        let kernel_vec: KernelFastfetch = fetch_module("Kernel");

        let cpu_item = cpu_vec.into_iter().next().expect("CPU Data is not found!");

        let processor = format!(
            "{} {}x{} Core",
            cpu_item.result.cpu, cpu_item.result.cores.physical, cpu_item.result.cores.logical
        );

        let kernel_item = kernel_vec
            .into_iter()
            .next()
            .expect("Kernel Data is not found!");

        let kernel_version = format!(
            "{} {}",
            kernel_item.result.release, kernel_item.result.architecture
        );

        let username = user_vec
            .into_iter()
            .next()
            .map(|item| item.result.into_iter().map(|u| u.name).collect())
            .unwrap_or_default();

        let mem_item = mem_vec.into_iter().next().expect("Data Memory Error");
        let os_item = os_vec.into_iter().next().expect("Data OS Error");

        Computer {
            processor,
            memory: mem_item.result.total,
            operating_system: os_item.result.pretty_name,
            kernel_version,
            username,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn computer_info() -> Computer {
    Computer::new()
}
