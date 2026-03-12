use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Matches `loop_var < literal` pattern (commutative).
pub(in crate::mir::builder) fn matches_loop_var_less_than_literal(
    expr: &ASTNode,
    loop_var: &str,
    value: i64,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    let is_literal = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::Literal {
                value: LiteralValue::Integer(v),
                ..
            } if *v == value
        )
    };
    (is_loop_var(left.as_ref()) && is_literal(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_literal(left.as_ref()))
}

/// Matches `loop_var == literal` pattern (commutative).
pub(in crate::mir::builder) fn matches_loop_var_equal_literal(
    expr: &ASTNode,
    loop_var: &str,
    value: i64,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    let is_literal = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::Literal {
                value: LiteralValue::Integer(v),
                ..
            } if *v == value
        )
    };
    (is_loop_var(left.as_ref()) && is_literal(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_literal(left.as_ref()))
}

/// Matches loop variable comparison pattern.
pub(in crate::mir::builder) fn matches_loop_var_compare(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_rel = matches!(
        operator,
        BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
    );
    if !is_rel {
        return false;
    }
    let is_var = |node: &ASTNode| matches!(node, ASTNode::Variable { .. });
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    (is_loop_var(left.as_ref()) && is_var(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_var(left.as_ref()))
}
