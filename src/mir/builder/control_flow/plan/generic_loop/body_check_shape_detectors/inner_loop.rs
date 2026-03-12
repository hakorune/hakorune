use super::super::body_check::expr_matchers::{
    matches_if_else_return_literal, matches_if_else_return_literal_local,
    matches_if_else_return_literal_var, matches_if_return_literal, matches_if_return_local,
    matches_if_return_var, matches_loop_var_less_than_literal,
};
use super::utils::matches_assignment_add_literal;
use crate::ast::{ASTNode, LiteralValue};

/// Matches inner loop if-return increment pattern.
pub fn matches_inner_loop_if_return_increment(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 2 {
        return false;
    }
    if !matches_if_return_literal(&body[0], loop_var, 0) {
        return false;
    }
    matches_assignment_add_literal(&body[1], loop_var, 1)
}

/// Matches inner loop if-return-var increment pattern.
pub fn matches_inner_loop_if_return_var_increment(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 2 {
        return false;
    }
    if !matches_if_return_var(&body[0], loop_var, loop_var) {
        return false;
    }
    matches_assignment_add_literal(&body[1], loop_var, 1)
}

/// Matches inner loop if-return-local increment pattern.
pub fn matches_inner_loop_if_return_local_increment(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 2 {
        return false;
    }
    if !matches_if_return_local(&body[0], loop_var, 0) {
        return false;
    }
    matches_assignment_add_literal(&body[1], loop_var, 1)
}

/// Matches inner loop if-else-return pattern.
pub fn matches_inner_loop_if_else_return(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 2) {
        return false;
    }
    if body.len() != 1 {
        return false;
    }
    matches_if_else_return_literal(&body[0], loop_var, 0)
}

/// Matches inner loop if-else-return-var pattern.
pub fn matches_inner_loop_if_else_return_var(
    loop_stmt: &ASTNode,
    loop_var: &str,
    else_var: &str,
) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 2) {
        return false;
    }
    if body.len() != 1 {
        return false;
    }
    matches_if_else_return_literal_var(&body[0], loop_var, 0).map_or(false, |v| v == else_var)
}

/// Matches inner loop if-else-return-local pattern.
pub fn matches_inner_loop_if_else_return_local(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 2) {
        return false;
    }
    if body.len() != 1 {
        return false;
    }
    matches_if_else_return_literal_local(&body[0], loop_var, 0)
}

/// Matches inner loop if-else-if-return pattern.
pub fn matches_inner_loop_if_else_if_return(loop_stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Loop {
        condition, body, ..
    } = loop_stmt
    else {
        return false;
    };
    if !matches_loop_var_less_than_literal(condition, loop_var, 3) {
        return false;
    }
    if body.len() != 1 {
        return false;
    }
    // Note: matches_if_else_if_return_literal is defined in nested_loop_program2.rs in original.
    // However, it's a helper function. We should probably move `matches_if_else_if_return_literal` to `utils.rs` or keep it here if only used here.
    // But it's used in `nested_loop_program2` as well.
    // So let's check where `matches_if_else_if_return_literal` is defined.
    // It was defined in the main file.
    // We should move it to `inner_loop.rs` or `utils.rs`.
    // It seems closely related to `inner_loop` patterns, but also used by `nested_loop_program2`.
    // Let's assume it's available via `super::nested_loop_program2`? No that creates circular dependency.
    // Better to put shared helpers in `utils.rs` or duplicate.
    // `matches_if_else_if_return_literal` is generic enough. Let's put it in `utils.rs` or `inner_loop.rs` and make it public.
    // Since `inner_loop.rs` is using it, let's put it here and make it `pub(super)`.
    matches_if_else_if_return_literal(&body[0], loop_var, 0, 1, 0)
}

/// Matches if-else-if-return literal pattern.
pub fn matches_if_else_if_return_literal(
    stmt: &ASTNode,
    loop_var: &str,
    first_cond: i64,
    second_cond: i64,
    return_literal: i64,
) -> bool {
    use super::super::body_check::expr_matchers::matches_loop_var_equal_literal;

    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !matches_loop_var_equal_literal(condition, loop_var, first_cond) {
        return false;
    }
    if then_body.len() != 1 {
        return false;
    }
    if !matches_return_literal(&then_body[0], return_literal) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    if else_body.len() != 1 {
        return false;
    }
    let ASTNode::If {
        condition: inner_condition,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = &else_body[0]
    else {
        return false;
    };
    if !matches_loop_var_equal_literal(inner_condition, loop_var, second_cond) {
        return false;
    }
    if inner_then.len() != 1 {
        return false;
    }
    if !matches_return_literal(&inner_then[0], return_literal) {
        return false;
    }
    let Some(inner_else) = inner_else else {
        return false;
    };
    if inner_else.len() != 1 {
        return false;
    }
    matches_return_literal(&inner_else[0], return_literal)
}

/// Matches return literal pattern.
pub fn matches_return_literal(stmt: &ASTNode, literal: i64) -> bool {
    matches!(
        stmt,
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal)
    )
}
