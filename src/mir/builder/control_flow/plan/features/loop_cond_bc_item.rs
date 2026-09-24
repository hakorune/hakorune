//! LoopCondBreakContinueItem variant lowering.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::features::exit_if_map::lower_if_exit_stmt_with_break_phi_args;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::parts::conditional_update::lower_conditional_update_if_assume_with_break_phi_args_recipe_first;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_bc::LOOP_COND_ERR;
use super::loop_cond_bc_item_stmt::lower_loop_cond_stmt;
use super::loop_cond_bc_util::{
    get_stmt, lower_simple_effect_stmt, lower_simple_effect_stmt_body_input,
};

/// Source-port preparation for the recipe's direct statement item.
///
/// Complex LoopCond variants keep their existing owner until their nested and
/// exit contexts are parameterized. Returning `None` here is deliberate: the
/// source caller must not fall through to the raw item lowerer.
pub(in crate::mir::builder) fn lower_loop_cond_item_input<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    item: &LoopCondBreakContinueItem,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let LoopCondBreakContinueItem::Stmt(stmt_ref) = item else {
        if let LoopCondBreakContinueItem::ExitLeaf { kind, stmt } = item {
            let statement = port
                .body_stmt(body, stmt.index())
                .map_err(|error| error.render())?;
            return parts::exit::lower_loop_cond_exit_source_input(
                port,
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                statement,
                *kind,
                error_prefix,
            )
            .map(Some);
        }
        return Ok(None);
    };
    lower_simple_effect_stmt_body_input(
        port,
        body,
        *stmt_ref,
        builder,
        current_bindings,
        carrier_phis,
        carrier_updates,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_loop_cond_item(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &RecipeBody,
    item: &LoopCondBreakContinueItem,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(exit_if) = item.as_exit_if() {
        if let Some(exit_allowed_block) = exit_if.exit_allowed_block {
            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &exit_allowed_block.arena,
                &exit_allowed_block.block,
                LOOP_COND_ERR,
                Some(&builder.function_state.variable_ctx.variable_map),
            )?;
            return parts::entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                LOOP_COND_ERR,
            );
        }

        let stmt = get_stmt(body, exit_if.if_stmt)?;
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        else {
            return Err(format!("{LOOP_COND_ERR}: exit_if is not if"));
        };
        match lower_if_exit_stmt_with_break_phi_args(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            condition,
            then_body,
            else_body.as_ref(),
            LOOP_COND_ERR,
        ) {
            Ok(plans) => return Ok(plans),
            Err(err) => {
                if !err.contains("if body must be single-exit") {
                    return Err(err);
                }
                // Narrow fallback: complex nested-exit tails are lowered through the
                // generic stmt path, which can still reject with a direct-exit contract.
                return lower_loop_cond_stmt(
                    builder,
                    current_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    carrier_updates,
                    false,
                    stmt,
                );
            }
        }
    }

    if item.is_tail_break() {
        if let Some(exit_allowed_block) = item.tail_break_exit_allowed_block() {
            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &exit_allowed_block.arena,
                &exit_allowed_block.block,
                LOOP_COND_ERR,
                Some(&builder.function_state.variable_ctx.variable_map),
            )?;
            return parts::entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                LOOP_COND_ERR,
            );
        }

        return Ok(vec![CorePlan::Exit(
            parts::exit::build_break_with_phi_args(
                break_phi_dsts,
                current_bindings,
                LOOP_COND_ERR,
            )?,
        )]);
    }

    match item {
        LoopCondBreakContinueItem::Stmt(stmt_ref) => {
            let stmt = get_stmt(body, *stmt_ref)?;
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
            Err(format!("{LOOP_COND_ERR}: recipe stmt mismatch {:?}", stmt))
        }
        LoopCondBreakContinueItem::ProgramBlock { stmt, stmt_only } => {
            if let Some(recipe) = stmt_only.as_ref() {
                let verified = parts::entry::verify_no_exit_block_with_pre(
                    &recipe.arena,
                    &recipe.block,
                    LOOP_COND_ERR,
                    Some(&builder.function_state.variable_ctx.variable_map),
                )?;
                return parts::entry::lower_no_exit_block_with_stmt_lowerer_verified(
                    builder,
                    current_bindings,
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
                                    "[freeze:contract][recipe] program_block: missing break_phi_dsts: ctx={}",
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
                );
            }

            let stmt = get_stmt(body, *stmt)?;
            if let Some(recipe) =
                try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), true).or_else(
                    || try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), false),
                )
            {
                let verified = parts::entry::verify_exit_allowed_block_with_pre(
                    &recipe.arena,
                    &recipe.block,
                    LOOP_COND_ERR,
                    Some(&builder.function_state.variable_ctx.variable_map),
                )?;
                let plans = parts::entry::lower_exit_allowed_block_verified(
                    builder,
                    current_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    verified,
                    LOOP_COND_ERR,
                )?;
                return Ok(plans);
            }
            lower_loop_cond_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                carrier_updates,
                false,
                stmt,
            )
        }
        LoopCondBreakContinueItem::ContinueIfWithElse {
            if_stmt,
            continue_in_then,
            continue_prelude,
            fallthrough_body,
        } => {
            let stmt = get_stmt(body, *if_stmt)?;
            let ASTNode::If { condition, .. } = stmt else {
                return Err(format!("{LOOP_COND_ERR}: continue_if is not if"));
            };
            super::loop_cond_bc_continue_if::lower_continue_if_with_else(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                condition,
                *continue_in_then,
                continue_prelude,
                fallthrough_body,
                LOOP_COND_ERR,
            )
        }
        LoopCondBreakContinueItem::ConditionalUpdateIf {
            if_stmt: _,
            cond_view,
            then_body,
            then_exit,
            else_body,
            else_exit,
        } => lower_conditional_update_if_assume_with_break_phi_args_recipe_first(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            break_phi_dsts,
            cond_view,
            then_body.as_ref(),
            *then_exit,
            else_body.as_ref(),
            *else_exit,
            LOOP_COND_ERR,
        ),
        LoopCondBreakContinueItem::GeneralIf(recipe) => {
            let verified = parts::entry::verify_no_exit_block_with_pre(
                &recipe.arena,
                &recipe.block,
                LOOP_COND_ERR,
                Some(&builder.function_state.variable_ctx.variable_map),
            )?;
            parts::entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                verified,
                LOOP_COND_ERR,
            )
        }
        LoopCondBreakContinueItem::NestedLoopDepth1 { .. } => Err(format!(
            "[freeze:contract][recipe] nested_loop_depth1 item has no physical owner: ctx={}",
            LOOP_COND_ERR
        )),
        LoopCondBreakContinueItem::ElseOnlyReturnIf {
            if_stmt,
            cond_view,
            then_no_exit,
            else_return_stmt,
        } => {
            let stmt = get_stmt(body, *if_stmt)?;
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{LOOP_COND_ERR}: ElseOnlyReturnIf expects If"));
            };
            let else_body = else_body
                .as_ref()
                .ok_or_else(|| format!("{LOOP_COND_ERR}: ElseOnlyReturnIf requires else branch"))?;
            super::loop_cond_bc_else_patterns::lower_else_only_return_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                cond_view,
                then_no_exit.as_ref(),
                then_body,
                else_body,
                *else_return_stmt,
            )
        }
        LoopCondBreakContinueItem::ThenOnlyReturnIf {
            if_stmt,
            cond_view,
            then_return_stmt,
            else_no_exit,
        } => {
            let stmt = get_stmt(body, *if_stmt)?;
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{LOOP_COND_ERR}: ThenOnlyReturnIf expects If"));
            };
            let else_body = else_body
                .as_ref()
                .ok_or_else(|| format!("{LOOP_COND_ERR}: ThenOnlyReturnIf requires else branch"))?;
            super::loop_cond_bc_else_patterns::lower_then_only_return_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                cond_view,
                else_no_exit.as_ref(),
                then_body,
                else_body,
                *then_return_stmt,
            )
        }
        LoopCondBreakContinueItem::ElseOnlyBreakIf {
            if_stmt,
            cond_view,
            then_no_exit,
            else_break_stmt,
        } => {
            let stmt = get_stmt(body, *if_stmt)?;
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{LOOP_COND_ERR}: ElseOnlyBreakIf expects If"));
            };
            let else_body = else_body
                .as_ref()
                .ok_or_else(|| format!("{LOOP_COND_ERR}: ElseOnlyBreakIf requires else branch"))?;
            super::loop_cond_bc_else_patterns::lower_else_only_break_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                cond_view,
                then_no_exit.as_ref(),
                then_body,
                else_body,
                *else_break_stmt,
            )
        }
        LoopCondBreakContinueItem::ThenOnlyBreakIf {
            if_stmt,
            cond_view,
            then_break_stmt,
            else_no_exit,
        } => {
            let stmt = get_stmt(body, *if_stmt)?;
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{LOOP_COND_ERR}: ThenOnlyBreakIf expects If"));
            };
            let else_body = else_body
                .as_ref()
                .ok_or_else(|| format!("{LOOP_COND_ERR}: ThenOnlyBreakIf requires else branch"))?;
            super::loop_cond_bc_else_patterns::lower_then_only_break_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                cond_view,
                else_no_exit.as_ref(),
                then_body,
                else_body,
                *then_break_stmt,
            )
        }
        LoopCondBreakContinueItem::ElseGuardBreakIf { .. } => {
            let else_guard = item
                .as_else_guard_break_if()
                .ok_or_else(|| format!("{LOOP_COND_ERR}: ElseGuardBreakIf payload mismatch"))?;

            let stmt = get_stmt(body, else_guard.if_stmt)?;
            let ASTNode::If { condition, .. } = stmt else {
                return Err(format!("{LOOP_COND_ERR}: ElseGuardBreakIf expects If"));
            };
            super::loop_cond_bc_else_patterns::lower_else_guard_break_if_with_exit_allowed(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                condition,
                else_guard.then_no_exit,
                else_guard.then_recipe,
                else_guard.else_exit_allowed,
                else_guard.else_recipe,
            )
        }
        // Phase 29bq BoxCount: ExitLeaf (break/continue/return terminal)
        LoopCondBreakContinueItem::ExitLeaf {
            kind,
            stmt: stmt_ref,
        } => parts::entry::lower_loop_cond_exit_leaf(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            body,
            *kind,
            *stmt_ref,
            LOOP_COND_ERR,
        ),
        // Phase 29bq BoxCount: ExitIfTree (nested if with all branches ending in exit)
        LoopCondBreakContinueItem::ExitIfTree {
            if_stmt: _,
            cond_view,
            mode,
            then_body: then_recipe,
            else_body: else_recipe,
        } => parts::if_::lower_loop_cond_exit_if_tree(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            cond_view,
            *mode,
            then_recipe,
            else_recipe.as_ref(),
            LOOP_COND_ERR,
        ),
        _ => Err(format!(
            "{LOOP_COND_ERR}: unsupported item variant: {:?}",
            item
        )),
    }
}
