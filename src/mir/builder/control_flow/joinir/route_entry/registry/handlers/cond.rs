use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::lower::single_planner::{
    planner_rule_route_label, PlanRuleId,
};
use crate::mir::builder::control_flow::recipes::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::common::{
    release_allows_loop_cond_break_continue, release_allows_loop_cond_continue_only,
    release_skips_nested_loop, route_standard,
};
use super::super::types::{PlannerFirstMode, RouterEnv, StandardEntry};

pub(crate) fn route_loop_true_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if release_skips_nested_loop(ctx, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopTrueBreak),
        missing_contract_msg:
            "loop_true_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_true_break_continue_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopTrueBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if !release_allows_loop_cond_break_continue(ctx, outcome, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondBreak),
        missing_contract_msg:
            "loop_cond_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_break_continue_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_only(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if !release_allows_loop_cond_continue_only(ctx, outcome, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueOnly),
        missing_contract_msg:
            "loop_cond_continue_only requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_only_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueOnly),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_with_return(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if release_skips_nested_loop(ctx, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueWithReturn),
        missing_contract_msg:
            "loop_cond_continue_with_return requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_with_return_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueWithReturn),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_return_in_body(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondReturnInBody),
        missing_contract_msg:
            "loop_cond_return_in_body requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_return_in_body_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondReturnInBody),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}
