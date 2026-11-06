//! Compat-only planner/lowering exports for the lower owner surface.
//!
//! Ownership still lives under `plan/`; this module keeps the lower-side wiring
//! grouped explicitly until the actual move happens.


pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::planner::Freeze;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan,
    LoopStepMode, LoweredRecipe,
};
