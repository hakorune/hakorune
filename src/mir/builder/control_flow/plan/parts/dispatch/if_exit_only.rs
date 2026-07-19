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

#[derive(Clone, Copy)]
pub(in crate::mir::builder::control_flow::plan::parts) enum ExitIfBranchV1 {
    Then,
    Else,
}

#[derive(Clone, Copy)]
pub(in crate::mir::builder::control_flow::plan::parts) enum ExitIfStatePolicyV1 {
    ExitOnly(IfMode),
    ElseOnlyExit,
    ThenOnlyExit,
}

/// Carrier-neutral state owner for exit-bearing `if` lowering.
///
/// Branch/source carriers stay with the caller. This core owns only the
/// snapshot/reset/continuing-side restoration transaction and delegates
/// condition materialization through one callback.
pub(in crate::mir::builder::control_flow::plan::parts) fn lower_exit_if_state_core<
    LowerBranch,
    LowerCondition,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    policy: ExitIfStatePolicyV1,
    has_else: bool,
    error_prefix: &str,
    mut lower_branch: LowerBranch,
    mut lower_condition: LowerCondition,
) -> Result<Vec<LoweredRecipe>, String>
where
    LowerBranch: FnMut(
        ExitIfBranchV1,
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
    ) -> Result<Vec<LoweredRecipe>, String>,
    LowerCondition: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        Vec<LoweredRecipe>,
        Option<Vec<LoweredRecipe>>,
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    if matches!(policy, ExitIfStatePolicyV1::ExitOnly(IfMode::ExitAll)) && !has_else {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
            error_prefix
        ));
    }
    if matches!(policy, ExitIfStatePolicyV1::ExitOnly(IfMode::ExitIf)) && has_else {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_if_forbids_else: ctx={}",
            error_prefix
        ));
    }

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let then_plans = lower_branch(ExitIfBranchV1::Then, builder, current_bindings)?;
    let then_state = matches!(policy, ExitIfStatePolicyV1::ElseOnlyExit).then(|| {
        (
            builder.variable_ctx.variable_map.clone(),
            current_bindings.clone(),
        )
    });

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    if matches!(policy, ExitIfStatePolicyV1::ExitOnly(IfMode::ElseOnlyExit)) {
        return Err(format!(
            "[freeze:contract][recipe] else_only_exit_not_in_exit_only_if: ctx={}",
            error_prefix
        ));
    }
    if matches!(policy, ExitIfStatePolicyV1::ExitOnly(IfMode::ThenOnlyExit)) {
        return Err(format!(
            "[freeze:contract][recipe] then_only_exit_not_in_exit_only_if: ctx={}",
            error_prefix
        ));
    }

    let else_plans = match policy {
        ExitIfStatePolicyV1::ExitOnly(IfMode::ExitAll)
        | ExitIfStatePolicyV1::ElseOnlyExit
        | ExitIfStatePolicyV1::ThenOnlyExit => Some(lower_branch(
            ExitIfBranchV1::Else,
            builder,
            current_bindings,
        )?),
        ExitIfStatePolicyV1::ExitOnly(IfMode::ExitIf) => None,
        ExitIfStatePolicyV1::ExitOnly(IfMode::ElseOnlyExit | IfMode::ThenOnlyExit) => {
            unreachable!("invalid exit-only modes returned above")
        }
    };
    let else_state = matches!(policy, ExitIfStatePolicyV1::ThenOnlyExit).then(|| {
        (
            builder.variable_ctx.variable_map.clone(),
            current_bindings.clone(),
        )
    });

    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    let plans = lower_condition(builder, current_bindings, then_plans, else_plans)?;

    if let Some((continuing_map, continuing_bindings)) = then_state.or(else_state) {
        builder.variable_ctx.variable_map = continuing_map;
        *current_bindings = continuing_bindings;
    }

    Ok(plans)
}

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
    let mut lower_branch = |branch,
                            builder: &mut MirBuilder,
                            current_bindings: &mut BTreeMap<_, _>| {
        let block = match branch {
            ExitIfBranchV1::Then => then_block,
            ExitIfBranchV1::Else => else_block.expect("mode contract checked before state core"),
        };
        lower_exit_only_block(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            arena,
            block,
            error_prefix,
        )
    };
    let mut lower_condition = |builder: &mut MirBuilder,
                               current_bindings: &mut BTreeMap<_, _>,
                               then_plans,
                               else_plans| {
        lower_cond_branch(
            builder,
            current_bindings,
            cond_view,
            then_plans,
            else_plans,
            Vec::new(),
            error_prefix,
        )
    };
    lower_exit_if_state_core(
        builder,
        current_bindings,
        ExitIfStatePolicyV1::ExitOnly(mode),
        else_block.is_some(),
        error_prefix,
        &mut lower_branch,
        &mut lower_condition,
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

    let mut lower_branch =
        |branch, builder: &mut MirBuilder, current_bindings: &mut BTreeMap<_, _>| match branch {
            ExitIfBranchV1::Then => super::block::lower_exit_allowed_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                then_block,
                error_prefix,
            ),
            ExitIfBranchV1::Else => lower_exit_only_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                else_block,
                error_prefix,
            ),
        };
    let mut lower_condition = |builder: &mut MirBuilder,
                               current_bindings: &mut BTreeMap<_, _>,
                               then_plans,
                               else_plans| {
        lower_cond_branch(
            builder,
            current_bindings,
            cond_view,
            then_plans,
            else_plans,
            Vec::new(),
            error_prefix,
        )
    };
    lower_exit_if_state_core(
        builder,
        current_bindings,
        ExitIfStatePolicyV1::ElseOnlyExit,
        true,
        error_prefix,
        &mut lower_branch,
        &mut lower_condition,
    )
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

    let mut lower_branch =
        |branch, builder: &mut MirBuilder, current_bindings: &mut BTreeMap<_, _>| match branch {
            ExitIfBranchV1::Then => lower_exit_only_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                then_block,
                error_prefix,
            ),
            ExitIfBranchV1::Else => super::block::lower_exit_allowed_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                else_block,
                error_prefix,
            ),
        };
    let mut lower_condition = |builder: &mut MirBuilder,
                               current_bindings: &mut BTreeMap<_, _>,
                               then_plans,
                               else_plans| {
        lower_cond_branch(
            builder,
            current_bindings,
            cond_view,
            then_plans,
            else_plans,
            Vec::new(),
            error_prefix,
        )
    };
    lower_exit_if_state_core(
        builder,
        current_bindings,
        ExitIfStatePolicyV1::ThenOnlyExit,
        true,
        error_prefix,
        &mut lower_branch,
        &mut lower_condition,
    )
}
