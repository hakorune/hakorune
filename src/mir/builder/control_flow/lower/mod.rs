//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

mod planner_compat;

pub(in crate::mir::builder) mod expectations;
pub(in crate::mir::builder) mod normalize;

#[allow(unused_imports)]
pub(in crate::mir::builder) use self::planner_compat::{
    loop_body_has_nested_loop, planner_rule_route_label, router_shadow_pre_plan_guard_error, tags,
    try_build_outcome, CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan,
    CorePlan, Freeze, LoopStepMode, LoweredRecipe, PlanBuildOutcome, PlanLowerer, PlanRuleId,
};
