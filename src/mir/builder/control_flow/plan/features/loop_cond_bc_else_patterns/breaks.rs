use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::super::loop_cond_bc::LOOP_COND_ERR;

/// Lower if-else where only else has break
pub(in crate::mir::builder::control_flow::plan::features) fn lower_else_only_break_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_no_exit: Option<&NoExitBlockRecipe>,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    else_break_stmt: StmtRef,
) -> Result<Vec<LoweredRecipe>, String> {
    let br_stmt = else_body
        .get(else_break_stmt.index())
        .ok_or_else(|| format!("{LOOP_COND_ERR}: ElseOnlyBreakIf else_break_stmt out of range"))?;
    if !matches!(br_stmt, ASTNode::Break { .. }) {
        return Err(format!("{LOOP_COND_ERR}: ElseOnlyBreakIf expects Break"));
    }

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let mut fallthrough_end_map: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut fallthrough_end_bindings: Option<BTreeMap<String, crate::mir::ValueId>> = None;

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let plans = if let Some(then_no_exit) = then_no_exit {
                parts::entry::lower_no_exit_block_with_stmt_lowerer(
                    builder,
                    bindings,
                    carrier_step_phis,
                    Some(break_phi_dsts),
                    &then_no_exit.arena,
                    &then_no_exit.block,
                    LOOP_COND_ERR,
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
                                    "[freeze:contract][recipe] ElseOnlyBreakIf: missing break_phi_dsts: ctx={}",
                                    LOOP_COND_ERR
                                )
                            })?;
                            super::super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
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
                )?
            } else {
                let Some(recipe) = try_build_no_exit_block_recipe(then_body, true) else {
                    return Err(format!(
                        "[freeze:contract][recipe] ElseOnlyBreakIf then_body must be NoExit: ctx={LOOP_COND_ERR}"
                    ));
                };

                parts::entry::lower_no_exit_block_with_stmt_lowerer(
                    builder,
                    bindings,
                    carrier_step_phis,
                    Some(break_phi_dsts),
                    &recipe.arena,
                    &recipe.block,
                    LOOP_COND_ERR,
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
                                    "[freeze:contract][recipe] ElseOnlyBreakIf: missing break_phi_dsts: ctx={}",
                                    LOOP_COND_ERR
                                )
                            })?;
                            super::super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
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
                )?
            };

            fallthrough_end_map = Some(builder.variable_ctx.variable_map.clone());
            fallthrough_end_bindings = Some(bindings.clone());

            builder.variable_ctx.variable_map = pre_if_map.clone();
            *bindings = pre_bindings.clone();

            Ok(plans)
        };

    let mut lower_else =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let exit =
                parts::exit::build_break_with_phi_args(break_phi_dsts, bindings, LOOP_COND_ERR)?;
            let plans = vec![CorePlan::Exit(exit)];

            builder.variable_ctx.variable_map = pre_if_map.clone();
            *bindings = pre_bindings.clone();

            Ok(plans)
        };

    let should_update_binding =
        |_name: &str, _bindings: &BTreeMap<String, crate::mir::ValueId>| false;

    let if_plans = parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_ERR,
        &mut lower_then,
        Some(&mut lower_else),
        &should_update_binding,
    )?;

    let Some(fallthrough_end_map) = fallthrough_end_map else {
        return Err(format!(
            "{LOOP_COND_ERR}: ElseOnlyBreakIf missing fallthrough end map"
        ));
    };
    let Some(fallthrough_end_bindings) = fallthrough_end_bindings else {
        return Err(format!(
            "{LOOP_COND_ERR}: ElseOnlyBreakIf missing fallthrough end bindings"
        ));
    };

    builder.variable_ctx.variable_map = fallthrough_end_map;
    *current_bindings = fallthrough_end_bindings;

    Ok(if_plans)
}

/// Lower if-else where only then has break
pub(in crate::mir::builder::control_flow::plan::features) fn lower_then_only_break_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    else_no_exit: Option<&NoExitBlockRecipe>,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    then_break_stmt: StmtRef,
) -> Result<Vec<LoweredRecipe>, String> {
    let br_stmt = then_body
        .get(then_break_stmt.index())
        .ok_or_else(|| format!("{LOOP_COND_ERR}: ThenOnlyBreakIf then_break_stmt out of range"))?;
    if !matches!(br_stmt, ASTNode::Break { .. }) {
        return Err(format!("{LOOP_COND_ERR}: ThenOnlyBreakIf expects Break"));
    }

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let mut fallthrough_end_map: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut fallthrough_end_bindings: Option<BTreeMap<String, crate::mir::ValueId>> = None;

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let exit =
                parts::exit::build_break_with_phi_args(break_phi_dsts, bindings, LOOP_COND_ERR)?;
            let plans = vec![CorePlan::Exit(exit)];

            builder.variable_ctx.variable_map = pre_if_map.clone();
            *bindings = pre_bindings.clone();

            Ok(plans)
        };

    let mut lower_else =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let plans = if let Some(else_no_exit) = else_no_exit {
                parts::entry::lower_no_exit_block_with_stmt_lowerer(
                    builder,
                    bindings,
                    carrier_step_phis,
                    Some(break_phi_dsts),
                    &else_no_exit.arena,
                    &else_no_exit.block,
                    LOOP_COND_ERR,
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
                                    "[freeze:contract][recipe] ThenOnlyBreakIf: missing break_phi_dsts: ctx={}",
                                    LOOP_COND_ERR
                                )
                            })?;
                            super::super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
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
                )?
            } else {
                let Some(recipe) = try_build_no_exit_block_recipe(else_body, true) else {
                    return Err(format!(
                        "[freeze:contract][recipe] ThenOnlyBreakIf else_body must be NoExit: ctx={LOOP_COND_ERR}"
                    ));
                };
                parts::entry::lower_no_exit_block_with_stmt_lowerer(
                    builder,
                    bindings,
                    carrier_step_phis,
                    Some(break_phi_dsts),
                    &recipe.arena,
                    &recipe.block,
                    LOOP_COND_ERR,
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
                                    "[freeze:contract][recipe] ThenOnlyBreakIf: missing break_phi_dsts: ctx={}",
                                    LOOP_COND_ERR
                                )
                            })?;
                            super::super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
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
                )?
            };

            fallthrough_end_map = Some(builder.variable_ctx.variable_map.clone());
            fallthrough_end_bindings = Some(bindings.clone());

            builder.variable_ctx.variable_map = pre_if_map.clone();
            *bindings = pre_bindings.clone();

            Ok(plans)
        };

    let should_update_binding =
        |_name: &str, _bindings: &BTreeMap<String, crate::mir::ValueId>| false;

    let if_plans = parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_ERR,
        &mut lower_then,
        Some(&mut lower_else),
        &should_update_binding,
    )?;

    let Some(fallthrough_end_map) = fallthrough_end_map else {
        return Err(format!(
            "{LOOP_COND_ERR}: ThenOnlyBreakIf missing fallthrough end map"
        ));
    };
    let Some(fallthrough_end_bindings) = fallthrough_end_bindings else {
        return Err(format!(
            "{LOOP_COND_ERR}: ThenOnlyBreakIf missing fallthrough end bindings"
        ));
    };

    builder.variable_ctx.variable_map = fallthrough_end_map;
    *current_bindings = fallthrough_end_bindings;

    Ok(if_plans)
}
