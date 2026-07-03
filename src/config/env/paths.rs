//! Path resolution helpers (SSOT for env-derived paths).

use super::{env_string_trimmed_with_alias, warn_alias_once};
use std::path::{Path, PathBuf};

fn env_string_trimmed(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Best-effort directory of the current executable.
pub fn nyrt_entry_exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

/// Display-form current working directory for NyRT startup diagnostics.
pub fn nyrt_entry_current_dir_display() -> Option<String> {
    std::env::current_dir()
        .ok()
        .map(|p| p.display().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "windows")]
fn append_unique_path_segment(path_value: &mut String, segment: &Path) {
    let segment_s = segment.display().to_string();
    if !path_value
        .split(';')
        .any(|existing| existing.eq_ignore_ascii_case(&segment_s))
    {
        if !path_value.is_empty() {
            path_value.push(';');
        }
        path_value.push_str(&segment_s);
    }
}

/// Apply Windows-specific NyRT entry path shaping for DLL / plugin discovery.
#[cfg(target_os = "windows")]
pub fn nyrt_entry_apply_windows_path_shaping(exe_dir: &Path) {
    let mut path_val = std::env::var("PATH").unwrap_or_default();
    append_unique_path_segment(&mut path_val, exe_dir);
    let plug = exe_dir.join("plugins");
    if plug.is_dir() {
        append_unique_path_segment(&mut path_val, &plug);
    }
    std::env::set_var("PATH", &path_val);

    match std::env::var("PYTHONHOME") {
        Ok(v) => {
            let pb = PathBuf::from(&v);
            if pb.is_relative() {
                let abs = exe_dir.join(pb);
                std::env::set_var("PYTHONHOME", abs.display().to_string());
            }
        }
        Err(_) => {
            let cand = exe_dir.join("python");
            if cand.is_dir() {
                std::env::set_var("PYTHONHOME", cand.display().to_string());
            }
        }
    }
}

/// No-op on non-Windows targets.
#[cfg(not(target_os = "windows"))]
pub fn nyrt_entry_apply_windows_path_shaping(_exe_dir: &Path) {}

/// Repo root hint.
///
/// `HAKO_ROOT` is the preferred spelling. `NYASH_ROOT` remains a compatibility
/// alias while callers migrate.
pub fn hako_root() -> Option<String> {
    env_string_trimmed_with_alias("HAKO_ROOT", "NYASH_ROOT")
}

/// Compatibility wrapper for existing callers.
pub fn nyash_root() -> Option<String> {
    hako_root()
}

/// Ensure both preferred and compatibility root variables are visible to child
/// tooling. Used by dev-mode CLI setup after the user explicitly opted in.
pub fn ensure_root_aliases_from_cwd() {
    let hako_root = env_string_trimmed("HAKO_ROOT");
    let nyash_root = env_string_trimmed("NYASH_ROOT");
    match (hako_root, nyash_root) {
        (None, None) => {
            if let Ok(cwd) = std::env::current_dir() {
                let root = cwd.display().to_string();
                std::env::set_var("HAKO_ROOT", &root);
                std::env::set_var("NYASH_ROOT", root);
            }
        }
        (None, Some(root)) => {
            warn_alias_once("NYASH_ROOT", "HAKO_ROOT");
            std::env::set_var("HAKO_ROOT", root);
        }
        (Some(root), None) => {
            std::env::set_var("NYASH_ROOT", root);
        }
        (Some(_), Some(_)) => {}
    }
}

/// Hakorune executable path hint.
///
/// `HAKO_BIN` is the preferred spelling. `NYASH_BIN` remains a compatibility
/// alias while callers migrate.
pub fn hako_bin() -> Option<String> {
    env_string_trimmed_with_alias("HAKO_BIN", "NYASH_BIN")
}

/// Compatibility wrapper for existing callers.
pub fn nyash_bin() -> Option<String> {
    hako_bin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_path_helpers_have_current_process_context() {
        assert!(nyrt_entry_exe_dir().is_some());
        assert!(nyrt_entry_current_dir_display().is_some());
    }

    #[test]
    fn hako_root_prefers_primary_and_trims_value() {
        crate::test_support::with_env_vars(
            &[
                ("HAKO_ROOT", Some("  /tmp/hakorune-root  ")),
                ("NYASH_ROOT", Some("/tmp/legacy-root")),
            ],
            || {
                assert_eq!(hako_root(), Some("/tmp/hakorune-root".to_string()));
                assert_eq!(nyash_root(), Some("/tmp/hakorune-root".to_string()));
            },
        );
    }

    #[test]
    fn hako_root_uses_legacy_alias_when_primary_empty() {
        crate::test_support::with_env_vars(
            &[
                ("HAKO_ROOT", Some("   ")),
                ("NYASH_ROOT", Some(" /tmp/legacy-root ")),
            ],
            || {
                assert_eq!(hako_root(), Some("/tmp/legacy-root".to_string()));
            },
        );
    }

    #[test]
    fn hako_bin_prefers_primary_and_keeps_legacy_wrapper() {
        crate::test_support::with_env_vars(
            &[
                ("HAKO_BIN", Some(" target/release/hakorune ")),
                ("NYASH_BIN", Some("target/release/nyash")),
            ],
            || {
                assert_eq!(hako_bin(), Some("target/release/hakorune".to_string()));
                assert_eq!(nyash_bin(), Some("target/release/hakorune".to_string()));
            },
        );
    }
}
