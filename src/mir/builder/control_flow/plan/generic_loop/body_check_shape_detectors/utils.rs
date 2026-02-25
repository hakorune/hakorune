use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Matches loop variable assignment to a specific variable.
pub fn matches_loop_var_assign_to(stmt: &ASTNode, loop_var: &str, value_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    let ASTNode::Variable { name, .. } = value.as_ref() else {
        return false;
    };
    name == value_var
}

/// Matches loop variable assignment to any of a set of variables.
pub fn matches_loop_var_assign_any(
    stmt: &ASTNode,
    loop_var: &str,
    value_vars: &[String],
) -> bool {
    if value_vars.is_empty() {
        return false;
    }
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    let ASTNode::Variable { name, .. } = value.as_ref() else {
        return false;
    };
    value_vars.iter().any(|var| var == name)
}

/// Matches `next <= loop_var` pattern.
pub fn matches_next_le_loop_var(condition: &ASTNode, loop_var: &str, next_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == next_var)
        && matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
}

/// Check if expression is `loop_var + 1` (commutative).
pub fn is_loop_var_plus_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    let is_one = |node: &ASTNode| {
        matches!(node, ASTNode::Literal { value: LiteralValue::Integer(1), .. })
    };
    (is_loop_var(left.as_ref()) && is_one(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_one(left.as_ref()))
}

/// Check if expression is `loop_var - 1`.
pub fn is_loop_var_minus_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    matches!(
        (left.as_ref(), right.as_ref()),
        (
            node_left,
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        ) if is_loop_var(node_left)
    )
}

/// Matches `loop_var % 10` pattern.
pub fn matches_mod_ten_of_loop_var(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Modulo,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
        && matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(10),
                ..
            }
        )
}

/// Matches assignment with literal addition pattern.
pub fn matches_assignment_add_literal(stmt: &ASTNode, var: &str, literal: i64) -> bool {
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

/// Matches debug guard block pattern.
pub fn matches_debug_guard_block(block: &ASTNode, loop_var: &str, next_var: &str) -> bool {
    let ASTNode::Program { statements, .. } = block else {
        return false;
    };
    if statements.len() != 1 {
        return false;
    }
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = &statements[0]
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    let ASTNode::If {
        condition,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = &then_body[0]
    else {
        return false;
    };
    if inner_else.is_some() {
        return false;
    }
    if !matches_next_le_loop_var(condition, loop_var, next_var) {
        return false;
    }
    inner_then
        .iter()
        .any(ASTNode::contains_non_local_exit_outside_loops)
}
