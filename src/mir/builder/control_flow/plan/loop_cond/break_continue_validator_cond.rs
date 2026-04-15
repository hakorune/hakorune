//! Conditional update pattern validators for loop_cond_break_continue facts extraction.
//!
//! These validators check for conditional update patterns: if statements that contain
//! assignments/locals with optional exit at the end of the branch.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::flatten_stmt_list;
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::prelude_has_loop_like_stmt;
use crate::mir::builder::control_flow::plan::recipe_tree::common::ExitKind;

/// Check if an if statement is a conditional update pattern.
///
/// A conditional update pattern contains assignments/locals with an optional
/// exit (break/continue) at the end of the branch.
pub(in super::super) fn is_conditional_update_if(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_extended: bool,
) -> bool {
    // ConditionalUpdate lowering currently lowers condition prelude as effects-only.
    // Loop-like prelude stmts require branch-plan lowering, so keep this shape out-of-scope
    // for ConditionalUpdate and let `GeneralIf` handle it.
    let cond_view = CondBlockView::from_expr(condition);
    if prelude_has_loop_like_stmt(&cond_view.prelude_stmts) {
        return false;
    }

    if !is_supported_bool_expr_with_canon(condition, allow_extended) {
        return false;
    }

    let mut saw_assignment = false;
    let flat_then = flatten_stmt_list(then_body);
    if !is_conditional_update_branch_view(&flat_then, &mut saw_assignment, allow_extended) {
        return false;
    }
    if let Some(else_body) = else_body {
        let flat_else = flatten_stmt_list(else_body);
        if !is_conditional_update_branch_view(&flat_else, &mut saw_assignment, allow_extended) {
            return false;
        }
    }

    saw_assignment
}

fn is_conditional_update_branch_view(
    body: &[&ASTNode],
    saw_assignment: &mut bool,
    allow_extended: bool,
) -> bool {
    let mut saw_exit = false;
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
                if !is_conditional_update_value_expr(value, allow_extended) {
                    return false;
                }
                *saw_assignment = true;
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if !is_allowed_local_inits(variables, initial_values, allow_extended) {
                    return false;
                }
                *saw_assignment = true;
            }
            ASTNode::If { .. } => return false,
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last || saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    true
}

/// Build a recipe for a conditional update branch.
///
/// Returns (body_recipe, exit_kind) where:
/// - body_recipe is a NoExitBlockRecipe for the statements before the exit
/// - exit_kind is the exit statement if present (break/continue)
pub(in super::super) fn build_conditional_update_branch_recipe(
    body: &[ASTNode],
    allow_extended: bool,
) -> Option<(Option<NoExitBlockRecipe>, Option<ExitKind>)> {
    let mut flat: Vec<ASTNode> = flatten_stmt_list(body)
        .into_iter()
        .map(|stmt| stmt.clone())
        .collect();

    let mut exit = None;
    match flat.last() {
        Some(ASTNode::Break { .. }) => {
            exit = Some(ExitKind::Break { depth: 1 });
            flat.pop();
        }
        Some(ASTNode::Continue { .. }) => {
            exit = Some(ExitKind::Continue { depth: 1 });
            flat.pop();
        }
        _ => {}
    }

    if flat.is_empty() {
        return Some((None, exit));
    }

    // Contract:
    // - This builder is called only after `is_conditional_update_if(...)` matched.
    // - Therefore, `flat` must contain only assignment/local updates (no if/loops/prints/calls).
    // - We store the effect-stmt list as a `NoExitBlockRecipe` for recipe-first lowering.
    let recipe = try_build_no_exit_block_recipe(&flat, allow_extended)?;
    Some((Some(recipe), exit))
}

fn is_allowed_local_inits(
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    allow_extended: bool,
) -> bool {
    if variables.len() != initial_values.len() {
        return false;
    }
    for init in initial_values {
        let Some(init) = init.as_ref() else {
            return false;
        };
        if !is_conditional_update_value_expr(init, allow_extended) {
            return false;
        }
    }
    true
}

fn is_conditional_update_value_expr(ast: &ASTNode, allow_extended: bool) -> bool {
    let _ = allow_extended;
    is_pure_value_expr_for_generic_loop(ast)
}
