//! Accept kind determination for loop_cond_break_continue facts extraction.
//!
//! This module determines which LoopCondBreakAcceptKind should be assigned
//! based on the analysis counters collected during recipe building.

use crate::mir::builder::control_flow::plan::extractors::common_helpers::ControlFlowCounts;
use super::break_continue_recipe::{LoopCondBreakContinueItem, LoopCondBreakContinueRecipe};
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::break_continue_types::LoopCondBreakAcceptKind;

/// Determine the accept kind based on analysis counters.
pub(super) fn determine_accept_kind(
    counts: &ControlFlowCounts,
    exit_if_seen: usize,
    continue_if_seen: usize,
    conditional_update_seen: usize,
    recipe: &LoopCondBreakContinueRecipe,
) -> Result<LoopCondBreakAcceptKind, Freeze> {
    let else_only_return_count = recipe
        .items
        .iter()
        .filter(|item| matches!(item, LoopCondBreakContinueItem::ElseOnlyReturnIf { .. }))
        .count();
    let else_only_break_count = recipe
        .items
        .iter()
        .filter(|item| {
            matches!(
                item,
                LoopCondBreakContinueItem::ElseOnlyBreakIf { .. }
                    | LoopCondBreakContinueItem::ThenOnlyBreakIf { .. }
            )
        })
        .count();
    let else_only_exit_count = else_only_return_count + else_only_break_count;
    let exit_if_only = exit_if_seen.saturating_sub(else_only_exit_count);
    let mut categories = 0usize;
    if exit_if_only > 0 {
        categories += 1;
    }
    if continue_if_seen > 0 {
        categories += 1;
    }
    if conditional_update_seen > 0 {
        categories += 1;
    }
    if else_only_return_count > 0 {
        categories += 1;
    }
    if else_only_break_count > 0 {
        categories += 1;
    }

    if categories == 0 {
        if counts.has_nested_loop {
            return Ok(LoopCondBreakAcceptKind::NestedLoopOnly);
        }
        // Check for NestedLoopOnly pattern: recipe contains nested loops but no if-break/continue patterns
        let has_nested_loop = recipe.items.iter().any(|item| {
            matches!(
                item,
                LoopCondBreakContinueItem::NestedLoopDepth1 {
                    loop_stmt: _,
                    nested: _
                }
            )
        });
        if has_nested_loop {
            return Ok(LoopCondBreakAcceptKind::NestedLoopOnly);
        }
        let has_program_block = recipe
            .items
            .iter()
            .any(|item| matches!(item, LoopCondBreakContinueItem::ProgramBlock { .. }));
        if has_program_block {
            return Ok(LoopCondBreakAcceptKind::ProgramBlockNoExit);
        }
        return Err(Freeze::bug(
            "loop_cond_break_continue: accept_kind missing (no categories)",
        ));
    }
    if categories > 1 {
        return Ok(LoopCondBreakAcceptKind::MixedIf);
    }
    if else_only_return_count > 0 {
        return Ok(LoopCondBreakAcceptKind::ElseOnlyReturn);
    }
    if else_only_break_count > 0 {
        return Ok(LoopCondBreakAcceptKind::ElseOnlyBreak);
    }
    if exit_if_only > 0 {
        let no_break_or_continue = counts.break_count == 0 && counts.continue_count == 0;
        if no_break_or_continue && counts.return_count > 0 {
            return Ok(LoopCondBreakAcceptKind::ReturnOnlyBody);
        }
        if counts.return_count > 0 {
            return Ok(LoopCondBreakAcceptKind::ReturnInExitIf);
        }
        return Ok(LoopCondBreakAcceptKind::ExitIf);
    }
    if continue_if_seen > 0 {
        return Ok(LoopCondBreakAcceptKind::ContinueIf);
    }
    if conditional_update_seen > 0 {
        return Ok(LoopCondBreakAcceptKind::ConditionalUpdate);
    }
    Err(Freeze::bug(
        "loop_cond_break_continue: accept_kind missing (unreachable)",
    ))
}
