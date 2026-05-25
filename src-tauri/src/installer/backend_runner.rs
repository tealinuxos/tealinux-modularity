use modularitea_libs::infrastructure::Pacman;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BackendResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl BackendResult {
    pub fn from_output(output: std::process::Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            stdout: "".to_string(),
            stderr: msg.to_string(),
            exit_code: -1,
        }
    }

    pub fn ok(msg: &str) -> Self {
        Self {
            success: true,
            stdout: msg.to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        }
    }
}

/// Run a binary via pkexec
fn run_pkexec(binary_path: &str, args: &[&str]) -> BackendResult {
    let mut cmd = Command::new("pkexec");
    cmd.arg(binary_path);
    cmd.args(args);

    eprintln!(
        "[backend_runner] Running: pkexec {} {}",
        binary_path,
        args.join(" ")
    );

    match cmd.output() {
        Ok(output) => {
            let result = BackendResult::from_output(output);
            eprintln!(
                "[backend_runner] Exit code: {}, success: {}",
                result.exit_code, result.success
            );
            if !result.stderr.is_empty() {
                eprintln!("[backend_runner] stderr: {}", result.stderr);
            }
            result
        }
        Err(e) => BackendResult::error(&format!("Failed to spawn pkexec: {}", e)),
    }
}

/// Install packages via pacman (uses pkexec for root)
#[tauri::command]
#[specta::specta]
pub async fn install_packages(packages: Vec<String>) -> BackendResult {
    if packages.is_empty() {
        return BackendResult::error("No packages specified");
    }

    // Build pacman command: pacman -S --noconfirm --needed pkg1 pkg2 ...
    let mut args = vec!["-S", "--noconfirm", "--needed"];
    let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
    args.extend_from_slice(&pkg_refs);

    run_pkexec("pacman", &args)
}

/// Remove packages via pacman (uses pkexec for root).
/// Tries each package individually so one dependency failure doesn't block the rest.
/// When `force` is true, uses `-Rdd` (skip dependency checks) instead of `-R`.
#[tauri::command]
#[specta::specta]
pub async fn remove_packages(packages: Vec<String>, force: bool) -> BackendResult {
    if packages.is_empty() {
        return BackendResult::error("No packages specified");
    }

    let remove_flag = if force { "-Rdd" } else { "-R" };
    eprintln!("[remove_packages] Mode: {} (force={})", remove_flag, force);

    let mut all_stdout = String::new();
    let mut all_stderr = String::new();
    let mut succeeded = 0u32;
    let mut failed = 0u32;

    for pkg in &packages {
        // Resolve real package name (handles virtual packages like 'netcat')
        let real_name = Pacman::resolve_package(pkg);

        if real_name != *pkg {
            eprintln!("[remove_packages] Resolved '{}' to '{}'", pkg, real_name);
        }

        eprintln!("[remove_packages] Removing '{}'...", real_name);
        let result = run_pkexec("pacman", &[remove_flag, "--noconfirm", &real_name]);

        if result.success {
            succeeded += 1;
            all_stdout.push_str(&format!("✓ Removed '{}'\n", pkg));
        } else {
            // Check if it's a "target not found" error
            if result.stderr.contains("target not found") {
                // Double check if it's actually gone
                let is_still_there = check_package_installed(pkg.clone()).await;
                if !is_still_there {
                    succeeded += 1;
                    all_stdout.push_str(&format!("✓ '{}' was already not present\n", pkg));
                    continue;
                }
            }

            failed += 1;
            // Check if it's a dependency issue
            if result.stderr.contains("could not satisfy dependencies")
                || result.stderr.contains("breaks dependency")
            {
                let detailed_msg = result
                    .stderr
                    .lines()
                    .find(|l| l.contains("required by"))
                    .map(|l| l.trim_start_matches(":: ").trim())
                    .unwrap_or("other packages depend on it");

                all_stderr.push_str(&format!("✗ Cannot remove '{}': {}\n", pkg, detailed_msg));
            } else {
                all_stderr.push_str(&format!(
                    "✗ Failed to remove '{}': {}\n",
                    pkg,
                    result.stderr.trim()
                ));
            }
        }
    }

    if succeeded > 0 {
        all_stdout.push_str(&format!("\n{} package(s) removed successfully.", succeeded));
    }
    if failed > 0 {
        all_stderr.push_str(&format!("\n{} package(s) could not be removed.", failed));
    }

    BackendResult {
        success: failed == 0,
        stdout: all_stdout,
        stderr: all_stderr,
        exit_code: if failed == 0 { 0 } else { 1 },
    }
}

