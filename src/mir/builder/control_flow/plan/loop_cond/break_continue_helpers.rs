//! Helper functions for loop_cond_break_continue facts extraction.
//!
//! This module contains utility functions that are used across the facts extraction pipeline:
//! - Variable collection from expressions
//! - Nested loop validation
//! - Guard break detection
//! - Continue branch signature collection

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    flatten_stmt_list, is_true_literal, walk_stmt_list,
};
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::{
    try_extract_generic_loop_v0_facts, try_extract_generic_loop_v1_facts,
};
use crate::mir::builder::control_flow::plan::loop_cond_shared::branch_tail_is_continue_flattened;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::try_extract_loop_true_break_continue_facts;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::facts::try_extract_nested_loop_depth1_facts;
use std::collections::BTreeSet;

use super::break_continue_types::{ContinueBranchSig, MAX_NESTED_LOOPS};

/// Collect variable names referenced in an expression.
///
/// Returns true if all sub-expressions were successfully processed,
/// false if an unsupported expression type was encountered.
pub(super) fn collect_vars_from_expr(ast: &ASTNode, vars: &mut BTreeSet<String>) -> bool {
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
            true
        }
        ASTNode::Literal { .. } => true,
        ASTNode::UnaryOp { operand, .. } => collect_vars_from_expr(operand, vars),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_vars_from_expr(left, vars) && collect_vars_from_expr(right, vars)
        }
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            if !collect_vars_from_expr(object, vars) {
                return false;
            }
            for arg in arguments {
                if !collect_vars_from_expr(arg, vars) {
                    return false;
                }
            }
            true
        }
        ASTNode::FunctionCall { arguments, .. } => {
            for arg in arguments {
                if !collect_vars_from_expr(arg, vars) {
                    return false;
                }
            }
            true
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            if !collect_vars_from_expr(callee, vars) {
                return false;
            }
            for arg in arguments {
                if !collect_vars_from_expr(arg, vars) {
                    return false;
                }
            }
            true
        }
        ASTNode::FieldAccess { object, .. } => collect_vars_from_expr(object, vars),
        ASTNode::Index { target, index, .. } => {
            collect_vars_from_expr(target, vars) && collect_vars_from_expr(index, vars)
        }
        ASTNode::New { arguments, .. } => {
            for arg in arguments {
                if !collect_vars_from_expr(arg, vars) {
                    return false;
                }
            }
            true
        }
        ASTNode::This { .. } | ASTNode::Me { .. } => true,
        ASTNode::ThisField { .. } | ASTNode::MeField { .. } => true,
        _ => false,
    }
}

/// Check if a nested loop is allowed based on its structure.
///
/// A nested loop is allowed if:
/// - It has no return statements
/// - It matches one of the supported loop route shapes (loop_true_break_continue, generic_loop, etc.)
pub(super) fn is_nested_loop_allowed(
    condition: &ASTNode,
    body: &[ASTNode],
    allow_extended: bool,
    debug: bool,
) -> bool {
    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    if counts.return_count > 0 {
        return false;
    }
    if is_true_literal(condition) {
        return matches!(
            try_extract_loop_true_break_continue_facts(condition, body),
            Ok(Some(_))
        );
    }
    if counts.has_nested_loop {
        return false;
    }
    if allow_extended {
        if matches!(
            try_extract_generic_loop_v0_facts(condition, body),
            Ok(Some(_))
        ) {
            return true;
        }
        if matches!(
            try_extract_generic_loop_v1_facts(condition, body),
            Ok(Some(_))
        ) {
            return true;
        }
        // Phase 12: Use unified nested_loop_depth1 facts extraction
        if try_extract_nested_loop_depth1_facts(condition, body).is_some() {
            return true;
        }
        // Accept nested loops (depth=1) conservatively when:
        // - the condition is a supported bool expr (canon-capable),
        // - the body has no further nested loops/returns (checked above).
        //
        // Lowering will use `nested_loop_depth1_any` if specialized nested-loop facts don't match.
        return is_supported_bool_expr_with_canon(condition, allow_extended);
    }
    matches!(
        super::break_continue_facts::try_extract_loop_cond_break_continue_facts_inner(
            condition,
            body,
            false,
            allow_extended,
            debug,
            MAX_NESTED_LOOPS,
            None,
        ),
        Ok(Some(_))
    )
}

