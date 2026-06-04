//! Compat-only planner/lowering exports for the lower owner surface.
//!
//! Ownership still lives under `plan/`; this module keeps the lower-side wiring
//! grouped explicitly until the actual move happens.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

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
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan,
    LoopStepMode, LoweredRecipe,
};

pub(in crate::mir::builder) fn router_shadow_pre_plan_guard_error(
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
) -> Option<String> {
    crate::mir::builder::control_flow::plan::composer::shadow_pre_plan_guard_error(ctx, outcome)
}

pub(in crate::mir::builder) fn loop_body_has_nested_loop(body: &[ASTNode]) -> bool {
    crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop(body)
}
