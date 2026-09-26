//! LoopCond item dispatch over the located callable-loop source port.
//!
//! `build_loop_cond_break_continue_recipe_inner` issues exactly one
//! `LoopCondBreakContinueItem` per recipe body statement, so the item ordinal
//! locates the `GeneralIf` statement that carries no `StmtRef`; payload
//! `StmtRef`s (`ProgramBlock`, `ExitIfTree`) remain the item authority where
//! they exist. Recipe block contracts stay the packaging authority:
//! `ProgramBlock` re-derives the exit-allowed singleton exactly like the raw
//! item owner, `GeneralIf` consumes the issued no-exit recipe, and
//! `ExitIfTree` consumes its issued ExitOnly branch recipes through the
//! shared `lower_exit_if_state_core` transaction. Every other variant is a
//! named reject; this dispatcher never falls through to
//! `lower_loop_cond_item`.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    try_build_exit_allowed_block_recipe, ExitAllowedBlockRecipe,
};
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_item::lower_loop_cond_item_input;

use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::ValueId;

use super::callable_loop_source::{require_condition_view_match, CallableLoopSourcePartsBlockV1};
use super::callable_loop_source_lowering::{
    lower_callable_loop_source_parts_block, render_source_error,
    CallableLoopSourcePartsLoweringHooksV1, SOURCE_PARTS_ERR,
};
use super::dispatch::{PartsAssociatedBlockModeV1, PartsAssociatedLoweringHooksV1};
use super::PartsAssociatedSourceErrorV1;

