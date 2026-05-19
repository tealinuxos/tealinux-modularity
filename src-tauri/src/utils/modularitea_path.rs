#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Resolve helper binary (`modularitea-grub`, `pkexec`, …) via `PATH`, then `/usr/bin`, `/usr/local/bin`.
pub fn resolve_on_path<O: AsRef<std::ffi::OsStr>>(name: O) -> Option<PathBuf> {
    let name = name.as_ref();
    #[cfg(unix)]
    if name.as_bytes().contains(&b'/') {
        let p = PathBuf::from(name);
        return p.exists().then_some(p);
    }

    if let Some(paths) = std::env::var_os("PATH") {
        let hit =
            std::env::split_paths(&paths).map(|dir| dir.join(name)).find(|p| p.is_file());
        if hit.is_some() {
            return hit;
        }
    }

    for root in ["/usr/bin", "/usr/local/bin"] {
        let p = Path::new(root).join(name);
        if p.is_file() {
            return Some(p);
        }
    }

    None
}
