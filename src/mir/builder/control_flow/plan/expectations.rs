//! Expectation checks for plan/composer routing (SSOT).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;

pub(in crate::mir::builder) fn should_expect_plan(
    outcome: &PlanBuildOutcome,
    _ctx: &LoopRouteContext,
) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    facts.facts.string_is_integer().is_some()
}
