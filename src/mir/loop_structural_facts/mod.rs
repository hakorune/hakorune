//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod direct_accum_effect_plan;
mod direct_accum_exclusivity;
mod direct_accum_observation;
#[allow(dead_code)]
pub(crate) mod generic_g0;
#[cfg(test)]
mod generic_resolved_carrier_facts_snapshot;
mod resolved_source_adapter;
mod selected_demand;
mod types;

#[allow(unused_imports)]
pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, bind_resolved_loop_source_forest_v1, LoopRootSourceBindingRejectV1,
    LoopSourceForestBindingRejectV1, VerifiedLoopSourceForestBindingMemberV1,
    VerifiedLoopSourceForestBindingV1, VerifiedLoopRootSourceV1,
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
#[cfg(test)]
pub(crate) use generic_resolved_carrier_facts_snapshot::{
    issue_generic_resolved_carrier_facts_v1, ResolvedCarrierDispositionV1,
    VerifiedGenericResolvedCarrierFactsV1,
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

#[cfg(test)]
pub(crate) use selected_demand::{
    verified_loop_structural_facts_for_test, verified_loop_structural_facts_for_test_with_frame,
};

#[cfg(test)]
mod tests;
