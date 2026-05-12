use modularitea_libs::infrastructure::tools_utils::mode;
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum DnsProvider {
    Cloudflare,
    Google,
    Quad9,
    Opendns,
    Adguard,
}

impl DnsProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            DnsProvider::Cloudflare => "cloudflare",
            DnsProvider::Google => "google",
            DnsProvider::Quad9 => "quad9",
            DnsProvider::Opendns => "opendns",
            DnsProvider::Adguard => "adguard",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum CpuProfile {
    Powersave,
    Performance,
    Ondemand,
}

impl CpuProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            CpuProfile::Powersave => "powersave",
            CpuProfile::Performance => "performance",
            CpuProfile::Ondemand => "ondemand",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum SwapMode {
    Enable,
    Disable,
}

impl From<SwapMode> for mode::SwapMode {
    fn from(mode: SwapMode) -> Self {
        match mode {
            SwapMode::Enable => mode::SwapMode::Enable,
            SwapMode::Disable => mode::SwapMode::Disable,
        }
    }
}
