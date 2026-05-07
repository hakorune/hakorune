//! Path resolution helpers (SSOT for env-derived paths).

use super::warn_alias_once;

fn env_string_trimmed(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Repo root hint.
///
/// `HAKO_ROOT` is the preferred spelling. `NYASH_ROOT` remains a compatibility
/// alias while callers migrate.
pub fn hako_root() -> Option<String> {
    if let Some(root) = env_string_trimmed("HAKO_ROOT") {
        return Some(root);
    }
    if let Some(root) = env_string_trimmed("NYASH_ROOT") {
        warn_alias_once("NYASH_ROOT", "HAKO_ROOT");
        return Some(root);
    }
    None
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
    if let Some(bin) = env_string_trimmed("HAKO_BIN") {
        return Some(bin);
    }
    if let Some(bin) = env_string_trimmed("NYASH_BIN") {
        warn_alias_once("NYASH_BIN", "HAKO_BIN");
        return Some(bin);
    }
    None
}

/// Compatibility wrapper for existing callers.
pub fn nyash_bin() -> Option<String> {
    hako_bin()
}
