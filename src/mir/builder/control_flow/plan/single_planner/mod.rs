//! Phase 29ai P5: Single-planner bridge (router → 1 entrypoint)
//!
//! SSOT entrypoint for DomainPlan extraction. Router should call only this.
//! Contract: keep `Result<_, String>` to preserve existing behavior/messages.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;

use super::planner::PlanBuildOutcome;
use super::DomainPlan;

mod rules;
mod rule_order;

pub(in crate::mir::builder) use rule_order::{
    planner_rule_semantic_label, PlanRuleId,
};

pub(in crate::mir::builder) fn try_build_domain_plan_with_outcome(
    ctx: &LoopPatternContext,
) -> Result<(Option<DomainPlan>, PlanBuildOutcome), String> {
    rules::try_build_domain_plan_with_outcome(ctx)
}

pub(in crate::mir::builder) fn try_build_outcome(
    ctx: &LoopPatternContext,
) -> Result<PlanBuildOutcome, String> {
    // Keep legacy tuple API for compatibility; router path consumes outcome only.
    let (_plan, outcome) = rules::try_build_domain_plan_with_outcome(ctx)?;
    Ok(outcome)
}
