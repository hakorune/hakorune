use std::sync::OnceLock;

#[inline(always)]
fn parse_enabled(value: &str) -> bool {
    matches!(value, "1" | "on" | "true" | "yes")
}

static PERF_OBSERVE_ENABLED: OnceLock<bool> = OnceLock::new();

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
