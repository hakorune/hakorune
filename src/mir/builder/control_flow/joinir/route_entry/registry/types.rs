use super::super::router::LoopRouteContext;
use super::execution_witness::{RouteAttemptOutcomeV1, RouteExecutionAttemptV1};
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
    &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String>;

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

/// The only routes whose release, absent-contract decline is a shared policy.
///
/// This is intentionally an exact vocabulary rather than a boolean route
/// flag. Adding another route requires an explicit policy decision here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SharedAbsentContractDeclineRouteV1 {
    LoopTrueEarlyExit,
    LoopArrayJoin,
    ScanWithInit,
    SplitScan,
}

impl SharedAbsentContractDeclineRouteV1 {
    pub(crate) const fn route_id(self) -> LoopRouteId {
        match self {
            Self::LoopTrueEarlyExit => LoopRouteId::LoopTrueEarlyExit,
            Self::LoopArrayJoin => LoopRouteId::LoopArrayJoin,
            Self::ScanWithInit => LoopRouteId::ScanWithInit,
            Self::SplitScan => LoopRouteId::SplitScan,
        }
    }

    pub(crate) fn declines(self, planner_required: bool, recipe_contract_present: bool) -> bool {
        !planner_required && !recipe_contract_present
    }
}

pub(crate) struct StandardEntry {
    pub route_label: &'static str,
    pub missing_contract_msg: &'static str,
    pub compose: ComposeFn,
    pub planner_required_only: bool,
    pub absent_contract_decline: Option<SharedAbsentContractDeclineRouteV1>,
    pub planner_first: PlannerFirstMode,
    pub plan_rule: Option<PlanRuleId>,
    pub flowbox_via_strict: FlowboxVia,
    pub flowbox_via_release: FlowboxVia,
}

#[cfg(test)]
mod tests {
    use super::{RouterEnv, SharedAbsentContractDeclineRouteV1};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::{
        route_id::LoopRouteId, RouteExecutionWitnessV1,
    };

    #[test]
    fn shared_absent_contract_decline_vocabulary_is_exactly_the_four_confirmed_routes() {
        use SharedAbsentContractDeclineRouteV1 as Policy;

        let policies = [
            Policy::LoopTrueEarlyExit,
            Policy::LoopArrayJoin,
            Policy::ScanWithInit,
            Policy::SplitScan,
        ];
        assert_eq!(
            policies.map(Policy::route_id),
            [
                LoopRouteId::LoopTrueEarlyExit,
                LoopRouteId::LoopArrayJoin,
                LoopRouteId::ScanWithInit,
                LoopRouteId::SplitScan,
            ]
        );

        let env = RouterEnv {
            strict_or_dev: false,
            planner_required: false,
            has_body_local: false,
        };
        let schedule = [LoopRouteId::LoopTrueEarlyExit];
        let absent_contract = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        assert!(policies
            .into_iter()
            .all(|policy| policy.declines(false, absent_contract.recipe_contract_present())));
    }
}
