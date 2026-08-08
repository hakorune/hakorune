//! Pure logical JoinSig contract split into small, caller-zero owners.
//!
//! This facade publishes the historical `join_sig::*` API while the child
//! modules own model, port, visibility, and flow responsibilities. No child
//! module owns Recipe selection, physical IDs, MIR mutation, or publication.

mod flow;
mod model;
mod port;
mod visibility;

pub(crate) use flow::LoopJoinSigElaboratorV1;
pub(crate) use model::{
    LoopJoinBranchArmV1, LoopJoinBranchExitV1, LoopJoinBranchV1, LoopJoinEdgeRoleV1,
    LoopJoinEdgeV1, LoopJoinLoopV1, LoopJoinPayloadV1, LoopJoinPortBindingV1, LoopJoinPortV1,
    LoopJoinSigRejectReasonV1, LoopJoinSigV1, VerifiedLoopAfterBindingV1, VerifiedLoopJoinSigV1,
};

pub(super) use flow::Flow;
pub(super) use port::port_bindings;
pub(super) use visibility::visible_payloads;
