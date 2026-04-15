//! Expectation checks for lower/composer routing.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::planner::PlanBuildOutcome;

pub(in crate::mir::builder) fn should_expect_plan(
    outcome: &PlanBuildOutcome,
    _ctx: &LoopRouteContext,
) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    facts.facts.string_is_integer().is_some()
}
