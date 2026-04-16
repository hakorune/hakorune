use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::super::loop_cond_bc::LOOP_COND_ERR;

/// Lower if-else where else has guard breaks (exit-ifs)
/// Pattern: if cond { non-exit } else { (if guard { break })+ + non-exit }
/// Recipe-first: both branches are pre-classified recipes.
/// 箱内ローカル（exit_if_map.rs には追加しない）
pub(in crate::mir::builder::control_flow::plan::features) fn lower_else_guard_break_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_no_exit: Option<&NoExitBlockRecipe>,
    then_recipe: &LoopCondBreakContinueRecipe,
    else_recipe: &LoopCondBreakContinueRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_else_guard_break_if_with_exit_allowed(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        condition,
        then_no_exit,
        then_recipe,
        None,
        else_recipe,
    )
}

pub(in crate::mir::builder::control_flow::plan::features) fn lower_else_guard_break_if_with_exit_allowed(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_no_exit: Option<&NoExitBlockRecipe>,
    then_recipe: &LoopCondBreakContinueRecipe,
    else_exit_allowed: Option<&ExitAllowedBlockRecipe>,
    else_recipe: &LoopCondBreakContinueRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let cond_view = CondBlockView::from_expr(condition);

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            if let Some(then_no_exit) = then_no_exit {
                return parts::entry::lower_no_exit_block(
                    builder,
                    bindings,
                    carrier_step_phis,
                    Some(break_phi_dsts),
                    &then_no_exit.arena,
                    &then_no_exit.block,
                    LOOP_COND_ERR,
                );
            }

            // Legacy fallback: lower `then_recipe` by items, but forbid exits.
            let mut carrier_updates = BTreeMap::new();
            let mut block_plans = Vec::new();
            for item in &then_recipe.items {
                let mut plans = super::super::loop_cond_bc_item::lower_loop_cond_item(
                    builder,
                    bindings,
                    carrier_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    &mut carrier_updates,
                    &then_recipe.body,
                    item,
                    false, // No carrier propagation inside blocks
                )?;
                if plans.iter().any(|plan| {
                    matches!(plan, CorePlan::Exit(_))
                        || matches!(plan, CorePlan::Effect(CoreEffectPlan::ExitIf { .. }))
                }) {
                    return Err(format!("{LOOP_COND_ERR}: if body contains exit"));
                }
                block_plans.append(&mut plans);
            }
            Ok(block_plans)
        };

    let mut lower_else =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            if let Some(exit_allowed) = else_exit_allowed {
                return parts::entry::lower_exit_allowed_block(
                    builder,
                    bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    &exit_allowed.arena,
                    &exit_allowed.block,
                    LOOP_COND_ERR,
                );
            }
            lower_else_guard_break_body(
                builder,
                bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                else_recipe,
            )
        };

    let should_update_binding = |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
        carrier_phis.contains_key(name) || bindings.contains_key(name)
    };

    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_ERR,
        &mut lower_then,
        Some(&mut lower_else),
        &should_update_binding,
    )
}

/// Lower else body with guard breaks (ExitIf items allowed).
/// Unlike lower_loop_cond_recipe_block, this allows exit plans.
pub(in crate::mir::builder::control_flow::plan::features) fn lower_else_guard_break_body(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    recipe: &LoopCondBreakContinueRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut carrier_updates = BTreeMap::new();
    let mut block_plans = Vec::new();
    for item in &recipe.items {
        // Re-use lower_loop_cond_item which already handles ExitIf and Stmt.
        let mut plans = super::super::loop_cond_bc_item::lower_loop_cond_item(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            &mut carrier_updates,
            &recipe.body,
            item,
            false, // No carrier propagation inside else guard blocks
        )?;
        // NOTE: Unlike lower_loop_cond_recipe_block, we ALLOW exits here
        // because else body contains guard breaks (ExitIf items).
        block_plans.append(&mut plans);
    }
    Ok(block_plans)
}
