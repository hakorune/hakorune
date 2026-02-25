//! Phase 191: LoopForm Utilities
//!
//! Responsibility: Debug and bypass-related utility functions
//! - Debug logging control
//! - Bypass flag management
//! - Environment variable handling

use super::variable_models::LoopBypassFlags;

/// Phase 27.4C Cleanup: Check if LoopForm debug logging is enabled
///
/// Returns true if the `NYASH_LOOPFORM_DEBUG` environment variable is set.
/// This helper avoids duplicate checks across multiple locations.
#[inline]
pub fn is_loopform_debug_enabled() -> bool {
    std::env::var("NYASH_LOOPFORM_DEBUG").is_ok()
}

/// Phase 27.4-C Refactor: Get JoinIR Loop φ bypass flags
///
/// **Purpose**: Centralize Header/Exit φ bypass determination.
/// Avoids duplicate toggle checks across multiple locations.
///
/// # Arguments
/// - `fn_name` - Function name (e.g., "Main.skip/1")
///
/// # Returns
/// - `LoopBypassFlags` - Header/Exit bypass status
pub fn get_loop_bypass_flags(_fn_name: &str) -> LoopBypassFlags {
    LoopBypassFlags {
        // Phase 73: Header φ bypass experiment discontinued (always OFF).
        // Only verified in LoopScopeShape/JoinIR mainline.
        header: false,
    }
}

/// Check if JoinIR Exit φ bypass is enabled
///
/// Returns true only when both of the following are set:
/// - NYASH_JOINIR_EXPERIMENT=1
/// - (legacy) Exit φ bypass experimental flag
///
/// Phase 73: Exit φ bypass experiment discontinued (always OFF).
/// JoinIR path is verified in LoopScopeShape/Exit φ mainline.
pub fn joinir_exit_bypass_enabled() -> bool {
    false
}

/// Check if a function is a JoinIR Exit φ bypass target
///
/// Currently limited to 2 functions:
/// - Main.skip/1
/// - FuncScannerBox.trim/1
pub fn is_joinir_exit_bypass_target(func_name: &str) -> bool {
    matches!(func_name, "Main.skip/1" | "FuncScannerBox.trim/1")
}

/// Load bypass flags from environment variables
///
/// Returns (skip_pinned, skip_carrier, skip_exit) tuple
pub fn load_bypass_flags_from_env() -> (bool, bool, bool) {
    let skip_pinned = std::env::var("NYASH_BYPASS_PINNED_PHI")
        .map(|v| v == "1")
        .unwrap_or(false);
    let skip_carrier = std::env::var("NYASH_BYPASS_CARRIER_PHI")
        .map(|v| v == "1")
        .unwrap_or(false);
    let skip_exit = std::env::var("NYASH_BYPASS_EXIT_PHI")
        .map(|v| v == "1")
        .unwrap_or(false);
    (skip_pinned, skip_carrier, skip_exit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_loopform_debug_enabled() {
        // Should return false if env var not set (default case)
        // Note: Actual value depends on environment, so we just check it compiles
        let _ = is_loopform_debug_enabled();
    }

    #[test]
    fn test_get_loop_bypass_flags() {
        let flags = get_loop_bypass_flags("Main.skip/1");
        // Phase 73: Always disabled
        assert!(!flags.header);
    }

    #[test]
    fn test_joinir_exit_bypass_enabled() {
        // Phase 73: Always disabled
        assert!(!joinir_exit_bypass_enabled());
    }

    #[test]
    fn test_is_joinir_exit_bypass_target() {
        assert!(is_joinir_exit_bypass_target("Main.skip/1"));
        assert!(is_joinir_exit_bypass_target("FuncScannerBox.trim/1"));
        assert!(!is_joinir_exit_bypass_target("Other.function/1"));
    }

    #[test]
    fn test_load_bypass_flags_default() {
        // Environment variables may or may not be set, so we just verify it works
        let (skip_p, skip_c, skip_e) = load_bypass_flags_from_env();
        // Just ensure it returns booleans
        let _ = (skip_p, skip_c, skip_e);
    }
}
