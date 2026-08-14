//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod direct_accum_effect_plan;
mod direct_accum_exclusivity;
mod direct_accum_observation;
#[allow(dead_code)]
pub(crate) mod generic_g0;
mod generic_g0_observation;
#[cfg(test)]
mod generic_resolved_carrier_facts_snapshot;
mod loop_cond_break_continue_observation;
mod loop_cond_break_continue_source;
mod loop_true_break_continue_observation;
mod loop_true_break_continue_source;
mod nested_predicate_observation;
mod nested_predicate_source;
mod resolved_source_adapter;
// Caller-zero until the immediately following complete S6C Facts row.
#[allow(dead_code)]
mod s6c_exit_tail;
#[allow(dead_code)]
mod s6c_scan_with_init;
mod selected_demand;
mod types;
mod variable_accum_break;
mod variable_accum_recurrence;
mod variable_accum_recurrence_validation;

#[allow(unused_imports)]
pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, bind_resolved_loop_source_forest_v1, LoopRootSourceBindingRejectV1,
    LoopSourceForestBindingRejectV1, VerifiedLoopRootSourceV1,
    VerifiedLoopSourceForestBindingMemberV1, VerifiedLoopSourceForestBindingV1,
};

#[cfg(test)]
pub(crate) use resolved_source_adapter::projection_for_test;

