//! Phase 86: Centralized JoinIR error tag generation
//!
//! ## Purpose
//! Provides SSOT for error message formatting to ensure consistency across
//! all JoinIR lowering error paths.
//!
//! ## Benefits
//! - **Consistency**: All error tags use standardized formatting
//! - **Discoverability**: Easy to audit all error types in one place
//! - **Typo prevention**: Compiler catches typos vs. string literals
//! - **Maintenance**: Change tag format in one place
//!
//! ## Usage
//! ```rust,ignore
//! // Before:
//! return Err(format!("[joinir/freeze] route not supported: {}", reason));
//!
//! // After:
//! use crate::mir::join_ir::lowering::error_tags;
//! return Err(error_tags::freeze(&format!("route not supported: {}", reason)));
//! ```

/// JoinIR freeze error - route not supported by current lowering implementation
///
/// Used when a JoinIR route cannot be lowered to MIR due to limitations
/// in the current implementation.
///
/// # Example
/// ```rust,ignore
/// return Err(freeze("Loop lowering failed: unsupported loop-break shape"));
/// // Output: "[joinir/freeze] Loop lowering failed: unsupported loop-break shape"
/// ```
pub fn freeze(diagnostic: &str) -> String {
    format!("[joinir/freeze] {}", diagnostic)
}

/// Create error message with hint (Phase 109)
///
/// Format: "[joinir/<category>/<tag>] <msg>  Hint: <hint>"
///
/// # Panics
/// Panics if hint is empty (must provide actionable fix suggestion)
///
/// # Example
/// ```rust,ignore
/// return Err(freeze_with_hint(
///     "phase107/balanced_depth_scan/missing_tail_inc",
///     "tail increment not found",
///     "add 'i = i + 1' at top-level"
/// ));
/// // Output: "[joinir/phase107/balanced_depth_scan/missing_tail_inc] tail increment not found  Hint: add 'i = i + 1' at top-level"
/// ```
pub fn freeze_with_hint(tag: &str, msg: &str, hint: &str) -> String {
    assert!(!hint.is_empty(), "hint must not be empty (tag: {})", tag);
    format!("[joinir/{}] {}  Hint: {}", tag, msg, hint)
}

/// ExitLine contract violation - Phase 81+ exit metadata contract broken
///
/// Used when exit line reconnection or exit PHI construction violates
/// the established contract (e.g., missing carrier in variable_map).
///
/// # Example
/// ```rust,ignore
/// return Err(exit_line_contract("Carrier missing from variable_map"));
/// // Output: "[JoinIR/ExitLine/Contract] Carrier missing from variable_map"
/// ```
pub fn exit_line_contract(detail: &str) -> String {
    format!("[JoinIR/ExitLine/Contract] {}", detail)
}

/// Ownership relay runtime unsupported - Multi-hop relay not executable
///
/// Used when ownership plan validation detects a multi-hop relay pattern
/// that cannot be executed at runtime (implementation limitation).
///
/// # Example
/// ```rust,ignore
/// return Err(ownership_relay_unsupported(&format!(
///     "Multihop relay: var='{}', path={:?}", var, path
/// )));
/// // Output: "[ownership/relay:runtime_unsupported] Multihop relay: var='x', path=[...]"
/// ```
pub fn ownership_relay_unsupported(diagnostic: &str) -> String {
    format!("[ownership/relay:runtime_unsupported] {}", diagnostic)
}

/// Route detection failure - JoinIR route could not be classified
///
/// Used when route detection fails to match a loop structure to any
/// known lowering route.
///
/// # Example
/// ```rust,ignore
/// return Err(route_detection_failed("if_phi_join", "Missing if-phi"));
/// // Output: "[joinir/route/if_phi_join] Detection failed: Missing if-phi"
/// ```
pub fn route_detection_failed(route_name: &str, reason: &str) -> String {
    format!("[joinir/route/{}] Detection failed: {}", route_name, reason)
}

