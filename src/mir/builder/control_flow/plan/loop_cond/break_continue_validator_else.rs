//! Else-only pattern validators for loop_cond_break_continue facts extraction.
//!
//! These validators check for patterns where exit statements appear only in
//! the else branch, not in the then branch.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use super::break_continue_recipe::{LoopCondBreakContinueItem, LoopCondBreakContinueRecipe};
use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

use super::break_continue_validator_prelude::then_only_return_prelude_is_allowed_local_then_return_value;
use super::break_continue_helpers::{body_has_any_exit, branch_has_exit_or_loop};

/// Check if-else pattern: then=non-exit body, else=single return
/// Box-local (does not touch shared is_exit_if_stmt)
/// Includes condition validation (is_supported_bool_expr_with_canon required)
pub(in super::super) fn is_else_only_return_if_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if !allow_return {
        return false;
    }
    // Condition validation (required)
    if !is_supported_bool_expr_with_canon(condition, allow_return) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    // then is normal stmt only (no exit)
    if body_has_any_exit(then_body) {
        return false;
    }
    // else is:
    // - `return <value>`
    // - `print(<expr>); return <value>` (allow_extended only)
    // - `local t = <pure>; return t`
    match else_body.as_slice() {
        [ASTNode::Return { value: Some(_), .. }] => true,
        [ASTNode::Print { .. }, ASTNode::Return { value: Some(_), .. }] => true,
        [prelude @ ASTNode::Local { variables, .. }, ASTNode::Return { value: Some(value), .. }]
            if matches!(value.as_ref(), ASTNode::Variable { name, .. } if variables.len() == 1 && variables[0] == *name)
                && then_only_return_prelude_is_allowed_local_then_return_value(
                    std::slice::from_ref(prelude),
                    value,
                    allow_return,
                ) =>
        {
            true
        }
        _ => false,
    }
}

/// Check if-else pattern: then=single return, else=non-exit body
/// Box-local (does not touch shared is_exit_if_stmt)
/// Includes condition validation (is_supported_bool_expr_with_canon required)
pub(in super::super) fn is_then_only_return_if_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_return: bool,
) -> bool {
    if !allow_return {
        return false;
    }
    // Condition validation (required)
    if !is_supported_bool_expr_with_canon(condition, allow_return) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    // then is a single return, optionally preceded by a single local-init that returns that local:
    // - `return <expr>`
    // - `local t = <pure>; return <value>`
    let then_has_supported_return = match then_body {
        [ASTNode::Return { value: Some(_), .. }] => true,
        [prelude, ASTNode::Return { value: Some(value), .. }]
            if then_only_return_prelude_is_allowed_local_then_return_value(
                std::slice::from_ref(prelude),
                value,
                allow_return,
            ) =>
        {
            true
        }
        _ => false,
    };
    if !then_has_supported_return {
        return false;
    }
    // else is normal stmt only (no exit, including nested returns/break/continue)
    !branch_has_exit_or_loop(else_body)
}

/// Phase 29bq: Check for else-only-break pattern.
/// Pattern: if cond { non-exit } else { break }
/// Used for loops like: loop(cond) { if ch == " " { e = e - 1 } else { break } }
///
/// Note: This pattern is always enabled regardless of allow_extended mode.
/// The else-only-break pattern is well-defined and safe to support in all modes.
pub(in super::super) fn is_else_only_break_if_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    _allow_extended: bool,
) -> bool {
    // Condition must be supported - always use extended=true for this pattern
    // since else-only-break is a well-defined pattern that should work with
    // the full range of conditions (OR chains, etc.)
    if !is_supported_bool_expr_with_canon(condition, true) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    // then must have no exits
    if body_has_any_exit(then_body) {
        return false;
    }
    // else must be a single break statement
    if else_body.len() != 1 {
        return false;
    }
    matches!(&else_body[0], ASTNode::Break { .. })
}

