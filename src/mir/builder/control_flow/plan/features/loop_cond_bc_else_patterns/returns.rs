use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::MirBuilder;
use crate::mir::policies::return_prelude_policy::{
    else_only_return_prelude_stmt_is_allowed, then_only_return_prelude_stmt_is_allowed,
};
use std::collections::BTreeMap;

use super::super::loop_cond_bc::LOOP_COND_ERR;

/// Lower if-else where only else has return
/// 箱内ローカル（exit_if_map.rs には追加しない）
pub(in crate::mir::builder::control_flow::plan::features) fn lower_else_only_return_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_no_exit: Option<&NoExitBlockRecipe>,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    else_return_stmt: StmtRef,
) -> Result<Vec<LoweredRecipe>, String> {
    // else → return exit plan
    let ret_stmt = else_body.get(else_return_stmt.index()).ok_or_else(|| {
        format!("{LOOP_COND_ERR}: ElseOnlyReturnIf else_return_stmt out of range")
    })?;
    let ASTNode::Return {
        value: Some(ret_expr),
        ..
    } = ret_stmt
    else {
        return Err(format!(
            "{LOOP_COND_ERR}: ElseOnlyReturnIf expects Return with value"
        ));
    };

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
                                    "[freeze:contract][recipe] ElseOnlyReturnIf: missing break_phi_dsts: ctx={}",
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
                        "[freeze:contract][recipe] ElseOnlyReturnIf then_body must be NoExit: ctx={LOOP_COND_ERR}"
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
                                    "[freeze:contract][recipe] ElseOnlyReturnIf: missing break_phi_dsts: ctx={}",
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
            let mut plans = Vec::new();

            let prelude = &else_body[..else_return_stmt.index()];
            plans.extend(lower_single_stmt_prelude(
                builder,
                bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                prelude,
                else_only_return_prelude_stmt_is_allowed,
                "ElseOnlyReturnIf prelude must be a single local-init or print",
            )?);

            plans.extend(parts::entry::lower_return_with_effects(
                builder,
                Some(ret_expr),
                bindings,
                LOOP_COND_ERR,
            )?);

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
            "{LOOP_COND_ERR}: ElseOnlyReturnIf missing fallthrough end map"
        ));
    };
    let Some(fallthrough_end_bindings) = fallthrough_end_bindings else {
        return Err(format!(
            "{LOOP_COND_ERR}: ElseOnlyReturnIf missing fallthrough end bindings"
        ));
    };

    builder.variable_ctx.variable_map = fallthrough_end_map;
    *current_bindings = fallthrough_end_bindings;

    Ok(if_plans)
}

/// Lower if-else where only then has return
pub(in crate::mir::builder::control_flow::plan::features) fn lower_then_only_return_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    else_no_exit: Option<&NoExitBlockRecipe>,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    then_return_stmt: StmtRef,
) -> Result<Vec<LoweredRecipe>, String> {
    // then → return exit plan
    let ret_stmt = then_body.get(then_return_stmt.index()).ok_or_else(|| {
        format!("{LOOP_COND_ERR}: ThenOnlyReturnIf then_return_stmt out of range")
    })?;
    let ASTNode::Return {
        value: Some(ret_expr),
        ..
    } = ret_stmt
    else {
        return Err(format!(
            "{LOOP_COND_ERR}: ThenOnlyReturnIf expects Return with value"
        ));
    };

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let mut fallthrough_end_map: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut fallthrough_end_bindings: Option<BTreeMap<String, crate::mir::ValueId>> = None;

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let mut plans = Vec::new();

            let prelude = &then_body[..then_return_stmt.index()];
            plans.extend(lower_single_stmt_prelude(
                builder,
                bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                prelude,
                then_only_return_prelude_stmt_is_allowed,
                "ThenOnlyReturnIf prelude must be a single local-init",
            )?);

            plans.extend(parts::entry::lower_return_with_effects(
                builder,
                Some(ret_expr),
                bindings,
                LOOP_COND_ERR,
            )?);

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
                                    "[freeze:contract][recipe] ThenOnlyReturnIf: missing break_phi_dsts: ctx={}",
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
                        "[freeze:contract][recipe] ThenOnlyReturnIf else_body must be NoExit: ctx={LOOP_COND_ERR}"
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
                                    "[freeze:contract][recipe] ThenOnlyReturnIf: missing break_phi_dsts: ctx={}",
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
            "{LOOP_COND_ERR}: ThenOnlyReturnIf missing fallthrough end map"
        ));
    };
    let Some(fallthrough_end_bindings) = fallthrough_end_bindings else {
        return Err(format!(
            "{LOOP_COND_ERR}: ThenOnlyReturnIf missing fallthrough end bindings"
        ));
    };

    builder.variable_ctx.variable_map = fallthrough_end_map;
    *current_bindings = fallthrough_end_bindings;

    Ok(if_plans)
}

fn lower_single_stmt_prelude(
    builder: &mut MirBuilder,
    bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[ASTNode],
    is_allowed: fn(&ASTNode) -> bool,
    error_label: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if prelude.is_empty() {
        return Ok(Vec::new());
    }
    if prelude.len() != 1 || !is_allowed(&prelude[0]) {
        return Err(format!(
            "[freeze:contract][recipe] {error_label}: ctx={LOOP_COND_ERR}"
        ));
    }

    let mut carrier_updates = BTreeMap::new();
    let mut plans = Vec::new();
    for stmt in prelude {
        plans.extend(super::super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
            builder,
            bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            &mut carrier_updates,
            false,
            stmt,
        )?);
    }
    Ok(plans)
}
