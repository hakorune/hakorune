//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! During folderization, recipe compose helpers still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::composer::{
    compose_match_return_branchn, shadow_pre_plan_guard_error, strict_nested_loop_guard,
    try_compose_core_loop_v2_nested_minimal, MatchReturnPlan,
};
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
