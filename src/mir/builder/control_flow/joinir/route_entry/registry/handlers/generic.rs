use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::recipes::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::types::{route_labels, RouterEnv};
use super::debug_log_recipe_entry;
use crate::mir::builder::control_flow::joinir::route_entry::router::lower_verified_core_plan;

pub(crate) fn route_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    debug_log_recipe_entry(route_labels::GENERIC_LOOP_V1, env);
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.generic_loop_v1().is_none() {
        return Ok(None);
    }
    let core_plan = match RecipeComposer::compose_generic_loop_v1_recipe(builder, facts, ctx) {
        Ok(core_plan) => core_plan,
        Err(_err) if !env.strict_or_dev => return Ok(None),
        Err(err) => return Err(err.to_string()),
    };
    if env.strict_or_dev && facts.nested_loop {
        return lower_verified_core_plan(
            builder,
            ctx,
            env.strict_or_dev,
            outcome.facts.as_ref(),
            core_plan,
            FlowboxVia::Shadow,
        );
    }
    if !env.strict_or_dev {
        if PlanVerifier::verify(&core_plan).is_err() {
            return Ok(None);
        }
        return match PlanLowerer::lower(builder, core_plan, ctx) {
            Ok(value) => Ok(value),
            Err(_) => Ok(None),
        };
    }
    PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
    PlanLowerer::lower(builder, core_plan, ctx)
}

pub(crate) fn route_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    debug_log_recipe_entry(route_labels::GENERIC_LOOP_V0, env);
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.generic_loop_v0().is_none() {
        return Ok(None);
    }
    let core_plan = match RecipeComposer::compose_generic_loop_v0_recipe(builder, facts, ctx) {
        Ok(core_plan) => core_plan,
        Err(_err) if !env.strict_or_dev => return Ok(None),
        Err(err) => return Err(err.to_string()),
    };
    if env.strict_or_dev && facts.nested_loop {
        return lower_verified_core_plan(
            builder,
            ctx,
            env.strict_or_dev,
            outcome.facts.as_ref(),
            core_plan,
            FlowboxVia::Shadow,
        );
    }
    if !env.strict_or_dev {
        if PlanVerifier::verify(&core_plan).is_err() {
            return Ok(None);
        }
        return match PlanLowerer::lower(builder, core_plan, ctx) {
            Ok(value) => Ok(value),
            Err(_) => Ok(None),
        };
    }
    PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
    PlanLowerer::lower(builder, core_plan, ctx)
}