pub(crate) use direct_accum_effect_plan::{
    DirectAccumBindingEffectEntryV1, DirectAccumBindingEffectRoleV1,
    VerifiedDirectAccumBindingEffectPlanV1,
};
pub(crate) use direct_accum_exclusivity::{
    issue_direct_accum_disjointness_v1, DirectAccumDisjointnessRejectV1,
    VerifiedDirectAccumDisjointnessV1,
};
pub(crate) use direct_accum_observation::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    DirectAccumSourceAttemptOutcomeV1, DirectAccumSourceDeclineV1, DirectAccumSourceIdentityV1,
    DirectAccumSourceRejectV1, DirectAccumSourceUnresolvedV1, VerifiedDirectAccumSourceAttemptV1,
};
pub(crate) use generic_g0_observation::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1, GenericG0SourceAttemptOutcomeV1,
    GenericG0SourceDeclineV1, GenericG0SourceIdentityV1, GenericG0SourceRejectV1,
    GenericG0SourceUnresolvedV1, VerifiedGenericG0SourceAttemptV1,
};
#[cfg(test)]
pub(crate) use generic_resolved_carrier_facts_snapshot::{
    issue_generic_resolved_carrier_facts_v1, ResolvedCarrierDispositionV1,
    VerifiedGenericResolvedCarrierFactsV1,
};
pub(crate) use loop_cond_break_continue_observation::{
    LoopCondObservationCoverageV1, LoopCondObservationModeV1, LoopCondSourceAttemptOutcomeV1,
    LoopCondSourceDeclineV1, LoopCondSourceIdentityV1, LoopCondSourceRejectV1,
    LoopCondSourceUnresolvedV1, VerifiedLoopCondSourceAttemptV1,
};
pub(crate) use loop_cond_break_continue_source::{
    VerifiedLoopCondBreakContinueSourceProjectionV1, VerifiedLoopCondBreakContinueSourceShapeV1,
};
pub(crate) use loop_true_break_continue_observation::{
    map_loop_true_source_binding_reject, LoopTrueObservationCoverageV1, LoopTrueObservationModeV1,
    LoopTrueSourceAttemptOutcomeV1, LoopTrueSourceDeclineV1, LoopTrueSourceIdentityV1,
    LoopTrueSourceRejectV1, LoopTrueSourceUnresolvedV1, VerifiedLoopTrueSourceAttemptV1,
};
pub(crate) use loop_true_break_continue_source::{
    VerifiedLoopTrueBreakContinueSourceProjectionV1, VerifiedLoopTrueBreakContinueSourceShapeV1,
};
pub(crate) use nested_predicate_observation::{
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceAttemptOutcomeV1, NestedPredicateSourceDeclineV1,
    NestedPredicateSourceIdentityV1, NestedPredicateSourceRejectV1,
    NestedPredicateSourceUnresolvedV1, VerifiedNestedPredicateSourceAttemptV1,
};
pub(crate) use nested_predicate_source::{
    NestedBindingEvidenceV1, NestedChildBodyRoleV1, NestedObservedRecurrenceOwnerV1,
    NestedPredicateConditionEvidenceV1, NestedPredicateUpdateEvidenceV1, NestedRootBodyRoleV1,
    NestedRootInitializerEvidenceV1, VerifiedNestedLoopSourceProjectionV1,
    VerifiedNestedLoopSourceShapeV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_exit_tail::{
    issue_s6c_exit_tail_source_coseal_v1, S6CExitRoleV1, S6CExitTailSourceCoSealRefV1,
    S6CExitTailSourceCoSealRejectV1, VerifiedS6CExitTailSourceCoSealV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_scan_with_init::{
    issue_s6c_scan_with_init_facts_v1, S6CScanWithInitFactsRefV1, S6CScanWithInitFactsRejectV1,
    VerifiedS6CScanWithInitFactsV1,
};
#[allow(unused_imports)]
pub(crate) use selected_demand::{
    issue_direct_accum_structural_facts_v1, issue_selected_loop_recipe_demand_v1,
    DirectAccumFactsPayloadRejectV1, DirectAccumSingletonObservationRejectV1,
    SelectedLoopDemandRejectV1, VerifiedDirectAccumSingletonObservationV1,
    VerifiedLoopStructuralFactsV1, VerifiedSelectedLoopRecipeDemandV1,
};
pub(crate) use types::{
    DirectAccumObservedShapeV1, DirectAccumStructuralShapeV1, DirectAccumUpdateShapeV1,
};
pub(crate) use variable_accum_break::{
    issue_variable_accum_break_facts_v1, VariableAccumBreakAssignmentObservationV1,
    VariableAccumBreakBindingObservationV1, VariableAccumBreakBindingRoleV1,
    VariableAccumBreakCompareV1, VariableAccumBreakConditionObservationV1,
    VariableAccumBreakCoverageV1, VariableAccumBreakFactsIssueV1,
    VariableAccumBreakInputObservationV1, VariableAccumBreakInputRoleV1,
    VariableAccumBreakObservationCoverageV1, VariableAccumBreakOperationRoleV1,
    VariableAccumBreakSourceAttemptOutcomeV1, VariableAccumBreakSourceDeclineV1,
    VariableAccumBreakSourceIdentityV1, VariableAccumBreakSourceRejectV1,
    VariableAccumBreakSourceUnresolvedV1, VariableAccumBreakValueClassV1,
    VerifiedVariableAccumBreakFactsV1, VerifiedVariableAccumBreakSourceAttemptV1,
};
#[allow(unused_imports)]
pub(crate) use variable_accum_recurrence::{
    issue_variable_accum_recurrence_facts_v1, VariableAccumRecurrenceAccumulatorUpdateV1,
    VariableAccumRecurrenceBindingObservationV1, VariableAccumRecurrenceBindingRoleV1,
    VariableAccumRecurrenceConditionObservationV1, VariableAccumRecurrenceConditionOperatorV1,
    VariableAccumRecurrenceCoverageV1, VariableAccumRecurrenceFactsIssueV1,
    VariableAccumRecurrenceInductionStepV1, VariableAccumRecurrenceInputObservationV1,
    VariableAccumRecurrenceInputRoleV1, VariableAccumRecurrenceObservationCoverageV1,
    VariableAccumRecurrenceSourceAttemptOutcomeV1, VariableAccumRecurrenceSourceDeclineV1,
    VariableAccumRecurrenceSourceIdentityV1, VariableAccumRecurrenceSourceRejectV1,
    VariableAccumRecurrenceSourceRoleV1, VariableAccumRecurrenceSourceUnresolvedV1,
    VariableAccumRecurrenceValueClassV1, VerifiedVariableAccumRecurrenceFactsV1,
    VerifiedVariableAccumRecurrenceSourceAttemptV1,
};
pub(crate) use variable_accum_recurrence_validation::source_coherence_is_exact;

#[cfg(test)]
pub(crate) use selected_demand::{
    verified_loop_structural_facts_for_test, verified_loop_structural_facts_for_test_with_frame,
};

#[cfg(test)]
mod s6c_exit_tail_tests;
#[cfg(test)]
mod tests;
