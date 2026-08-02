use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::execution_witness::{
    PostEffectRetryDebtV1, PreEffectDeclineReasonV1, RouteAttemptOutcomeV1, RouteExecutionAttemptV1,
};
use super::super::types::route_labels;
use super::debug_log_recipe_entry;
use crate::mir::builder::control_flow::joinir::route_entry::router::lower_verified_core_plan;

pub(crate) fn route_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    debug_log_recipe_entry(route_labels::GENERIC_LOOP_V1, attempt);
    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            PreEffectDeclineReasonV1::GenericFactsUnavailable,
        ));
    };
    if facts.facts.generic_loop_v1().is_none() {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            PreEffectDeclineReasonV1::GenericFactsUnavailable,
        ));
    }
    let core_plan = RecipeComposer::compose_generic_loop_v1_recipe(builder, facts, ctx)
        .map_err(|error| error.to_string())?;
    if attempt.strict_or_dev() {
        return lower_verified_core_plan(
            builder,
            ctx,
            attempt.strict_or_dev(),
            compose_facts,
            core_plan,
            FlowboxVia::Shadow,
        )
        .map(|result| match result {
            Some(value) => RouteAttemptOutcomeV1::Succeeded(value),
            None => {
                RouteAttemptOutcomeV1::PostEffectRetryDebt(PostEffectRetryDebtV1::GenericLegacy)
            }
        });
    }
    if PlanVerifier::verify(&core_plan).is_err() {
        return Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(
            PostEffectRetryDebtV1::GenericLegacy,
        ));
    }
    match PlanLowerer::lower(builder, core_plan, ctx) {
        Ok(Some(value)) => Ok(RouteAttemptOutcomeV1::Succeeded(value)),
        Ok(None) | Err(_) => Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(
            PostEffectRetryDebtV1::GenericLegacy,
        )),
    }
}

pub(crate) fn route_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    debug_log_recipe_entry(route_labels::GENERIC_LOOP_V0, attempt);
    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            PreEffectDeclineReasonV1::GenericFactsUnavailable,
        ));
    };
    if facts.facts.generic_loop_v0().is_none() {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            PreEffectDeclineReasonV1::GenericFactsUnavailable,
        ));
    }
    let core_plan = RecipeComposer::compose_generic_loop_v0_recipe(builder, facts, ctx)
        .map_err(|error| error.to_string())?;
    if attempt.strict_or_dev() {
        return lower_verified_core_plan(
            builder,
            ctx,
            attempt.strict_or_dev(),
            compose_facts,
            core_plan,
            FlowboxVia::Shadow,
        )
        .map(|result| match result {
            Some(value) => RouteAttemptOutcomeV1::Succeeded(value),
            None => {
                RouteAttemptOutcomeV1::PostEffectRetryDebt(PostEffectRetryDebtV1::GenericLegacy)
            }
        });
    }
    if PlanVerifier::verify(&core_plan).is_err() {
        return Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(
            PostEffectRetryDebtV1::GenericLegacy,
        ));
    }
    match PlanLowerer::lower(builder, core_plan, ctx) {
        Ok(Some(value)) => Ok(RouteAttemptOutcomeV1::Succeeded(value)),
        Ok(None) | Err(_) => Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(
            PostEffectRetryDebtV1::GenericLegacy,
        )),
    }
}
