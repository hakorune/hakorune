//! Real-world pattern helpers for loop_break facts.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_condition::{
    matches_eq_empty_string, matches_ge_zero,
};

pub(in crate::mir::builder::control_flow::plan) fn match_seg_if_else(
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
    let ASTNode::Variable {
        name: start_var, ..
    } = &arguments[0]
    else {
        return false;
    };
    if start_var != loop_var {
        return false;
    }

    match (&arguments[1], end_var, end_length_of) {
        (ASTNode::Variable { name, .. }, Some(var), None) => name == var,
        (
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            },
            None,
            Some(owner),
        ) => {
            if method != "length" || !arguments.is_empty() {
                return false;
            }
            matches!(object.as_ref(), ASTNode::Variable { name, .. } if name == owner)
        }
        _ => false,
    }
}

pub(in crate::mir::builder::control_flow::plan) fn match_break_if(
    stmt: &ASTNode,
    seg_var: &str,
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

pub(in crate::mir::builder::control_flow::plan) fn match_loop_increment(
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
    if !matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::Integer(v), .. } if *v == sep_len)
    {
        return None;
    }
    Some(true)
}
