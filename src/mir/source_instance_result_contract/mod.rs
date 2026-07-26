//! One bounded source-only instance-call result contract.

mod contract;
mod preloop_association;
mod preloop_located_argument;
mod rejection;
mod target;

pub(crate) use contract::{
    seal_nested_instance_result_contract, RejectedNestedInstanceResultContractV1,
    SealedNestedInstanceResultContractV1,
};
pub(crate) use preloop_association::{
    prepare_preloop_nested_result_association_v1, PreloopNestedResultAssociationErrorV1,
    PreloopNestedResultAssociationStageV1, PreparedPreloopNestedResultAssociationV1,
    RejectedPreloopNestedResultAssociationV1,
};
pub(crate) use preloop_located_argument::{
    prepare_preloop_located_argument_v1, PreloopLocatedArgumentErrorV1,
    PreloopLocatedArgumentStageV1, PreparedPreloopLocatedArgumentV1,
    RejectedPreloopLocatedArgumentV1,
};
pub(crate) use rejection::{
    CurrentOwnerInstanceResultTargetErrorV1, NestedInstanceResultContractErrorV1,
    NestedInstanceResultContractStageV1,
};
pub(crate) use target::VerifiedCurrentOwnerInstanceResultTargetV1;

#[cfg(test)]
mod tests;
