use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::builder::control_flow::plan::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use super::super::router::LoopPatternContext;

pub(crate) struct RouterEnv {
    pub strict_or_dev: bool,
    pub planner_required: bool,
    pub has_loopbodylocal: bool,
}

pub(crate) type PredicateFn = fn(&CanonicalLoopFacts) -> bool;
pub(crate) type RouteFn = fn(
    &mut MirBuilder,
    &LoopPatternContext,
    &PlanBuildOutcome,
    &RouterEnv,
) -> Result<Option<ValueId>, String>;

pub(crate) struct Entry {
    pub name: &'static str,
    pub predicate: PredicateFn,
    pub route: Option<RouteFn>,
}

#[derive(Clone, Copy)]
pub(crate) enum PlannerFirstMode {
    Never,
    StrictOrDev,
    StrictOrDevPlannerRequired,
}

pub(crate) type ComposeFn = fn(
    &mut MirBuilder,
    &CanonicalLoopFacts,
    &LoopPatternContext,
) -> Result<CorePlan, Freeze>;

pub(crate) struct StandardEntry {
    pub missing_contract_msg: &'static str,
    pub compose: ComposeFn,
    pub planner_required_only: bool,
    pub skip_without_contract: bool,
    pub planner_first: PlannerFirstMode,
    pub plan_rule: Option<PlanRuleId>,
    pub flowbox_via_strict: FlowboxVia,
    pub flowbox_via_release: FlowboxVia,
}
