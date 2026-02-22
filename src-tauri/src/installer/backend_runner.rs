use modularitea_libs::infrastructure::Pacman;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::process::Command;

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