/// Generic JoinIR lowering error with custom tag
///
/// Used for one-off error cases that don't fit the common patterns.
///
/// # Example
/// ```rust,ignore
/// return Err(lowering_error("boundary/injection", "Invalid boundary block"));
/// // Output: "[joinir/lowering/boundary/injection] Invalid boundary block"
/// ```
pub fn lowering_error(subsystem: &str, detail: &str) -> String {
    format!("[joinir/lowering/{}] {}", subsystem, detail)
}

/// Phase 145 P2: ANF order violation error
///
/// Used when impure expressions appear in immediate position without hoisting.
///
/// # Example
/// ```rust,ignore
/// return Err(anf_order_violation("f() + g()", "both calls not hoisted"));
/// // Output: "[joinir/anf/order_violation] f() + g(): both calls not hoisted"
/// ```
pub fn anf_order_violation(expr: &str, reason: &str) -> String {
    format!("[joinir/anf/order_violation] {}: {}", expr, reason)
}

/// Phase 145 P2: ANF pure required error
///
/// Used when impure expression appears in pure-only scope (e.g., loop condition).
///
/// # Example
/// ```rust,ignore
/// return Err(anf_pure_required("loop(iter.hasNext())", "impure in condition"));
/// // Output: "[joinir/anf/pure_required] loop(iter.hasNext()): impure in condition"
/// ```
pub fn anf_pure_required(expr: &str, reason: &str) -> String {
    format!("[joinir/anf/pure_required] {}: {}", expr, reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freeze_tag() {
        let err = freeze("Test error");
        assert!(err.starts_with("[joinir/freeze]"));
        assert!(err.contains("Test error"));
    }

    #[test]
    fn test_exit_line_contract_tag() {
        let err = exit_line_contract("Missing carrier");
        assert!(err.starts_with("[JoinIR/ExitLine/Contract]"));
        assert!(err.contains("Missing carrier"));
    }

    #[test]
    fn test_ownership_relay_unsupported_tag() {
        let err = ownership_relay_unsupported("Multihop not supported");
        assert!(err.starts_with("[ownership/relay:runtime_unsupported]"));
        assert!(err.contains("Multihop not supported"));
    }

    #[test]
    fn test_route_detection_failed_tag() {
        let err = route_detection_failed("loop_break", "Missing break");
        assert!(err.contains("[joinir/route/loop_break]"));
        assert!(err.contains("Detection failed"));
        assert!(err.contains("Missing break"));
    }

    #[test]
    fn test_lowering_error_tag() {
        let err = lowering_error("boundary", "Invalid block");
        assert!(err.contains("[joinir/lowering/boundary]"));
        assert!(err.contains("Invalid block"));
    }

    #[test]
    fn test_freeze_with_hint_format() {
        let result = freeze_with_hint(
            "phase107/balanced_depth_scan/missing_tail_inc",
            "tail increment not found",
            "add 'i = i + 1' at top-level",
        );
        assert!(result.contains("[joinir/phase107/balanced_depth_scan/missing_tail_inc]"));
        assert!(result.contains("Hint:"));
        assert!(result.contains("add 'i = i + 1'"));
    }

    #[test]
    #[should_panic(expected = "hint must not be empty")]
    fn test_freeze_with_hint_empty_hint_panics() {
        freeze_with_hint("test/tag", "message", "");
    }

    #[test]
    fn test_anf_order_violation_tag() {
        let err = anf_order_violation("f() + g()", "both calls not hoisted");
        assert!(err.contains("[joinir/anf/order_violation]"));
        assert!(err.contains("f() + g()"));
        assert!(err.contains("both calls not hoisted"));
    }

    #[test]
    fn test_anf_pure_required_tag() {
        let err = anf_pure_required("loop(iter.hasNext())", "impure in condition");
        assert!(err.contains("[joinir/anf/pure_required]"));
        assert!(err.contains("loop(iter.hasNext())"));
        assert!(err.contains("impure in condition"));
    }
}
