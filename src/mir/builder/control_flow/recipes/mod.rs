//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! `RecipeBody` and shape refs are recipes-owned. Recipe compose helpers still
//! live under `plan/` behind a compat surface.
//! Non-`plan/` consumers should depend on this module first.

mod body;
mod composer_compat;
pub(in crate::mir::builder) mod loop_scan_methods_v0;
pub(in crate::mir::builder) mod refs;
pub(in crate::mir::builder) mod scan_loop_segments;

pub(in crate::mir::builder) use self::body::{RecipeBody, StmtIdx, StmtRange};
pub(in crate::mir::builder) use self::composer_compat::{
    compose_match_return_branchn, shadow_pre_plan_guard_error, strict_nested_loop_guard,
    try_compose_core_loop_v2_nested_minimal, MatchReturnPlan, RecipeComposer,
};
