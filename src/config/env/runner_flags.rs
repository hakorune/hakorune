//! Runner/pipeline environment flags and helpers

use super::{env_bool, env_string};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasmRoutePolicyMode {
    Default,
    RustNative,
}

fn parse_wasm_route_policy_mode(raw: Option<&str>) -> Result<WasmRoutePolicyMode, String> {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(WasmRoutePolicyMode::Default);
    };
    match value.to_ascii_lowercase().as_str() {
        "default" => Ok(WasmRoutePolicyMode::Default),
        "rust_native" => Ok(WasmRoutePolicyMode::RustNative),
        other => Err(format!(
            "[freeze:contract][wasm/route-policy] NYASH_WASM_ROUTE_POLICY='{}' (allowed: default|rust_native)",
            other
        )),
    }
}

pub fn debug_using() -> bool {
    env_bool("NYASH_DEBUG_USING")
}

pub fn modules_env() -> Option<String> {
    env_string("NYASH_MODULES")
}

pub fn using_path_env() -> Option<String> {
    env_string("NYASH_USING_PATH")
}

pub fn aliases_env() -> Option<String> {
    env_string("NYASH_ALIASES")
}

pub fn using_resolver_first() -> bool {
    env_bool("HAKO_USING_RESOLVER_FIRST")
}

pub fn using_ssot_enabled() -> bool {
    env_bool("HAKO_USING_SSOT")
}

pub fn using_ssot_invoking() -> bool {
    env_bool("HAKO_USING_SSOT_INVOKING")
}

pub fn using_ssot_invoking_raw() -> Option<String> {
    env_string("HAKO_USING_SSOT_INVOKING")
}

pub fn set_using_ssot_invoking(value: Option<&str>) {
    if let Some(v) = value {
        std::env::set_var("HAKO_USING_SSOT_INVOKING", v);
    } else {
        let _ = std::env::remove_var("HAKO_USING_SSOT_INVOKING");
    }
}

pub fn using_ssot_relative() -> bool {
    env_bool("HAKO_USING_SSOT_RELATIVE")
}

pub fn using_ssot_relative_ambig_first_n() -> Option<usize> {
    env_string("HAKO_USING_SSOT_RELATIVE_AMBIG_FIRST_N").and_then(|s| s.parse::<usize>().ok())
}

pub fn emit_mir_trace() -> bool {
    env_bool("NYASH_EMIT_MIR_TRACE")
}

pub fn deps_json_path() -> Option<String> {
    env_string("NYASH_DEPS_JSON")
}

pub fn fields_top_strict() -> bool {
    env_bool("NYASH_FIELDS_TOP_STRICT")
}

pub fn using_strict() -> bool {
    env_bool("NYASH_USING_STRICT")
}

pub fn set_cli_verbose(enabled: bool) {
    if enabled {
        std::env::set_var("NYASH_CLI_VERBOSE", "1");
    }
}

pub fn set_gc_mode(mode: &str) {
    if !mode.trim().is_empty() {
        std::env::set_var("NYASH_GC_MODE", mode);
    }
}

pub fn set_vm_stats(enabled: bool) {
    if enabled {
        std::env::set_var("NYASH_VM_STATS", "1");
    }
}

pub fn set_vm_stats_json(enabled: bool) {
    if enabled {
        std::env::set_var("NYASH_VM_STATS_JSON", "1");
    }
}

pub fn sched_trace_enabled() -> bool {
    env_bool("NYASH_SCHED_TRACE")
}

pub fn sched_poll_budget() -> usize {
    env_string("NYASH_SCHED_POLL_BUDGET")
        .and_then(|s| s.parse().ok())
        .filter(|&n: &usize| n > 0)
        .unwrap_or(1)
}

/// Whether safepoint bridge should invoke scheduler poll.
///
/// Default: true (keep runtime progress semantics unchanged).
/// Set `NYASH_SCHED_POLL_IN_SAFEPOINT=0` to disable for perf/diagnostic trials.
pub fn sched_poll_in_safepoint() -> bool {
    match env_string("NYASH_SCHED_POLL_IN_SAFEPOINT")
        .map(|s| s.trim().to_ascii_lowercase())
        .as_deref()
    {
        None => true,
        Some("1" | "true" | "on") => true,
        Some("0" | "false" | "off") => false,
        Some(invalid) => {
            eprintln!(
                "[freeze:contract][sched/poll_in_safepoint] NYASH_SCHED_POLL_IN_SAFEPOINT='{}' (allowed: 0|1|off|on|false|true)",
                invalid
            );
            std::process::exit(1);
        }
    }
}

pub fn ring0_log_level() -> Option<String> {
    env_string("NYASH_RING0_LOG_LEVEL")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// WASM output route policy.
///
/// Env:
/// - `NYASH_WASM_ROUTE_POLICY=default`
/// - default: `default`
pub fn wasm_route_policy_mode() -> WasmRoutePolicyMode {
    let raw = env_string("NYASH_WASM_ROUTE_POLICY");
    match parse_wasm_route_policy_mode(raw.as_deref()) {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}

/// Whether WASM route trace line is emitted.
///
/// Env:
/// - `NYASH_WASM_ROUTE_TRACE=1` enables a single stable trace line
///   with policy/plan/shape_id when compile-wasm route is chosen.
pub fn wasm_route_trace_enabled() -> bool {
    env_bool("NYASH_WASM_ROUTE_TRACE")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_route_policy_defaults_to_default() {
        assert_eq!(
            parse_wasm_route_policy_mode(None).expect("default should parse"),
            WasmRoutePolicyMode::Default
        );
        assert_eq!(
            parse_wasm_route_policy_mode(Some("")).expect("empty should parse"),
            WasmRoutePolicyMode::Default
        );
        assert_eq!(
            parse_wasm_route_policy_mode(Some("default")).expect("explicit default should parse"),
            WasmRoutePolicyMode::Default
        );
    }

    #[test]
    fn wasm_route_policy_rejects_invalid_value() {
        let err =
            parse_wasm_route_policy_mode(Some("auto")).expect_err("invalid policy must fail-fast");
        assert!(err.starts_with("[freeze:contract][wasm/route-policy]"));
    }

    #[test]
    fn wasm_route_policy_accepts_rust_native() {
        assert_eq!(
            parse_wasm_route_policy_mode(Some("rust_native")).expect("rust_native should parse"),
            WasmRoutePolicyMode::RustNative
        );
    }

    #[test]
    fn wasm_route_policy_rejects_legacy_aliases() {
        let err_legacy =
            parse_wasm_route_policy_mode(Some("legacy")).expect_err("legacy alias must fail-fast");
        assert!(err_legacy.contains("(allowed: default|rust_native)"));
        let err_legacy_rust = parse_wasm_route_policy_mode(Some("legacy-wasm-rust"))
            .expect_err("legacy-wasm-rust alias must fail-fast");
        assert!(err_legacy_rust.contains("(allowed: default|rust_native)"));
    }

    #[test]
    fn wasm_route_trace_defaults_off_contract() {
        std::env::remove_var("NYASH_WASM_ROUTE_TRACE");
        assert!(!wasm_route_trace_enabled());
    }
}
