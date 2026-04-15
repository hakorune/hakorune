//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod composer;
pub(in crate::mir::builder) mod expectations;
pub(in crate::mir::builder) mod normalize;
pub(in crate::mir::builder) mod single_planner;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::branchn::CoreBranchArmPlan;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::edgecfg_facade::{
    ExitKind, Frag,
};
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::PlanLowerer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::planner::{
    build_plan_with_facts, build_plan_with_facts_ctx, tags, Freeze, PlanBuildOutcome,
    PlannerContext,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan, CoreLoopPlan, CorePlan,
    LoopStepMode, LoweredRecipe,
};
