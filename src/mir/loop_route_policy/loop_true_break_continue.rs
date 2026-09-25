//! Policy-owned admission for the bounded LoopTrue branch profile.
//!
//! The verified source projection is consumed once, and only the profile
//! brand, source frame, and already-sealed projection cross this boundary.
//! Recipe, JoinSig, route execution, and physical lowering remain outside
//! this module.

use crate::mir::compiler::loop_true_break_continue_projection::VerifiedLoopTrueBreakContinueSourceProjectionV1;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: LoopTrueBreakContinuePolicySealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct LoopTrueBreakContinuePolicySealV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinuePolicyDemandV1 {
    receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1,
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
}

impl VerifiedLoopTrueBreakContinuePolicyDemandV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopTrueBreakContinuePolicyReceiptV1,
        VerifiedLoopTrueBreakContinueSourceProjectionV1,
    ) {
        (self.receipt, self.projection)
    }
}

impl VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }
}

/// Seals the policy demand directly from the verified source projection. The
/// family candidate was already selected upstream, so no route schedule or
/// winner cursor exists here.
pub(crate) fn issue_loop_true_break_continue_policy_demand_v1(
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
) -> VerifiedLoopTrueBreakContinuePolicyDemandV1 {
    VerifiedLoopTrueBreakContinuePolicyDemandV1 {
        receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
            frame_key: projection.root_frame_key().clone(),
            _seal: LoopTrueBreakContinuePolicySealV1,
        },
        projection,
    }
}
