//! Phase 29ao P0: CorePlan composer scaffold (CanonicalLoopFacts -> CorePlan)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: coreloop_v0/ and coreloop_v1/ moved to composer/ root

pub(super) mod coreloop_gates;
pub(super) mod coreloop_single_entry;
mod coreloop_v0;
#[cfg(test)]
mod coreloop_v0_tests;
pub(super) mod coreloop_v1;
#[cfg(test)]
mod coreloop_v1_tests;
pub(super) mod coreloop_v2_nested_minimal;
mod branchn_return;
mod shadow_adopt;

#[allow(unused_imports)] // Facade-style re-exports; not all are used in every build/profile.
pub(in crate::mir::builder) use shadow_adopt::{
    strict_nested_loop_guard,
    try_release_adopt_pre_plan,
    try_release_adopt_nested_minimal,
    try_release_adopt_is_integer,
    try_release_adopt_starts_with,
    try_release_adopt_int_to_str,
    try_release_adopt_escape_map,
    try_release_adopt_split_lines,
    try_release_adopt_skip_ws,
    try_release_adopt_generic_loop_v0,
    try_shadow_adopt_pre_plan,
    try_shadow_adopt_is_integer,
    try_shadow_adopt_starts_with,
    try_shadow_adopt_int_to_str,
    try_shadow_adopt_escape_map,
    try_shadow_adopt_split_lines,
    try_shadow_adopt_skip_ws,
    try_shadow_adopt_generic_loop_v0,
    try_shadow_adopt_nested_minimal,
    PrePlanShadowOutcome,
    ShadowAdoptOutcome,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use coreloop_single_entry::try_compose_core_loop_from_facts;
#[allow(unused_imports)]
pub(in crate::mir::builder) use coreloop_v0::try_compose_core_loop_v0;
pub(in crate::mir::builder) use branchn_return::{
    compose_match_return_branchn, MatchReturnPlan,
};
