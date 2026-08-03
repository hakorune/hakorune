//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod direct_accum_effect_plan;
mod resolved_source_adapter;
mod selected_demand;
mod types;

#[allow(unused_imports)]
pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};

pub(crate) use direct_accum_effect_plan::{
    DirectAccumBindingEffectEntryV1, DirectAccumBindingEffectRoleV1,
    VerifiedDirectAccumBindingEffectPlanV1,
};
#[allow(unused_imports)]
pub(crate) use selected_demand::{
    issue_direct_accum_structural_facts_v1, issue_selected_loop_recipe_demand_v1,
    DirectAccumFactsPayloadRejectV1, SelectedLoopDemandRejectV1, VerifiedLoopStructuralFactsV1,
    VerifiedSelectedLoopRecipeDemandV1,
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
