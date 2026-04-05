use std::sync::OnceLock;

use super::contract;

#[inline(always)]
fn parse_enabled(value: &str) -> bool {
    matches!(value, "1" | "on" | "true" | "yes")
}

static PERF_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();

pub(crate) fn enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("NYASH_PERF_TRACE")
            .ok()
            .as_deref()
            .is_some_and(parse_enabled)
    }
    #[cfg(not(test))]
    {
        *PERF_TRACE_ENABLED.get_or_init(|| {
            std::env::var("NYASH_PERF_TRACE")
                .ok()
                .as_deref()
                .is_some_and(parse_enabled)
        })
    }
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
