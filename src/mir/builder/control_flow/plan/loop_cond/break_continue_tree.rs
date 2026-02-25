//! ExitIfTree recipe builder for loop_cond_break_continue facts extraction.
//!
//! This module handles the `ExitIfTree` pattern: nested if-in-loop with exit on all branches.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    ExitOnlyBlockRecipe, try_build_exit_only_block_recipe,
};
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use super::break_continue_recipe::LoopCondBreakContinueItem;
use crate::mir::builder::control_flow::plan::recipe_tree::common::IfMode;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;

/// Build an ExitIfTree item from an if statement (recursive).
/// - Returns Some(item) if the if statement is an exit-if tree (all branches end with exit)
/// - Returns None if the pattern doesn't match (fallback to other patterns)
pub(super) fn build_exit_if_tree_recipe(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    stmt_idx: usize,
    allow_extended: bool,
) -> Option<LoopCondBreakContinueItem> {
    // Condition must be a supported bool expr (Facts SSOT).
    if !is_supported_bool_expr_with_canon(condition, allow_extended) {
        return None;
    }

    // then_body must be an exit-only RecipeBlock and must end with exit on all paths.
    let then_block: ExitOnlyBlockRecipe =
        try_build_exit_only_block_recipe(then_body, allow_extended)?;
    if !then_block.ends_with_exit_on_all_paths() {
        return None;
    }

    // else_body (if present) must also be exit-only and must end with exit on all paths.
    let else_block: Option<ExitOnlyBlockRecipe> = if let Some(else_body) = else_body {
        let block = try_build_exit_only_block_recipe(else_body, allow_extended)?;
        if !block.ends_with_exit_on_all_paths() {
            return None;
        }
        Some(block)
    } else {
        None
    };

    let mode = if else_body.is_some() {
        debug_assert!(else_block.is_some());
        IfMode::ExitAll
    } else {
        IfMode::ExitIf
    };

    Some(LoopCondBreakContinueItem::ExitIfTree {
        if_stmt: StmtRef::new(stmt_idx),
        cond_view: CondBlockView::from_expr(condition),
        mode,
        then_body: then_block,
        else_body: else_block,
    })
}
