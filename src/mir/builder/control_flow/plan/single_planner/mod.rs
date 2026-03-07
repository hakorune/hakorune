//! Phase 29ai P5: Single-planner bridge (router → 1 entrypoint)
//!
//! SSOT entrypoint for planner outcome extraction. Router should call only this.
//! Contract: keep `Result<_, String>` to preserve existing behavior/messages.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

use super::planner::PlanBuildOutcome;

mod rules;
mod rule_order;

pub(in crate::mir::builder) use rule_order::{
    planner_rule_route_label, planner_rule_semantic_label, planner_rule_tag_name, PlanRuleId,
};

pub(in crate::mir::builder) fn try_build_outcome(
    ctx: &LoopRouteContext,
) -> Result<PlanBuildOutcome, String> {
    rules::try_build_outcome(ctx)
}
