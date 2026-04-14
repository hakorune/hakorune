use super::facts_helpers::{
    try_extract_break_continue_pure, try_extract_methodcall, try_extract_no_break_or_continue,
    try_extract_no_break_or_continue_pure,
};
use super::facts_types::NestedLoopDepth1Facts;
use crate::ast::{ASTNode, BinaryOperator};

/// Try to extract unified nested loop depth1 facts.
///
/// Tries all kinds in priority order and returns the first match.
/// Priority: BreakContinuePure > NoBreakOrContinuePure > MethodCall > NoBreakOrContinue
pub(in crate::mir::builder) fn try_extract_nested_loop_depth1_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    // Common condition check: must be Less or LessEqual comparison
    if !matches!(
        condition,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less | BinaryOperator::LessEqual,
            ..
        }
    ) {
        return None;
    }

    // Try BreakContinuePure first (most specific for pure code with control flow)
    if let Some(facts) = try_extract_break_continue_pure(condition, body) {
        return Some(facts);
    }

    // Try NoBreakOrContinuePure (pure code without control flow)
    if let Some(facts) = try_extract_no_break_or_continue_pure(condition, body) {
        return Some(facts);
    }

    // Try MethodCall (requires calls, allows control flow)
    if let Some(facts) = try_extract_methodcall(condition, body) {
        return Some(facts);
    }

    // Try NoBreakOrContinue (requires calls, no control flow)
    if let Some(facts) = try_extract_no_break_or_continue(condition, body) {
        return Some(facts);
    }

    None
}
