//! Group-if and nested-loop route lowering functions.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_recipe::{
    ContinueOnlyRecipe, ContinueOnlyStmtRecipe,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_co_block::lower_continue_only_block;
use super::loop_cond_co_helpers::{get_body_span, get_body_stmt, map_mutates_existing_vars};

const LOOP_COND_CONTINUE_ONLY_ERR: &str = "[normalizer] loop_cond_continue_only";

/// Lower a group-if statement.
pub(super) fn lower_group_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    if_stmt: &StmtRef,
    body: &BodyView<'_>,
    then_body: &ContinueOnlyRecipe,
    else_body: Option<&ContinueOnlyRecipe>,
) -> Result<Vec<LoweredRecipe>, String> {
    let stmt = get_body_stmt(body, *if_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
    let ASTNode::If { condition, .. } = stmt else {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: recipe if mismatch (GroupIf)"
        ));
    };
    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Then
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let then_view = BodyView::Recipe(&then_body.body);
    let then_plans = lower_continue_only_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &then_view,
        &then_body.items,
    )?;
    let then_map = builder.variable_ctx.variable_map.clone();

    // Else
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let else_plans = match else_body {
        Some(body) => {
            let else_view = BodyView::Recipe(&body.body);
            Some(lower_continue_only_block(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                &else_view,
                &body.items,
            )?)
        }
        None => None,
    };
    let else_map = builder.variable_ctx.variable_map.clone();

    // Fallthrough mutation is out-of-scope: no join generation here.
    if map_mutates_existing_vars(&pre_if_map, &then_map)
        || map_mutates_existing_vars(&pre_if_map, &else_map)
    {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: group-if fallthrough mutates existing vars (join out-of-scope)"
        ));
    }

    builder.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;
    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
    let mut else_plans_once = else_plans;
    let has_else = else_plans_once.is_some();
    let mut lower_else =
        |_builder: &mut MirBuilder, _bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            Ok(else_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_ONLY_ERR}: internal error: else_plans consumed twice")
            })?)
        };
    let lower_else: Option<
        &mut dyn FnMut(
            &mut MirBuilder,
            &mut BTreeMap<String, crate::mir::ValueId>,
        ) -> Result<Vec<LoweredRecipe>, String>,
    > = if has_else {
        Some(&mut lower_else)
    } else {
        None
    };

    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_CONTINUE_ONLY_ERR,
        &mut |_builder, _bindings| {
            Ok(then_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_ONLY_ERR}: internal error: then_plans consumed twice")
            })?)
        },
        lower_else,
        &|_name, _bindings| false,
    )
}

/// Phase 29bq: Lower ContinueIfNestedLoop recipe.
/// Pattern: `if <outer_cond> { <prelude>; loop(...) { ... }; <postlude>; continue }`
#[allow(clippy::too_many_arguments)]
pub(super) fn lower_continue_if_nested_loop(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    if_stmt: &StmtRef,
    body: &BodyView<'_>,
    inner_loop_prelude_span: StmtSpan,
    inner_loop_prelude_items: &[ContinueOnlyStmtRecipe],
    inner_loop_body: &RecipeBody,
    inner_loop_stmt: &StmtRef,
    inner_loop_postlude_span: StmtSpan,
    inner_loop_postlude_items: &[ContinueOnlyStmtRecipe],
) -> Result<Vec<LoweredRecipe>, String> {
    let if_node = get_body_stmt(body, *if_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
    let ASTNode::If {
        condition: outer_condition,
        then_body,
        ..
    } = if_node
    else {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: recipe if mismatch (ContinueIfNestedLoop)"
        ));
    };
    let saved_map = builder.variable_ctx.variable_map.clone();
    let saved_bindings = current_bindings.clone();

    builder.variable_ctx.variable_map = saved_map.clone();
    let mut branch_bindings = saved_bindings.clone();

    // Lower prelude statements
    let then_view = BodyView::Slice(then_body);
    let prelude_body = get_body_span(
        &then_view,
        inner_loop_prelude_span,
        LOOP_COND_CONTINUE_ONLY_ERR,
        "inner loop prelude",
    )?;
    let prelude_view = BodyView::Slice(prelude_body);
    let mut then_plans = lower_continue_only_block(
        builder,
        &mut branch_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &prelude_view,
        inner_loop_prelude_items,
    )?;

    // Lower nested loop using nested_loop_depth1
    let inner_view = BodyView::Recipe(inner_loop_body);
    let inner_loop_node =
        get_body_stmt(&inner_view, *inner_loop_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
    let (loop_condition, loop_body) = match inner_loop_node {
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => (condition.as_ref(), body.as_slice()),
        ASTNode::ForRange { .. } => {
            return Err(format!(
                "{LOOP_COND_CONTINUE_ONLY_ERR}: ForRange in nested loop not supported"
            ));
        }
        _ => {
            return Err(format!(
                "{LOOP_COND_CONTINUE_ONLY_ERR}: expected Loop/While in ContinueIfNestedLoop"
            ));
        }
    };

    let nested_plan = lower_nested_loop_depth1_any(
        builder,
        loop_condition,
        loop_body,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;
    then_plans.push(nested_plan);

    // Lower postlude statements
    let postlude_body = get_body_span(
        &then_view,
        inner_loop_postlude_span,
        LOOP_COND_CONTINUE_ONLY_ERR,
        "inner loop postlude",
    )?;
    if crate::config::env::joinir_dev::debug_enabled() {
        let (postlude_start, postlude_end) = inner_loop_postlude_span.indices();
        let loop_idx = postlude_start.saturating_sub(1);
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:continue_only] ctx=loop_cond_continue_only loop_idx={} prelude_len={} postlude_len={} then_len={} postlude_span=({},{})",
            loop_idx,
            prelude_body.len(),
            postlude_body.len(),
            then_body.len(),
            postlude_start,
            postlude_end
        ));
    }
    let postlude_view = BodyView::Slice(postlude_body);
    let mut post_plans = lower_continue_only_block(
        builder,
        &mut branch_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &postlude_view,
        inner_loop_postlude_items,
    )?;
    then_plans.append(&mut post_plans);

    // Add continue exit
    let exit = parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        &branch_bindings,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;
    then_plans.push(CorePlan::Exit(exit));

    builder.variable_ctx.variable_map = saved_map;
    *current_bindings = saved_bindings;

    let cond_view = CondBlockView::from_expr(outer_condition);
    let mut then_plans_once = Some(then_plans);
    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_CONTINUE_ONLY_ERR,
        &mut |_builder, _bindings| {
            Ok(then_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_ONLY_ERR}: internal error: then_plans consumed twice")
            })?)
        },
        None,
        &|_name, _bindings| false,
    )
}
