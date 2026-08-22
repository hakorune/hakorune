//! Pure logical JoinSig contract split into small, caller-zero owners.
//!
//! This facade publishes the historical `join_sig::*` API while the child
//! modules own model, port, visibility, and flow responsibilities. No child
//! module owns Recipe selection, physical IDs, MIR mutation, or publication.

mod flow;
mod model;
mod port;
pub(super) mod recipe_view;
mod recipe_view_v2;
mod transfer_view_v1;
mod transfer_view_v2;
mod v2;
mod visibility;

pub(crate) use flow::LoopJoinSigElaboratorV1;
pub(crate) use model::{
    LoopJoinBranchArmV1, LoopJoinBranchArmV2, LoopJoinBranchExitTargetV2, LoopJoinBranchExitV1,
    LoopJoinBranchExitV2, LoopJoinBranchV1, LoopJoinBranchV2, LoopJoinEdgeRoleV1, LoopJoinEdgeV1,
    LoopJoinEdgeV2, LoopJoinLoopV1, LoopJoinLoopV2, LoopJoinNextItemV1, LoopJoinPayloadV1,
    LoopJoinPayloadV2, LoopJoinPortBindingV1, LoopJoinPortBindingV2, LoopJoinPortV1,
    LoopJoinSigRejectReasonV1, LoopJoinSigV1, LoopJoinSigV2, VerifiedLoopAfterBindingV1,
    VerifiedLoopJoinSigV1, VerifiedLoopJoinSigV2,
};
pub(crate) use transfer_view_v1::{
    LoopJoinBoundaryTransferRefV1, LoopJoinLogicalTransferRejectV1, LoopJoinLogicalTransferViewV1,
};
pub(crate) use transfer_view_v2::{
    LoopJoinAfterRefV2, LoopJoinBoundaryTransferRefV2, LoopJoinBranchArmTransferRefV2,
    LoopJoinBranchExitRefV2, LoopJoinBranchTransferRefV2, LoopJoinLogicalTransferRejectV2,
    LoopJoinLogicalTransferViewV2, LoopJoinSummaryTransferRefV2,
};
pub(crate) use v2::{
    issue_sole_root_carrier_join_closure_v2, LoopJoinClosureRejectV2, VerifiedLoopJoinClosureV2,
};

pub(super) use flow::Flow;
pub(super) use model::{
    LoopJoinBranch, LoopJoinBranchArm, LoopJoinBranchExit, LoopJoinBranchTarget, LoopJoinPayload,
};
#[cfg(test)]
pub(super) use port::port_bindings;
#[cfg(test)]
pub(super) use visibility::visible_payloads;
pub(super) use visibility::visible_payloads_from_view;
