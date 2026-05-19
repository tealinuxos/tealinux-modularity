pub mod command;
pub mod models;
mod mirror_countries;

use crate::pkexec_args::{merged_output_text, pkexec_stderr_local_result, run_pkexec_program};
use mirror_countries::is_allowed_mirror_country;
use crate::utils::error_libs::{LocalCommandOutput, LocalModulariteaError};
use crate::utils::modularitea_path::resolve_on_path;
use modularitea_libs::infrastructure::news_parser::{NewsParser, ParsedNewsItem};
use serde::Serialize;
use specta::Type;
use std::path::Path;
use std::process::Command;

const ZRAM_CONF_PATH: &str = "/etc/systemd/zram-generator.conf";
const RESOLV_CONF_PATH: &str = "/etc/resolv.conf";
const CPU_FREQ_GOV_PATH: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";

#[derive(Debug, Clone, Serialize, Type)]
pub struct ParsedNewsItemDto {
    pub url: String,
    pub title: String,
    pub descriptive: String,
    pub thumbnail: Option<String>,
}

impl From<ParsedNewsItem> for ParsedNewsItemDto {
    fn from(i: ParsedNewsItem) -> Self {
        ParsedNewsItemDto {
            url: i.url,
            title: i.title,
            descriptive: i.descriptive,
            thumbnail: i.thumbnail,
        }
    }
}

#[derive(Debug, Serialize, Type)]
pub struct ApiResultParsedNews {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ParsedNewsItemDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Type)]
pub struct ApiResultVoid {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Type)]
pub struct ApiResultBool {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Type)]
pub struct ApiResultStr {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

pub(crate) fn ok_void() -> ApiResultVoid {
    ApiResultVoid {
        success: true,
        error: None,
        code: None,
    }
}

pub fn err_void<E: ToString>(e: E, code: &'static str) -> ApiResultVoid {
    ApiResultVoid {
        success: false,
        error: Some(e.to_string()),
        code: Some(code.to_string()),
    }
}

pub fn err_str<E: ToString>(e: E, code: &'static str) -> ApiResultStr {
    ApiResultStr {
        success: false,
        data: None,
        error: Some(e.to_string()),
        code: Some(code.to_string()),
    }
}

fn classify_local_err(e: &LocalModulariteaError) -> &'static str {
    match e {
        LocalModulariteaError::PrivilegeError { .. }
        | LocalModulariteaError::PkexecNotFound
        | LocalModulariteaError::PolkitCancelled => "PERMISSION_DENIED",
        LocalModulariteaError::CommandError { stderr, .. } => {
            let s = stderr.to_lowercase();
            if s.contains("network") || s.contains("timeout") || s.contains("could not resolve") {
                "NETWORK_ERROR"
            } else {
                "COMMAND_FAILED"
            }
        }
        LocalModulariteaError::InternalError(s) if s.to_lowercase().contains("not found") => {
            "INVALID_ARGUMENT"
        }
        _ => "UNKNOWN_ERROR",
    }
}

pub fn classify_any_msg(s: &str) -> &'static str {
    let l = s.to_lowercase();
    if l.contains("permission")
        || l.contains("denied")
        || l.contains("polkit")
        || l.contains("not authorized")
    {
        return "PERMISSION_DENIED";
    }
    if l.contains("timeout") || l.contains("timed out") {
        return "TIMEOUT";
    }
    if l.contains("network") || l.contains("resolve") || l.contains("dns") {
        return "NETWORK_ERROR";
    }
    "UNKNOWN_ERROR"
}

pub fn map_mkerr_void(msg: impl Into<String>, code_str: &'static str) -> ApiResultVoid {
    ApiResultVoid {
        success: false,
        error: Some(msg.into()),
        code: Some(code_str.to_string()),
    }
}

