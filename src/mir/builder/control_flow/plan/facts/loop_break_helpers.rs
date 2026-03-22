//! Phase 29ai P11: Shared helper functions for loop_break extraction
//! Shared helpers for loop_break facts extraction.
//!
//! These helpers are used by multiple subset detectors.
//!
//! Note: All functions use `pub(in crate::mir::builder::control_flow::plan::facts)`
//! visibility to allow re-export via `pub(in crate::mir::builder::control_flow::plan::facts) use helpers::*;` in mod.rs.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_common::{
    add, has_continue_statement, has_return_statement, index_of_call, index_of_call_expr,
    length_call, lit_bool, lit_int, lit_str, substring_call, var,
};

pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_break_if::{
    extract_break_if_parts, find_break_if_parts,
};

// ============================================================================
// Section: Local Variable Extraction
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_substring_char(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(String, String, ASTNode)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let ch_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable {
        name: haystack_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    // end = i + 1
    match &arguments[1] {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
                return None;
            }
            if !matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    ..
                }
            ) {
                return None;
            }
        }
        _ => return None,
    }

    Some((ch_var, haystack_var.clone(), expr.as_ref().clone()))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_this_index_of(
    stmt: &ASTNode,
    ch_var: &str,
) -> Option<(String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let d_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "index_of" || arguments.len() != 2 {
        return None;
    }
    if !matches!(object.as_ref(), ASTNode::This { .. } | ASTNode::Me { .. }) {
        return None;
    }
    let ASTNode::Variable {
        name: digits_var, ..
    } = &arguments[0]
    else {
        return None;
    };
    if !matches!(&arguments[1], ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }

    Some((digits_var.clone(), d_var))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_indexof_local(
    stmt: &ASTNode,
) -> Option<(String, String, String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let j_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "indexOf" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable {
        name: haystack_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(sep_lit),
        ..
    } = &arguments[0]
    else {
        return None;
    };
    let ASTNode::Variable { name: loop_var, .. } = &arguments[1] else {
        return None;
    };

    Some((
        j_var,
        haystack_var.clone(),
        sep_lit.clone(),
        loop_var.clone(),
    ))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_empty_string(
    stmt: &ASTNode,
) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let seg_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(value),
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if value != "" {
        return None;
    }
    Some(seg_var)
}

pub(in crate::mir::builder::control_flow::plan::facts) fn find_local_init_expr(
    body: &[ASTNode],
    name: &str,
) -> Option<ASTNode> {
    for stmt in body {
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        else {
            continue;
        };
        if variables.len() != 1 || initial_values.len() != 1 {
            continue;
        }
        if variables[0] != name {
            continue;
        }
        let Some(expr) = initial_values[0].as_ref() else {
            return None;
        };
        return Some((*expr.clone()).clone());
    }
    None
}

// ============================================================================
// Section: Condition Matching
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn match_break_if_less_than_zero(
    stmt: &ASTNode,
) -> Option<String> {
    let (cond, update_opt) = extract_break_if_parts(stmt)?;
    if update_opt.is_some() {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = cond
    else {
        return None;
    };
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        }
    ) {
        return None;
    }
    Some(name.clone())
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_acc_update_mul10_plus_d(
    stmt: &ASTNode,
    d_var: &str,
) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable {
        name: carrier_var, ..
    } = target.as_ref()
    else {
        return None;
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    // left: acc * 10
    match left.as_ref() {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Multiply,
            left: mul_lhs,
            right: mul_rhs,
            ..
        } => {
            if !matches!(mul_lhs.as_ref(), ASTNode::Variable { name, .. } if name == carrier_var) {
                return None;
            }
            if !matches!(
                mul_rhs.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    ..
                }
            ) {
                return None;
            }
        }
        _ => return None,
    }

    // right: d
    if !matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == d_var) {
        return None;
    }

    Some(carrier_var.clone())
}

pub(in crate::mir::builder::control_flow::plan::facts) fn matches_ge_zero(
    node: &ASTNode,
    var_name: &str,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::GreaterEqual,
        left,
        right,
        ..
    } = node
    else {
        return false;
    };
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name)
        && matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(0),
                ..
            }
        )
}

pub(in crate::mir::builder::control_flow::plan::facts) fn matches_eq_empty_string(
    node: &ASTNode,
    var_name: &str,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = node
    else {
        return false;
    };
    matches_eq_empty_string_sides(left.as_ref(), right.as_ref(), var_name)
        || matches_eq_empty_string_sides(right.as_ref(), left.as_ref(), var_name)
}

