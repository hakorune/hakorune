//! Box Factory environment variable configuration
//!
//! Phase 286B: Consolidates NYASH_BOX_FACTORY_* and NYASH_PLUGIN_* flags
//! Prevents direct std::env::{var,set_var,remove_var} access (AGENTS.md 5.3)

use crate::box_factory::FactoryPolicy;
use std::sync::atomic::{AtomicU8, Ordering};

const BOX_FACTORY_POLICY_MODE_UNSET: u8 = u8::MAX;
static BOX_FACTORY_POLICY_MODE_CACHE: AtomicU8 = AtomicU8::new(BOX_FACTORY_POLICY_MODE_UNSET);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginExecMode {
    ModuleFirst,
    DynamicOnly,
    DynamicFirst,
}

/// NYASH_BOX_FACTORY_POLICY: builtin_first, compat_plugin_first, strict_plugin_first
pub fn box_factory_policy() -> Option<String> {
    std::env::var("NYASH_BOX_FACTORY_POLICY").ok()
}

/// Parse NYASH_BOX_FACTORY_POLICY into a FactoryPolicy without allocating on the
/// common default path.
pub fn box_factory_policy_mode() -> Option<FactoryPolicy> {
    let cached = BOX_FACTORY_POLICY_MODE_CACHE.load(Ordering::Relaxed);
    if cached != BOX_FACTORY_POLICY_MODE_UNSET {
        return decode_box_factory_policy_mode(cached);
    }

    let raw = std::env::var_os("NYASH_BOX_FACTORY_POLICY")?;
    let raw = raw.to_str()?;
    let parsed = parse_box_factory_policy(raw);
    let _ = BOX_FACTORY_POLICY_MODE_CACHE.compare_exchange(
        BOX_FACTORY_POLICY_MODE_UNSET,
        encode_box_factory_policy_mode(parsed),
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
    parsed
}

fn parse_box_factory_policy(raw: &str) -> Option<FactoryPolicy> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.eq_ignore_ascii_case("builtin_first") {
        Some(FactoryPolicy::BuiltinFirst)
    } else if trimmed.eq_ignore_ascii_case("compat_plugin_first") {
        Some(FactoryPolicy::CompatPluginFirst)
    } else if trimmed.eq_ignore_ascii_case("strict_plugin_first") {
        Some(FactoryPolicy::StrictPluginFirst)
    } else {
        None
    }
}

fn encode_box_factory_policy_mode(mode: Option<FactoryPolicy>) -> u8 {
    match mode {
        Some(FactoryPolicy::BuiltinFirst) => 1,
        Some(FactoryPolicy::CompatPluginFirst) => 2,
        Some(FactoryPolicy::StrictPluginFirst) => 3,
        None => 0,
    }
}

fn decode_box_factory_policy_mode(value: u8) -> Option<FactoryPolicy> {
    match value {
        1 => Some(FactoryPolicy::BuiltinFirst),
        2 => Some(FactoryPolicy::CompatPluginFirst),
        3 => Some(FactoryPolicy::StrictPluginFirst),
        _ => None,
    }
}

/// Set NYASH_BOX_FACTORY_POLICY (used for tests/scenarios)
pub fn set_box_factory_policy(policy: &str) {
    std::env::set_var("NYASH_BOX_FACTORY_POLICY", policy);
    BOX_FACTORY_POLICY_MODE_CACHE.store(
        encode_box_factory_policy_mode(parse_box_factory_policy(policy)),
        Ordering::Relaxed,
    );
}

/// Reset NYASH_BOX_FACTORY_POLICY
pub fn reset_box_factory_policy() {
    std::env::remove_var("NYASH_BOX_FACTORY_POLICY");
    BOX_FACTORY_POLICY_MODE_CACHE.store(BOX_FACTORY_POLICY_MODE_UNSET, Ordering::Relaxed);
}

/// NYASH_USE_PLUGIN_BUILTINS enable
pub fn use_plugin_builtins() -> bool {
    std::env::var("NYASH_USE_PLUGIN_BUILTINS")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
        .unwrap_or(false)
}

/// NYASH_PLUGIN_OVERRIDE_TYPES list (comma-separated)
pub fn plugin_override_types() -> Option<Vec<String>> {
    std::env::var("NYASH_PLUGIN_OVERRIDE_TYPES").ok().map(|s| {
        s.split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    })
}

