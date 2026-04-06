use std::sync::OnceLock;

#[inline(always)]
fn parse_enabled(value: &str) -> bool {
    matches!(value, "1" | "on" | "true" | "yes")
}

static PERF_OBSERVE_ENABLED: OnceLock<bool> = OnceLock::new();
static PERF_BYPASS_GC_ALLOC_ENABLED: OnceLock<bool> = OnceLock::new();

pub(crate) fn enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("NYASH_PERF_COUNTERS")
            .ok()
            .as_deref()
            .is_some_and(parse_enabled)
    }
    #[cfg(not(test))]
    {
        *PERF_OBSERVE_ENABLED.get_or_init(|| {
            std::env::var("NYASH_PERF_COUNTERS")
                .ok()
                .as_deref()
                .is_some_and(parse_enabled)
        })
    }
}

pub(crate) fn bypass_gc_alloc_enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("NYASH_PERF_BYPASS_GC_ALLOC")
            .ok()
            .as_deref()
            .is_some_and(parse_enabled)
    }
    #[cfg(not(test))]
    {
        *PERF_BYPASS_GC_ALLOC_ENABLED.get_or_init(|| {
            std::env::var("NYASH_PERF_BYPASS_GC_ALLOC")
                .ok()
                .as_deref()
                .is_some_and(parse_enabled)
        })
    }
}
