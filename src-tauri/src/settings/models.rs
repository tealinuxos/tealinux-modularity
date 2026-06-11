use modularitea_libs::infrastructure::tools_utils::mode;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
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

    pub fn from_lib_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "cloudflare" => Some(Self::Cloudflare),
            "google" => Some(Self::Google),
            "quad9" => Some(Self::Quad9),
            "opendns" => Some(Self::Opendns),
            "adguard" => Some(Self::Adguard),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
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

    pub fn from_lib_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "powersave" => Some(Self::Powersave),
            "performance" => Some(Self::Performance),
            "ondemand" => Some(Self::Ondemand),
            _ => None,
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
