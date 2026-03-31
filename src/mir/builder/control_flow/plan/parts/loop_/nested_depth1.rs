use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::try_lower_nested_loop_depth1;
use crate::mir::builder::control_flow::plan::scan_loop_segments::NestedLoopRecipe;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::super::verify;

/// Lower a nested `loop(cond) { ... }` statement when the body is already represented
/// as a stmt-only `RecipeBlock` (Facts-provided payload).
///
/// This is a thin adapter to keep `features/*` free from re-scanning the inner loop body.
/// Behavior is intentionally aligned with the existing nested-loop lowering path.
pub(in crate::mir::builder) fn lower_nested_loop_depth1_stmt_only(
    builder: &mut MirBuilder,
    cond_view: &CondBlockView,
    body_recipe: &StmtOnlyBlockRecipe,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    if !cond_view.prelude_stmts.is_empty() {
        return Err(format!(
            "[freeze:contract][recipe] nested_loop_cond_prelude_unsupported: ctx={}",
            error_prefix
        ));
    }

    verify::verify_stmt_only_block_contract_if_enabled(
        &body_recipe.arena,
        &body_recipe.block,
        error_prefix,
    )?;

    let body = body_recipe
        .arena
        .get(body_recipe.block.body_id)
        .ok_or_else(|| {
            format!(
                "[freeze:contract][recipe] invalid_body_id: ctx={}",
                error_prefix
            )
        })?;

    match lower_nested_loop_depth1_any(builder, &cond_view.tail_expr, &body.body, error_prefix) {
        Ok(plan) => Ok(plan),
        Err(any_err) => match try_lower_nested_loop_depth1(
            builder,
            &cond_view.tail_expr,
            &body.body,
            error_prefix,
        )? {
            Some(plan) => Ok(plan),
            None => Err(any_err),
        },
    }
}

/// Lower a nested loop represented as `scan_loop_segments::NestedLoopRecipe` when the nested body
/// is available as a stmt-only recipe payload.
///
/// This is a scan-pipeline SSOT entry: it prefers the nested-loop stmt-only fastpath and otherwise
/// asks the caller to fall back to the single-planner route.
pub(in crate::mir::builder) fn lower_nested_loop_recipe_stmt_only(
    builder: &mut MirBuilder,
    _current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    _carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    _break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let Some(body_stmt_only) = nested.body_stmt_only.as_ref() else {
        return Ok(None);
    };

    if !nested.cond_view.prelude_stmts.is_empty() {
        return Err(format!(
            "[freeze:contract][recipe] nested_loop_cond_prelude_unsupported: ctx={}",
            error_prefix
        ));
    }

    let plan = lower_nested_loop_depth1_stmt_only(
        builder,
        &nested.cond_view,
        body_stmt_only,
        error_prefix,
    )?;
    Ok(Some(vec![plan]))
}

