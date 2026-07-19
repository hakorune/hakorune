//! Exit-only if lowering - exit-focused if handling.
//!
//! Contains:
//! - lower_exit_only_if
//! - lower_else_only_exit_if
//! - lower_then_only_exit_if

use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_entry::lower_cond_branch;
use crate::mir::builder::control_flow::plan::recipe_tree::{IfMode, RecipeBlock, RecipeBodies};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::block::lower_exit_only_block;

// ============================================================================
// Exit-only if lowering
// ============================================================================

pub(in crate::mir::builder::control_flow::plan::parts) fn lower_exit_only_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    cond_view: &crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView,
    mode: IfMode,
    then_block: &RecipeBlock,
    else_block: Option<&RecipeBlock>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    // Mode contract (fail-fast)
    if matches!(mode, IfMode::ExitAll) && else_block.is_none() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
            error_prefix
        ));
    }
    if matches!(mode, IfMode::ExitIf) && else_block.is_some() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_if_forbids_else: ctx={}",
            error_prefix
        ));
    }

    // Save state at if entry
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Lower then branch
    let then_plans = lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        then_block,
        error_prefix,
    )?;

    // Reset to pre-if state
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    // Lower else branch (ExitAll only)
    let else_plans = match mode {
        IfMode::ExitAll => {
            let eb = else_block.ok_or_else(|| {
                format!(
                    "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
                    error_prefix
                )
            })?;
            Some(lower_exit_only_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                eb,
                error_prefix,
            )?)
        }
        IfMode::ExitIf => None,
        IfMode::ElseOnlyExit => {
            // ElseOnlyExit is handled by lower_else_only_exit_if, not this function
            return Err(format!(
                "[freeze:contract][recipe] else_only_exit_not_in_exit_only_if: ctx={}",
                error_prefix
            ));
        }
        IfMode::ThenOnlyExit => {
            // ThenOnlyExit is handled by lower_then_only_exit_if, not this function
            return Err(format!(
                "[freeze:contract][recipe] then_only_exit_not_in_exit_only_if: ctx={}",
                error_prefix
            ));
        }
    };

    // Reset to pre-if state for condition
    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    // Build if plan (no joins for exit-only)
    lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        Vec::new(),
        error_prefix,
    )
}

// ============================================================================
// Else-only exit if lowering
// ============================================================================

/// Lower an if where then=fallthrough (no exit), else=exit-only.
///
/// Contract:
/// - then_block: exit-allowed (may fall through)
/// - else_block: exit-only (must exit)
/// - After if: state is from then branch (else exits, no join needed)
pub(in crate::mir::builder::control_flow::plan::parts) fn lower_else_only_exit_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    cond_view: &crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView,
    then_block: &RecipeBlock,
    else_block: Option<&RecipeBlock>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    // Contract: else_block must be present for ElseOnlyExit
    let else_block = else_block.ok_or_else(|| {
        format!(
            "[freeze:contract][recipe] else_only_exit_requires_else: ctx={}",
            error_prefix
        )
    })?;

    // Save state at if entry
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Lower then branch (exit-allowed, may fall through)
    let then_plans = super::block::lower_exit_allowed_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        then_block,
        error_prefix,
    )?;

    // Capture then's final state (this continues after the if)
    let then_map = builder.variable_ctx.variable_map.clone();
    let then_bindings = current_bindings.clone();

    // Reset to pre-if state for else branch
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    // Lower else branch (exit-only, must exit)
    let else_plans = lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        else_block,
        error_prefix,
    )?;

    // Reset to pre-if state for condition lowering
    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    // Build if plan (no joins since else exits)
    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        Some(else_plans),
        Vec::new(),
        error_prefix,
    )?;

    // After the if, state is from the then branch (else exits)
    builder.variable_ctx.variable_map = then_map;
    *current_bindings = then_bindings;

    Ok(plans)
}

// ============================================================================
// Then-only exit if lowering
// ============================================================================

/// Lower an if where then=exit-only and else=fallthrough.
///
/// Contract:
/// - then_block: exit-only (must exit)
/// - else_block: exit-allowed (may fall through)
/// - After if: state is from else branch (then exits, no join needed)
pub(in crate::mir::builder::control_flow::plan::parts) fn lower_then_only_exit_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    cond_view: &crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView,
    then_block: &RecipeBlock,
    else_block: Option<&RecipeBlock>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let else_block = else_block.ok_or_else(|| {
        format!(
            "[freeze:contract][recipe] then_only_exit_requires_else: ctx={}",
            error_prefix
        )
    })?;

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let then_plans = lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        then_block,
        error_prefix,
    )?;

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    let else_plans = super::block::lower_exit_allowed_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        else_block,
        error_prefix,
    )?;

    let else_map = builder.variable_ctx.variable_map.clone();
    let else_bindings = current_bindings.clone();

    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        Some(else_plans),
        Vec::new(),
        error_prefix,
    )?;

    builder.variable_ctx.variable_map = else_map;
    *current_bindings = else_bindings;

    Ok(plans)
}
