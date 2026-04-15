//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! During folderization, recipe compose helpers still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

mod composer_compat;

pub(in crate::mir::builder) use self::composer_compat::{
    compose_match_return_branchn, shadow_pre_plan_guard_error, strict_nested_loop_guard,
    try_compose_core_loop_v2_nested_minimal, MatchReturnPlan, RecipeBody, RecipeComposer,
};
