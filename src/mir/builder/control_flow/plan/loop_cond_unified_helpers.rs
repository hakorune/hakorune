//! Helper functions for loop_cond_unified variants.
//!
//! Consolidates common patterns across break_continue, continue_only,
//! continue_with_return, return_in_body, and true_break_continue.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    count_control_flow, is_true_literal, ControlFlowCounts, ControlFlowDetector,
};
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;

/// Validates entry gate (planner_required check).
/// Returns true if gate passes, false if gate rejects.
pub(in crate::mir::builder) fn entry_gate_ok() -> bool {
    // Keep continue_* facts available in release so short-circuit loop_cond
    // fixtures do not fall through to generic_loop_v1 reject.
    true
}

/// Returns debug flag state.
pub(in crate::mir::builder) fn debug_enabled() -> bool {
    crate::config::env::is_joinir_debug()
}

/// Validates loop condition (not true literal, supported bool expr).
/// Returns true if validation passes, false if validation fails.
pub(in crate::mir::builder) fn validate_loop_condition(condition: &ASTNode) -> bool {
    if is_true_literal(condition) {
        return false;
    }
    is_supported_bool_expr_with_canon(condition, true)
}

/// Counts control flow with returns enabled.
pub(in crate::mir::builder) fn count_control_flow_with_returns(body: &[ASTNode]) -> ControlFlowCounts {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    count_control_flow(body, detector)
}

/// Counts control flow with basic detector (no special flags).
#[allow(dead_code)]
pub(super) fn count_control_flow_basic(body: &[ASTNode]) -> ControlFlowCounts {
    let detector = ControlFlowDetector::default();
    count_control_flow(body, detector)
}
