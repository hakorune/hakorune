//! Expectation checks for plan/composer routing (SSOT).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::DomainPlan;

pub(in crate::mir::builder) fn should_expect_shadow_adopt(
    domain_plan: &DomainPlan,
    outcome: &PlanBuildOutcome,
    _ctx: &LoopPatternContext,
) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    let _ = domain_plan;
    let _ = facts;
    false
}

pub(in crate::mir::builder) fn should_expect_plan(
    outcome: &PlanBuildOutcome,
    _ctx: &LoopPatternContext,
) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    if facts.facts.pattern_is_integer.is_some() {
        return true;
    }
    false
}
