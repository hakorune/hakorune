//! Plugin initialization for LLVM mode
//!
//! Handles plugin host initialization and diagnostic checks.

/// Plugin initialization Box
///
/// **Responsibility**: Initialize plugins and run diagnostics
/// **Input**: None
/// **Output**: Result<(), String>
pub struct PluginInitBox;

impl PluginInitBox {
    /// Initialize plugin host and run diagnostics
    ///
    /// This function:
    /// 1. Initializes the plugin host for method_id injection
    /// 2. Runs friendly plugin guard diagnostics (non-strict mode)
    pub fn init() -> Result<(), String> {
        // Initialize plugin host so method_id injection can resolve plugin calls
        crate::runner_plugin_init::init_bid_plugins();

        // Friendly plugin guard (non‑strict): unify diagnostics across modes
        crate::runner::modes::common_util::plugin_guard::check_and_report(
            false,
            crate::config::env::env_bool("NYASH_JSON_ONLY"),
            "llvm",
        );

        Ok(())
    }
}
