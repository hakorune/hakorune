use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::Freeze;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::router::{lower_verified_core_plan, LoopRouteContext};
use super::execution_witness::{RouteAttemptOutcomeV1, RouteExecutionAttemptV1};
use super::types::StandardEntry;
use super::utils::emit_planner_first;

mod generic;
mod routes;
pub(crate) use routes::*;

pub(crate) fn route_generic_loop_v0_at_attempt(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    generic::route_generic_loop_v0(builder, ctx, compose_facts, attempt)
        .map(RouteAttemptOutcomeV1::from_legacy)
}

pub(crate) fn route_generic_loop_v1_at_attempt(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    generic::route_generic_loop_v1(builder, ctx, compose_facts, attempt)
        .map(RouteAttemptOutcomeV1::from_legacy)
}

fn debug_log_recipe_entry(route_label: &str, attempt: &RouteExecutionAttemptV1<'_, '_>) {
    if !crate::config::env::joinir_dev::debug_enabled() {
        return;
    }
    let entry_state = if attempt.planner_required() {
        "recipe_contract enforced"
    } else {
        "recipe-only entry"
    };
    let ring0 = crate::runtime::get_global_ring0();
    ring0
        .log
        .debug(&format!("[recipe:entry] {}: {}", route_label, entry_state));
}

fn route_standard(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
    entry: &StandardEntry,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if entry.planner_required_only && !attempt.planner_required() {
        return Ok(RouteAttemptOutcomeV1::Retry);
    }
    if attempt.planner_required() && !attempt.recipe_contract_present() {
        return Err(Freeze::contract(entry.missing_contract_msg).to_string());
    }
    if let Some(policy) = entry.absent_contract_decline {
        if policy.declines(
            attempt.planner_required(),
            attempt.recipe_contract_present(),
        ) {
            return attempt
                .issue_shared_absent_contract_decline(policy)
                .map(RouteAttemptOutcomeV1::SharedAbsentContractDeclined);
        }
    }

    if let Some(rule) = entry.plan_rule {
        emit_planner_first_from_attempt(entry.planner_first, attempt, rule);
    }
    debug_log_recipe_entry(entry.route_label, attempt);

    let facts = compose_facts.expect("facts present for route_standard");
    let core_plan = with_standard_compose_binding_boundary(builder, |builder| {
        (entry.compose)(builder, facts, ctx)
    })
    .map_err(|freeze| freeze.to_string())?;
    let via = if attempt.strict_or_dev() {
        entry.flowbox_via_strict
    } else {
        entry.flowbox_via_release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        via,
    )
    .map(RouteAttemptOutcomeV1::from_legacy)
}

fn with_standard_compose_binding_boundary<T, E, F>(builder: &mut MirBuilder, f: F) -> Result<T, E>
where
    F: FnOnce(&mut MirBuilder) -> Result<T, E>,
{
    let saved = builder.function_state.variable_ctx.variable_map.clone();
    let result = f(builder);
    builder.function_state.variable_ctx.variable_map = saved;
    result
}

fn emit_planner_first_from_attempt(
    mode: super::types::PlannerFirstMode,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
    rule: crate::mir::builder::control_flow::lower::PlanRuleId,
) {
    let env = super::types::RouterEnv {
        strict_or_dev: attempt.strict_or_dev(),
        planner_required: attempt.planner_required(),
        has_body_local: attempt.has_body_local(),
    };
    emit_planner_first(mode, &env, rule);
}

fn release_skips_nested_loop(
    ctx: &LoopRouteContext,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> bool {
    !attempt.planner_required() && detect_nested_loop(ctx.body)
}

fn release_allows_loop_cond_continue_only(
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> bool {
    if attempt.planner_required() || !detect_nested_loop(ctx.body) {
        return true;
    }
    compose_facts
        .and_then(|facts| facts.facts.loop_cond_continue_only())
        .is_some()
}

fn release_allows_loop_cond_break_continue(
    _ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> bool {
    if attempt.planner_required() {
        return true;
    }
    let Some(facts) = compose_facts.and_then(|facts| facts.facts.loop_cond_break_continue()) else {
        return false;
    };
    // Release route allows nested-loop shapes only when loop_cond_break_continue
    // found an explicit exit-driven form. Keep passive cluster forms blocked.
    facts.release_allowed()
}

#[cfg(test)]
mod tests {
    use super::with_standard_compose_binding_boundary;
    use crate::mir::builder::MirBuilder;
    use crate::mir::ValueId;

    #[test]
    fn standard_route_compose_restores_variable_map_before_lower() {
        let mut builder = MirBuilder::new();
        let outer = ValueId(10);
        let scratch = ValueId(20);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("outer".to_string(), outer);

        let result: Result<(), ()> =
            with_standard_compose_binding_boundary(&mut builder, |builder| {
                builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .insert("scratch".to_string(), scratch);
                builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .insert("outer".to_string(), scratch);
                Ok(())
            });

        assert!(result.is_ok());
        assert_eq!(
            builder
                .function_state
                .variable_ctx
                .variable_map
                .get("outer"),
            Some(&outer)
        );
        assert!(
            !builder
                .function_state
                .variable_ctx
                .variable_map
                .contains_key("scratch"),
            "compose scratch binding must not be visible to PlanLowerer pre-loop snapshots"
        );
    }
}
