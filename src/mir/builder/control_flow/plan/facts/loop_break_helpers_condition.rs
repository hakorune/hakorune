//! Condition matching helpers for loop_break extraction.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(in crate::mir::builder::control_flow::plan) fn match_break_if_less_than_zero(
    stmt: &ASTNode,
) -> Option<String> {
    let (cond, update_opt) =
        crate::mir::builder::control_flow::plan::loop_break::facts::helpers_break_if::extract_break_if_parts(stmt)?;
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

pub(in crate::mir::builder::control_flow::plan) fn match_acc_update_mul10_plus_d(
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