/// Update pacman database (uses pkexec for root)
#[tauri::command]
#[specta::specta]
pub async fn update_db() -> BackendResult {
    run_pkexec("pacman", &["-Sy"])
}

/// Install a full profile.
///
/// This reads the profile data from the frontend (which already parsed the TOML),
/// then calls pacman install + systemctl enable as needed.
#[tauri::command]
#[specta::specta]
pub async fn install_profile(
    profile_name: String,
    packages: Vec<String>,
    services: Vec<String>,
) -> BackendResult {
    if profile_name.is_empty() {
        return BackendResult::error("No profile name specified");
    }

    eprintln!(
        "[install_profile] Installing profile '{}': {} packages, {} services",
        profile_name,
        packages.len(),
        services.len()
    );

    let mut all_stdout = String::new();
    let mut all_stderr = String::new();

    // Step 0: Auto-update pacman database to avoid stale mirror 404 errors
    if !packages.is_empty() {
        eprintln!("[install_profile] Updating pacman database...");
        let db_result = run_pkexec("pacman", &["-Sy"]);
        if db_result.success {
            all_stdout.push_str("✓ Package database updated\n");
        } else {
            all_stderr.push_str(&format!(
                "⚠ Database update warning: {}\n",
                db_result.stderr
            ));
            // Don't fail — continue with install, it might still work
        }
    }

    // Step 1: Install packages via pacman
    if !packages.is_empty() {
        eprintln!(
            "[install_profile] Installing {} packages...",
            packages.len()
        );

        let mut args = vec!["-S", "--noconfirm", "--needed"];
        let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&pkg_refs);

        let result = run_pkexec("pacman", &args);
        all_stdout.push_str(&result.stdout);

        if !result.success {
            all_stderr.push_str(&format!("Package installation failed: {}\n", result.stderr));
            return BackendResult {
                success: false,
                stdout: all_stdout,
                stderr: all_stderr,
                exit_code: result.exit_code,
            };
        }

        all_stdout.push_str(&format!(
            "✓ {} packages installed successfully\n",
            packages.len()
        ));
    }

    // Step 2: Enable services via systemctl
    if !services.is_empty() {
        eprintln!("[install_profile] Enabling {} services...", services.len());

        for svc in &services {
            let result = run_pkexec("systemctl", &["enable", "--now", svc]);

            if result.success {
                all_stdout.push_str(&format!("✓ Service '{}' enabled and started\n", svc));
            } else {
                all_stderr.push_str(&format!(
                    " Failed to enable service '{}': {}\n",
                    svc, result.stderr
                ));
                // Don't fail the whole profile — just warn
            }
        }
    }

    all_stdout.push_str(&format!(
        "\n Profile '{}' installed successfully!",
        profile_name
    ));

    BackendResult {
        success: true,
        stdout: all_stdout,
        stderr: all_stderr,
        exit_code: 0,
    }
}

/// Uninstall a profile's packages
#[tauri::command]
#[specta::specta]
pub async fn uninstall_profile(
    profile_name: String,
    packages: Vec<String>,
    services: Vec<String>,
) -> BackendResult {
    if profile_name.is_empty() {
        return BackendResult::error("No profile name specified");
    }

    let mut all_stdout = String::new();

    // Step 1: Disable services
    for svc in &services {
        let result = run_pkexec("systemctl", &["disable", "--now", svc]);
        if result.success {
            all_stdout.push_str(&format!("✓ Service '{}' disabled\n", svc));
        }
    }

    // Step 2: Remove packages
    if !packages.is_empty() {
        let mut args = vec!["-Rns", "--noconfirm"];
        let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&pkg_refs);

        let result = run_pkexec("pacman", &args);
        if !result.success {
            return BackendResult {
                success: false,
                stdout: all_stdout,
                stderr: format!("Package removal failed: {}", result.stderr),
                exit_code: result.exit_code,
            };
        }
        all_stdout.push_str(&format!("✓ {} packages removed\n", packages.len()));
    }

    all_stdout.push_str(&format!("\n Profile '{}' uninstalled!", profile_name));
    BackendResult::ok(&all_stdout)
}

