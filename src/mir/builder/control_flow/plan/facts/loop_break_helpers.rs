//! Phase 29ai P11: Shared helper functions for loop_break extraction
//! Shared helpers for loop_break facts extraction.
//!
//! These helpers are used by multiple subset detectors.
//!
//! Note: All functions use `pub(in crate::mir::builder::control_flow::plan::facts)`
//! visibility to allow re-export via `pub(in crate::mir::builder::control_flow::plan::facts) use helpers::*;` in mod.rs.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

// ============================================================================
// Section: AST Node Constructors
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_str(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn length_call(obj: &str) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(obj)),
        method: "length".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn index_of_call(haystack: &str, sep: &str, loop_var: &str) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "indexOf".to_string(),
        arguments: vec![lit_str(sep), var(loop_var)],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn index_of_call_expr(haystack: &str, needle: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "indexOf".to_string(),
        arguments: vec![needle],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn substring_call(haystack: &str, start: ASTNode, end: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "substring".to_string(),
        arguments: vec![start, end],
        span: Span::unknown(),
    }
}

// ============================================================================
// Section: Control Flow Helpers
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn has_continue_statement(body: &[ASTNode]) -> bool {
    use crate::mir::builder::control_flow::plan::extractors::common_helpers::has_continue_statement as common_has_continue;
    common_has_continue(body)
}

pub(in crate::mir::builder::control_flow::plan::facts) fn has_return_statement(body: &[ASTNode]) -> bool {
    use crate::mir::builder::control_flow::plan::extractors::common_helpers::has_return_statement as common_has_return;
    common_has_return(body)
}

// ============================================================================
// Section: Break/If Pattern Extraction
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_break_if_parts(stmt: &ASTNode) -> Option<(ASTNode, Option<ASTNode>)> {
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

    let has_break_at_end = then_body
        .last()
        .map(|n| matches!(n, ASTNode::Break { .. }))
        .unwrap_or(false);
    if !has_break_at_end {
        return None;
    }

    let carrier_update_in_break = if then_body.len() == 1 {
        None
    } else if then_body.len() == 2 {
        match &then_body[0] {
            ASTNode::Assignment { value, .. } => Some(value.as_ref().clone()),
            _ => return None,
        }
    } else {
        return None;
    };

    Some((condition.as_ref().clone(), carrier_update_in_break))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn find_break_if_parts(body: &[ASTNode]) -> Option<(usize, ASTNode, Option<ASTNode>)> {
    for (idx, stmt) in body.iter().enumerate() {
        if let Some(parts) = extract_break_if_parts(stmt) {
            return Some((idx, parts.0, parts.1));
        }
    }
    None
}

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
    } = stmt else {
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
    } = expr.as_ref() else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: haystack_var, .. } = object.as_ref() else {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_this_index_of(stmt: &ASTNode, ch_var: &str) -> Option<(String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt else {
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
    } = expr.as_ref() else {
        return None;
    };
    if method != "index_of" || arguments.len() != 2 {
        return None;
    }
    if !matches!(object.as_ref(), ASTNode::This { .. } | ASTNode::Me { .. }) {
        return None;
    }
    let ASTNode::Variable { name: digits_var, .. } = &arguments[0] else {
        return None;
    };
    if !matches!(&arguments[1], ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }

    Some((digits_var.clone(), d_var))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_indexof_local(stmt: &ASTNode) -> Option<(String, String, String, String)> {
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
    let ASTNode::Variable { name: haystack_var, .. } = object.as_ref() else {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_empty_string(stmt: &ASTNode) -> Option<String> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn find_local_init_expr(body: &[ASTNode], name: &str) -> Option<ASTNode> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn match_break_if_less_than_zero(stmt: &ASTNode) -> Option<String> {
    let (cond, update_opt) = extract_break_if_parts(stmt)?;
    if update_opt.is_some() {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = cond else {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn match_acc_update_mul10_plus_d(stmt: &ASTNode, d_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name: carrier_var, .. } = target.as_ref() else {
        return None;
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref() else {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn matches_ge_zero(node: &ASTNode, var_name: &str) -> bool {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn matches_eq_empty_string(node: &ASTNode, var_name: &str) -> bool {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_var(condition: &ASTNode) -> Option<String> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_break_condition(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
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
        ASTNode::UnaryOp { operator, operand, .. } => {
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
        ASTNode::This { .. } => ASTNode::This { span: Span::unknown() },
        ASTNode::Me { .. } => ASTNode::This { span: Span::unknown() },
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

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_increment(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_var_for_len_condition(condition: &ASTNode) -> Option<String> {
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
pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_var_for_plan_subset(condition: &ASTNode) -> Option<String> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_loop_increment_at_end(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
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

pub(in crate::mir::builder::control_flow::plan::facts) fn has_assignment_after(body: &[ASTNode], start_idx: usize, var_name: &str) -> bool {
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

// ============================================================================
// Section: Real-world Pattern Helpers
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::facts) fn match_seg_if_else(
    stmt: &ASTNode,
    j_var: &str,
    seg_var: &str,
    haystack_var: &str,
    loop_var: &str,
) -> Option<bool> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    let else_body = else_body.as_ref()?;
    if then_body.len() != 1 || else_body.len() != 1 {
        return None;
    }
    if !matches_ge_zero(condition.as_ref(), j_var) {
        return None;
    }

    let then_expr = extract_substring_assignment(&then_body[0], seg_var, haystack_var)?;
    let else_expr = extract_substring_assignment(&else_body[0], seg_var, haystack_var)?;

    if !matches_substring_args(&then_expr, loop_var, Some(j_var), None) {
        return None;
    }
    if !matches_substring_args(&else_expr, loop_var, None, Some(haystack_var)) {
        return None;
    }

    Some(true)
}

fn extract_substring_assignment(
    stmt: &ASTNode,
    seg_var: &str,
    haystack_var: &str,
) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != seg_var {
        return None;
    }
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: obj_name, .. } = object.as_ref() else {
        return None;
    };
    if obj_name != haystack_var {
        return None;
    }
    Some(value.as_ref().clone())
}

fn matches_substring_args(
    expr: &ASTNode,
    loop_var: &str,
    end_var: Option<&str>,
    end_length_of: Option<&str>,
) -> bool {
    let ASTNode::MethodCall { arguments, .. } = expr else {
        return false;
    };
    if arguments.len() != 2 {
        return false;
    }
    let ASTNode::Variable { name: start_var, .. } = &arguments[0] else {
        return false;
    };
    if start_var != loop_var {
        return false;
    }

    match (&arguments[1], end_var, end_length_of) {
        (ASTNode::Variable { name, .. }, Some(var), None) => name == var,
        (ASTNode::MethodCall { object, method, arguments, .. }, None, Some(owner)) => {
            if method != "length" || !arguments.is_empty() {
                return false;
            }
            matches!(object.as_ref(), ASTNode::Variable { name, .. } if name == owner)
        }
        _ => false,
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_break_if(stmt: &ASTNode, seg_var: &str) -> Option<bool> {
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
    if !matches_eq_empty_string(condition.as_ref(), seg_var) {
        return None;
    }
    Some(true)
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
    j_var: &str,
    sep_len: i64,
) -> Option<bool> {
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
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == j_var) {
        return None;
    }
    if !matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::Integer(v), .. } if *v == sep_len) {
        return None;
    }
    Some(true)
}
