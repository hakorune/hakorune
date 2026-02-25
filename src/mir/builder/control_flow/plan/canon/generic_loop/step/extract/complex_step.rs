use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(super) fn extract_complex_step_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    for stmt in body {
        let ASTNode::Assignment { target, value, .. } = stmt else {
            continue;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            continue;
        };
        if name != loop_var {
            continue;
        }
        let ASTNode::BinaryOp {
            operator: BinaryOperator::Divide,
            left,
            right,
            ..
        } = value.as_ref()
        else {
            continue;
        };
        let ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: sub_left,
            right: sub_right,
            ..
        } = left.as_ref()
        else {
            continue;
        };
        let ASTNode::Variable { name: sub_name, .. } = sub_left.as_ref() else {
            continue;
        };
        if sub_name != loop_var {
            continue;
        }
        if !matches!(
            sub_right.as_ref(),
            ASTNode::Variable { .. } | ASTNode::Literal { .. }
        ) {
            continue;
        }
        if !matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(_),
                ..
            }
        ) {
            continue;
        }
        return Some(value.as_ref().clone());
    }
    None
}
