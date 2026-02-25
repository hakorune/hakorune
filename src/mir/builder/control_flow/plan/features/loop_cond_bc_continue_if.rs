//! Continue-if pattern handler.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(super) fn lower_continue_if_with_else(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    continue_in_then: bool,
    continue_prelude: &Option<NoExitBlockRecipe>,
    fallthrough_body: &Option<NoExitBlockRecipe>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let saved_map = builder.variable_ctx.variable_map.clone();
    let saved_bindings = current_bindings.clone();

    let cond_view = CondBlockView::from_expr(condition);

    let mut fallthrough_map: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut fallthrough_bindings: Option<BTreeMap<String, crate::mir::ValueId>> = None;

    let mut lower_continue_branch =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            builder.variable_ctx.variable_map = saved_map.clone();
            let mut branch_bindings = saved_bindings.clone();

            let mut plans = Vec::new();
            if let Some(recipe) = continue_prelude.as_ref() {
                plans.extend(parts::entry::lower_no_exit_block_with_stmt_lowerer(
                builder,
                &mut branch_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                &recipe.arena,
                &recipe.block,
                error_prefix,
                || {
                    let mut carrier_updates = BTreeMap::new();
                    move |builder: &mut MirBuilder,
                          bindings: &mut BTreeMap<String, crate::mir::ValueId>,
                          carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
                          break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
                          stmt: &ASTNode,
                          _error_prefix: &str| {
                        let break_phi_dsts = break_phi_dsts.ok_or_else(|| {
                            format!(
                                "[freeze:contract][recipe] continue_if_with_else: missing break_phi_dsts: ctx={}",
                                error_prefix
                            )
                        })?;
                        super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
                            builder,
                            bindings,
                            carrier_phis,
                            carrier_step_phis,
                            break_phi_dsts,
                            &mut carrier_updates,
                            false,
                            stmt,
                        )
                    }
                },
                |name, bindings| bindings.contains_key(name),
            )?);
            }
            let mut pred_bindings = branch_bindings.clone();
            for (name, _) in carrier_step_phis {
                if let Some(value_id) = builder.variable_ctx.variable_map.get(name) {
                    pred_bindings.insert(name.clone(), *value_id);
                }
            }
            let exit = parts::exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                &pred_bindings,
                error_prefix,
            )?;
            plans.push(CorePlan::Exit(exit));

            builder.variable_ctx.variable_map = saved_map.clone();
            *bindings = saved_bindings.clone();
            Ok(plans)
        };

    let mut lower_fallthrough_branch =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            builder.variable_ctx.variable_map = saved_map.clone();
            *bindings = saved_bindings.clone();

            let mut plans = Vec::new();
            if let Some(recipe) = fallthrough_body.as_ref() {
                plans.extend(parts::entry::lower_no_exit_block_with_stmt_lowerer(
                builder,
                bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                &recipe.arena,
                &recipe.block,
                error_prefix,
                || {
                    let mut carrier_updates = BTreeMap::new();
                    move |builder: &mut MirBuilder,
                          bindings: &mut BTreeMap<String, crate::mir::ValueId>,
                          carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
                          break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
                          stmt: &ASTNode,
                          _error_prefix: &str| {
                        let break_phi_dsts = break_phi_dsts.ok_or_else(|| {
                            format!(
                                "[freeze:contract][recipe] continue_if_with_else: missing break_phi_dsts: ctx={}",
                                error_prefix
                            )
                        })?;
                        super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
                            builder,
                            bindings,
                            carrier_phis,
                            carrier_step_phis,
                            break_phi_dsts,
                            &mut carrier_updates,
                            false,
                            stmt,
                        )
                    }
                },
                |name, bindings| bindings.contains_key(name),
            )?);
            }

            fallthrough_map = Some(builder.variable_ctx.variable_map.clone());
            fallthrough_bindings = Some(bindings.clone());

            builder.variable_ctx.variable_map = saved_map.clone();
            *bindings = saved_bindings.clone();
            Ok(plans)
        };

    builder.variable_ctx.variable_map = saved_map.clone();
    *current_bindings = saved_bindings.clone();

    let (then_lowerer, else_lowerer) = if continue_in_then {
        (
            &mut lower_continue_branch
                as &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
            &mut lower_fallthrough_branch
                as &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
        )
    } else {
        (
            &mut lower_fallthrough_branch
                as &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
            &mut lower_continue_branch
                as &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
        )
    };

    let plans = parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        error_prefix,
        then_lowerer,
        Some(else_lowerer),
        &|_name, _bindings| false,
    )?;

    let fallthrough_map = fallthrough_map.ok_or_else(|| {
        format!(
            "[freeze:contract][recipe] continue_if_fallthrough_map_missing: ctx={}",
            error_prefix
        )
    })?;
    let fallthrough_bindings = fallthrough_bindings.ok_or_else(|| {
        format!(
            "[freeze:contract][recipe] continue_if_fallthrough_bindings_missing: ctx={}",
            error_prefix
        )
    })?;

    builder.variable_ctx.variable_map = fallthrough_map;
    *current_bindings = fallthrough_bindings;

    Ok(plans)
}
