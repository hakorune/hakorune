mod call;
mod compare;
mod control;
mod literal;

pub(in crate::mir::builder) use call::*;
pub(in crate::mir::builder) use compare::*;
pub(in crate::mir::builder) use control::*;
pub(in crate::mir::builder) use literal::*;

use crate::ast::{ASTNode, BinaryOperator};

/// Matches trim condition with method call pattern.
pub(in crate::mir::builder) fn matches_trim_cond_with_methodcall(
    condition: &ASTNode,
    loop_var: &str,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };
    (matches_loop_var_compare(left.as_ref(), loop_var)
        && matches_is_space_call(right.as_ref(), loop_var))
        || (matches_loop_var_compare(right.as_ref(), loop_var)
            && matches_is_space_call(left.as_ref(), loop_var))
}

/// Matches `var = var + literal` assignment pattern.
pub(in crate::mir::builder) fn matches_assignment_add_literal(
    stmt: &ASTNode,
    var: &str,
    literal: i64,
) -> bool {
    use crate::ast::LiteralValue;
    let ASTNode::Assignment {
        target,
        value: assign_value,
        ..
    } = stmt
    else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != var {
        return false;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = assign_value.as_ref()
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == var);
    let is_literal = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::Literal {
                value: LiteralValue::Integer(v),
                ..
            } if *v == literal
        )
    };
    (is_loop_var(left.as_ref()) && is_literal(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_literal(left.as_ref()))
}
