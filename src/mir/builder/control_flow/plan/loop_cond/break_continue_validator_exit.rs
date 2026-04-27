//! Exit-if pattern validators for loop_cond_break_continue facts extraction.
//!
//! These validators check for exit-if patterns where the then branch
//! ends with break/continue/return.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::facts::stmt_walk::flatten_stmt_list;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies, RecipeItem};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

use super::break_continue_tree::build_exit_if_tree_recipe;
use super::break_continue_validator_else::{
    is_else_only_return_if_shape, is_then_only_return_if_shape,
};
use super::break_continue_validator_prelude::{
    branch_effects_only, exit_prelude_is_allowed, exit_prelude_is_allowed_for_break,
    return_prelude_is_allowed,
};

/// Check if an if statement is an exit-if pattern.
///
/// Exit-if patterns have a then branch that ends with break/continue/return,
/// and an optional else branch that also ends with an exit.
pub(in super::super) fn is_exit_if_stmt(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if !is_supported_bool_expr_with_canon(condition, allow_return) {
        return false;
    }
    if is_exit_if_with_prelude(then_body, else_body, allow_return) {
        return true;
    }
    if is_exit_if_with_return_before_continue(then_body, else_body, allow_return) {
        return true;
    }
    if is_exit_if_with_nested_exit(then_body, else_body, allow_return) {
        return true;
    }
    if then_body.len() != 1 {
        return false;
    }
    let then_is_return = allow_return && matches!(then_body[0], ASTNode::Return { .. });
    if !matches!(
        then_body[0],
        ASTNode::Break { .. } | ASTNode::Continue { .. }
    ) && !then_is_return
    {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.len() != 1 {
            return false;
        }
        let else_is_return = allow_return && matches!(else_body[0], ASTNode::Return { .. });
        if !matches!(
            else_body[0],
            ASTNode::Break { .. } | ASTNode::Continue { .. }
        ) && !else_is_return
        {
            return false;
        }
    }
    true
}

/// Check for exit-if pattern with return before continue.
///
/// Pattern: `if cond { prelude; if cond2 { return }; continue }`
pub(in super::super) fn is_exit_if_with_return_before_continue(
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if !allow_return || else_body.is_some() {
        return false;
    }
    if then_body.len() < 2 {
        return false;
    }
    if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
        return false;
    }
    let return_if = &then_body[then_body.len() - 2];
    if !is_return_if_stmt(return_if, allow_return) {
        return false;
    }
    if then_body.len() == 2 {
        return true;
    }
    branch_effects_only(&then_body[..then_body.len() - 2], false)
}

/// Check for exit-if pattern with prelude statements.
///
/// Pattern: `if cond { prelude; break/continue/return }`
pub(in super::super) fn is_exit_if_with_prelude(
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if else_body.is_some() {
        return false;
    }
    let Some(last) = then_body.last() else {
        return false;
    };
    if matches!(last, ASTNode::Return { .. }) {
        if !allow_return {
            return false;
        }
        if then_body.len() == 1 {
            return true;
        }
        return return_prelude_is_allowed(&then_body[..then_body.len() - 1], last, allow_return);
    }
    if matches!(last, ASTNode::Break { .. } | ASTNode::Continue { .. }) {
        if !allow_return {
            return false;
        }
        if then_body.len() == 1 {
            return true;
        }
        let prelude = &then_body[..then_body.len() - 1];
        return match last {
            ASTNode::Break { .. } => exit_prelude_is_allowed_for_break(prelude, allow_return),
            ASTNode::Continue { .. } => exit_prelude_is_allowed(prelude, allow_return),
            _ => false,
        };
    }
    false
}

