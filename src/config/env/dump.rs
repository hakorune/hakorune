//! Dump / diagnostics env helpers.

use std::sync::atomic::{AtomicU8, Ordering};

const CLI_VERBOSE_LEVEL_UNSET: u8 = u8::MAX;
static CLI_VERBOSE_LEVEL_CACHE: AtomicU8 = AtomicU8::new(CLI_VERBOSE_LEVEL_UNSET);

/// Optional dump path for MIR printer output (JSON v0 route only).
pub fn rust_mir_dump_path() -> Option<String> {
    std::env::var("RUST_MIR_DUMP_PATH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// CLI verbose level (0=quiet, 1=verbose, 2=trace).
pub fn cli_verbose_level() -> u8 {
    let cached = CLI_VERBOSE_LEVEL_CACHE.load(Ordering::Relaxed);
    if cached != CLI_VERBOSE_LEVEL_UNSET {
        return cached;
    }

    let parsed = match std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() {
        Some("2") => 2,
        Some("1") => 1,
        _ => 0,
    };
    let _ = CLI_VERBOSE_LEVEL_CACHE.compare_exchange(
        CLI_VERBOSE_LEVEL_UNSET,
        parsed,
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
    CLI_VERBOSE_LEVEL_CACHE.load(Ordering::Relaxed)
}

/// True when CLI verbose level >= 1.
pub fn cli_verbose_enabled() -> bool {
    cli_verbose_level() > 0
}

/// Update the cached CLI verbose level alongside the process env.
pub fn set_cli_verbose_level(level: u8) {
    let normalized = match level {
        0 => 0,
        1 => 1,
        _ => 2,
    };
    match normalized {
        0 => {
            std::env::remove_var("NYASH_CLI_VERBOSE");
        }
        1 => {
            std::env::set_var("NYASH_CLI_VERBOSE", "1");
        }
        _ => {
            std::env::set_var("NYASH_CLI_VERBOSE", "2");
        }
    }
    CLI_VERBOSE_LEVEL_CACHE.store(normalized, Ordering::Relaxed);
}

#[cfg(test)]
pub fn reset_cli_verbose_cache() {
    CLI_VERBOSE_LEVEL_CACHE.store(CLI_VERBOSE_LEVEL_UNSET, Ordering::Relaxed);
}

/// Leak report level (0=off, 1=summary, 2=verbose).
pub fn leak_log_level() -> u8 {
    match std::env::var("NYASH_LEAK_LOG").ok().as_deref() {
        Some("2") => 2,
        Some("1") => 1,
        _ => 0,
    }
}
