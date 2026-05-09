//! Elevated subprocess helpers: `pkexec` with explicit argv only (no shell).

use crate::utils::error_libs::{LocalCommandOutput, LocalModulariteaError};
use crate::utils::modularitea_path::resolve_on_path;
use std::path::Path;
use std::process::{Command, Output};

pub fn run_pkexec_program(program: &Path, args: &[String]) -> Result<Output, LocalModulariteaError> {
    let pkexec = resolve_on_path("pkexec").ok_or(LocalModulariteaError::PkexecNotFound)?;
    let mut cmd = Command::new(pkexec);
    cmd.arg(program);
    for a in args {
        cmd.arg(a);
    }
    cmd.output()
        .map_err(|e| LocalModulariteaError::IoError(e.to_string()))
}

pub fn pkexec_stderr_local_result(name: &str, out: Output) -> Result<LocalCommandOutput, LocalModulariteaError> {
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let exit_code = out.status.code().unwrap_or(-1);
    let success = out.status.success();

    let combined = format!("{}{}", stdout, stderr).to_lowercase();
    if combined.contains("not authorized")
        || combined.contains("polkit authentication agent")
        || (combined.contains("polkit") && combined.contains("cancel"))
    {
        return Err(LocalModulariteaError::PolkitCancelled);
    }

    if !success {
        let err_txt = if stderr.is_empty() { stdout.clone() } else { stderr };
        return Err(LocalModulariteaError::CommandError {
            command: name.to_string(),
            exit_code: Some(exit_code),
            stderr: err_txt,
        });
    }

    Ok(LocalCommandOutput {
        exit_code,
        stdout,
        stderr,
        success: true,
    })
}

pub fn merged_output_text(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
    .trim()
    .to_string()
}
