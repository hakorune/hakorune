use crate::ast::{ASTNode, BinaryOperator};

use super::super::body_check::expr_matchers::matches_trim_cond_with_methodcall;
use super::super::body_check_extractors::extract_next_i_var;
use super::super::facts::stmt_classifier::{is_local_decl, is_local_init};
use crate::mir::builder::control_flow::facts::canon::generic_loop::matches_loop_increment;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop;

use super::utils::*;

/// Matches the UsingCollector line scan pattern.
///
/// Shape: 3 statements - local decl, if (with no exits), loop increment
pub fn matches_usingcollector_line_scan_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 3 {
        return false;
    }
    if !is_local_decl(&body[0], loop_var) {
        return false;
    }
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        return false;
    };
    if then_body.is_empty() {
        return false;
    }
    if then_body
        .iter()
        .any(ASTNode::contains_non_local_exit_outside_loops)
    {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        if else_body
            .iter()
            .any(ASTNode::contains_non_local_exit_outside_loops)
        {
            return false;
        }
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches the scan all boxes next_i pattern.
///
/// Shape: 4-5 statements - next_i extraction, local decl, if, optional debug guard, step
pub fn matches_scan_all_boxes_next_i_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if !(body.len() == 4 || body.len() == 5) {
        return false;
    }
    let Some(next_var) = extract_next_i_var(&body[0], loop_var) else {
        return false;
    };
    if !(is_loop_var_plus_one(loop_increment, loop_var)
        || matches!(loop_increment, ASTNode::Variable { name, .. } if name == &next_var))
    {
        return false;
    }
    if !is_local_decl(&body[1], loop_var) {
        return false;
    }
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = &body[2]
    else {
        return false;
    };
    if then_body
        .iter()
        .any(ASTNode::contains_non_local_exit_outside_loops)
    {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body
            .iter()
            .any(ASTNode::contains_non_local_exit_outside_loops)
        {
            return false;
        }
    }
    let step_idx = body.len() - 1;
    if body.len() == 5 && !matches_debug_guard_block(&body[3], loop_var, &next_var) {
        return false;
    }
    matches_loop_var_assign_to(&body[step_idx], loop_var, &next_var)
}

/// Matches the itoa complex step pattern.
///
/// Shape: 3 statements - itoa digit local, itoa digit append, loop increment
pub fn matches_rewriteknown_itoa_complex_step_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 3 {
        return false;
    }
    if !matches_itoa_digit_local(&body[0], loop_var) {
        return false;
    }
    if !matches_itoa_digit_append_assignment(&body[1], loop_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches the trim and method call pattern.
///
/// Checks for specific condition pattern with trim and method call.
pub fn matches_rewriteknown_trim_and_methodcall_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if body.len() != 1 {
        return false;
    }
    if !matches_loop_increment(&body[0], loop_var, loop_increment) {
        return false;
    }
    matches_trim_cond_with_methodcall(condition, loop_var)
}

/// Matches the itoa digit local pattern.
///
/// Shape: `local d = loop_var % 10`
pub fn matches_itoa_digit_local(stmt: &ASTNode, loop_var: &str) -> bool {
    if !is_local_init(stmt, loop_var) {
        return false;
    }
    let ASTNode::Local { initial_values, .. } = stmt else {
        return false;
    };
    if initial_values.len() != 1 {
        return false;
    }
    let Some(init) = &initial_values[0] else {
        return false;
    };
    matches_mod_ten_of_loop_var(init.as_ref(), loop_var)
}

/// Matches the itoa digit append assignment pattern.
///
/// Shape: `target = target + if_expr` where if_expr is a pure value expression
pub fn matches_itoa_digit_append_assignment(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable {
        name: target_name, ..
    } = target.as_ref()
    else {
        return false;
    };
    if target_name == loop_var {
        return false;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return false;
    };

    let (if_expr, var_name) = match (left.as_ref(), right.as_ref()) {
        (ASTNode::If { .. }, ASTNode::Variable { name, .. }) => (left.as_ref(), name),
        (ASTNode::Variable { name, .. }, ASTNode::If { .. }) => (right.as_ref(), name),
        _ => return false,
    };
    if var_name != target_name {
        return false;
    }
    is_pure_value_expr_for_generic_loop(if_expr)
}

/// Matches the scan-while-predicate pattern.
///
/// Shape: 4 statements - local decl, two method calls, loop increment
pub fn matches_scan_while_predicate_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 4 {
        return false;
    }
    // body[0]: local decl (not declaring loop_var)
    if !is_local_decl(&body[0], loop_var) {
        return false;
    }
    // body[1]: method call / function call only
    if !matches!(
        body[1],
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. }
    ) {
        return false;
    }
    // body[2]: method call / function call only
    if !matches!(
        body[2],
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. }
    ) {
        return false;
    }
    // body[3]: step at last position
    matches_loop_increment(&body[3], loop_var, loop_increment)
}

/// Matches the effect-step-only pattern.
///
/// Shape: 2 statements - effect call, loop increment
pub fn matches_effect_step_only_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 2 {
        return false;
    }
    // body[0]: method call / function call only
    if !matches!(
        body[0],
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. }
    ) {
        return false;
    }
    // body[1]: step at last position
    matches_loop_increment(&body[1], loop_var, loop_increment)
}
