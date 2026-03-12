use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::matches_loop_increment;
use crate::mir::builder::control_flow::plan::coreloop_body_contract::is_effect_only_stmt;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::{
    is_pure_value_expr_for_generic_loop, is_supported_bool_expr_for_generic_loop,
};

use super::assign::is_simple_assignment;
use super::local::is_local_decl;
use super::local::is_local_init;

/// Checks if a statement is an exit-if (if with single break/continue/return in then-body)
pub(in crate::mir::builder) fn is_exit_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }
    matches!(
        then_body[0],
        ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. }
    )
}

/// Checks if a statement is an effect-if (if with effect-only statements)
pub(in crate::mir::builder) fn is_effect_if(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if then_body.is_empty() {
        return false;
    }
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }

    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        return then_body
            .iter()
            .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment))
            && else_body
                .iter()
                .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment));
    }

    let last_is_continue = matches!(then_body.last(), Some(ASTNode::Continue { .. }));
    let mut saw_increment = false;

    for (idx, inner) in then_body.iter().enumerate() {
        let is_last = idx + 1 == then_body.len();
        if last_is_continue && is_last && matches!(inner, ASTNode::Continue { .. }) {
            continue;
        }
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if !last_is_continue || idx + 2 != then_body.len() || saw_increment {
                return false;
            }
            saw_increment = true;
            continue;
        }
        if is_effect_stmt(inner, loop_var, loop_increment) {
            continue;
        }
        return false;
    }

    true
}

/// Checks if a statement is a pure effect-if (no loop var/increment dependencies)
pub(in crate::mir::builder) fn is_effect_if_pure(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if then_body.is_empty() {
        return false;
    }
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        return then_body.iter().all(is_effect_only_stmt)
            && else_body.iter().all(is_effect_only_stmt);
    }
    then_body.iter().all(is_effect_only_stmt)
}

/// Checks if a statement is a break-else-effect-if pattern
pub(in crate::mir::builder) fn is_break_else_effect_if(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    if else_body.is_empty() {
        return false;
    }
    else_body
        .iter()
        .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment))
}

/// Checks if a statement is an effect statement
pub(in crate::mir::builder) fn is_effect_stmt(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if is_simple_assignment(stmt, loop_var) || is_local_decl(stmt, loop_var) {
        return true;
    }
    if is_effect_only_stmt(stmt) {
        return true;
    }
    if is_effect_if(stmt, loop_var, loop_increment) {
        return true;
    }
    if is_break_else_effect_if(stmt, loop_var, loop_increment) {
        return true;
    }
    false
}

/// Checks if a statement is a conditional update if
pub(in crate::mir::builder) fn is_conditional_update_if(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }

    let mut saw_assignment = false;
    if !is_conditional_update_branch(then_body, loop_var, &mut saw_assignment) {
        return false;
    }
    if let Some(else_body) = else_body {
        if !is_conditional_update_branch(else_body, loop_var, &mut saw_assignment) {
            return false;
        }
    }

    saw_assignment
}

fn is_conditional_update_branch(
    body: &[ASTNode],
    loop_var: &str,
    saw_assignment: &mut bool,
) -> bool {
    let mut saw_exit = false;
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                if name == loop_var {
                    return false;
                }
                if !is_pure_value_expr_for_generic_loop(value) {
                    return false;
                }
                *saw_assignment = true;
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last {
                    return false;
                }
                if saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    true
}

/// Checks if a statement is a general if (recursive, depth-limited)
pub(in crate::mir::builder) fn is_general_if_full(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
    depth: usize,
) -> bool {
    if depth > 3 {
        return false;
    }
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !is_supported_bool_expr_for_generic_loop(condition) {
        return false;
    }
    if then_body.is_empty() {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        return body_is_general_if_branch(then_body, loop_var, loop_increment, depth + 1)
            && body_is_general_if_branch(else_body, loop_var, loop_increment, depth + 1);
    }
    body_is_general_if_branch(then_body, loop_var, loop_increment, depth + 1)
}

fn body_is_general_if_branch(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    depth: usize,
) -> bool {
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            return false;
        }
        if is_simple_assignment(stmt, loop_var) {
            continue;
        }
        if is_local_init(stmt, loop_var) {
            continue;
        }
        if is_effect_only_stmt(stmt) {
            continue;
        }
        if matches!(
            stmt,
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. }
        ) {
            continue;
        }
        if is_general_if_full(stmt, loop_var, loop_increment, depth) {
            continue;
        }
        return false;
    }
    true
}

/// Checks if body has any continue statement (recursive in if/scope)
pub(in crate::mir::builder) fn body_has_continue(body: &[ASTNode]) -> bool {
    body.iter().any(stmt_has_continue)
}

fn stmt_has_continue(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_continue)
                || else_body
                    .as_ref()
                    .map_or(false, |body| body.iter().any(stmt_has_continue))
        }
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_continue),
        _ => false,
    }
}
