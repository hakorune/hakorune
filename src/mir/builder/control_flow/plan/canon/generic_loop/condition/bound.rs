use crate::ast::{ASTNode, LiteralValue};
use crate::mir::policies::BoundExpr;

use super::candidates::{extract_var_candidate, is_supported_comparison_operator};

pub(super) fn extract_bound_from_condition(
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
