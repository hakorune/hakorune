//! Phase 29ao P21: loop_simple_while subset policy (step-only body)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(crate) fn is_loop_simple_while_step_only_body(body: &[ASTNode], loop_var: &str) -> bool {
    if body.len() != 1 {
        return false;
    }

    let ASTNode::Assignment { target, value, .. } = &body[0] else {
        return false;
    };

    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
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

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }

    matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    )
}
