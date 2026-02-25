//! Selfhost compiler-related environment flags
//!
//! This module groups all selfhost NY compiler flags.
//! Use this for IDE autocomplete to discover selfhost compiler flags easily.

use super::warn_alias_once;

// Self-host compiler knobs
pub fn ny_compiler_timeout_ms() -> u64 {
    std::env::var("NYASH_NY_COMPILER_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000)
}

/// Optional timeout override for Stage-1 emit routes (`--hako-emit-mir-json` / emit-program).
///
/// Falls back to `NYASH_NY_COMPILER_TIMEOUT_MS` when unset.
pub fn ny_compiler_emit_timeout_ms() -> u64 {
    std::env::var("NYASH_STAGE1_EMIT_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(ny_compiler_timeout_ms)
}

/// Emit-only flag for selfhost compiler (default ON to avoid execution).
pub fn ny_compiler_emit_only() -> bool {
    std::env::var("NYASH_NY_COMPILER_EMIT_ONLY").unwrap_or_else(|_| "1".to_string()) == "1"
}

/// Path to external selfhost compiler executable (when enabled).
pub fn use_ny_compiler_exe() -> bool {
    std::env::var("NYASH_USE_NY_COMPILER_EXE").ok().as_deref() == Some("1")
}

/// Path to external selfhost compiler executable (when enabled).
pub fn ny_compiler_exe_path() -> Option<String> {
    std::env::var("NYASH_NY_COMPILER_EXE_PATH").ok()
}

/// Pass `-- --min-json` to child selfhost compiler (minimal JSON output).
pub fn ny_compiler_min_json() -> bool {
    std::env::var("NYASH_NY_COMPILER_MIN_JSON").ok().as_deref() == Some("1")
}

/// When true, child reads tmp/ny_parser_input.ny instead of stdin/source text.
pub fn selfhost_read_tmp() -> bool {
    std::env::var("NYASH_SELFHOST_READ_TMP").ok().as_deref() == Some("1")
}

/// Pass `-- --stage3` to child selfhost compiler to allow Stage-3 surface.
pub fn ny_compiler_stage3() -> bool {
    std::env::var("NYASH_NY_COMPILER_STAGE3").ok().as_deref() == Some("1")
}

pub fn ny_compiler_child_args() -> Option<String> {
    // Pass-through args to selfhost child (space-separated).
    std::env::var("NYASH_NY_COMPILER_CHILD_ARGS").ok()
}

pub fn ny_compiler_use_tmp_only() -> bool {
    std::env::var("NYASH_NY_COMPILER_USE_TMP_ONLY")
        .ok()
        .as_deref()
        == Some("1")
}

/// Use Python MVP harness for Ny compiler (NYASH_NY_COMPILER_USE_PY=1).
pub fn ny_compiler_use_py() -> bool {
    std::env::var("NYASH_NY_COMPILER_USE_PY").ok().as_deref() == Some("1")
}

/// Dev-only escape hatch: force inline selfhost path (NYASH_SELFHOST_INLINE_FORCE=1).
pub fn selfhost_inline_force() -> bool {
    std::env::var("NYASH_SELFHOST_INLINE_FORCE").ok().as_deref() == Some("1")
}

/// Consolidated toggle for selfhost NY compiler pipeline.
/// Primary: NYASH_USE_NY_COMPILER=0|1（明示指定のみ有効）。Legacy disables accepted (with warning):
/// NYASH_DISABLE_NY_COMPILER/HAKO_DISABLE_NY_COMPILER (any true value disables).
pub fn use_ny_compiler() -> bool {
    fn env_bool(key: &str) -> bool {
        match std::env::var(key).ok() {
            Some(v) => {
                let lv = v.to_ascii_lowercase();
                lv == "1" || lv == "true" || lv == "on"
            }
            None => false,
        }
    }

    // Primary knob takes precedence when explicitly set
    if let Some(v) = std::env::var("NYASH_USE_NY_COMPILER").ok() {
        let lv = v.trim().to_ascii_lowercase();
        return lv == "1" || lv == "true" || lv == "on";
    }
    // Legacy disable aliases — if any is true, treat as disabled and warn
    if env_bool("NYASH_DISABLE_NY_COMPILER") {
        warn_alias_once("NYASH_DISABLE_NY_COMPILER", "NYASH_USE_NY_COMPILER=0");
        return false;
    }
    if env_bool("HAKO_DISABLE_NY_COMPILER") {
        warn_alias_once("HAKO_DISABLE_NY_COMPILER", "NYASH_USE_NY_COMPILER=0");
        return false;
    }
    // Phase 25.1b: Default OFF（selfhost NY compiler は明示 opt-in のみ）
    false
}

/// Mainline guard for MirBuilder delegate route (`env.mirbuilder.emit`).
///
/// When true, delegate route must fail-fast instead of silently converting
/// Program(JSON) via Rust provider.
pub fn mirbuilder_delegate_forbidden() -> bool {
    std::env::var("HAKO_SELFHOST_NO_DELEGATE")
        .ok()
        .as_deref()
        == Some("1")
}

/// Stable freeze tag for MirBuilder delegate forbidden contract.
pub const MIRBUILDER_DELEGATE_FORBIDDEN_TAG: &str =
    "[freeze:contract][mirbuilder/delegate-forbidden]";

/// Build a stable fail-fast message for blocked `env.mirbuilder.emit`.
pub fn mirbuilder_delegate_forbidden_message(label: &str) -> String {
    format!(
        "{} {} blocked (HAKO_SELFHOST_NO_DELEGATE=1)",
        MIRBUILDER_DELEGATE_FORBIDDEN_TAG,
        label
    )
}
