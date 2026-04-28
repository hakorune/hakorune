//! ASTNode statement lowering for loop_cond_break_continue.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::features::exit_if_map::lower_if_exit_stmt_with_break_phi_args;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::try_lower_nested_loop_depth1;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::parts::conditional_update::try_lower_general_if;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_bc::LOOP_COND_ERR;
use super::loop_cond_bc_util::{
    direct_exit_reject, is_direct_exit_reject, lower_simple_effect_stmt,
    lower_stmt_list_no_direct_exit, lower_stmt_list_no_exit, DirectExitRejectReason,
};

pub(in crate::mir::builder) fn lower_loop_cond_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    is_last: bool,
    stmt: &ASTNode,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(plans) = lower_simple_effect_stmt(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        carrier_updates,
        stmt,
        LOOP_COND_ERR,
    )? {
        return Ok(plans);
    }

    match stmt {
        ASTNode::Program { statements, .. } => lower_stmt_list_no_exit(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            statements,
            LOOP_COND_ERR,
        ),
        ASTNode::ScopeBox { body, .. } => lower_stmt_list_no_exit(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            body,
            LOOP_COND_ERR,
        ),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_if_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            condition,
            then_body,
            else_body.as_ref(),
        ),
        ASTNode::Loop {
            condition, body, ..
        } => {
            // Prefer the recipe-first nested-loop lowering path when possible.
            // Keep the unified nested_loop_depth1 path as a fallback to avoid acceptance loss.
            let any_err =
                match lower_nested_loop_depth1_any(builder, condition, body, LOOP_COND_ERR) {
                    Ok(plan) => {
                        super::loop_cond_bc_nested_carriers::apply_loop_final_values_to_bindings(
                            builder,
                            current_bindings,
                            &plan,
                        );
                        super::loop_cond_bc::sync_carrier_bindings(
                            builder,
                            current_bindings,
                            carrier_phis,
                        );
                        return Ok(vec![plan]);
                    }
                    Err(err) => err,
                };
            let Some(plan) = try_lower_nested_loop_depth1(builder, condition, body, LOOP_COND_ERR)?
            else {
                return Err(any_err);
            };
            super::loop_cond_bc::sync_carrier_bindings(builder, current_bindings, carrier_phis);
            Ok(vec![plan])
        }
        ASTNode::Break { .. } => {
            if !is_last {
                return Err(direct_exit_reject(
                    LOOP_COND_ERR,
                    DirectExitRejectReason::BreakMustBeLast,
                ));
            }
            Ok(vec![CorePlan::Exit(
                parts::exit::build_break_with_phi_args(
                    break_phi_dsts,
                    current_bindings,
                    LOOP_COND_ERR,
                )?,
            )])
        }
        ASTNode::Continue { .. } => Err(direct_exit_reject(
            LOOP_COND_ERR,
            DirectExitRejectReason::ExitMustBeInsideIf,
        )),
        ASTNode::Return { value, .. } => {
            if !is_last {
                return Err(direct_exit_reject(
                    LOOP_COND_ERR,
                    DirectExitRejectReason::ReturnMustBeLast,
                ));
            }
            parts::entry::lower_return_with_effects(
                builder,
                value.as_ref().map(|v| v.as_ref()),
                current_bindings,
                LOOP_COND_ERR,
            )
        }
        _ => Err(format!("{LOOP_COND_ERR}: unsupported stmt {:?}", stmt)),
    }
}

