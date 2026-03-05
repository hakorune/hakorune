//! Phase 29ao P0: CorePlan composer scaffold (CanonicalLoopFacts -> CorePlan)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: coreloop_v0/ and coreloop_v1/ moved to composer/ root

pub(super) mod coreloop_gates;
#[cfg(test)]
pub(super) mod coreloop_single_entry;
#[cfg(test)]
mod coreloop_v0;
#[cfg(test)]
mod coreloop_v0_tests;
#[cfg(test)]
pub(super) mod coreloop_v1;
#[cfg(test)]
mod coreloop_v1_tests;
pub(super) mod coreloop_v2_nested_minimal;
mod branchn_return;
mod shadow_adopt;

pub(in crate::mir::builder) use shadow_adopt::{
    strict_nested_loop_guard,
    shadow_pre_plan_guard_error,
};
pub(in crate::mir::builder) use coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal;
pub(in crate::mir::builder) use branchn_return::{
    compose_match_return_branchn, MatchReturnPlan,
};
