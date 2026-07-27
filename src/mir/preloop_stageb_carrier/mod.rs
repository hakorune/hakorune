//! Bounded source-only proof for the selected pre-loop Stage-B carrier.
//!
//! See `README.md` for the authority and non-authority boundary.

mod activation;
mod candidate_selection;
mod function_ingress;
mod module_install;
mod outer_result;
mod rejection;
mod source_inventory;
mod source_inventory_error;

#[allow(unused_imports)]
pub(crate) use activation::{
    prepare_preloop_stageb_carrier_rows_v1, OwnedPreloopCarrierAssignmentTargetV1,
    OwnedPreloopStageBCarrierRowV1, PreparedPreloopStageBFunctionBodyRecipeV1,
    VerifiedPreloopStageBCarrierActivationPlanV1,
};
#[allow(unused_imports)]
pub(crate) use candidate_selection::{
    seal_preloop_stageb_candidate_selection_v1, PreloopStageBCandidateSelectionErrorV1,
    RejectedPreloopStageBCandidateSelectionV1, VerifiedPreloopStageBAmbiguousCandidatesV1,
    VerifiedPreloopStageBCandidateSelectionV1, VerifiedPreloopStageBNoCandidateV1,
    VerifiedPreloopStageBSelectedCandidateV1,
};
#[allow(unused_imports)]
pub(crate) use function_ingress::{
    PreloopStageBFunctionIngressCauseV1, PreloopStageBFunctionIngressStageV1,
    PreloopStageBInstanceDraftSourceErrorV1, PreparedPreloopStageBFunctionIngressV1,
    PreparedPreloopStageBInstanceDraftSourceV1, RejectedPreloopStageBFunctionIngressV1,
};
#[allow(unused_imports)]
pub(in crate::mir) use module_install::{
    InstalledPreloopStageBActivationContextPartsV1,
    PreparedPreloopStageBActivationContextInstallV1, PreparedPreloopStageBActivationLedgerPartsV1,
    RejectedPreloopStageBActivationContextInstallV1,
};
#[allow(unused_imports)]
pub(crate) use outer_result::{
    seal_preloop_outer_carrier_result_v1, SealedPreloopOuterCarrierResultContractV1,
};
#[allow(unused_imports)]
pub(crate) use rejection::{
    PreloopOuterCarrierResultContractErrorV1, PreloopOuterCarrierResultContractStageV1,
    RejectedPreloopOuterCarrierResultContractV1,
};
#[allow(unused_imports)]
pub(crate) use source_inventory::{
    inventory_preloop_stageb_candidates_v1, PreloopStageBCandidateIdentityV1,
    VerifiedPreloopStageBCandidateInventoryV1,
};
#[allow(unused_imports)]
pub(crate) use source_inventory_error::{
    PreloopStageBSourceInventoryCauseV1, PreloopStageBSourceInventoryErrorV1,
    PreloopStageBSourceInventoryStageV1,
};

#[cfg(test)]
mod function_ingress_tests;
#[cfg(test)]
mod source_inventory_tests;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
