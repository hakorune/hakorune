//! Policy-owned admission for the bounded LoopCond branch profile.
//!
//! The typed source map is consumed once, and only the profile brand,
//! source frame, and already-sealed map cross this boundary. Recipe,
//! JoinSig, route execution, and physical lowering remain outside this
//! module.

use crate::mir::compiler::loop_cond_break_continue_typed_map::VerifiedLoopCondBreakContinueTypedSourceMapV1;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinuePolicyReceiptV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: LoopCondBreakContinuePolicySealV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinuePolicyDemandV1 {
    receipt: VerifiedLoopCondBreakContinuePolicyReceiptV1,
    map: VerifiedLoopCondBreakContinueTypedSourceMapV1,
}

#[derive(Debug, PartialEq, Eq)]
struct LoopCondBreakContinuePolicySealV1;

impl VerifiedLoopCondBreakContinuePolicyDemandV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopCondBreakContinuePolicyReceiptV1,
        VerifiedLoopCondBreakContinueTypedSourceMapV1,
    ) {
        (self.receipt, self.map)
    }
}

impl VerifiedLoopCondBreakContinuePolicyReceiptV1 {
    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }
}

/// Seals the policy demand directly from the verified typed source map. The
/// family candidate was already selected upstream, so no route schedule or
/// winner cursor exists here.
pub(crate) fn issue_loop_cond_break_continue_policy_demand_v1(
    map: VerifiedLoopCondBreakContinueTypedSourceMapV1,
) -> VerifiedLoopCondBreakContinuePolicyDemandV1 {
    VerifiedLoopCondBreakContinuePolicyDemandV1 {
        receipt: VerifiedLoopCondBreakContinuePolicyReceiptV1 {
            frame_key: map.root_frame_key().clone(),
            _seal: LoopCondBreakContinuePolicySealV1,
        },
        map,
    }
}
