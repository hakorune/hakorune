//! Bounded source-only proof for the selected pre-loop Stage-B carrier.
//!
//! See `README.md` for the authority and non-authority boundary.

mod activation;
mod outer_result;
mod rejection;

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

#[cfg(test)]
mod tests;
