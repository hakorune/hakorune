use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(in crate::mir::builder) fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

pub(in crate::mir::builder) fn is_int_lit(ast: &ASTNode, value: i64) -> bool {
    matches!(ast, ASTNode::Literal { value: LiteralValue::Integer(v), .. } if *v == value)
}

pub(in crate::mir::builder) fn is_var_plus_one(ast: &ASTNode, var: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } if as_var_name(left.as_ref()) == Some(var) && is_int_lit(right.as_ref(), 1)
    )
}

pub(in crate::mir::builder) fn is_var_plus_expr(ast: &ASTNode, var: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            ..
        } if as_var_name(left.as_ref()) == Some(var)
    )
}

pub(in crate::mir::builder) fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_name(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}
