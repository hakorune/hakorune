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
