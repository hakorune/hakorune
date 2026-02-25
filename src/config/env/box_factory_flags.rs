//! Box Factory environment variable configuration
//!
//! Phase 286B: Consolidates NYASH_BOX_FACTORY_* and NYASH_PLUGIN_* flags
//! Prevents direct std::env::{var,set_var,remove_var} access (AGENTS.md 5.3)

/// NYASH_BOX_FACTORY_POLICY: builtin_first, compat_plugin_first, strict_plugin_first
pub fn box_factory_policy() -> Option<String> {
    std::env::var("NYASH_BOX_FACTORY_POLICY").ok()
}

/// Set NYASH_BOX_FACTORY_POLICY (used for tests/scenarios)
pub fn set_box_factory_policy(policy: &str) {
    std::env::set_var("NYASH_BOX_FACTORY_POLICY", policy);
}

/// Reset NYASH_BOX_FACTORY_POLICY
pub fn reset_box_factory_policy() {
    std::env::remove_var("NYASH_BOX_FACTORY_POLICY");
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
