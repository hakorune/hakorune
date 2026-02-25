#![allow(dead_code)]
//! Else-only-return and else-guard-break pattern handlers.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueRecipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::return_prelude_policy::{
    else_only_return_prelude_stmt_is_allowed, then_only_return_prelude_stmt_is_allowed,
};
use std::collections::BTreeMap;

use super::loop_cond_bc::LOOP_COND_ERR;

/// Lower if-else where only else has return
/// 箱内ローカル（exit_if_map.rs には追加しない）
pub(super) fn lower_else_only_return_if(
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
pub(super) fn lower_then_only_return_if(
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
        plans.extend(super::loop_cond_bc_item_stmt::lower_loop_cond_stmt(
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

/// Lower if-else where only else has break
pub(super) fn lower_else_only_break_if(
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
pub(super) fn lower_then_only_break_if(
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

/// Lower if-else where else has guard breaks (exit-ifs)
/// Pattern: if cond { non-exit } else { (if guard { break })+ + non-exit }
/// Recipe-first: both branches are pre-classified recipes.
/// 箱内ローカル（exit_if_map.rs には追加しない）
pub(super) fn lower_else_guard_break_if(
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

pub(super) fn lower_else_guard_break_if_with_exit_allowed(
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
                let mut plans = super::loop_cond_bc_item::lower_loop_cond_item(
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
pub(super) fn lower_else_guard_break_body(
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
        let mut plans = super::loop_cond_bc_item::lower_loop_cond_item(
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

// NOTE: ExitIfTree lowering moved to `plan/parts/if.rs` (M3).
