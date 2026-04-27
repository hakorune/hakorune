//! Compat-only planner/lowering exports for the lower owner surface.
//!
//! Ownership still lives under `plan/`; this module keeps the lower-side wiring
//! grouped explicitly until the actual move happens.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::edgecfg::api::ExitKind;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::planner::{
    tags, Freeze, PlanBuildOutcome,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::single_planner::{
    planner_rule_route_label, try_build_outcome, PlanRuleId,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::{
    CoreBranchArmPlan, CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan,
    CoreLoopPlan, CorePlan, Frag, LoopStepMode, LoweredRecipe,
};
