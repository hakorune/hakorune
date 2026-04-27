//! Exit-only if lowering - exit-focused if handling.
//!
//! Contains:
//! - lower_exit_only_item
//! - lower_exit_only_if
//! - lower_else_only_exit_if

use super::super::exit as parts_exit;
use super::super::stmt as parts_stmt;
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_branch;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::block::lower_exit_only_block;

// ============================================================================
// Exit-only item lowering
// ============================================================================

pub(super) fn lower_exit_only_item(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_id: crate::mir::builder::control_flow::plan::recipe_tree::BodyId,
    item: &RecipeItem,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let body = arena.get(body_id).ok_or_else(|| {
        format!(
            "[freeze:contract][recipe] invalid_body_id: ctx={}",
            error_prefix
        )
    })?;

    #[allow(unreachable_patterns)]
    match item {
        RecipeItem::Stmt(stmt_ref) => {
            let stmt = body.get_ref(*stmt_ref).ok_or_else(|| {
                format!("{}: missing stmt idx={}", error_prefix, stmt_ref.index())
            })?;
            parts_stmt::lower_return_prelude_stmt(
                builder,
                current_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                stmt,
                error_prefix,
            )
        }
        RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } => {
            let plan = super::super::loop_::lower_loop_v0(
                builder,
                current_bindings,
                cond_view,
                *body_contract,
                arena,
                body_block,
                error_prefix,
            )?;
            Ok(vec![plan])
        }
        RecipeItem::Exit { kind, stmt } => parts_exit::lower_loop_cond_exit_leaf(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            body,
            *kind,
            *stmt,
            error_prefix,
        ),
        RecipeItem::IfV2 {
            if_stmt: _,
            cond_view,
            contract,
            then_block,
            else_block,
        } => match contract {
            IfContractKind::ExitOnly { mode } => lower_exit_only_if(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                cond_view,
                *mode,
                then_block,
                else_block.as_ref().map(|b| b.as_ref()),
                error_prefix,
            ),
            IfContractKind::ExitAllowed {
                mode: IfMode::ElseOnlyExit,
            } => lower_else_only_exit_if(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                cond_view,
                then_block,
                else_block.as_ref().map(|b| b.as_ref()),
                error_prefix,
            ),
            _ => Err(format!(
                "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={}",
                error_prefix
            )),
        },
        _ => Err(format!(
            "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={}",
            error_prefix
        )),
    }
}

// ============================================================================
// Exit-only if lowering
// ============================================================================

fn lower_exit_only_if(
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
    // Mode contract (fail-fast)
    if matches!(mode, IfMode::ExitAll) && else_block.is_none() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
            error_prefix
        ));
    }
    if matches!(mode, IfMode::ExitIf) && else_block.is_some() {
        return Err(format!(
            "[freeze:contract][recipe] if_exit_if_forbids_else: ctx={}",
            error_prefix
        ));
    }

    // Save state at if entry
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Lower then branch
    let then_plans = lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        then_block,
        error_prefix,
    )?;

    // Reset to pre-if state
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    // Lower else branch (ExitAll only)
    let else_plans = match mode {
        IfMode::ExitAll => {
            let eb = else_block.ok_or_else(|| {
                format!(
                    "[freeze:contract][recipe] if_exit_all_requires_else: ctx={}",
                    error_prefix
                )
            })?;
            Some(lower_exit_only_block(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                arena,
                eb,
                error_prefix,
            )?)
        }
        IfMode::ExitIf => None,
        IfMode::ElseOnlyExit => {
            // ElseOnlyExit is handled by lower_else_only_exit_if, not this function
            return Err(format!(
                "[freeze:contract][recipe] else_only_exit_not_in_exit_only_if: ctx={}",
                error_prefix
            ));
        }
    };

    // Reset to pre-if state for condition
    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    // Build if plan (no joins for exit-only)
    lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        Vec::new(),
        error_prefix,
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
fn lower_else_only_exit_if(
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

    // Save state at if entry
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Lower then branch (exit-allowed, may fall through)
    let then_plans = super::block::lower_exit_allowed_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        then_block,
        error_prefix,
    )?;

    // Capture then's final state (this continues after the if)
    let then_map = builder.variable_ctx.variable_map.clone();
    let then_bindings = current_bindings.clone();

    // Reset to pre-if state for else branch
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    // Lower else branch (exit-only, must exit)
    let else_plans = lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        else_block,
        error_prefix,
    )?;

    // Reset to pre-if state for condition lowering
    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;

    // Build if plan (no joins since else exits)
    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        Some(else_plans),
        Vec::new(),
        error_prefix,
    )?;

    // After the if, state is from the then branch (else exits)
    builder.variable_ctx.variable_map = then_map;
    *current_bindings = then_bindings;

    Ok(plans)
}