/// Lower one issued LoopCond item through the located source port.
///
/// `body` is the co-sealed loop-body carrier; `item_index` is the item's
/// ordinal in `recipe.items`, which the recipe builder keeps 1:1 with the
/// body statements. A `Synthetic` body input cannot reach a located
/// statement, so the seal constructors reject it before any Builder effect.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn lower_loop_cond_source_item<'view, 'ledger: 'view>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    body: &CallableLoopSourceBodyInputV1<'view>,
    item_index: usize,
    item: &LoopCondBreakContinueItem,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match item {
        LoopCondBreakContinueItem::Stmt(_) | LoopCondBreakContinueItem::ExitLeaf { .. } => {
            lower_loop_cond_item_input(
                &port,
                body,
                item,
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                carrier_updates,
                error_prefix,
            )?
            .ok_or_else(|| {
                format!("{SOURCE_PARTS_ERR} loop-cond-item-unhandled: ctx={error_prefix}")
            })
        }
        LoopCondBreakContinueItem::ProgramBlock { stmt, stmt_only } => {
            if stmt_only.is_some() {
                // A StmtOnly recipe carries the flattened container list,
                // which cannot align 1:1 with the located carrier; keep it a
                // named reject until its own slice co-seals the projection.
                return Err(format!(
                    "{SOURCE_PARTS_ERR} program-block-stmt-only-unlocated: ctx={error_prefix}"
                ));
            }
            let source = port
                .body_stmt(body, stmt.index())
                .map_err(|error| error.render())?;
            let Some(recipe) = try_build_exit_allowed_block_recipe(
                std::slice::from_ref(port.stmt_syntax(&source)),
                true,
            )
            .or_else(|| {
                try_build_exit_allowed_block_recipe(
                    std::slice::from_ref(port.stmt_syntax(&source)),
                    false,
                )
            }) else {
                return Err(format!(
                    "{SOURCE_PARTS_ERR} program-block-recipe-unlocatable: ctx={error_prefix}"
                ));
            };
            let block = CallableLoopSourcePartsBlockV1::singleton(
                &recipe.arena,
                &recipe.block,
                source,
                &port,
            )
            .map_err(|error| render_source_error(error, error_prefix))?;
            let plans = lower_callable_loop_source_parts_block(
                port,
                &block,
                PartsAssociatedBlockModeV1::ExitAllowed,
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                carrier_updates,
                error_prefix,
            )?;
            Ok(plans)
        }
        LoopCondBreakContinueItem::GeneralIf(recipe) => {
            // `GeneralIf` carries no `StmtRef`; the issued recipe body is the
            // one verbatim `from_ref(if_stmt)` clone, and `singleton`
            // re-proves it against the located statement at the item ordinal.
            let source = port
                .body_stmt(body, item_index)
                .map_err(|error| error.render())?;
            let block = CallableLoopSourcePartsBlockV1::singleton(
                &recipe.arena,
                &recipe.block,
                source,
                &port,
            )
            .map_err(|error| render_source_error(error, error_prefix))?;
            lower_callable_loop_source_parts_block(
                port,
                &block,
                PartsAssociatedBlockModeV1::NoExit,
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                carrier_updates,
                error_prefix,
            )
        }
        LoopCondBreakContinueItem::ExitIfTree {
            if_stmt,
            cond_view,
            mode,
            then_body,
            else_body,
        } => {
            let source = port
                .body_stmt(body, if_stmt.index())
                .map_err(|error| error.render())?;
            let ASTNode::If {
                else_body: ast_else_body,
                ..
            } = port.stmt_syntax(&source)
            else {
                return Err(render_source_error(
                    PartsAssociatedSourceErrorV1::RecipeBodyMismatch,
                    error_prefix,
                ));
            };
            if else_body.is_some() != ast_else_body.is_some() {
                return Err(render_source_error(
                    PartsAssociatedSourceErrorV1::RecipeBodyMismatch,
                    error_prefix,
                ));
            }
            let condition = port
                .child_expr_from_stmt(&source, ExprChildRoleV1::IfCondition)
                .map_err(|error| error.render())?;
            require_condition_view_match(cond_view, port.expr_syntax(&condition))
                .map_err(|error| render_source_error(error, error_prefix))?;
            let then_carrier = port
                .child_body_from_stmt(&source, BodyChildRoleV1::IfThen)
                .map_err(|error| error.render())?;
            let else_carrier = else_body
                .as_ref()
                .map(|_| port.child_body_from_stmt(&source, BodyChildRoleV1::IfElse))
                .transpose()
                .map_err(|error| error.render())?;
            let then_block = CallableLoopSourcePartsBlockV1::located_body(
                &then_body.arena,
                &then_body.block,
                then_carrier.clone(),
                &port,
            )
            .map_err(|error| render_source_error(error, error_prefix))?;
            let else_block = else_body
                .as_ref()
                .zip(else_carrier.as_ref())
                .map(|(recipe, carrier)| {
                    CallableLoopSourcePartsBlockV1::located_body(
                        &recipe.arena,
                        &recipe.block,
                        carrier.clone(),
                        &port,
                    )
                })
                .transpose()
                .map_err(|error| render_source_error(error, error_prefix))?;
            let mut hooks = CallableLoopSourcePartsLoweringHooksV1 {
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                break_phi_dsts,
                carrier_updates,
                error_prefix,
            };
            hooks.lower_explicit_if(
                port,
                source,
                condition,
                then_carrier,
                else_carrier,
                IfContractKind::ExitOnly { mode: *mode },
                then_block,
                else_block,
            )
        }
        LoopCondBreakContinueItem::ConditionalUpdateIf {
            if_stmt,
            cond_view,
            then_body,
            then_exit,
            else_body,
            else_exit,
        } => {
            // `else_body`/`else_exit` are both `None` only when the source
            // `if` has no else at all; an else holding just a tail exit
            // records `else_body = None` + `else_exit = Some(..)`.
            let source = port
                .body_stmt(body, if_stmt.index())
                .map_err(|error| error.render())?;
            let ASTNode::If {
                else_body: ast_else_body,
                ..
            } = port.stmt_syntax(&source)
            else {
                return Err(render_source_error(
                    PartsAssociatedSourceErrorV1::RecipeBodyMismatch,
                    error_prefix,
                ));
            };
            if (else_body.is_some() || else_exit.is_some()) != ast_else_body.is_some() {
                return Err(render_source_error(
                    PartsAssociatedSourceErrorV1::RecipeBodyMismatch,
                    error_prefix,
                ));
            }
            let condition = port
                .child_expr_from_stmt(&source, ExprChildRoleV1::IfCondition)
                .map_err(|error| error.render())?;
            require_condition_view_match(cond_view, port.expr_syntax(&condition))
                .map_err(|error| render_source_error(error, error_prefix))?;
            let then_carrier = port
                .child_body_from_stmt(&source, BodyChildRoleV1::IfThen)
                .map_err(|error| error.render())?;
            let else_carrier = ast_else_body
                .as_ref()
                .map(|_| port.child_body_from_stmt(&source, BodyChildRoleV1::IfElse))
                .transpose()
                .map_err(|error| error.render())?;
            verify_cond_update_branch_recipe(
                &port,
                &then_carrier,
                then_body.is_some(),
                *then_exit,
                error_prefix,
            )?;
            if let Some(else_carrier) = &else_carrier {
                verify_cond_update_branch_recipe(
                    &port,
                    else_carrier,
                    else_body.is_some(),
                    *else_exit,
                    error_prefix,
                )?;
            }
            super::super::conditional_update::try_lower_conditional_update_if_input(
                &port,
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                Some(break_phi_dsts),
                condition,
                &then_carrier,
                else_carrier.as_ref(),
                error_prefix,
            )?
            .ok_or_else(|| {
                format!("{SOURCE_PARTS_ERR} loop-cond-item-conditional-update-unsupported: ctx={error_prefix}")
            })
        }
        _ => Err(format!(
            "{SOURCE_PARTS_ERR} loop-cond-item-unsupported: ctx={error_prefix}"
        )),
    }
}

