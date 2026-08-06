//! Owned, caller-zero M3-C snapshot plus M3-E policy evidence for Loop rows.
//!
//! `evaluate` is structural validation/sealing only; `policy` is the M3-E pure
//! evaluator. See `README.md` for the authority boundary.

#[cfg(test)]
mod adapter;
mod direct_accum_observation;
#[cfg(test)]
mod direct_accum_observation_tests;
mod evaluate;
mod family_admission;
#[cfg(test)]
mod family_admission_tests;
#[cfg(test)]
mod family_selection;
mod generic_g0;
mod generic_g0_observation;
#[cfg(test)]
mod generic_g0_observation_tests;
#[cfg(test)]
mod generic_g0_tests;
mod loop_cond_break_continue_observation;
#[cfg(test)]
mod loop_cond_break_continue_observation_tests;
mod loop_true_break_continue;
mod loop_true_break_continue_observation;
#[cfg(test)]
mod loop_true_break_continue_observation_tests;
#[cfg(test)]
mod loop_true_break_continue_tests;
mod nested_predicate_observation;
#[cfg(test)]
mod nested_predicate_observation_tests;
mod policy;
mod policy_evidence;
mod schema;

#[allow(unused_imports)]
pub(crate) use direct_accum_observation::{
    issue_direct_accum_family_observation_v1, DirectAccumFamilyObservationV1,
    DirectAccumObservationContextV1, DirectAccumObservationDeclineV1,
    DirectAccumObservationEvidenceV1, DirectAccumObservationRejectV1,
    DirectAccumObservationUnresolvedV1, VerifiedDirectAccumFamilyCandidateV1,
};
#[allow(unused_imports)]
pub(crate) use evaluate::freeze_loop_route_schedule_v1;
pub(crate) use family_admission::{
    assemble_loop_family_admission_window_v1, LoopFamilyAdmissionAssemblyOutcomeV1,
    LoopFamilyAdmissionCoverageV1, LoopFamilyAdmissionFailureEvidenceV1,
    LoopFamilyAdmissionIssueV1, LoopFamilyAdmissionModeV1,
    VerifiedLoopFamilyAdmissionRowsV1, VerifiedLoopFamilyAdmissionWindowV1,
    LoopFamilyObservationRowV1,
    LoopFamilyTagV1,
};
#[cfg(test)]
pub(crate) use family_selection::{
    select_canonical_family_for_test, CanonicalFamilySelectionOutcomeV1,
    CanonicalFamilySelectorInputV1, FamilySelectionRejectV1, FamilySelectionUnresolvedV1,
    GenericFamilyEvidenceV1,
};
#[allow(unused_imports)]
pub(crate) use generic_g0::{
    issue_generic_g0_candidate_v1, GenericG0CoverageV1, GenericG0PolicyContextV1,
    GenericG0PolicyModeV1, GenericG0PolicyOutcomeV1, GenericG0PolicyProfileV1,
    GenericG0PolicyRejectV1, GenericG0PolicyUnresolvedV1, VerifiedGenericFamilyObservationG0,
};
#[allow(unused_imports)]
pub(crate) use generic_g0_observation::{
    issue_generic_g0_family_observation_v1, GenericG0FamilyObservationV1,
    GenericG0ObservationContextV1, GenericG0ObservationDeclineV1, GenericG0ObservationEvidenceV1,
    GenericG0ObservationRejectV1, GenericG0ObservationUnresolvedV1,
    VerifiedGenericG0FamilyCandidateV1,
};
pub(crate) use loop_cond_break_continue_observation::{
    issue_loop_cond_family_observation_v1, LoopCondFamilyObservationV1,
    LoopCondObservationContextV1, LoopCondObservationDeclineV1, LoopCondObservationEvidenceV1,
    LoopCondObservationRejectV1, LoopCondObservationUnresolvedV1,
    VerifiedLoopCondFamilyCandidateV1,
};
#[allow(unused_imports)]
pub(crate) use loop_true_break_continue::{
    issue_loop_true_break_continue_policy_demand_v1, LoopTrueBreakContinuePolicyDemandRejectV1,
    VerifiedLoopTrueBreakContinuePolicyDemandV1, VerifiedLoopTrueBreakContinuePolicyReceiptV1,
};
pub(crate) use loop_true_break_continue_observation::{
    issue_loop_true_family_observation_v1, LoopTrueFamilyObservationV1,
    LoopTrueObservationContextV1, LoopTrueObservationDeclineV1, LoopTrueObservationEvidenceV1,
    LoopTrueObservationRejectV1, LoopTrueObservationUnresolvedV1,
    VerifiedLoopTrueFamilyCandidateV1,
};
#[allow(unused_imports)]
pub(crate) use nested_predicate_observation::{
    issue_nested_predicate_family_observation_v1, NestedPredicateFamilyObservationV1,
    NestedPredicateObservationContextV1, NestedPredicateObservationDeclineV1,
    NestedPredicateObservationEvidenceV1, NestedPredicateObservationRejectV1,
    NestedPredicateObservationUnresolvedV1, VerifiedNestedPredicateFamilyCandidateV1,
};
#[allow(unused_imports)]
pub(crate) use policy::{
    evaluate_frozen_loop_route_schedule_v1, issue_direct_accum_route_admission_v1,
    DirectAccumRouteAdmissionRejectV1, LoopPolicyBlockedReasonV1, LoopQualifiedV1,
    LoopRoutePolicyEvaluationV1, VerifiedDirectAccumPolicyHandoffV1,
    VerifiedDirectAccumPolicyReceiptV1, VerifiedDirectAccumRouteAdmissionV1,
    VerifiedLoopPolicyWinnerV1,
};
#[cfg(test)]
pub(crate) use policy::{issue_policy_winner_for_test, issue_policy_winner_for_test_with_frame};
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
