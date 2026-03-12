use super::super::body_check::expr_matchers::{
    matches_if_else_return_literal, matches_if_else_return_literal_local,
    matches_if_else_return_literal_var, matches_if_return_literal, matches_if_return_var,
    matches_local_init_literal, matches_loop_var_less_than_literal,
};
use super::inner_loop::*;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::matches_loop_increment;

/// Matches parse program2 nested loop if-return pattern.
pub fn matches_parse_program2_nested_loop_if_return_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() == 2
        && matches_if_return_literal(&body[0], loop_var, 0)
        && matches_loop_increment(&body[1], loop_var, loop_increment)
    {
        return true;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 0) else {
        return false;
    };
    if !matches_inner_loop_if_return_increment(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-else-return pattern.
pub fn matches_parse_program2_nested_loop_if_else_return_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 1) else {
        return false;
    };
    if !matches_inner_loop_if_else_return(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-return-var pattern.
pub fn matches_parse_program2_nested_loop_if_return_var_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() == 2
        && matches_if_return_var(&body[0], loop_var, loop_var)
        && matches_loop_increment(&body[1], loop_var, loop_increment)
    {
        return true;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 0) else {
        return false;
    };
    if !matches_inner_loop_if_return_var_increment(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-return-local pattern.
pub fn matches_parse_program2_nested_loop_if_return_local_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 0) else {
        return false;
    };
    if !matches_inner_loop_if_return_local_increment(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-else-return-var pattern.
pub fn matches_parse_program2_nested_loop_if_else_return_var_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 4 {
        return false;
    }
    let Some(return_var) = matches_local_init_literal(&body[0], loop_var, 0) else {
        return false;
    };
    let Some(inner_var) = matches_local_init_literal(&body[1], loop_var, 1) else {
        return false;
    };
    if !matches_inner_loop_if_else_return_var(&body[2], &inner_var, &return_var) {
        return false;
    }
    matches_loop_increment(&body[3], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-else-return-local pattern.
pub fn matches_parse_program2_nested_loop_if_else_return_local_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 1) else {
        return false;
    };
    if !matches_inner_loop_if_else_return_local(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-else-if-return pattern.
pub fn matches_parse_program2_nested_loop_if_else_if_return_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    if !matches_loop_var_less_than_literal(condition, loop_var, 1) {
        return false;
    }
    if body.len() != 3 {
        return false;
    }
    let Some(inner_var) = matches_local_init_literal(&body[0], loop_var, 2) else {
        return false;
    };
    if !matches_inner_loop_if_else_if_return(&body[1], &inner_var) {
        return false;
    }
    matches_loop_increment(&body[2], loop_var, loop_increment)
}

/// Matches parse program2 nested loop if-else-return literal pattern (simplified).
pub fn matches_parse_program2_nested_loop_if_else_return_literal(
    body: &[ASTNode],
    loop_var: &str,
) -> bool {
    body.len() == 1 && matches_if_else_return_literal(&body[0], loop_var, 0)
}

/// Matches parse program2 nested loop if-else-return var-local pattern.
pub fn matches_parse_program2_nested_loop_if_else_return_var_local(
    body: &[ASTNode],
    loop_var: &str,
) -> bool {
    if body.len() != 2 {
        return false;
    }
    let Some(_return_var) = matches_local_init_literal(&body[0], loop_var, 0) else {
        return false;
    };
    matches_if_else_return_literal_var(&body[1], loop_var, 0).is_some()
}

/// Matches parse program2 nested loop if-else-if-return pattern (simplified).
pub fn matches_parse_program2_nested_loop_if_else_if_return(
    body: &[ASTNode],
    loop_var: &str,
) -> bool {
    if body.len() != 1 {
        return false;
    }
    matches_if_else_if_return_literal(&body[0], loop_var, 0, 1, 0)
}

/// Matches parse program2 nested loop if-else-if-else-return pattern (simplified).
pub fn matches_parse_program2_nested_loop_if_else_if_else_return(
    body: &[ASTNode],
    _loop_var: &str,
) -> bool {
    if body.len() != 1 {
        return false;
    }
    // Check for nested if-else-if-else pattern
    let ASTNode::If { .. } = &body[0] else {
        return false;
    };
    // This is a simplified check - the full pattern would require deeper inspection
    false
}

/// Matches parse program2 nested loop if-else-return literal-local pattern (simplified).
pub fn matches_parse_program2_nested_loop_if_else_return_literal_local(
    body: &[ASTNode],
    loop_var: &str,
) -> bool {
    if body.len() != 1 {
        return false;
    }
    matches_if_else_return_literal_local(&body[0], loop_var, 0)
}
