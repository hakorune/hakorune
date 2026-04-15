//! Compatibility owner surface for lowering-side composition helpers.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::composer::{
    compose_match_return_branchn, shadow_pre_plan_guard_error, strict_nested_loop_guard,
    try_compose_core_loop_v2_nested_minimal, MatchReturnPlan,
};