/// Enable a service via systemctl (uses pkexec for root)
#[tauri::command]
#[specta::specta]
pub async fn enable_service(service_name: String, start_now: bool) -> BackendResult {
    if service_name.is_empty() {
        return BackendResult::error("No service name specified");
    }
    if start_now {
        run_pkexec("systemctl", &["enable", "--now", &service_name])
    } else {
        run_pkexec("systemctl", &["enable", &service_name])
    }
}

/// Disable a service via systemctl (uses pkexec for root)
#[tauri::command]
#[specta::specta]
pub async fn disable_service(service_name: String, stop_now: bool) -> BackendResult {
    if service_name.is_empty() {
        return BackendResult::error("No service name specified");
    }
    if stop_now {
        run_pkexec("systemctl", &["disable", "--now", &service_name])
    } else {
        run_pkexec("systemctl", &["disable", &service_name])
    }
}

#[tauri::command]
#[specta::specta]
pub async fn check_package_installed(package_name: String) -> bool {
    Pacman::is_installed(&package_name).unwrap_or(false)
}

/// Per-package size information from `pacman -Si`
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PackageDownloadInfo {
    pub name: String,
    pub download_size_bytes: f64,
    pub install_size_bytes: f64,
    pub download_size_human: String,
    pub install_size_human: String,
    pub available: bool,
}

/// Summary of total download/install size for a list of packages
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PackageSizeInfo {
    pub packages: Vec<PackageDownloadInfo>,
    pub total_download_bytes: f64,
    pub total_install_bytes: f64,
    pub total_download_human: String,
    pub total_install_human: String,
}

/// Parse a pacman size string like "42.50 MiB" or "1.23 GiB" into bytes
fn parse_pacman_size(s: &str) -> f64 {
    let s = s.trim();
    let parts: Vec<&str> = s.splitn(2, ' ').collect();
    if parts.len() < 2 {
        return 0.0;
    }
    let value: f64 = parts[0].parse().unwrap_or(0.0);
    match parts[1] {
        "B" => value,
        "KiB" => value * 1024.0,
        "MiB" => value * 1024.0 * 1024.0,
        "GiB" => value * 1024.0 * 1024.0 * 1024.0,
        "kB" | "KB" => value * 1000.0,
        "MB" => value * 1000.0 * 1000.0,
        "GB" => value * 1000.0 * 1000.0 * 1000.0,
        _ => 0.0,
    }
}

/// Format bytes into a human-readable string with appropriate unit
fn format_bytes(bytes: f64) -> String {
    if bytes <= 0.0 {
        return "0 B".to_string();
    }
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes;
    let mut unit = UNITS[0];
    for u in UNITS {
        if value < 1024.0 {
            unit = u;
            break;
        }
        value /= 1024.0;
        unit = u;
    }
    format!("{:.1} {}", value, unit)
}