fn matches_eq_empty_string_sides(var_node: &ASTNode, lit_node: &ASTNode, var_name: &str) -> bool {
    if !matches!(var_node, ASTNode::Variable { name, .. } if name == var_name) {
        return false;
    }
    matches!(
        lit_node,
        ASTNode::Literal {
            value: LiteralValue::String(value),
            ..
        } if value.is_empty()
    )
}

// ============================================================================
// Section: Trim Whitespace Helpers
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_var(
    condition: &ASTNode,
) -> Option<String> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };

    match operator {
        BinaryOperator::Less | BinaryOperator::LessEqual => {
            if matches!(
                right.as_ref(),
                ASTNode::MethodCall { object, method, arguments, .. }
                    if method == "length"
                        && arguments.is_empty()
                        && matches!(object.as_ref(), ASTNode::Variable { .. })
            ) {
                return Some(loop_var.clone());
            }
        }
        BinaryOperator::GreaterEqual => {
            if matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    ..
                }
            ) {
                return Some(loop_var.clone());
            }
        }
        _ => {}
    }

    None
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_break_condition(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<ASTNode> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }

    let whitespace_call = match condition.as_ref() {
        ASTNode::UnaryOp {
            operator, operand, ..
        } => {
            use crate::ast::UnaryOperator;
            if !matches!(operator, UnaryOperator::Not) {
                return None;
            }
            match_is_whitespace_call(operand.as_ref(), loop_var)?
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            if matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    ..
                }
            ) {
                match_is_whitespace_call(left.as_ref(), loop_var)?
            } else if matches!(
                left.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    ..
                }
            ) {
                match_is_whitespace_call(right.as_ref(), loop_var)?
            } else {
                return None;
            }
        }
        _ => return None,
    };

    Some(ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(whitespace_call),
        right: Box::new(lit_bool(false)),
        span: Span::unknown(),
    })
}

fn match_is_whitespace_call(expr: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "is_whitespace" || arguments.len() != 1 {
        return None;
    }
    let normalized_object = match object.as_ref() {
        ASTNode::This { .. } => ASTNode::This {
            span: Span::unknown(),
        },
        ASTNode::Me { .. } => ASTNode::This {
            span: Span::unknown(),
        },
        _ => return None,
    };
    if !matches_substring_at_loop_var(&arguments[0], loop_var) {
        return None;
    }
    Some(ASTNode::MethodCall {
        object: Box::new(normalized_object),
        method: method.clone(),
        arguments: arguments.clone(),
        span: Span::unknown(),
    })
}

fn matches_substring_at_loop_var(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return false;
    };
    if method != "substring" || arguments.len() != 2 {
        return false;
    }
    if !matches!(object.as_ref(), ASTNode::Variable { .. }) {
        return false;
    }
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }
    match &arguments[1] {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
                && matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                )
        }
        _ => false,
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add | BinaryOperator::Subtract,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }
    Some(value.as_ref().clone())
}

// ============================================================================
// Section: Loop Variable Extraction
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_var_for_len_condition(
    condition: &ASTNode,
) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less | BinaryOperator::LessEqual,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(
        right.as_ref(),
        ASTNode::MethodCall { object, method, arguments, .. }
            if method == "length"
                && arguments.is_empty()
                && matches!(object.as_ref(), ASTNode::Variable { .. })
    ) {
        return None;
    }
    Some(name.clone())
}

/// Extract loop variable from `i < N` condition where N is an integer literal.
pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_var_for_plan_subset(
    condition: &ASTNode,
) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }

    Some(name.clone())
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_increment_at_end(
    body: &[ASTNode],
    loop_var: &str,
) -> Option<ASTNode> {
    let last = body.last()?;
    let ASTNode::Assignment { target, value, .. } = last else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }
    Some(value.as_ref().clone())
}

pub(in crate::mir::builder::control_flow::plan::facts) fn has_assignment_after(
    body: &[ASTNode],
    start_idx: usize,
    var_name: &str,
) -> bool {
    for stmt in body.iter().skip(start_idx + 1) {
        let ASTNode::Assignment { target, .. } = stmt else {
            continue;
        };
        if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == var_name) {
            return true;
        }
    }
    false
}

pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_realworld::{
    match_break_if, match_loop_increment, match_seg_if_else,
};
