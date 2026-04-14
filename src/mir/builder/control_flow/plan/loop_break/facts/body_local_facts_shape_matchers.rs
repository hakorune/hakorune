//! loop_break body-local promotion facts shape-specific matchers.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::body_local_facts::LoopBodyLocalShape;
mod body_local_digit_matcher;
mod body_local_trim_matcher;

pub(super) fn try_match_trim_seg(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    body_local_trim_matcher::try_match_trim_seg(break_condition, body, break_idx, loop_var)
}

pub(super) fn try_match_digit_pos(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    body_local_digit_matcher::try_match_digit_pos(break_condition, body, break_idx, loop_var)
}

pub(super) fn extract_eq_whitespace(node: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = node
    else {
        return None;
    };
    extract_var_eq_whitespace(left.as_ref(), right.as_ref())
        .or_else(|| extract_var_eq_whitespace(right.as_ref(), left.as_ref()))
}

fn extract_var_eq_whitespace(var_node: &ASTNode, lit_node: &ASTNode) -> Option<String> {
    let ASTNode::Variable { name, .. } = var_node else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(value),
        ..
    } = lit_node
    else {
        return None;
    };
    if value == " " || value == "\t" {
        Some(name.clone())
    } else {
        None
    }
}

pub(super) fn extract_substring_loop_slice(expr: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: s_var, .. } = object.as_ref() else {
        return None;
    };
    let ASTNode::Variable { name: i_var, .. } = &arguments[0] else {
        return None;
    };
    if i_var != loop_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = &arguments[1]
    else {
        return None;
    };
    let ASTNode::Variable { name: left_var, .. } = left.as_ref() else {
        return None;
    };
    if left_var != loop_var {
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
    Some(s_var.clone())
}

pub(super) fn extract_indexof_expr(expr: &ASTNode) -> Option<(String, String)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "indexOf" || arguments.len() != 1 {
        return None;
    }
    let ASTNode::Variable {
        name: digits_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    let ASTNode::Variable { name: ch_var, .. } = &arguments[0] else {
        return None;
    };
    Some((digits_var.clone(), ch_var.clone()))
}