/// Check if a statement is an if-return pattern.
pub(in super::super) fn is_return_if_stmt(stmt: &ASTNode, allow_return: bool) -> bool {
    if !allow_return {
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
    if else_body.is_some() {
        return false;
    }
    if !is_supported_bool_expr_with_canon(condition, allow_return) {
        return false;
    }
    if then_body.len() != 1 {
        return false;
    }
    matches!(then_body[0], ASTNode::Return { .. })
}

/// Check for exit-if with nested exit pattern.
///
/// Pattern: `if cond { prelude; if inner_cond { exit } }`
pub(in super::super) fn is_exit_if_with_nested_exit(
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if else_body.is_some() {
        return false;
    }
    let Some(last) = then_body.last() else {
        return false;
    };
    let ASTNode::If {
        condition,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = last
    else {
        return false;
    };
    if !is_exit_if_stmt(condition, inner_then, inner_else.as_ref(), allow_return) {
        return false;
    }
    if then_body.len() == 1 {
        return true;
    }
    exit_prelude_is_allowed(&then_body[..then_body.len() - 1], allow_return)
}

/// Check if returns in a loop body are only within exit-if patterns.
pub(in super::super) fn returns_only_in_exit_if(body: &[ASTNode], allow_return: bool) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                // Phase 29bq BoxCount: accept `if { return } else { if { return } else { return } }` via
                // the exit-allowed recipe builder (SSOT), because lowering for this shape
                // must go through the recipe path (Parts) rather than the simple exit-if map.
                if try_build_else_nested_exit_if_return_exit_allowed_recipe(stmt, allow_return)
                    .is_some()
                {
                    continue;
                }
                // Phase 29bq BoxCount: accept ExitIfTree (both branches exit), including
                // `else { local; return }` shapes, via the same builder used by the recipe layer.
                if build_exit_if_tree_recipe(
                    condition,
                    then_body,
                    else_body.as_ref(),
                    0,
                    allow_return,
                )
                .is_some()
                {
                    continue;
                }
                // else-only return pattern
                if is_else_only_return_if_shape(
                    condition,
                    then_body,
                    else_body.as_ref(),
                    allow_return,
                ) {
                    continue;
                }
                // then-only return pattern
                if is_then_only_return_if_shape(
                    condition,
                    then_body,
                    else_body.as_ref(),
                    allow_return,
                ) {
                    continue;
                }
                if is_exit_if_stmt(condition, then_body, else_body.as_ref(), allow_return) {
                    continue;
                }
            }
            ASTNode::Program { statements, .. } => {
                if !returns_only_in_exit_if(statements, allow_return) {
                    return false;
                }
                continue;
            }
            ASTNode::ScopeBox { body, .. } => {
                if !returns_only_in_exit_if(body, allow_return) {
                    return false;
                }
                continue;
            }
            _ => {}
        }
        if stmt.contains_return_stmt() {
            return false;
        }
    }
    true
}

/// Phase 29bq BoxCount: allow `if { return } else { if { return } }` inside loop bodies.
///
/// Also accepts `if { return } else { if { return } else { return } }`.
///
/// This shape is intentionally narrow to avoid acceptance drift:
/// - outer `then` must be a single `return`
/// - outer `else` must be a single nested `if` whose `then` is a single `return`
///   and whose `else` (when present) is also a single `return`
fn is_else_nested_exit_if_return_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&[ASTNode]>,
    allow_return: bool,
) -> bool {
    if !allow_return {
        return false;
    }
    if !is_supported_bool_expr_with_canon(condition, allow_return) {
        return false;
    }
    let then_flat = flatten_stmt_list(then_body);
    if then_flat.len() != 1 || !matches!(then_flat[0], ASTNode::Return { .. }) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    let else_flat = flatten_stmt_list(else_body);
    if else_flat.len() != 1 {
        return false;
    }
    let ASTNode::If {
        condition: inner_cond,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = else_flat[0]
    else {
        return false;
    };
    if !is_supported_bool_expr_with_canon(inner_cond, allow_return) {
        return false;
    }
    let inner_then_flat = flatten_stmt_list(inner_then);
    if inner_then_flat.len() != 1 || !matches!(inner_then_flat[0], ASTNode::Return { .. }) {
        return false;
    }
    match inner_else.as_ref() {
        None => true,
        Some(inner_else) => {
            let inner_else_flat = flatten_stmt_list(inner_else);
            inner_else_flat.len() == 1 && matches!(inner_else_flat[0], ASTNode::Return { .. })
        }
    }
}

pub(in super::super) fn try_build_else_nested_exit_if_return_exit_allowed_recipe(
    stmt: &ASTNode,
    allow_return: bool,
) -> Option<ExitAllowedBlockRecipe> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };

    if !is_else_nested_exit_if_return_shape(
        condition.as_ref(),
        then_body.as_slice(),
        else_body.as_ref().map(|v| v.as_slice()),
        allow_return,
    ) {
        return None;
    }

    // Build a minimal exit-allowed recipe that lowers the underlying `if` statement via
    // `parts::stmt::lower_return_prelude_stmt` (SSOT), avoiding the exit-if map limitations.
    let mut arena = RecipeBodies::new();
    let body_id = arena.register(RecipeBody::new(vec![stmt.clone()]));
    let block = RecipeBlock::new(body_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    Some(ExitAllowedBlockRecipe { arena, block })
}
