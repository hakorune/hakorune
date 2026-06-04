use std::sync::OnceLock;

use super::contract;

static PERF_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();

pub(crate) fn enabled() -> bool {
    crate::env_flags::flag_parsed_cached(
        &PERF_TRACE_ENABLED,
        "NYASH_PERF_TRACE",
        crate::env_flags::parse_trueish,
    )
}

pub(crate) fn flush() {
    if enabled() {
        eprintln!(
            "[perf/trace] enabled routes={},{} sink=stderr mode=placeholder",
            contract::STORE_ARRAY_STR,
            contract::CONST_SUFFIX
        );
    }
}
