//! Shared nested loop plan lowering (facts/recipe-first).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::nested_loop_plan_bridge::{
    lower_nested_loop_plan_with_recipe_first_bridge,
    try_compose_loop_cond_continue_with_return_recipe_bridge,
};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_loop_cond_continue_with_return_recipe(
    builder: &mut MirBuilder,
    outcome: &PlanBuildOutcome,
    nested_ctx: &LoopRouteContext,
    stage: &str,
    planner_required: bool,
) -> Result<Option<LoweredRecipe>, String> {
    try_compose_loop_cond_continue_with_return_recipe_bridge(
        builder,
        outcome,
        nested_ctx,
        stage,
        planner_required,
    )
}

pub(in crate::mir::builder) fn lower_nested_loop_plan_with_recipe_first(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
    tag: &str,
) -> Result<LoweredRecipe, String> {
    lower_nested_loop_plan_with_recipe_first_bridge(builder, condition, body, ctx, error_prefix, tag)
}