/// NYASH_DISABLE_PLUGINS enable
pub fn disable_plugins() -> bool {
    std::env::var("NYASH_DISABLE_PLUGINS")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
        .unwrap_or(false)
}

/// NYASH_DEBUG_PLUGIN enable
pub fn debug_plugin() -> bool {
    std::env::var("NYASH_DEBUG_PLUGIN")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
        .unwrap_or(false)
}

/// NYASH_DEV_PROVIDER_TRACE enable (dev-only provider/method selection trace)
pub fn dev_provider_trace() -> bool {
    std::env::var("NYASH_DEV_PROVIDER_TRACE")
        .ok()
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        })
        .unwrap_or(false)
}

fn parse_plugin_exec_mode(raw: Option<&str>) -> Result<PluginExecMode, String> {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(PluginExecMode::ModuleFirst);
    };
    match value.to_ascii_lowercase().as_str() {
        "module_first" => Ok(PluginExecMode::ModuleFirst),
        "dynamic_only" => Ok(PluginExecMode::DynamicOnly),
        "dynamic_first" => Ok(PluginExecMode::DynamicFirst),
        other => Err(format!(
            "[freeze:contract][plugin/exec-mode] NYASH_PLUGIN_EXEC_MODE='{}' (allowed: module_first|dynamic_only|dynamic_first)",
            other
        )),
    }
}

/// Plugin execution mode switch for de-Rust transition.
///
/// Env:
/// - `NYASH_PLUGIN_EXEC_MODE=module_first|dynamic_only|dynamic_first`
/// - default: `module_first`
pub fn plugin_exec_mode() -> PluginExecMode {
    let raw = std::env::var("NYASH_PLUGIN_EXEC_MODE").ok();
    match parse_plugin_exec_mode(raw.as_deref()) {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_factory::FactoryPolicy;
    use std::sync::Mutex;

    static BOX_FACTORY_POLICY_ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn box_factory_policy_mode_defaults_to_none() {
        let _lock = BOX_FACTORY_POLICY_ENV_LOCK.lock().unwrap();
        let prev = box_factory_policy();
        reset_box_factory_policy();

        assert_eq!(box_factory_policy_mode(), None);

        if let Some(v) = prev {
            set_box_factory_policy(&v);
        }
    }

    #[test]
    fn box_factory_policy_mode_parses_known_values() {
        let _lock = BOX_FACTORY_POLICY_ENV_LOCK.lock().unwrap();
        let prev = box_factory_policy();

        set_box_factory_policy("builtin_first");
        assert_eq!(box_factory_policy_mode(), Some(FactoryPolicy::BuiltinFirst));

        set_box_factory_policy("compat_plugin_first");
        assert_eq!(
            box_factory_policy_mode(),
            Some(FactoryPolicy::CompatPluginFirst)
        );

        set_box_factory_policy("strict_plugin_first");
        assert_eq!(
            box_factory_policy_mode(),
            Some(FactoryPolicy::StrictPluginFirst)
        );

        if let Some(v) = prev {
            set_box_factory_policy(&v);
        } else {
            reset_box_factory_policy();
        }
    }

    #[test]
    fn plugin_exec_mode_defaults_to_module_first() {
        assert_eq!(
            parse_plugin_exec_mode(None).expect("default should parse"),
            PluginExecMode::ModuleFirst
        );
        assert_eq!(
            parse_plugin_exec_mode(Some("")).expect("empty should parse"),
            PluginExecMode::ModuleFirst
        );
    }

    #[test]
    fn plugin_exec_mode_accepts_all_modes() {
        assert_eq!(
            parse_plugin_exec_mode(Some("module_first")).expect("module_first should parse"),
            PluginExecMode::ModuleFirst
        );
        assert_eq!(
            parse_plugin_exec_mode(Some("dynamic_only")).expect("dynamic_only should parse"),
            PluginExecMode::DynamicOnly
        );
        assert_eq!(
            parse_plugin_exec_mode(Some("dynamic_first")).expect("dynamic_first should parse"),
            PluginExecMode::DynamicFirst
        );
    }

    #[test]
    fn plugin_exec_mode_rejects_invalid() {
        let err = parse_plugin_exec_mode(Some("auto")).expect_err("invalid mode must fail-fast");
        assert!(err.starts_with("[freeze:contract][plugin/exec-mode]"));
        assert!(err.contains("(allowed: module_first|dynamic_only|dynamic_first)"));
    }
}
