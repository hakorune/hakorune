//! Shared nested loop plan lowering (loop plan payload → CorePlan, with recipe-first fallback).

use crate::ast::ASTNode;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_nested_loop_plan_with_recipe_first(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
    tag: &str,
) -> Result<LoweredRecipe, String> {
    let nested_ctx = LoopRouteContext::new(
        condition,
        body,
        ctx.func_name,
        ctx.debug,
        ctx.in_static_box,
    );
    let outcome = single_planner::try_build_outcome(&nested_ctx)?;
    plan_trace::trace_outcome_snapshot(
        "nested_loop_plan_with_recipe_first",
        outcome.plan.is_some(),
        outcome.facts.is_some(),
        outcome.recipe_contract.is_some(),
    );

    if let Some(loop_plan) = outcome.plan.as_ref().cloned() {
        plan_trace::trace_outcome_path("nested_loop_plan_with_recipe_first", "planner_payload");
        return PlanNormalizer::normalize(builder, loop_plan, &nested_ctx);
    }

    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if planner_required {
        if let Some(facts) = outcome.facts.as_ref() {
            if facts.facts.loop_cond_break_continue.is_some() && outcome.recipe_contract.is_some()
            {
                plan_trace::trace_outcome_path(
                    "nested_loop_plan_with_recipe_first",
                    "recipe_loop_cond_break_continue",
                );
                return RecipeComposer::compose_loop_cond_break_continue_recipe(
                    builder,
                    facts,
                    &nested_ctx,
                )
                .map_err(|e| e.to_string());
            }
        }
    }

    plan_trace::trace_outcome_path("nested_loop_plan_with_recipe_first", "freeze_no_plan");
    Err(format!(
        "[freeze:contract][{tag}] nested loop has no plan: ctx={error_prefix}"
    ))
}
