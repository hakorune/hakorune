//! Macro system environment variable configuration
//!
//! Phase 286A: Consolidates NYASH_MACRO_* and NYASH_TEST_* flags
//! Prevents direct std::env::var access throughout the codebase (AGENTS.md 5.3)

/// NYASH Macro paths (comma-separated)
pub fn macro_paths() -> Option<String> {
    std::env::var("NYASH_MACRO_PATHS").ok()
}

/// Parse comma-separated macro paths into Vec
pub fn macro_paths_vec() -> Option<Vec<String>> {
    macro_paths().map(|s| {
        s.split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    })
}

/// NYASH_MACRO_BOX_NY mode enable
pub fn macro_box_ny() -> bool {
    std::env::var("NYASH_MACRO_BOX_NY")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_BOX_NY_PATHS (legacy alias)
pub fn macro_box_ny_paths() -> Option<String> {
    std::env::var("NYASH_MACRO_BOX_NY_PATHS").ok()
}

/// NYASH_MACRO_TOPLEVEL_ALLOW (deprecated)
pub fn macro_toplevel_allow() -> Option<bool> {
    std::env::var("NYASH_MACRO_TOPLEVEL_ALLOW")
        .ok()
        .map(|v| {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        })
}

/// NYASH_MACRO_BOX_CHILD_RUNNER (deprecated)
pub fn macro_box_child_runner() -> Option<bool> {
    std::env::var("NYASH_MACRO_BOX_CHILD_RUNNER")
        .ok()
        .map(|v| {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        })
}

/// NYASH_MACRO_TRACE enable
pub fn macro_trace() -> bool {
    std::env::var("NYASH_MACRO_TRACE")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_CLI_VERBOSE enable
pub fn macro_cli_verbose() -> bool {
    std::env::var("NYASH_CLI_VERBOSE")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_BOX_CHILD mode
pub fn macro_box_child() -> bool {
    std::env::var("NYASH_MACRO_BOX_CHILD")
        .ok()
        .map(|v| v != "0" && v != "false" && v != "off")
        .unwrap_or(true)
}

/// NYASH_MACRO_BOX_NY_IDENTITY_ROUNDTRIP enable
pub fn macro_box_ny_identity_roundtrip() -> bool {
    std::env::var("NYASH_MACRO_BOX_NY_IDENTITY_ROUNDTRIP")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_STRICT mode
pub fn macro_strict() -> Option<bool> {
    std::env::var("NYASH_MACRO_STRICT")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            !(v == "0" || v == "false" || v == "off")
        })
}

/// NYASH_MACRO_MAX_PASSES
pub fn macro_max_passes() -> Option<u32> {
    std::env::var("NYASH_MACRO_MAX_PASSES")
        .ok()
        .and_then(|s| s.parse().ok())
}

/// NYASH_MACRO_CYCLE_WINDOW
pub fn macro_cycle_window() -> Option<u32> {
    std::env::var("NYASH_MACRO_CYCLE_WINDOW")
        .ok()
        .and_then(|s| s.parse().ok())
}

/// NYASH_MACRO_DERIVE_ALL enable
pub fn macro_derive_all() -> bool {
    std::env::var("NYASH_MACRO_DERIVE_ALL")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_DERIVE target
pub fn macro_derive() -> Option<String> {
    std::env::var("NYASH_MACRO_DERIVE").ok()
}

/// NYASH_MACRO_TRACE_JSONL path
pub fn macro_trace_jsonl() -> Option<String> {
    std::env::var("NYASH_MACRO_TRACE_JSONL").ok()
}

/// NYASH_MACRO_DISABLE
pub fn macro_disable() -> Option<bool> {
    std::env::var("NYASH_MACRO_DISABLE")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
}

/// NYASH_MACRO_ENABLE
pub fn macro_enable() -> Option<bool> {
    std::env::var("NYASH_MACRO_ENABLE")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
}

/// NYASH_MACRO_BOX enable
pub fn macro_box() -> bool {
    std::env::var("NYASH_MACRO_BOX")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_BOX_EXAMPLE enable
pub fn macro_box_example() -> bool {
    std::env::var("NYASH_MACRO_BOX_EXAMPLE")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_MACRO_BOX_ENABLE list
pub fn macro_box_enable() -> Option<String> {
    std::env::var("NYASH_MACRO_BOX_ENABLE").ok()
}

// NYASH Test flags

/// NYASH_TEST_RUN enable
pub fn test_run() -> bool {
    std::env::var("NYASH_TEST_RUN")
        .ok()
        .as_deref()
        != Some("1")
}

/// NYASH_TEST_ARGS_JSON
pub fn test_args_json() -> Option<String> {
    std::env::var("NYASH_TEST_ARGS_JSON").ok()
}

/// NYASH_TEST_ARGS_DEFAULTS enable
pub fn test_args_defaults() -> bool {
    std::env::var("NYASH_TEST_ARGS_DEFAULTS")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_TEST_FILTER pattern
pub fn test_filter() -> Option<String> {
    std::env::var("NYASH_TEST_FILTER").ok()
}

/// NYASH_TEST_FORCE enable
pub fn test_force() -> bool {
    std::env::var("NYASH_TEST_FORCE")
        .ok()
        .as_deref()
        == Some("1")
}

/// NYASH_TEST_ENTRY mode (wrap/override)
pub fn test_entry() -> Option<String> {
    std::env::var("NYASH_TEST_ENTRY").ok()
}

/// NYASH_TEST_RETURN policy (tests/original)
pub fn test_return() -> Option<String> {
    std::env::var("NYASH_TEST_RETURN").ok()
}

/// NYASH_SYNTAX_SUGAR_LEVEL
pub fn macro_syntax_sugar_level() -> Option<String> {
    std::env::var("NYASH_SYNTAX_SUGAR_LEVEL").ok()
}

/// NYASH_MACRO_CAP_IO (capability: IO allowed)
pub fn macro_cap_io() -> Option<bool> {
    std::env::var("NYASH_MACRO_CAP_IO")
        .ok()
        .map(|v| {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        })
}

/// NYASH_MACRO_CAP_NET (capability: NET allowed)
pub fn macro_cap_net() -> Option<bool> {
    std::env::var("NYASH_MACRO_CAP_NET")
        .ok()
        .map(|v| {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        })
}
