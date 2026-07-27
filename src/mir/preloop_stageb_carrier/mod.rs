//! Bounded source-only proof for the selected pre-loop Stage-B carrier.
//!
//! See `README.md` for the authority and non-authority boundary.

mod activation;
mod outer_result;
mod rejection;
mod source_inventory;
mod source_inventory_error;

#[allow(unused_imports)]
pub(crate) use activation::{
    prepare_preloop_stageb_carrier_rows_v1, VerifiedPreloopStageBCarrierActivationPlanV1,
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
mod source_inventory_tests;
#[cfg(test)]
mod tests;
