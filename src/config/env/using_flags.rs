//! Using/namespace-related environment flags
//!
//! This module groups all `using` system and namespace flags.
//! Use this for IDE autocomplete to discover using/namespace flags easily.

use super::env_bool;
use super::warn_alias_once;

pub fn enable_using() -> bool {
    // Phase 15: デフォルトON（using systemはメイン機能）
    // NYASH_ENABLE_USING=0 で明示的に無効化可能。HAKO_ENABLE_USING は互換のため受理（警告）。
    match std::env::var("NYASH_ENABLE_USING").ok().as_deref() {
        Some("0") | Some("false") | Some("off") => return false,
        Some(_) => return true,
        None => {}
    }
    // Fallback to alias
    if let Some(v) = std::env::var("HAKO_ENABLE_USING").ok() {
        warn_alias_once("HAKO_ENABLE_USING", "NYASH_ENABLE_USING");
        let lv = v.to_ascii_lowercase();
        return !(lv == "0" || lv == "false" || lv == "off");
    }
    true // default ON
}

// ---- Using profiles (dev|ci|prod) ----
/// Return using profile string; default is "dev".
pub fn using_profile() -> String {
    std::env::var("NYASH_USING_PROFILE").unwrap_or_else(|_| "dev".to_string())
}

/// True when using profile is prod (disables some dev-only behaviors).
pub fn using_is_prod() -> bool {
    using_profile().eq_ignore_ascii_case("prod")
}

/// True when using profile is ci.
pub fn using_is_ci() -> bool {
    using_profile().eq_ignore_ascii_case("ci")
}

/// True when using profile is dev (default).
pub fn using_is_dev() -> bool {
    using_profile().eq_ignore_ascii_case("dev")
}

/// Allow `using "path"` statements in source (dev-only by default).
pub fn allow_using_file() -> bool {
    // SSOT 徹底: 全プロファイルで既定禁止（nyash.toml を唯一の真実に）
    // 明示オーバーライドでのみ許可（開発用緊急時）
    match std::env::var("NYASH_ALLOW_USING_FILE").ok().as_deref() {
        Some("1") | Some("true") | Some("on") => true,
        _ => false,
    }
}

/// Determine whether AST prelude merge for `using` is enabled.
/// Precedence:
/// 1) Explicit env `NYASH_USING_AST` = 1/true/on → enabled, = 0/false/off → disabled
/// 2) Default by profile: dev/ci → ON, prod → OFF
pub fn using_ast_enabled() -> bool {
    match std::env::var("NYASH_USING_AST")
        .ok()
        .as_deref()
        .map(|v| v.to_ascii_lowercase())
    {
        Some(ref s) if s == "1" || s == "true" || s == "on" => true,
        Some(ref s) if s == "0" || s == "false" || s == "off" => false,
        _ => !using_is_prod(), // dev/ci → true, prod → false
    }
}

// ---- Using/resolve diagnostics ----
pub fn resolve_trace() -> bool {
    env_bool("NYASH_RESOLVE_TRACE")
}

pub fn resolve_seam_debug() -> bool {
    env_bool("NYASH_RESOLVE_SEAM_DEBUG")
}

pub fn resolve_dump_merged_path() -> Option<String> {
    std::env::var("NYASH_RESOLVE_DUMP_MERGED")
        .ok()
        .filter(|s| !s.is_empty())
}

/// Auto-load [using.*] dylib packages (NYASH_USING_DYLIB_AUTOLOAD=1).
pub fn using_dylib_autoload() -> bool {
    env_bool("NYASH_USING_DYLIB_AUTOLOAD")
}