/// Lower if statement within loop-cond context.
fn lower_if_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(plans) = parts::entry::lower_conditional_update_if_with_break_phi_args(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        break_phi_dsts,
        condition,
        then_body,
        else_body,
        LOOP_COND_ERR,
    )? {
        return Ok(plans);
    }

    // NoExit join-if (no else) inside loop-cond bodies.
    //
    // This is needed for selfhost-derived code where a loop(cond) body contains
    // non-exit `if cond { ... }` statements, sometimes with nested loops inside the
    // then-branch (internal to the branch).
    //
    // Contract: the then-branch must not contain non-local exits (break/continue/return).
    // If the then-branch ends with a direct exit statement, we lower it via exit-if.
    if else_body.is_none() {
        if parts::exit_branch::split_exit_branch(then_body, LOOP_COND_ERR).is_ok() {
            return lower_if_exit_stmt_with_break_phi_args(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                condition,
                then_body,
                None,
                LOOP_COND_ERR,
            );
        }

        let cond_view = CondBlockView::from_expr(condition);

        let mut then_carrier_updates = BTreeMap::new();
        let mut lower_then =
            |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                let mut then_bindings = bindings.clone();
                let plans = lower_stmt_list_no_direct_exit(
                    builder,
                    &mut then_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    &mut then_carrier_updates,
                    then_body,
                    LOOP_COND_ERR,
                )?;
                *bindings = then_bindings;
                Ok(plans)
            };

        let should_update_binding =
            |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                bindings.contains_key(name)
            };

        return parts::entry::lower_if_join_with_branch_lowerers(
            builder,
            current_bindings,
            &cond_view,
            LOOP_COND_ERR,
            &mut lower_then,
            None,
            &should_update_binding,
        );
    }

    // Only use the recipe-first general-if path when both branches are NoExit.
    //
    // If a branch cannot be represented as `NoExitBlockRecipe`, skip this path to keep
    // other lowering rules in effect (no acceptance expansion).
    if let (Some(then_recipe), Some(else_body)) =
        (try_build_no_exit_block_recipe(then_body, true), else_body)
    {
        if let Some(else_recipe) = try_build_no_exit_block_recipe(else_body, true) {
            let then_key = (then_body.as_ptr(), then_body.len());
            let else_key = (else_body.as_ptr(), else_body.len());

            if let Some(plans) = try_lower_general_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                condition,
                then_body,
                Some(else_body),
                LOOP_COND_ERR,
                move |builder, bindings, carrier_phis, carrier_step_phis, body| {
                    let is_then =
                        std::ptr::eq(body.as_ptr(), then_key.0) && body.len() == then_key.1;
                    let is_else =
                        std::ptr::eq(body.as_ptr(), else_key.0) && body.len() == else_key.1;

                    let (arena, block) = if is_then {
                        (&then_recipe.arena, &then_recipe.block)
                    } else if is_else {
                        (&else_recipe.arena, &else_recipe.block)
                    } else {
                        return Err(format!(
                            "[freeze:contract][recipe] general_if: unexpected body slice: ctx={LOOP_COND_ERR}"
                        ));
                    };

                    let verified = parts::entry::verify_no_exit_block_with_pre(
                        arena,
                        block,
                        LOOP_COND_ERR,
                        Some(bindings),
                    )?;
                    parts::entry::lower_no_exit_block_with_stmt_lowerer_verified(
                        builder,
                        bindings,
                        carrier_step_phis,
                        Some(break_phi_dsts),
                        verified,
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
                                        "[freeze:contract][recipe] general_if: missing break_phi_dsts: ctx={}",
                                        LOOP_COND_ERR
                                    )
                                })?;
                                lower_loop_cond_stmt(
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
                    )
                },
            )? {
                return Ok(plans);
            }
        }
    }

    // Join-if fallback (stmt-list lowering) for cases that cannot be represented as
    // `NoExitBlockRecipe` (e.g. nested loops inside branches) and are not exit-if.
    //
    // Contract: the branch must not contain a direct exit statement at the top level
    // (Return/Break/Continue). Nested exit-if (`CoreEffectPlan::ExitIf`) is allowed.
    if let Some(else_body) = else_body {
        let cond_view = CondBlockView::from_expr(condition);

        let mut then_carrier_updates = BTreeMap::new();
        let mut lower_then =
            |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                let mut then_bindings = bindings.clone();
                let plans = lower_stmt_list_no_direct_exit(
                    builder,
                    &mut then_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    &mut then_carrier_updates,
                    then_body,
                    LOOP_COND_ERR,
                )?;
                *bindings = then_bindings;
                Ok(plans)
            };

        let mut else_carrier_updates = BTreeMap::new();
        let mut lower_else =
            |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                let mut else_bindings = bindings.clone();
                let plans = lower_stmt_list_no_direct_exit(
                    builder,
                    &mut else_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    &mut else_carrier_updates,
                    else_body,
                    LOOP_COND_ERR,
                )?;
                *bindings = else_bindings;
                Ok(plans)
            };

        let should_update_binding =
            |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                bindings.contains_key(name)
            };

        match parts::entry::lower_if_join_with_branch_lowerers(
            builder,
            current_bindings,
            &cond_view,
            LOOP_COND_ERR,
            &mut lower_then,
            Some(&mut lower_else),
            &should_update_binding,
        ) {
            Ok(plans) => return Ok(plans),
            Err(err) => {
                if !is_direct_exit_reject(&err) {
                    return Err(err);
                }
            }
        }
    }

    lower_if_exit_stmt_with_break_phi_args(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        condition,
        then_body,
        else_body,
        LOOP_COND_ERR,
    )
}
