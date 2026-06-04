use std::sync::OnceLock;

#[inline(always)]
pub(crate) fn cli_verbose_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    flag_on_cached(&ENABLED, "NYASH_CLI_VERBOSE")
}

#[cfg(feature = "perf-observe")]
#[inline(always)]
pub(crate) fn parse_trueish(value: &str) -> bool {
    matches!(value, "1" | "on" | "true" | "yes")
}

#[inline(always)]
pub(crate) fn jit_trace_len_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    flag_on_cached(&ENABLED, "NYASH_JIT_TRACE_LEN")
}

#[inline(always)]
pub(crate) fn flag_on(key: &str) -> bool {
    std::env::var(key).ok().as_deref() == Some("1")
}

#[inline(always)]
pub(crate) fn flag_on_cached(_cell: &'static OnceLock<bool>, key: &str) -> bool {
    #[cfg(test)]
    {
        flag_on(key)
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| flag_on(key))
    }
}

#[inline(always)]
pub(crate) fn flag_any_on_cached(
    _cell: &'static OnceLock<bool>,
    keys: &'static [&'static str],
) -> bool {
    #[inline(always)]
    fn enabled(keys: &[&str]) -> bool {
        keys.iter().any(|key| flag_on(key))
    }

    #[cfg(test)]
    {
        enabled(keys)
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| enabled(keys))
    }
}

#[inline(always)]
pub(crate) fn flag_default_on_cached(_cell: &'static OnceLock<bool>, key: &str) -> bool {
    #[cfg(test)]
    {
        !matches!(
            std::env::var(key).ok().as_deref(),
            Some("0" | "false" | "off" | "FALSE" | "OFF")
        )
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| {
            !matches!(
                std::env::var(key).ok().as_deref(),
                Some("0" | "false" | "off" | "FALSE" | "OFF")
            )
        })
    }
}

#[cfg(not(test))]
#[inline(always)]
pub(crate) fn flag_default_on(key: &str) -> bool {
    !matches!(
        std::env::var(key).ok().as_deref(),
        Some("0" | "false" | "off" | "FALSE" | "OFF")
    )
}

#[cfg(not(test))]
#[inline(always)]
pub(crate) fn u64_or(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(default)
}

#[cfg(feature = "perf-observe")]
#[inline(always)]
pub(crate) fn flag_parsed_cached(
    _cell: &'static OnceLock<bool>,
    key: &str,
    parse: fn(&str) -> bool,
) -> bool {
    #[cfg(test)]
    {
        std::env::var(key).ok().as_deref().is_some_and(parse)
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| std::env::var(key).ok().as_deref().is_some_and(parse))
    }
}
