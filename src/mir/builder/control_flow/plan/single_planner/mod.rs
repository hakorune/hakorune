//! Phase 29ai P5: Single-planner bridge (router → 1 entrypoint)
//!
//! SSOT entrypoint for planner outcome extraction. Router should call only this.
//! Contract: keep `Result<_, String>` to preserve existing behavior/messages.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;

use super::planner::PlanBuildOutcome;

mod rules;
mod rule_order;

pub(in crate::mir::builder) use rule_order::{
    planner_rule_semantic_label, PlanRuleId,
};

pub(in crate::mir::builder) fn try_build_outcome(
    ctx: &LoopPatternContext,
) -> Result<PlanBuildOutcome, String> {
    rules::try_build_outcome(ctx)
}
