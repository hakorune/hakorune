//! Policy-owned admission for the bounded LoopTrue branch profile.
//!
//! The schedule is consumed once, and only the profile brand, source frame,
//! and already-sealed source projection cross this boundary. Recipe, JoinSig,
//! route execution, and physical lowering remain outside this module.

use crate::mir::compiler::loop_true_break_continue_projection::VerifiedLoopTrueBreakContinueSourceProjectionV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

use super::{
    evaluate_frozen_loop_route_schedule_v1, FrozenLoopRouteScheduleV1, LoopPolicyBlockedReasonV1,
    LoopRoutePolicyEvaluationV1, VerifiedLoopPolicyWinnerV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: LoopTrueBreakContinuePolicySealV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinuePolicyDemandV1 {
    receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1,
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueBreakContinuePolicyDemandRejectV1 {
    PolicyBlocked(LoopPolicyBlockedReasonV1),
    Exhausted,
    WrongWinnerCursor { expected: usize, actual: usize },
    ExecutionFrameMismatch,
}

#[derive(Debug, PartialEq, Eq)]
struct LoopTrueBreakContinuePolicySealV1;

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

pub(crate) fn issue_loop_true_break_continue_policy_demand_v1(
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
    schedule: FrozenLoopRouteScheduleV1,
) -> Result<VerifiedLoopTrueBreakContinuePolicyDemandV1, LoopTrueBreakContinuePolicyDemandRejectV1>
{
    let evaluation = evaluate_frozen_loop_route_schedule_v1(&schedule, projection.root_frame_key());
    match evaluation {
        LoopRoutePolicyEvaluationV1::Qualified(qualified) => {
            let (_facts, winner) = qualified.into_parts();
            seal_loop_true_break_continue_policy_demand_v1(projection, winner)
        }
        LoopRoutePolicyEvaluationV1::Blocked(reason) => Err(
            LoopTrueBreakContinuePolicyDemandRejectV1::PolicyBlocked(reason),
        ),
        LoopRoutePolicyEvaluationV1::Exhausted => {
            Err(LoopTrueBreakContinuePolicyDemandRejectV1::Exhausted)
        }
    }
}

fn seal_loop_true_break_continue_policy_demand_v1(
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
    winner: VerifiedLoopPolicyWinnerV1,
) -> Result<VerifiedLoopTrueBreakContinuePolicyDemandV1, LoopTrueBreakContinuePolicyDemandRejectV1>
{
    if !winner.frame_key().matches(projection.root_frame_key()) {
        return Err(LoopTrueBreakContinuePolicyDemandRejectV1::ExecutionFrameMismatch);
    }
    let actual = winner.into_raw_cursor();
    let expected = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::LoopTrueBreakContinue)
        .expect("canonical route order contains LoopTrueBreakContinue");
    if actual != expected {
        return Err(
            LoopTrueBreakContinuePolicyDemandRejectV1::WrongWinnerCursor { expected, actual },
        );
    }
    let frame_key = projection.root_frame_key().clone();
    Ok(VerifiedLoopTrueBreakContinuePolicyDemandV1 {
        receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
            frame_key,
            _seal: LoopTrueBreakContinuePolicySealV1,
        },
        projection,
    })
}

#[cfg(test)]
pub(super) fn seal_loop_true_break_continue_policy_demand_for_test(
    projection: VerifiedLoopTrueBreakContinueSourceProjectionV1,
    winner: VerifiedLoopPolicyWinnerV1,
) -> Result<VerifiedLoopTrueBreakContinuePolicyDemandV1, LoopTrueBreakContinuePolicyDemandRejectV1>
{
    seal_loop_true_break_continue_policy_demand_v1(projection, winner)
}
