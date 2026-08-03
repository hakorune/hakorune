//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod resolved_source_adapter;
mod selected_demand;

#[allow(unused_imports)]
pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};

#[allow(unused_imports)]
pub(crate) use selected_demand::{
    issue_selected_loop_recipe_demand_v1, SelectedLoopDemandRejectV1,
    VerifiedLoopStructuralFactsV1, VerifiedSelectedLoopRecipeDemandV1,
};

#[cfg(test)]
pub(crate) use selected_demand::verified_loop_structural_facts_for_test;

#[cfg(test)]
mod tests;
