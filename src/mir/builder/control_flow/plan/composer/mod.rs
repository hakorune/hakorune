//! Phase 29ao P0: CorePlan composer scaffold (CanonicalLoopFacts -> CorePlan)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Old cfg-test versioned coreloop shelves retired in 291x-754.

mod branchn_return;
pub(super) mod coreloop_gates;
pub(super) mod coreloop_v2_nested_minimal;
mod shadow_adopt;

pub(in crate::mir::builder) use branchn_return::{compose_match_return_branchn, MatchReturnPlan};
pub(in crate::mir::builder) use coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal;
pub(in crate::mir::builder) use shadow_adopt::{
    shadow_pre_plan_guard_error, strict_nested_loop_guard,
};
