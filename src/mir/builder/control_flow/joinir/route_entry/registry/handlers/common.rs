use crate::mir::builder::control_flow::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    lower_verified_core_plan, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakAcceptKind;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::types::{RouterEnv, StandardEntry};
use super::super::utils::emit_planner_first;

pub(super) fn debug_log_recipe_entry(route_label: &str, env: &RouterEnv) {
    if !crate::config::env::joinir_dev::debug_enabled() {
        return;
    }
    let entry_state = if env.planner_required {
        "recipe_contract enforced"
    } else {
        "recipe-only entry"
    };
    let ring0 = crate::runtime::get_global_ring0();
    ring0
        .log
        .debug(&format!("[recipe:entry] {}: {}", route_label, entry_state));
}

pub(super) fn route_standard(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
    entry: &StandardEntry,
) -> Result<Option<ValueId>, String> {
    if entry.planner_required_only && !env.planner_required {
        return Ok(None);
    }
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(entry.missing_contract_msg).to_string());
    }
    if !env.planner_required && outcome.recipe_contract.is_none() && entry.skip_without_contract {
        return Ok(None);
    }

    if let Some(rule) = entry.plan_rule {
        emit_planner_first(entry.planner_first, env, rule);
    }
    debug_log_recipe_entry(entry.route_label, env);

    let facts = outcome
        .facts
        .as_ref()
        .expect("facts present for route_standard");
    let core_plan = (entry.compose)(builder, facts, ctx).map_err(|freeze| freeze.to_string())?;
    let via = if env.strict_or_dev {
        entry.flowbox_via_strict
    } else {
        entry.flowbox_via_release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        via,
    )
}

pub(super) fn release_skips_nested_loop(ctx: &LoopRouteContext, env: &RouterEnv) -> bool {
    !env.planner_required && detect_nested_loop(ctx.body)
}

pub(super) fn release_allows_loop_cond_continue_only(
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> bool {
    if env.planner_required || !detect_nested_loop(ctx.body) {
        return true;
    }
    outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.loop_cond_continue_only())
        .is_some()
}

pub(super) fn release_allows_loop_cond_break_continue(
    _ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> bool {
    if env.planner_required {
        return true;
    }
    let Some(facts) = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.loop_cond_break_continue())
    else {
        return false;
    };
    // Release route allows nested-loop shapes only when loop_cond_break_continue
    // found an explicit exit-driven form. Keep passive cluster forms blocked.
    !matches!(
        facts.accept_kind,
        LoopCondBreakAcceptKind::NestedLoopOnly | LoopCondBreakAcceptKind::ProgramBlockNoExit
    )
}
