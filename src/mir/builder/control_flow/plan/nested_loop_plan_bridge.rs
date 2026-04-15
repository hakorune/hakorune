//! Shared recipe-first fallback bridge for nested-loop lowering.

use crate::ast::ASTNode;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_preheader::apply_nested_loop_preheader_freshness;
use crate::mir::builder::control_flow::plan::nested_loop_plan_recipe_fallback::try_compose_nested_loop_recipe_fallback;
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_nested_loop_plan_with_recipe_first_bridge(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
    stage: &str,
    tag: &str,
) -> Result<LoweredRecipe, String> {
    if let Ok(plan) =
        nested_loop_depth1::lower_nested_loop_depth1_any(builder, condition, body, error_prefix)
    {
        return Ok(apply_nested_loop_preheader_freshness(builder, plan));
    }

    let nested_ctx =
        LoopRouteContext::new(condition, body, ctx.func_name, ctx.debug, ctx.in_static_box);
    let outcome = single_planner::try_build_outcome(&nested_ctx)?;
    plan_trace::trace_outcome_snapshot(
        stage,
        false,
        outcome.facts.is_some(),
        outcome.recipe_contract.is_some(),
    );

    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if let Some(recipe) = try_compose_nested_loop_recipe_fallback(
        builder,
        &outcome,
        &nested_ctx,
        stage,
        planner_required,
    )? {
        return Ok(recipe);
    }

    plan_trace::trace_outcome_path(stage, "freeze_no_plan");
    Err(format!(
        "[freeze:contract][{tag}] nested loop has no plan: ctx={error_prefix}"
    ))
}
