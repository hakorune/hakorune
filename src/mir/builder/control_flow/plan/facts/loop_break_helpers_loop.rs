//! Loop variable and increment helpers for loop_break extraction.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(in crate::mir::builder::control_flow::plan) fn extract_loop_var_for_len_condition(
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
pub(in crate::mir::builder::control_flow::plan) fn extract_loop_var_for_plan_subset(
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

pub(in crate::mir::builder::control_flow::plan) fn extract_loop_increment_at_end(
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
