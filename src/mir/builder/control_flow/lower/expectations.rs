//! Expectation checks for lower/composer routing.

use super::PlanBuildOutcome;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

pub(in crate::mir::builder) fn should_expect_plan(
    outcome: &PlanBuildOutcome,
    _ctx: &LoopRouteContext,
) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    facts.facts.string_is_integer().is_some()
}
