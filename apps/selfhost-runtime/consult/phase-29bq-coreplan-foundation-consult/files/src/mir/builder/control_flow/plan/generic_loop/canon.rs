//! Analysis-only canonical views for generic loop facts (no rewrite).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConditionCanon {
    pub loop_var_candidates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UpdateCanon {
    pub op: BinaryOperator,
    pub step: i64,
    pub commuted: bool,
}

pub(crate) fn canon_condition_for_generic_loop_v0(
    condition: &ASTNode,
) -> Option<ConditionCanon> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    if !matches!(
        operator,
        BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
    ) {
        return None;
    }

    let mut candidates = Vec::new();
    if let Some(name) = extract_var_candidate(left) {
        candidates.push(name);
    }
    if let Some(name) = extract_var_candidate(right) {
        if !candidates.iter().any(|v| v == &name) {
            candidates.push(name);
        }
    }

    if candidates.is_empty() {
        return None;
    }
    Some(ConditionCanon {
        loop_var_candidates: candidates,
    })
}

fn extract_var_candidate(expr: &ASTNode) -> Option<String> {
    match expr {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::Add | BinaryOperator::Subtract => {
                if let (ASTNode::Variable { name, .. }, ASTNode::Literal { .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    return Some(name.clone());
                }
                if let (ASTNode::Literal { .. }, ASTNode::Variable { name, .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    return Some(name.clone());
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn canon_update_for_loop_var(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<UpdateCanon> {
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
        BinaryOperator::Add => {
            if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { value, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if lname == loop_var {
                    return literal_step(operator, value, false);
                }
            }
            if let (ASTNode::Literal { value, .. }, ASTNode::Variable { name: rname, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if rname == loop_var {
                    return literal_step(operator, value, true);
                }
            }
        }
        BinaryOperator::Subtract | BinaryOperator::Divide => {
            if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { value, .. }) =
                (left.as_ref(), right.as_ref())
            {
                if lname == loop_var {
                    return literal_step(operator, value, false);
                }
            }
        }
        _ => {}
    }

    None
}

fn literal_step(
    op: &BinaryOperator,
    value: &LiteralValue,
    commuted: bool,
) -> Option<UpdateCanon> {
    let LiteralValue::Integer(step) = value else {
        return None;
    };
    Some(UpdateCanon {
        op: op.clone(),
        step: *step,
        commuted,
    })
}
