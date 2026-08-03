//! Owned, caller-zero M3-C snapshot plus M3-E policy evidence for Loop rows.
//!
//! `evaluate` is structural validation/sealing only; `policy` is the M3-E pure
//! evaluator. See `README.md` for the authority boundary.

#[cfg(test)]
mod adapter;
mod evaluate;
mod policy;
mod policy_evidence;
mod schema;

#[allow(unused_imports)]
pub(crate) use evaluate::freeze_loop_route_schedule_v1;
#[allow(unused_imports)]
pub(crate) use policy::{
    evaluate_frozen_loop_route_schedule_v1, LoopPolicyBlockedReasonV1, LoopQualifiedV1,
    LoopRoutePolicyEvaluationV1, VerifiedLoopPolicyWinnerV1,
};
#[allow(unused_imports)]
pub(crate) use policy_evidence::{
    LoopGenericDebtKeyV1, LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1,
    LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1,
};
#[allow(unused_imports)]
pub(crate) use schema::{
    FrozenLoopRouteObservationV1, FrozenLoopRouteRowV1, FrozenLoopRouteScheduleRejectV1,
    FrozenLoopRouteScheduleV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
    LoopPlannerContractObservationV1, LoopReleaseAdmissionObservationV1,
    LoopRouteSourceDispositionV1, LoopRouteSourceUnavailableV1, LoopRouteSuppressionCauseV1,
    LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_COUNT_V1,
    CANONICAL_LOOP_ROUTE_ORDER_V1,
};

#[cfg(test)]
mod tests;
