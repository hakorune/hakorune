//! Stage-1 / selfhost CLI environment helpers (SSOT).

use crate::config::env::env_bool;

/// Primary toggle: enable Stage-1 stub routing.
pub fn enabled() -> bool {
    env_bool("NYASH_USE_STAGE1_CLI")
        || env_bool("HAKO_STAGE1_ENABLE")
        || env_bool("HAKO_EMIT_PROGRAM_JSON")
        || env_bool("HAKO_EMIT_MIR_JSON")
}

/// Recursion guard when Stage-1 stub calls back into the runner.
pub fn child_invocation() -> bool {
    env_bool("NYASH_STAGE1_CLI_CHILD")
}

/// Stage-1 mode hint (emit-program / emit-mir / run).
pub fn mode() -> Option<String> {
    if let Some(m) = std::env::var("HAKO_STAGE1_MODE")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_MODE").ok())
    {
        let trimmed = m.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_ascii_lowercase().replace('_', "-"));
        }
    }
    if std::env::var("HAKO_EMIT_PROGRAM_JSON").ok().as_deref() == Some("1") {
        return Some("emit-program".into());
    }
    if std::env::var("HAKO_EMIT_MIR_JSON").ok().as_deref() == Some("1") {
        return Some("emit-mir".into());
    }
    if std::env::var("STAGE1_EMIT_PROGRAM_JSON").ok().as_deref() == Some("1") {
        return Some("emit-program".into());
    }
    if std::env::var("STAGE1_EMIT_MIR_JSON").ok().as_deref() == Some("1") {
        return Some("emit-mir".into());
    }
    if enabled() {
        return Some("run".into());
    }
    None
}

/// True when Stage-1 should emit Program(JSON v0).
pub fn emit_program_json() -> bool {
    matches!(
        mode().as_deref(),
        Some("emit-program" | "emit-program-json")
    )
}

/// True when Stage-1 should emit MIR(JSON).
pub fn emit_mir_json() -> bool {
    matches!(mode().as_deref(), Some("emit-mir" | "emit-mir-json"))
}

/// Input source path passed to Stage-1 stub (aliases included).
pub fn input_path() -> Option<String> {
    std::env::var("HAKO_STAGE1_INPUT")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_INPUT").ok())
        .or_else(|| std::env::var("STAGE1_SOURCE").ok())
        .or_else(|| std::env::var("STAGE1_INPUT").ok())
}

/// Program(JSON v0) path for Stage-1 emit-mir mode (aliases included).
pub fn program_json_path() -> Option<String> {
    std::env::var("HAKO_STAGE1_PROGRAM_JSON")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_PROGRAM_JSON").ok())
        .or_else(|| std::env::var("STAGE1_PROGRAM_JSON").ok())
}

/// Backend hint for Stage-1 run mode (aliases included).
pub fn backend_hint() -> Option<String> {
    std::env::var("HAKO_STAGE1_BACKEND")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_BACKEND").ok())
        .or_else(|| std::env::var("STAGE1_BACKEND").ok())
}

/// Optional override for Stage-1 CLI entry path.
pub fn entry_override() -> Option<String> {
    std::env::var("STAGE1_CLI_ENTRY")
        .ok()
        .or_else(|| std::env::var("HAKORUNE_STAGE1_ENTRY").ok())
}

/// Optional Stage-1 child args (passed through to stub).
pub fn child_args_env() -> Option<String> {
    std::env::var("NYASH_SCRIPT_ARGS_JSON").ok()
}

/// Stage-1 debug flag (verbose child stderr).
pub fn debug() -> bool {
    std::env::var("STAGE1_CLI_DEBUG").ok().as_deref() == Some("1")
}

fn parse_bool_override(raw: String) -> Option<bool> {
    let trimmed = raw.trim().to_ascii_lowercase();
    match trimmed.as_str() {
        "1" | "true" | "on" => Some(true),
        "0" | "false" | "off" => Some(false),
        _ => None,
    }
}

/// Shared binary-only direct route override.
///
/// - `Some(true)`: force binary-only direct route.
/// - `Some(false)`: disable binary-only direct route.
/// - `None`: no override (mainline default keeps direct route OFF).
pub fn binary_only_direct_override() -> Option<bool> {
    std::env::var("NYASH_STAGE1_BINARY_ONLY_DIRECT")
        .ok()
        .and_then(parse_bool_override)
}

/// Run-specific binary-only direct route override.
///
/// Precedence:
/// 1) `NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT`
/// 2) `NYASH_STAGE1_BINARY_ONLY_DIRECT`
/// 3) no override (`None`, mainline default keeps direct route OFF)
pub fn binary_only_run_direct_override() -> Option<bool> {
    std::env::var("NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT")
        .ok()
        .and_then(parse_bool_override)
        .or_else(binary_only_direct_override)
}

/// Effective toggle for emit-mir binary-only direct route.
/// Mainline default is OFF unless explicit override is set to true.
pub fn binary_only_emit_direct_enabled() -> bool {
    binary_only_direct_override().unwrap_or(false)
}

/// Effective toggle for run-mode binary-only direct route.
/// Mainline default is OFF unless explicit override is set to true.
pub fn binary_only_run_direct_enabled() -> bool {
    binary_only_run_direct_override().unwrap_or(false)
}