pub fn map_pkexec_void(res: Result<LocalCommandOutput, LocalModulariteaError>) -> ApiResultVoid {
    match res {
        Ok(_) => ok_void(),
        Err(e) => ApiResultVoid {
            success: false,
            error: Some(format!("{:?}", e)),
            code: Some(classify_local_err(&e).to_string()),
        },
    }
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_parsed_news(force_refresh: bool) -> ApiResultParsedNews {
    match tokio::task::spawn_blocking(move || match NewsParser::new() {
        Ok(parser) => {
            let r = if force_refresh {
                parser.force_refresh_cache()
            } else {
                parser.blackbox_fetcher()
            };
            r.map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    })
    .await
    {
        Ok(Ok(items)) => ApiResultParsedNews {
            success: true,
            data: Some(items.into_iter().map(ParsedNewsItemDto::from).collect()),
            error: None,
            code: None,
        },
        Ok(Err(msg)) => ApiResultParsedNews {
            success: false,
            data: None,
            error: Some(msg.clone()),
            code: Some(classify_any_msg(&msg).to_string()),
        },
        Err(e) => ApiResultParsedNews {
            success: false,
            data: None,
            error: Some(e.to_string()),
            code: Some("UNKNOWN_ERROR".to_string()),
        },
    }
}

#[tauri::command]
#[specta::specta]
pub async fn mirror_reflector_country_list() -> Vec<String> {
    mirror_countries::ALLOWED_REFLECTOR_COUNTRIES
        .iter()
        .map(|s| (*s).to_string())
        .collect()
}

#[tauri::command]
#[specta::specta]
pub async fn settings_refresh_mirror(country: String) -> ApiResultStr {
    let trimmed = country.trim().to_owned();
    if !is_allowed_mirror_country(&trimmed) {
        return ApiResultStr {
            success: false,
            data: None,
            error: Some(format!("disallowed country: {}", trimmed)),
            code: Some("INVALID_ARGUMENT".to_string()),
        };
    }

    use modularitea_libs::infrastructure::tools_utils::MirrorUtils;
    match tokio::task::spawn_blocking(move || MirrorUtils::set_country(Some(trimmed.clone())).refresh_fastest_mirror()).await {
        Ok(Ok(co)) => ApiResultStr {
            success: true,
            data: Some(format!("{}{}", co.stdout, co.stderr).trim().to_string()),
            error: None,
            code: None,
        },
        Ok(Err(e)) => {
            let s = format!("{}", e);
            ApiResultStr {
                success: false,
                data: None,
                error: Some(s.clone()),
                code: Some(classify_any_msg(&s).to_string()),
            }
        },
        Err(e) => err_str(e.to_string(), "UNKNOWN_ERROR"),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_change_dns(provider: String) -> ApiResultVoid {
    let key = provider.trim().to_lowercase();
    if !matches!(key.as_str(), "cloudflare" | "google" | "quad9") {
        return err_void(format!("unsupported DNS provider: {}", provider.trim()), "INVALID_ARGUMENT");
    }

    let Some(bin) = resolve_on_path("modularitea-dns-changer") else {
        return map_mkerr_void("modularitea-dns-changer binary not found", "COMMAND_FAILED");
    };

    match run_pkexec_program(&bin, &[key]) {
        Ok(out) => map_pkexec_void(pkexec_stderr_local_result("modularitea-dns-changer", out)),
        Err(e) => map_pkexec_void(Err(e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_dns_status_line() -> ApiResultStr {
    match tokio::task::spawn_blocking(|| std::fs::read_to_string(RESOLV_CONF_PATH)).await {
        Ok(Ok(content)) => {
            let ips: Vec<String> = content
                .lines()
                .filter_map(|ln| ln.strip_prefix("nameserver "))
                .map(|x| x.trim().to_string())
                .take(8)
                .collect();
            ApiResultStr {
                success: true,
                data: Some(ips.join(", ")),
                error: None,
                code: None,
            }
        },
        Ok(Err(e)) => err_str(format!("{}", e), "COMMAND_FAILED"),
        Err(e) => err_str(e.to_string(), "UNKNOWN_ERROR"),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_toggle_swap(enabled: bool) -> ApiResultVoid {
    let Some(bin) = resolve_on_path("modularitea-swap") else {
        return map_mkerr_void("modularitea-swap binary not found", "COMMAND_FAILED");
    };

    let arg = if enabled { "on" } else { "off" }.to_string();
    match run_pkexec_program(&bin, &[arg]) {
        Ok(out) => map_pkexec_void(pkexec_stderr_local_result("modularitea-swap", out)),
        Err(e) => map_pkexec_void(Err(e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_swap_enabled_state() -> ApiResultBool {
    match tokio::task::spawn_blocking(|| Path::new(ZRAM_CONF_PATH).exists()).await {
        Ok(ok) => ApiResultBool {
            success: true,
            data: Some(ok),
            error: None,
            code: None,
        },
        Err(e) => ApiResultBool {
            success: false,
            data: None,
            error: Some(e.to_string()),
            code: Some("UNKNOWN_ERROR".to_string()),
        },
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_clean_package_cache() -> ApiResultBool {
    let Some(bin) = resolve_on_path("modularitea-package-cache-cleaner") else {
        return ApiResultBool {
            success: false,
            data: None,
            error: Some("modularitea-package-cache-cleaner binary not found".into()),
            code: Some("COMMAND_FAILED".to_string()),
        };
    };

    match tokio::task::spawn_blocking(move || Command::new(&bin).output()).await {
        Ok(Ok(out)) => {
            if !out.status.success() {
                return ApiResultBool {
                    success: false,
                    data: Some(false),
                    error: Some(merged_output_text(&out)),
                    code: Some("COMMAND_FAILED".to_string()),
                };
            }
            ApiResultBool {
                success: true,
                data: Some(true),
                error: None,
                code: None,
            }
        },
        Ok(Err(e)) => ApiResultBool {
            success: false,
            data: Some(false),
            error: Some(e.to_string()),
            code: Some("COMMAND_FAILED".to_string()),
        },
        Err(e) => ApiResultBool {
            success: false,
            data: None,
            error: Some(e.to_string()),
            code: Some("UNKNOWN_ERROR".to_string()),
        },
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_set_cpu_profile(profile: String) -> ApiResultVoid {
    use modularitea_libs::infrastructure::tools_utils::CpuBooster;
    let p = profile.trim().to_lowercase();
    if !matches!(p.as_str(), "powersave" | "performance" | "ondemand") {
        return err_void(format!("invalid CPU profile: {}", profile.trim()), "INVALID_ARGUMENT");
    }

    match tokio::task::spawn_blocking(move || CpuBooster::set_profile(&p)).await {
        Ok(Ok(_)) => ok_void(),
        Ok(Err(e)) => {
            let s = format!("{}", e);
            err_void(s.clone(), classify_any_msg(&s))
        }
        Err(e) => err_void(e.to_string(), "UNKNOWN_ERROR"),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn settings_cpu_governor_line() -> ApiResultStr {
    match tokio::task::spawn_blocking(|| std::fs::read_to_string(CPU_FREQ_GOV_PATH)).await {
        Ok(Ok(s)) => ApiResultStr {
            success: true,
            data: Some(s.trim().to_string()),
            error: None,
            code: None,
        },
        Ok(Err(e)) => err_str(format!("{}", e), "COMMAND_FAILED"),
        Err(e) => err_str(e.to_string(), "UNKNOWN_ERROR"),
    }
}
