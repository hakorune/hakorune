//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

mod planner_compat;

pub(in crate::mir::builder) mod expectations;
pub(in crate::mir::builder) mod normalize;

#[allow(unused_imports)]
pub(in crate::mir::builder) use self::planner_compat::{
    planner_rule_route_label, tags, try_build_outcome, CoreBranchNPlan, CoreEffectPlan,
    CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan, Freeze, LoopStepMode, LoweredRecipe,
    PlanBuildOutcome, PlanLowerer, PlanRuleId,
};
