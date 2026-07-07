use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::policies::{BoundExpr, CmpOp};

use super::candidates::{extract_var_candidate, is_supported_comparison_operator};

pub(in crate::mir::builder) fn extract_bound_from_condition(
    condition: &ASTNode,
    candidates: &[String],
) -> Option<BoundExpr> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    if !is_supported_comparison_operator(operator) {
        return None;
    }

    if let Some(name) = extract_var_candidate(left) {
        if candidates.iter().any(|candidate| candidate == &name) {
            return bound_from_expr(right);
        }
    }
    if let Some(name) = extract_var_candidate(right) {
        if candidates.iter().any(|candidate| candidate == &name) {
            return bound_from_expr(left);
        }
    }

    None
}

pub(in crate::mir::builder) fn extract_cmp_from_condition(
    condition: &ASTNode,
    candidates: &[String],
) -> Option<CmpOp> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    if !is_supported_comparison_operator(operator) {
        return None;
    }

    if let Some(name) = extract_var_candidate(left) {
        if candidates.iter().any(|candidate| candidate == &name) {
            return cmp_from_operator(operator);
        }
    }
    if let Some(name) = extract_var_candidate(right) {
        if candidates.iter().any(|candidate| candidate == &name) {
            return cmp_from_operator(operator).and_then(invert_cmp);
        }
    }

    None
}

fn bound_from_expr(expr: &ASTNode) -> Option<BoundExpr> {
    match expr {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Some(BoundExpr::LiteralI64(*value)),
        ASTNode::Variable { name, .. } => Some(BoundExpr::Var(name.clone())),
        _ => None,
    }
}

fn cmp_from_operator(operator: &BinaryOperator) -> Option<CmpOp> {
    match operator {
        BinaryOperator::Less => Some(CmpOp::Lt),
        BinaryOperator::LessEqual => Some(CmpOp::Le),
        BinaryOperator::Greater => Some(CmpOp::Gt),
        BinaryOperator::GreaterEqual => Some(CmpOp::Ge),
        BinaryOperator::Equal => Some(CmpOp::Eq),
        BinaryOperator::NotEqual => Some(CmpOp::Ne),
        _ => None,
    }
}

fn invert_cmp(cmp: CmpOp) -> Option<CmpOp> {
    match cmp {
        CmpOp::Lt => Some(CmpOp::Gt),
        CmpOp::Le => Some(CmpOp::Ge),
        CmpOp::Gt => Some(CmpOp::Lt),
        CmpOp::Ge => Some(CmpOp::Le),
        CmpOp::Eq => Some(CmpOp::Eq),
        CmpOp::Ne => Some(CmpOp::Ne),
    }
}