/// Detect a "handled guard break" pattern at the end of a loop body.
///
/// Pattern: `if (var == 0) { break }`
///
/// Returns (true, Some(var_name)) if the pattern is detected, (false, None) otherwise.
pub(super) fn detect_handled_guard_break(body: &[ASTNode]) -> (bool, Option<String>) {
    let Some(stmt) = body.last() else {
        return (false, None);
    };
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return (false, None);
    };
    let Some(name) = extract_handled_guard_break_var(condition, then_body, else_body.as_ref())
    else {
        return (false, None);
    };
    (true, Some(name))
}

fn extract_handled_guard_break_var(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> Option<String> {
    if else_body.is_some() {
        return None;
    }
    if then_body.len() != 1 {
        return None;
    }
    if !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    var_equals_zero(left, right).or_else(|| var_equals_zero(right, left))
}

fn var_equals_zero(var: &ASTNode, lit: &ASTNode) -> Option<String> {
    let ASTNode::Variable { name, .. } = var else {
        return None;
    };
    if let ASTNode::Literal {
        value: LiteralValue::Integer(0),
        ..
    } = lit
    {
        return Some(name.clone());
    }
    None
}

/// Collect signatures of all continue branches in a loop body.
pub(super) fn collect_continue_branch_sigs(body: &[ASTNode]) -> Vec<ContinueBranchSig> {
    let mut out = Vec::new();
    walk_stmt_list(body, |stmt| {
        let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        else {
            return false;
        };
        if branch_tail_is_continue_flattened(then_body) {
            out.push(continue_branch_sig(then_body));
        }
        if let Some(else_body) = else_body {
            if branch_tail_is_continue_flattened(else_body) {
                out.push(continue_branch_sig(else_body));
            }
        }
        false
    });
    out
}

fn continue_branch_sig(body: &[ASTNode]) -> ContinueBranchSig {
    let mut has_assignment = false;
    let mut has_local = false;
    let stmt_count = flatten_stmt_list(body).len();
    walk_stmt_list(body, |stmt| {
        match stmt {
            ASTNode::Assignment { .. } => has_assignment = true,
            ASTNode::Local { .. } => has_local = true,
            _ => {}
        }
        false
    });
    ContinueBranchSig {
        stmt_count,
        has_assignment,
        has_local,
    }
}

/// Check if the body has any exit statement (break, continue, return).
pub(super) fn body_has_any_exit(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                return true;
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if body_has_any_exit(then_body) {
                    return true;
                }
                if let Some(else_body) = else_body {
                    if body_has_any_exit(else_body) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Check if the body has any exit or loop statement.
pub(super) fn branch_has_exit_or_loop(body: &[ASTNode]) -> bool {
    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    counts.has_nested_loop
        || counts.break_count > 0
        || counts.continue_count > 0
        || counts.return_count > 0
}

/// Phase 29bq BoxCount: detect ParserExprBox.parse_string2 loop(cond) shape.
pub(super) fn matches_parse_string2_shape(body: &[ASTNode]) -> bool {
    if body.len() != 7 {
        return false;
    }
    if !is_break_only_if(&body[0]) {
        return false;
    }
    if !matches!(body[1], ASTNode::Assignment { .. }) {
        return false;
    }
    if !matches!(body[2], ASTNode::Local { .. }) {
        return false;
    }
    if !is_return_exit_if(&body[3]) {
        return false;
    }
    if !is_escape_continue_if(&body[4]) {
        return false;
    }
    if !matches!(body[5], ASTNode::Assignment { .. }) {
        return false;
    }
    if !matches!(body[6], ASTNode::Assignment { .. }) {
        return false;
    }
    true
}

fn is_break_only_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    matches!(then_body.as_slice(), [ASTNode::Break { .. }])
}

fn is_return_exit_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    if then_body.len() != 3 {
        return false;
    }
    matches!(
        (&then_body[0], &then_body[1], &then_body[2]),
        (
            ASTNode::Assignment { .. },
            ASTNode::MethodCall { .. },
            ASTNode::Return { .. }
        )
    )
}

fn is_escape_continue_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    if then_body.len() != 10 {
        return false;
    }
    if !matches!(then_body[0], ASTNode::Local { .. }) {
        return false;
    }
    for idx in 1..=6 {
        let ASTNode::If {
            then_body,
            else_body,
            ..
        } = &then_body[idx]
        else {
            return false;
        };
        if else_body.is_some() {
            return false;
        }
        if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
            return false;
        }
    }
    if !matches!(then_body[7], ASTNode::Assignment { .. }) {
        return false;
    }
    if !matches!(then_body[8], ASTNode::Assignment { .. }) {
        return false;
    }
    matches!(then_body[9], ASTNode::Continue { .. })
}
