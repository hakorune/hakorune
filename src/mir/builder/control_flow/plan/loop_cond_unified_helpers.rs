//! Helper functions for loop_cond_unified variants.
//!
//! Consolidates common patterns across break_continue, continue_only,
//! continue_with_return, return_in_body, and true_break_continue.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, is_true_literal, ControlFlowCounts, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::loop_cond_shared::planner_required_for_loop_cond;

/// Validates entry gate (planner_required check).
/// Returns true if gate passes, false if gate rejects.
pub(super) fn entry_gate_ok() -> bool {
    planner_required_for_loop_cond()
}

/// Returns debug flag state.
pub(super) fn debug_enabled() -> bool {
    crate::config::env::is_joinir_debug()
}

/// Validates loop condition (not true literal, supported bool expr).
/// Returns true if validation passes, false if validation fails.
pub(super) fn validate_loop_condition(condition: &ASTNode) -> bool {
    if is_true_literal(condition) {
        return false;
    }
    is_supported_bool_expr_with_canon(condition, true)
}

/// Counts control flow with returns enabled.
pub(super) fn count_control_flow_with_returns(body: &[ASTNode]) -> ControlFlowCounts {
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

// ============================================================================
// true_break_continue 専用 helpers
// ============================================================================

/// true_break_continue 専用: nested loop の condition 検証
pub(super) fn is_supported_nested_loop_condition(condition: &ASTNode) -> bool {
    if is_true_literal(condition) {
        return true;
    }
    is_supported_bool_expr_for_true_loop(condition)
}

/// true_break_continue 専用: bool expr 検証
pub(super) fn is_supported_bool_expr_for_true_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::UnaryOp { operand, .. } => is_supported_bool_expr_for_true_loop(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr_for_true_loop(left)
                    && is_supported_bool_expr_for_true_loop(right)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr_for_true_loop(left)
                    && is_supported_value_expr_for_true_loop(right)
            }
            _ => false,
        },
        _ => false,
    }
}

/// true_break_continue 専用: value expr 検証
pub(super) fn is_supported_value_expr_for_true_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_supported_value_expr_for_true_loop(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
            ) && is_supported_value_expr_for_true_loop(left)
                && is_supported_value_expr_for_true_loop(right)
        }
        _ => false,
    }
}
