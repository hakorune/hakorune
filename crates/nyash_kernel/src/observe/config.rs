use std::sync::OnceLock;

static PERF_OBSERVE_ENABLED: OnceLock<bool> = OnceLock::new();
static PERF_BYPASS_GC_ALLOC_ENABLED: OnceLock<bool> = OnceLock::new();

pub(crate) fn enabled() -> bool {
    crate::env_flags::flag_parsed_cached(
        &PERF_OBSERVE_ENABLED,
        "NYASH_PERF_COUNTERS",
        crate::env_flags::parse_trueish,
    )
}

pub(crate) fn bypass_gc_alloc_enabled() -> bool {
    crate::env_flags::flag_parsed_cached(
        &PERF_BYPASS_GC_ALLOC_ENABLED,
        "NYASH_PERF_BYPASS_GC_ALLOC",
        crate::env_flags::parse_trueish,
    )
}
