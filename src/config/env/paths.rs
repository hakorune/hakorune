//! Path resolution helpers (SSOT for env-derived paths)

/// Repo root hint (NYASH_ROOT).
pub fn nyash_root() -> Option<String> {
    std::env::var("NYASH_ROOT")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
