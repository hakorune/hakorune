//! Provider-related environment readers (centralized)
//!
//! Minimizes scattered `std::env::var` calls across the runtime by
//! providing a small typed surface for provider selection knobs.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderPolicy {
    /// Current default: prefer plugin/dynamic providers by priority; use ring-1 as fallback
    StrictPluginFirst,
    /// Prefer ring-1 (static/core-ro) for stability/CI; fall back to plugin if unavailable
    SafeCoreFirst,
    /// Alias to SafeCoreFirst for future extension
    StaticPreferred,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileBoxMode {
    Auto,
    CoreRo,
    PluginOnly,
}

/// Read global provider policy (affects Auto mode only)
pub fn provider_policy_from_env() -> ProviderPolicy {
    match std::env::var("HAKO_PROVIDER_POLICY")
        .unwrap_or_else(|_| "strict-plugin-first".to_string())
        .as_str()
    {
        "safe-core-first" => ProviderPolicy::SafeCoreFirst,
        "static-preferred" => ProviderPolicy::StaticPreferred,
        _ => ProviderPolicy::StrictPluginFirst,
    }
}

/// Read FileBox mode from environment variables
pub fn filebox_mode_from_env() -> FileBoxMode {
    match std::env::var("NYASH_FILEBOX_MODE")
        .unwrap_or_else(|_| "auto".to_string())
        .as_str()
    {
        "core-ro" => FileBoxMode::CoreRo,
        "plugin-only" => FileBoxMode::PluginOnly,
        _ => {
            if std::env::var("NYASH_DISABLE_PLUGINS").as_deref() == Ok("1") {
                FileBoxMode::CoreRo
            } else {
                FileBoxMode::Auto
            }
        }
    }
}

/// Allow core-ro fallback override even if Fail-Fast is ON when:
/// - JSON-only pipeline is active (quiet structured I/O), or
/// - Explicit NYASH_FILEBOX_ALLOW_FALLBACK=1 is set.
pub fn allow_filebox_fallback_override(quiet_pipe: bool) -> bool {
    quiet_pipe || crate::config::env::env_bool("NYASH_FILEBOX_ALLOW_FALLBACK")
}
