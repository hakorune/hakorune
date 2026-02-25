//! Dump / diagnostics env helpers.

/// Optional dump path for MIR printer output (JSON v0 route only).
pub fn rust_mir_dump_path() -> Option<String> {
    std::env::var("RUST_MIR_DUMP_PATH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// CLI verbose level (0=quiet, 1=verbose, 2=trace).
pub fn cli_verbose_level() -> u8 {
    match std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() {
        Some("2") => 2,
        Some("1") => 1,
        _ => 0,
    }
}

/// True when CLI verbose level >= 1.
pub fn cli_verbose_enabled() -> bool {
    cli_verbose_level() > 0
}

/// Leak report level (0=off, 1=summary, 2=verbose).
pub fn leak_log_level() -> u8 {
    match std::env::var("NYASH_LEAK_LOG").ok().as_deref() {
        Some("2") => 2,
        Some("1") => 1,
        _ => 0,
    }
}
