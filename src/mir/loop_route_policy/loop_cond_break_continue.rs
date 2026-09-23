//! Policy-owned admission for the bounded LoopCond branch profile.
//!
//! The typed source map is consumed once, and only the profile brand,
//! source frame, and already-sealed map cross this boundary. Recipe,
//! JoinSig, route execution, and physical lowering remain outside this
//! module.

use crate::mir::compiler::loop_cond_break_continue_typed_map::VerifiedLoopCondBreakContinueTypedSourceMapV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

use super::{
    evaluate_frozen_loop_route_schedule_v1, FrozenLoopRouteScheduleV1, LoopPolicyBlockedReasonV1,
    LoopRoutePolicyEvaluationV1, VerifiedLoopPolicyWinnerV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondBreakContinuePolicyDemandRejectV1 {
    PolicyBlocked(LoopPolicyBlockedReasonV1),
    Exhausted,
    WrongWinnerCursor { expected: usize, actual: usize },
    ExecutionFrameMismatch,
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

pub(crate) fn issue_loop_cond_break_continue_policy_demand_v1(
    map: VerifiedLoopCondBreakContinueTypedSourceMapV1,
    schedule: FrozenLoopRouteScheduleV1,
) -> Result<VerifiedLoopCondBreakContinuePolicyDemandV1, LoopCondBreakContinuePolicyDemandRejectV1>
{
    let evaluation = evaluate_frozen_loop_route_schedule_v1(&schedule, map.root_frame_key());
    match evaluation {
        LoopRoutePolicyEvaluationV1::Qualified(qualified) => {
            let (_facts, winner) = qualified.into_parts();
            seal_loop_cond_break_continue_policy_demand_v1(map, winner)
        }
        LoopRoutePolicyEvaluationV1::Blocked(reason) => {
            Err(LoopCondBreakContinuePolicyDemandRejectV1::PolicyBlocked(reason))
        }
        LoopRoutePolicyEvaluationV1::Exhausted => {
            Err(LoopCondBreakContinuePolicyDemandRejectV1::Exhausted)
        }
    }
}

fn seal_loop_cond_break_continue_policy_demand_v1(
    map: VerifiedLoopCondBreakContinueTypedSourceMapV1,
    winner: VerifiedLoopPolicyWinnerV1,
) -> Result<VerifiedLoopCondBreakContinuePolicyDemandV1, LoopCondBreakContinuePolicyDemandRejectV1>
{
    if !winner.frame_key().matches(map.root_frame_key()) {
        return Err(LoopCondBreakContinuePolicyDemandRejectV1::ExecutionFrameMismatch);
    }
    let actual = winner.into_raw_cursor();
    let expected = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::LoopCondBreakContinue)
        .expect("canonical route order contains LoopCondBreakContinue");
    if actual != expected {
        return Err(
            LoopCondBreakContinuePolicyDemandRejectV1::WrongWinnerCursor { expected, actual },
        );
    }
    let frame_key = map.root_frame_key().clone();
    Ok(VerifiedLoopCondBreakContinuePolicyDemandV1 {
        receipt: VerifiedLoopCondBreakContinuePolicyReceiptV1 {
            frame_key,
            _seal: LoopCondBreakContinuePolicySealV1,
        },
        map,
    })
}
