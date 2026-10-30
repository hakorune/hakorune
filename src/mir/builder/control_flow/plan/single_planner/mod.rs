//! Phase 29ai P5: Single-planner bridge (router → 1 entrypoint)
//!
//! SSOT entrypoint for planner outcome extraction. Router should call only this.
//! Contract: keep `Result<_, String>` to preserve existing behavior/messages.

use super::planner::PlanBuildOutcome;

mod input;
mod rule_order;
mod rules;

pub(in crate::mir::builder) use input::CallableLoopFactsPlannerInputV1;

pub(in crate::mir::builder) use rule_order::{
    planner_rule_route_label, planner_rule_semantic_label, planner_rule_tag_name, PlanRuleId,
};

/// Source-aware Facts/Recipe entry with no structural route context.
///
/// The input owns only diagnostic labels and borrows the exact AST slice. The
/// route-neutral kernel must not classify a route or enter the registry.
pub(in crate::mir::builder) fn try_build_source_outcome(
    input: CallableLoopFactsPlannerInputV1<'_>,
) -> Result<PlanBuildOutcome, String> {
    rules::try_build_source_outcome(input)
}