/// Phase 29bq: Check for then-only-break pattern.
/// Pattern: if cond { break } else { non-exit }
///
/// Note: This pattern is always enabled regardless of allow_extended mode.
pub(in super::super) fn is_then_only_break_if_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    _allow_extended: bool,
) -> bool {
    // Condition must be supported - always use extended=true for this pattern.
    if !is_supported_bool_expr_with_canon(condition, true) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    // then must be a single break statement
    if then_body.len() != 1 || !matches!(&then_body[0], ASTNode::Break { .. }) {
        return false;
    }
    // else must have no exits (including nested ones)
    !branch_has_exit_or_loop(else_body)
}

/// Check: if cond { non-exit } else { prelude + (if guard { break })+ + non-exit }
/// Box-local (else-guard-break pattern detection)
pub(in super::super) fn is_else_guard_break_if_shape(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_extended: bool,
) -> bool {
    if !allow_extended {
        return false;
    }
    if !is_supported_bool_expr_with_canon(condition, allow_extended) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };

    // then: no exits
    if body_has_any_exit(then_body) {
        return false;
    }

    // else: must have at least one guard break
    let mut saw_guard_break = false;
    for stmt in else_body {
        match stmt {
            ASTNode::If {
                condition: guard_cond,
                then_body: guard_then,
                else_body: None,
                ..
            } => {
                if guard_then.len() == 1 && matches!(&guard_then[0], ASTNode::Break { .. }) {
                    if !is_supported_bool_expr_with_canon(guard_cond, allow_extended) {
                        return false;
                    }
                    saw_guard_break = true;
                    continue;
                }
                return false; // Other if patterns not allowed
            }
            ASTNode::Local { .. } | ASTNode::Assignment { .. } => {}
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } => {}
            _ => return false,
        }
    }
    saw_guard_break
}

pub(in super::super) fn build_else_guard_break_recipes(
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    allow_nested: bool,
    allow_extended: bool,
    max_nested_loops: usize,
    debug: bool,
) -> Option<(LoopCondBreakContinueRecipe, LoopCondBreakContinueRecipe)> {
    // Build then recipe (simple body, no exits)
    let then_recipe =
        build_branch_recipe(then_body, allow_nested, allow_extended, max_nested_loops, debug)?;

    // Build else recipe - guard breaks become ExitIf items
    let else_recipe = build_else_guard_break_body_recipe(else_body)?;

    Some((then_recipe, else_recipe))
}

fn build_branch_recipe(
    body: &[ASTNode],
    allow_nested: bool,
    allow_extended: bool,
    max_nested_loops: usize,
    debug: bool,
) -> Option<LoopCondBreakContinueRecipe> {
    let mut exit_if_seen = 0usize;
    let mut continue_if_seen = 0usize;
    let mut conditional_update_seen = 0usize;
    let mut nested_seen = 0usize;
    super::break_continue_item::build_loop_cond_break_continue_recipe_inner(
        body,
        allow_nested,
        allow_extended,
        max_nested_loops,
        debug,
        &mut exit_if_seen,
        &mut continue_if_seen,
        &mut conditional_update_seen,
        &mut nested_seen,
        false,
        false,
    )
}

pub(in super::super) fn build_else_guard_break_body_recipe(body: &[ASTNode]) -> Option<LoopCondBreakContinueRecipe> {
    let recipe_body = RecipeBody::new(body.to_vec());
    let mut items = Vec::with_capacity(recipe_body.len());
    for (idx, stmt) in recipe_body.body.iter().enumerate() {
        let stmt_ref = StmtRef::new(idx);
        match stmt {
            ASTNode::If {
                then_body,
                else_body: None,
                ..
            } => {
                if then_body.len() == 1 && matches!(&then_body[0], ASTNode::Break { .. }) {
                    // Guard break -> ExitIf
                    let exit_allowed_block = try_build_exit_allowed_block_recipe(
                        std::slice::from_ref(stmt),
                        true,
                    );
                    items.push(LoopCondBreakContinueItem::exit_if_with_optional_block(
                        stmt_ref,
                        exit_allowed_block,
                    ));
                    continue;
                }
                return None;
            }
            ASTNode::Assignment { .. } | ASTNode::Local { .. } => {
                items.push(LoopCondBreakContinueItem::Stmt(stmt_ref));
            }
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } => {
                items.push(LoopCondBreakContinueItem::Stmt(stmt_ref));
            }
            _ => return None,
        }
    }
    Some(LoopCondRecipe::new(recipe_body.body, items))
}