/// Verify the recipe's recorded branch expectations against the located
/// branch carrier: the tail Break/Continue must equal `recipe_exit`, and a
/// `None` body recipe must cover a located branch that holds no statements
/// besides that tail exit.
fn verify_cond_update_branch_recipe(
    port: &CallableLoopSourceExpressionPortV1<'_>,
    carrier: &CallableLoopSourceBodyInputV1<'_>,
    recipe_has_body: bool,
    recipe_exit: Option<ExitKind>,
    error_prefix: &str,
) -> Result<(), String> {
    let statements = port.body_statements(carrier);
    let derived_exit = match statements.last() {
        Some(ASTNode::Break { .. }) => Some(ExitKind::Break { depth: 1 }),
        Some(ASTNode::Continue { .. }) => Some(ExitKind::Continue { depth: 1 }),
        _ => None,
    };
    let exit_matches = matches!(
        (derived_exit, recipe_exit),
        (None, None)
            | (
                Some(ExitKind::Break { depth: 1 }),
                Some(ExitKind::Break { depth: 1 })
            )
            | (
                Some(ExitKind::Continue { depth: 1 }),
                Some(ExitKind::Continue { depth: 1 })
            )
    );
    let non_exit_len = statements.len() - usize::from(derived_exit.is_some());
    if !exit_matches || recipe_has_body != (non_exit_len > 0) {
        return Err(render_source_error(
            PartsAssociatedSourceErrorV1::RecipeBodyMismatch,
            error_prefix,
        ));
    }
    Ok(())
}

/// Drive the issued `BodyLoweringPolicy::ExitAllowed` body recipe against the
/// co-sealed loop-body carrier. This is the located-source counterpart of the
/// raw `verify_exit_allowed_block_with_pre` + `lower_exit_allowed_block_verified`
/// arm; the recipe stays the packaging authority and the driver keeps the
/// ExitAllowed postconditions.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn lower_loop_cond_source_exit_allowed_body<'view, 'ledger: 'view>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    body: &CallableLoopSourceBodyInputV1<'view>,
    recipe: &ExitAllowedBlockRecipe,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let block = CallableLoopSourcePartsBlockV1::located_body(
        &recipe.arena,
        &recipe.block,
        body.clone(),
        &port,
    )
    .map_err(|error| render_source_error(error, error_prefix))?;
    lower_callable_loop_source_parts_block(
        port,
        &block,
        PartsAssociatedBlockModeV1::ExitAllowed,
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        carrier_updates,
        error_prefix,
    )
}
