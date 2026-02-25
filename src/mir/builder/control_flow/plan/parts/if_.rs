//! If lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.

use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::recipe_tree::common::IfMode;
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_branch;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitOnlyBlockRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_loop_cond_exit_if_tree(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    _carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    mode: IfMode,
    then_recipe: &ExitOnlyBlockRecipe,
    else_recipe: Option<&ExitOnlyBlockRecipe>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if matches!(mode, IfMode::ExitAll) && else_recipe.is_none() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
            error_prefix
        ));
    }
    if matches!(mode, IfMode::ExitIf) && else_recipe.is_some() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_if_forbids_else_in_exit_if_tree: ctx={}",
            error_prefix
        ));
    }

    // Save state at if entry (both branches start from same point)
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // then_recipe → verify + lower exit-only block
    let then_verified = super::entry::verify_exit_only_block_with_pre(
        &then_recipe.arena,
        &then_recipe.block,
        error_prefix,
        Some(&pre_bindings),
    )?;
    let then_plans = super::entry::lower_exit_only_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        then_verified,
        error_prefix,
    )?;

    // Reset to pre-if state before lowering else
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    // else_recipe → convert to block and dispatch (ExitAll only)
    let else_plans = match mode {
        IfMode::ExitAll => {
            let else_recipe = else_recipe.ok_or_else(|| {
                format!(
                    "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
                    error_prefix
                )
            })?;
            let else_verified = super::entry::verify_exit_only_block_with_pre(
                &else_recipe.arena,
                &else_recipe.block,
                error_prefix,
                Some(&pre_bindings),
            )?;
            Some(super::entry::lower_exit_only_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                else_verified,
                error_prefix,
            )?)
        }
        IfMode::ExitIf => None,
        IfMode::ElseOnlyExit => {
            // ElseOnlyExit is handled by lower_else_only_exit_if in dispatch.rs,
            // not by this function which expects ExitOnlyBlockRecipe
            return Err(format!(
                "[freeze:contract][recipe] else_only_exit_not_allowed_in_exit_if_tree: ctx={}",
                error_prefix
            ));
        }
    };

    // Reset to pre-if state for condition evaluation
    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    // Build if plan (no join PHIs needed since all branches exit)
    lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        Vec::new(), // No join PHIs (all branches exit)
        error_prefix,
    )
}
