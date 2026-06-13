use crate::ast::ASTNode;

use super::UpdateLiteralMatch;

pub(super) fn match_update_literal(stmt: &ASTNode, loop_var: &str) -> Option<UpdateLiteralMatch> {
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
        operator,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    match operator {
        crate::ast::BinaryOperator::Add => {
            if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { value, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if lname == loop_var {
                    return Some(UpdateLiteralMatch {
                        op: operator.clone(),
                        literal: value.clone(),
                        commuted: false,
                    });
                }
            }
            if let (ASTNode::Literal { value, .. }, ASTNode::Variable { name: rname, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if rname == loop_var {
                    return Some(UpdateLiteralMatch {
                        op: operator.clone(),
                        literal: value.clone(),
                        commuted: true,
                    });
                }
            }
        }
        crate::ast::BinaryOperator::Subtract | crate::ast::BinaryOperator::Divide => {
            if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { value, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if lname == loop_var {
                    return Some(UpdateLiteralMatch {
                        op: operator.clone(),
                        literal: value.clone(),
                        commuted: false,
                    });
                }
            }
        }
        _ => {}
    }
    None
}
