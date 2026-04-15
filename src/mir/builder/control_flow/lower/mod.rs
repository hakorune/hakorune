//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod composer;
pub(in crate::mir::builder) mod expectations;
pub(in crate::mir::builder) mod normalize;
pub(in crate::mir::builder) mod planner;
pub(in crate::mir::builder) mod single_planner;
pub(in crate::mir::builder) mod step_mode;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::lowerer::*;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::branchn::CoreBranchArmPlan;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::edgecfg_facade::{
    ExitKind, Frag,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan, CoreLoopPlan,
    CorePlan, LoopStepMode, LoweredRecipe,
};
