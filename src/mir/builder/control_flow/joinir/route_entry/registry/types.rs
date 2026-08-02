use super::super::router::LoopRouteContext;
use super::execution_witness::RouteExecutionWitnessV1;
use super::route_id::LoopRouteId;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::{CorePlan, Freeze, PlanRuleId};
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

pub(crate) mod route_labels {
    pub(crate) const LOOP_CHAR_MAP: &str = "loop_char_map";
    pub(crate) const LOOP_ARRAY_JOIN: &str = "loop_array_join";
    pub(crate) const NESTED_LOOP_MINIMAL: &str = "nested_loop_minimal";
    pub(crate) const GENERIC_LOOP_V0: &str = "generic_loop_v0";
    pub(crate) const GENERIC_LOOP_V1: &str = "generic_loop_v1";
}

pub(crate) struct RouterEnv {
    pub strict_or_dev: bool,
    pub planner_required: bool,
    pub has_body_local: bool,
}

pub(crate) type PredicateFn = fn(&CanonicalLoopFacts) -> bool;
pub(crate) type RouteFn = fn(
    &mut MirBuilder,
    &LoopRouteContext,
    Option<&CanonicalLoopFacts>,
    &RouteExecutionWitnessV1<'_>,
) -> Result<Option<ValueId>, String>;

pub(crate) struct Entry {
    pub id: LoopRouteId,
    pub name: &'static str,
    pub predicate: PredicateFn,
    pub route: Option<RouteFn>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LegacyRouteSuccess {
    pub route: LoopRouteId,
    pub value: ValueId,
}

#[derive(Clone, Copy)]
pub(crate) enum PlannerFirstMode {
    Never,
    StrictOrDev,
    StrictOrDevPlannerRequired,
}

pub(crate) type ComposeFn =
    fn(&mut MirBuilder, &CanonicalLoopFacts, &LoopRouteContext) -> Result<CorePlan, Freeze>;

pub(crate) struct StandardEntry {
    pub route_label: &'static str,
    pub missing_contract_msg: &'static str,
    pub compose: ComposeFn,
    pub planner_required_only: bool,
    pub skip_without_contract: bool,
    pub planner_first: PlannerFirstMode,
    pub plan_rule: Option<PlanRuleId>,
    pub flowbox_via_strict: FlowboxVia,
    pub flowbox_via_release: FlowboxVia,
}
