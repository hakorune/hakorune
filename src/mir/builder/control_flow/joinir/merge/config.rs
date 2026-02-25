/// Phase 131 P1 Task 6: Merge configuration consolidation
///
/// Consolidates all merge-related configuration into a single structure
/// to reduce parameter clutter and improve maintainability.
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Enable detailed trace logs (dev mode)
    pub dev_log: bool,
    /// Enable strict contract verification (fail-fast on violations)
    pub strict_mode: bool,
    /// Exit reconnection mode (Phi or DirectValue)
    #[allow(dead_code)]
    pub exit_reconnect_mode: Option<crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode>,
    /// Allow missing exit block in contract checks (typically exit_block_id before insertion)
    #[allow(dead_code)]
    pub allow_missing_exit_block: bool,
}

impl MergeConfig {
    /// Default configuration for normal operation
    pub fn default() -> Self {
        Self {
            dev_log: crate::config::env::joinir_dev_enabled(),
            strict_mode: crate::config::env::joinir_strict_enabled(),
            exit_reconnect_mode: None,
            allow_missing_exit_block: true,
        }
    }

    /// Strict configuration for development/debugging (all checks enabled)
    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            dev_log: true,
            strict_mode: true,
            exit_reconnect_mode: None,
            allow_missing_exit_block: true,
        }
    }

    /// Configuration for specific debug session
    pub fn with_debug(debug: bool) -> Self {
        let mut config = Self::default();
        config.dev_log = debug || config.dev_log;
        config
    }
}