/// Get download and install sizes for a list of packages using `pacman -Si`
/// Only queries packages that are NOT already installed (to reduce noise).
#[tauri::command]
#[specta::specta]
pub async fn get_package_sizes(packages: Vec<String>) -> PackageSizeInfo {
    let mut pkg_infos: Vec<PackageDownloadInfo> = Vec::new();
    let mut total_download: f64 = 0.0;
    let mut total_install: f64 = 0.0;

    for pkg in &packages {
        // Run `pacman -Si <pkg>` to get sync db info (always available even if installed)
        let output = Command::new("pacman")
            .args(["--noconfirm", "-Si", pkg.as_str()])
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mut dl_bytes: f64 = 0.0;
                let mut inst_bytes: f64 = 0.0;
                let mut found = false;

                for line in stdout.lines() {
                    if line.starts_with("Download Size") {
                        if let Some(val) = line.splitn(2, ':').nth(1) {
                            dl_bytes = parse_pacman_size(val.trim());
                            found = true;
                        }
                    } else if line.starts_with("Installed Size") {
                        if let Some(val) = line.splitn(2, ':').nth(1) {
                            inst_bytes = parse_pacman_size(val.trim());
                        }
                    }
                }

                total_download += dl_bytes;
                total_install += inst_bytes;

                pkg_infos.push(PackageDownloadInfo {
                    name: pkg.clone(),
                    download_size_bytes: dl_bytes,
                    install_size_bytes: inst_bytes,
                    download_size_human: format_bytes(dl_bytes),
                    install_size_human: format_bytes(inst_bytes),
                    available: found,
                });
            }
            Err(e) => {
                eprintln!("[get_package_sizes] Error querying '{}': {}", pkg, e);
                pkg_infos.push(PackageDownloadInfo {
                    name: pkg.clone(),
                    download_size_bytes: 0.0,
                    install_size_bytes: 0.0,
                    download_size_human: "Unknown".to_string(),
                    install_size_human: "Unknown".to_string(),
                    available: false,
                });
            }
        }
    }

    PackageSizeInfo {
        packages: pkg_infos,
        total_download_bytes: total_download,
        total_install_bytes: total_install,
        total_download_human: format_bytes(total_download),
        total_install_human: format_bytes(total_install),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Async streaming install — emits Tauri events for realtime frontend updates
// ─────────────────────────────────────────────────────────────────────────────

/// Global cancel flag: set to true by cancel_install command.
static CANCEL_REQUESTED: OnceLock<Arc<AtomicBool>> = OnceLock::new();

fn cancel_flag() -> Arc<AtomicBool> {
    CANCEL_REQUESTED
        .get_or_init(|| Arc::new(AtomicBool::new(false)))
        .clone()
}

// ─── Event payloads ──────────────────────────────────────────────────────────

// ─── Event payloads ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstallStartedPayload {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstallLogPayload {
    pub task_id: String,
    pub line: String,
    pub stream: String, // "stdout" | "stderr" | "system"
    pub ts: u64,        // unix ms
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstallFinishedPayload {
    pub task_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstallProgressPayload {
    pub task_id: String,
    pub step: String, // e.g. "db-update" | "installing" | "services" | "done"
    pub percent: u8,  // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ActiveInstallLog {
    pub line: String,
    pub stream: String,
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ActiveInstallTask {
    pub task_id: String,
    pub profile_name: String,
    pub packages: Vec<String>,
    pub services: Vec<String>,
    pub step: String,
    pub percent: u8,
    pub started_at: u64,
    pub logs: Vec<ActiveInstallLog>,
}

static ACTIVE_INSTALLS: OnceLock<Mutex<HashMap<String, ActiveInstallTask>>> = OnceLock::new();

pub fn active_installs() -> &'static Mutex<HashMap<String, ActiveInstallTask>> {
    ACTIVE_INSTALLS.get_or_init(|| Mutex::new(HashMap::new()))
}

// ─── Helper: current unix timestamp in ms ────────────────────────────────────

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ─── Helpers: emit events to frontend & update registry ──────────────────────

fn emit_log(app: &tauri::AppHandle, task_id: &str, line: &str, stream: &str) {
    let ts = now_ms();
    let _ = app.emit(
        "install-log",
        InstallLogPayload {
            task_id: task_id.to_string(),
            line: line.to_string(),
            stream: stream.to_string(),
            ts,
        },
    );

    // Update ACTIVE_INSTALLS
    if let Ok(mut map) = active_installs().lock() {
        if let Some(task) = map.get_mut(task_id) {
            task.logs.push(ActiveInstallLog {
                line: line.to_string(),
                stream: stream.to_string(),
                ts,
            });
            if task.logs.len() > 500 {
                task.logs.remove(0);
            }
        }
    }
}

fn emit_progress(app: &tauri::AppHandle, task_id: &str, step: &str, percent: u8) {
    let _ = app.emit(
        "install-progress",
        InstallProgressPayload {
            task_id: task_id.to_string(),
            step: step.to_string(),
            percent,
        },
    );

    // Update ACTIVE_INSTALLS
    if let Ok(mut map) = active_installs().lock() {
        if let Some(task) = map.get_mut(task_id) {
            task.step = step.to_string();
            task.percent = percent;
        }
    }
}

fn emit_finished(app: &tauri::AppHandle, task_id: &str, success: bool, message: &str) {
    if success {
        let _ = app.emit(
            "install-finished",
            InstallFinishedPayload {
                task_id: task_id.to_string(),
                success: true,
                message: message.to_string(),
            },
        );
    } else {
        // Emit both install-failed and install-finished for compatibility
        let _ = app.emit(
            "install-failed",
            InstallFinishedPayload {
                task_id: task_id.to_string(),
                success: false,
                message: message.to_string(),
            },
        );
        let _ = app.emit(
            "install-finished",
            InstallFinishedPayload {
                task_id: task_id.to_string(),
                success: false,
                message: message.to_string(),
            },
        );
    }

    // Remove from ACTIVE_INSTALLS
    if let Ok(mut map) = active_installs().lock() {
        map.remove(task_id);
    }
}


// ─── Helper: run pkexec and stream output line by line ───────────────────────

fn run_pkexec_streaming(
    app: &tauri::AppHandle,
    task_id: &str,
    binary_path: &str,
    args: &[&str],
    cancel: &Arc<AtomicBool>,
) -> BackendResult {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;

    let mut cmd = Command::new("pkexec");
    cmd.arg(binary_path);
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    eprintln!(
        "[install_stream] Running: pkexec {} {}",
        binary_path,
        args.join(" ")
    );

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Failed to spawn pkexec: {}", e);
            emit_log(app, task_id, &msg, "system");
            return BackendResult::error(&msg);
        }
    };

    // Stream stdout in a thread
    let stdout_pipe = child.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout_pipe);
    let app_stdout = app.clone();
    let task_id_stdout = task_id.to_string();
    let cancel_stdout = cancel.clone();
    let stdout_buf = Arc::new(Mutex::new(String::new()));
    let stdout_buf_clone = stdout_buf.clone();

    let stdout_thread = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            if cancel_stdout.load(Ordering::Relaxed) {
                break;
            }
            if let Ok(l) = line {
                emit_log(&app_stdout, &task_id_stdout, &l, "stdout");
                let mut buf = stdout_buf_clone.lock().unwrap();
                buf.push_str(&l);
                buf.push('\n');
            }
        }
    });

    // Stream stderr in a thread
    let stderr_pipe = child.stderr.take().unwrap();
    let stderr_reader = BufReader::new(stderr_pipe);
    let app_stderr = app.clone();
    let task_id_stderr = task_id.to_string();
    let cancel_stderr = cancel.clone();
    let stderr_buf = Arc::new(Mutex::new(String::new()));
    let stderr_buf_clone = stderr_buf.clone();

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            if cancel_stderr.load(Ordering::Relaxed) {
                break;
            }
            if let Ok(l) = line {
                emit_log(&app_stderr, &task_id_stderr, &l, "stderr");
                let mut buf = stderr_buf_clone.lock().unwrap();
                buf.push_str(&l);
                buf.push('\n');
            }
        }
    });

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    let status = child.wait();

    let stdout = stdout_buf.lock().unwrap().clone();
    let stderr = stderr_buf.lock().unwrap().clone();

    match status {
        Ok(s) => BackendResult {
            success: s.success(),
            stdout,
            stderr,
            exit_code: s.code().unwrap_or(-1),
        },
        Err(_) => BackendResult {
            success: false,
            stdout,
            stderr: format!("{}\nProcess wait failed", stderr),
            exit_code: -1,
        },
    }
}

