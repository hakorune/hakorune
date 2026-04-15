//! Compat-only recipe/composer exports for the recipes owner surface.
//!
//! Ownership still lives under `plan/`; this module keeps that wiring grouped
//! explicitly on the recipes side until the actual move happens.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::composer::{
    compose_match_return_branchn, shadow_pre_plan_guard_error, strict_nested_loop_guard,
    try_compose_core_loop_v2_nested_minimal, MatchReturnPlan,
};
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
