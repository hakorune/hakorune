//! Compat-only planner/lowering exports for the lower owner surface.
//!
//! Ownership still lives under `plan/`; this module keeps the lower-side wiring
//! grouped explicitly until the actual move happens.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::edgecfg_facade::ExitKind;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::planner::{
    build_plan_with_facts, build_plan_with_facts_ctx, tags, Freeze, PlanBuildOutcome,
    PlannerContext,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::single_planner::{
    planner_rule_route_label, planner_rule_semantic_label, planner_rule_tag_name,
    try_build_outcome, PlanRuleId,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan, CoreLoopPlan, CorePlan,
    LoopStepMode, LoweredRecipe,
};