/// Install a full profile with realtime streaming via Tauri events.
///
/// Returns immediately with the task_id. Progress, logs, and completion
/// are delivered via events:
///   - `install-log`      → InstallLogPayload
///   - `install-progress` → InstallProgressPayload  
///   - `install-finished` → InstallFinishedPayload
#[tauri::command]
#[specta::specta]
pub async fn install_profile_async(
    app: tauri::AppHandle,
    task_id: String,
    profile_name: String,
    packages: Vec<String>,
    services: Vec<String>,
) -> String {
    // Reset cancel flag at the start of a new task
    cancel_flag().store(false, Ordering::Relaxed);

    let cancel = cancel_flag();
    let returned_task_id = task_id.clone();

    // Register active install in ACTIVE_INSTALLS
    let initial_task = ActiveInstallTask {
        task_id: task_id.clone(),
        profile_name: profile_name.clone(),
        packages: packages.clone(),
        services: services.clone(),
        step: "idle".to_string(),
        percent: 0,
        started_at: now_ms(),
        logs: vec![ActiveInstallLog {
            line: format!("Starting install: {}", profile_name),
            stream: "system".to_string(),
            ts: now_ms(),
        }],
    };
    if let Ok(mut map) = active_installs().lock() {
        map.insert(task_id.clone(), initial_task);
    }

    // Emit install-started
    let _ = app.emit("install-started", InstallStartedPayload { task_id: task_id.clone() });

    // Spawn a detached OS thread — NOT tied to the async command lifetime
    std::thread::spawn(move || {
        install_profile_worker(&app, &task_id, &profile_name, &packages, &services, &cancel);
    });

    returned_task_id
}


