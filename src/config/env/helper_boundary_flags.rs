//! Helper-boundary policy environment flags.
//!
//! This module centralizes policy toggles for runtime/helper hot lanes.

#[cfg(not(test))]
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostHandleAllocPolicyMode {
    /// Current default: reuse dropped handles in LIFO order.
    Lifo,
    /// Disable reuse; always issue fresh handles.
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringSpanCachePolicyMode {
    /// Current default: enable TLS string span cache.
    On,
    /// Disable TLS span cache (always bypass cache put/get).
    Off,
}

fn parse_host_handle_alloc_policy_mode(raw: Option<&str>) -> HostHandleAllocPolicyMode {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return HostHandleAllocPolicyMode::Lifo;
    };
    match value.to_ascii_lowercase().as_str() {
        "lifo" => HostHandleAllocPolicyMode::Lifo,
        "none" | "off" | "no-reuse" => HostHandleAllocPolicyMode::None,
        other => panic!(
            "[freeze:contract][helper-boundary/host-handle-policy] expected=lifo|none|off|no-reuse got={}",
            other
        ),
    }
}

fn parse_string_span_cache_policy_mode(raw: Option<&str>) -> StringSpanCachePolicyMode {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return StringSpanCachePolicyMode::On;
    };
    match value.to_ascii_lowercase().as_str() {
        "on" | "enabled" | "1" => StringSpanCachePolicyMode::On,
        "off" | "disabled" | "0" => StringSpanCachePolicyMode::Off,
        other => panic!(
            "[freeze:contract][helper-boundary/string-span-cache-policy] expected=on|off|enabled|disabled|1|0 got={}",
            other
        ),
    }
}

/// Host handle allocation policy mode.
///
/// Env:
/// - `NYASH_HOST_HANDLE_ALLOC_POLICY=lifo|none|off|no-reuse`
/// - default: `lifo`
pub fn host_handle_alloc_policy_mode() -> HostHandleAllocPolicyMode {
    #[cfg(test)]
    {
        parse_host_handle_alloc_policy_mode(
            std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY")
                .ok()
                .as_deref(),
        )
    }
    #[cfg(not(test))]
    {
        static MODE: OnceLock<HostHandleAllocPolicyMode> = OnceLock::new();
        *MODE.get_or_init(|| {
            parse_host_handle_alloc_policy_mode(
                std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY")
                    .ok()
                    .as_deref(),
            )
        })
    }
}

/// String span cache policy mode.
///
/// Env:
/// - `NYASH_STRING_SPAN_CACHE_POLICY=on|off|enabled|disabled|1|0`
/// - default: `on`
pub fn string_span_cache_policy_mode() -> StringSpanCachePolicyMode {
    #[cfg(test)]
    {
        parse_string_span_cache_policy_mode(
            std::env::var("NYASH_STRING_SPAN_CACHE_POLICY")
                .ok()
                .as_deref(),
        )
    }
    #[cfg(not(test))]
    {
        static MODE: OnceLock<StringSpanCachePolicyMode> = OnceLock::new();
        *MODE.get_or_init(|| {
            parse_string_span_cache_policy_mode(
                std::env::var("NYASH_STRING_SPAN_CACHE_POLICY")
                    .ok()
                    .as_deref(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_handle_alloc_policy_defaults_to_lifo() {
        assert_eq!(
            parse_host_handle_alloc_policy_mode(None),
            HostHandleAllocPolicyMode::Lifo
        );
        assert_eq!(
            parse_host_handle_alloc_policy_mode(Some("")),
            HostHandleAllocPolicyMode::Lifo
        );
    }

    #[test]
    fn host_handle_alloc_policy_accepts_none_aliases() {
        assert_eq!(
            parse_host_handle_alloc_policy_mode(Some("none")),
            HostHandleAllocPolicyMode::None
        );
        assert_eq!(
            parse_host_handle_alloc_policy_mode(Some("off")),
            HostHandleAllocPolicyMode::None
        );
        assert_eq!(
            parse_host_handle_alloc_policy_mode(Some("no-reuse")),
            HostHandleAllocPolicyMode::None
        );
    }

    #[test]
    #[should_panic(expected = "[freeze:contract][helper-boundary/host-handle-policy]")]
    fn host_handle_alloc_policy_invalid_value_panics() {
        let _ = parse_host_handle_alloc_policy_mode(Some("fifo"));
    }

    #[test]
    fn string_span_cache_policy_defaults_to_on() {
        assert_eq!(
            parse_string_span_cache_policy_mode(None),
            StringSpanCachePolicyMode::On
        );
        assert_eq!(
            parse_string_span_cache_policy_mode(Some("")),
            StringSpanCachePolicyMode::On
        );
    }

    #[test]
    fn string_span_cache_policy_accepts_off_aliases() {
        assert_eq!(
            parse_string_span_cache_policy_mode(Some("off")),
            StringSpanCachePolicyMode::Off
        );
        assert_eq!(
            parse_string_span_cache_policy_mode(Some("disabled")),
            StringSpanCachePolicyMode::Off
        );
        assert_eq!(
            parse_string_span_cache_policy_mode(Some("0")),
            StringSpanCachePolicyMode::Off
        );
    }

    #[test]
    #[should_panic(expected = "[freeze:contract][helper-boundary/string-span-cache-policy]")]
    fn string_span_cache_policy_invalid_value_panics() {
        let _ = parse_string_span_cache_policy_mode(Some("auto"));
    }
}