fn install_profile_worker(
    app: &tauri::AppHandle,
    task_id: &str,
    profile_name: &str,
    packages: &[String],
    services: &[String],
    cancel: &Arc<AtomicBool>,
) {
    eprintln!(
        "[install_worker] task={} profile={} pkgs={} svcs={}",
        task_id,
        profile_name,
        packages.len(),
        services.len()
    );

    // Step 0: Update pacman database
    if !packages.is_empty() {
        if cancel.load(Ordering::Relaxed) {
            emit_finished(app, task_id, false, "Install cancelled");
            return;
        }
        emit_progress(app, task_id, "db-update", 5);
        emit_log(app, task_id, "Updating package database…", "system");

        let db_result = run_pkexec_streaming(app, task_id, "pacman", &["-Sy"], cancel);
        if cancel.load(Ordering::Relaxed) {
            emit_finished(app, task_id, false, "Install cancelled");
            return;
        }
        if db_result.success {
            emit_log(app, task_id, "✓ Package database updated", "system");
        } else {
            emit_log(
                app,
                task_id,
                &format!("⚠ DB update warning: {}", db_result.stderr.trim()),
                "system",
            );
        }
    }

    // Step 1: Install packages via pacman
    if !packages.is_empty() {
        if cancel.load(Ordering::Relaxed) {
            emit_finished(app, task_id, false, "Install cancelled");
            return;
        }
        emit_progress(app, task_id, "installing", 20);
        emit_log(
            app,
            task_id,
            &format!("Installing {} package(s)…", packages.len()),
            "system",
        );

        let mut args = vec!["-S", "--noconfirm", "--needed"];
        let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&pkg_refs);

        let result = run_pkexec_streaming(app, task_id, "pacman", &args, cancel);

        if cancel.load(Ordering::Relaxed) {
            emit_finished(app, task_id, false, "Install cancelled");
            return;
        }

        if !result.success {
            let msg = format!("Package installation failed: {}", result.stderr.trim());
            emit_log(app, task_id, &msg, "system");
            emit_finished(app, task_id, false, &msg);
            return;
        }
        emit_log(
            app,
            task_id,
            &format!("✓ {} package(s) installed successfully", packages.len()),
            "system",
        );
    }

    // Step 2: Enable services
    let svc_total = services.len();
    for (i, svc) in services.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            emit_finished(app, task_id, false, "Install cancelled");
            return;
        }
        let pct = 80u8 + ((i + 1) * 15 / svc_total.max(1)) as u8;
        emit_progress(app, task_id, "services", pct.min(95));
        emit_log(app, task_id, &format!("Enabling service '{}'…", svc), "system");

        let result = run_pkexec_streaming(app, task_id, "systemctl", &["enable", "--now", svc], cancel);
        if result.success {
            emit_log(app, task_id, &format!("✓ Service '{}' enabled and started", svc), "system");
        } else {
            emit_log(
                app,
                task_id,
                &format!("⚠ Service '{}' warning: {}", svc, result.stderr.trim()),
                "system",
            );
        }
    }

    emit_progress(app, task_id, "done", 100);
    let success_msg = format!("✓ Profile '{}' installed successfully!", profile_name);
    emit_log(app, task_id, &success_msg, "system");
    emit_finished(app, task_id, true, &success_msg);
}

/// Cancel the currently running async install.
#[tauri::command]
#[specta::specta]
pub fn cancel_install() {
    eprintln!("[install] Cancel requested by frontend");
    cancel_flag().store(true, Ordering::Relaxed);
}

/// Query currently active background installations.
#[tauri::command]
#[specta::specta]
pub async fn get_active_installs() -> Vec<ActiveInstallTask> {
    if let Ok(map) = active_installs().lock() {
        map.values().cloned().collect()
    } else {
        Vec::new()
    }
}

