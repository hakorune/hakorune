//! Policy-owned admission for the bounded Generic residual profile.
//!
//! The typed source map is consumed once, and only the profile brand,
//! source frame, and already-sealed map cross this boundary. The winner
//! cursor must be the `GenericLoopV1` position in the frozen canonical
//! schedule; a `[V0, V1]` overlap selection wins at `GenericLoopV0` and
//! is a typed `WrongWinnerCursor`, never re-minted as V1 or G0
//! provenance. Recipe, JoinSig, route execution, and physical lowering
//! remain outside this module.

use crate::mir::compiler::generic_residual_typed_map::VerifiedGenericResidualTypedSourceMapV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

use super::{
    evaluate_frozen_loop_route_schedule_v1, FrozenLoopRouteScheduleV1, LoopPolicyBlockedReasonV1,
    LoopRoutePolicyEvaluationV1, VerifiedLoopPolicyWinnerV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResidualPolicyReceiptV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: GenericResidualPolicySealV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResidualPolicyDemandV1 {
    receipt: VerifiedGenericResidualPolicyReceiptV1,
    map: VerifiedGenericResidualTypedSourceMapV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericResidualPolicyDemandRejectV1 {
    PolicyBlocked(LoopPolicyBlockedReasonV1),
    Exhausted,
    WrongWinnerCursor { expected: usize, actual: usize },
    ExecutionFrameMismatch,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericResidualPolicySealV1;

impl VerifiedGenericResidualPolicyDemandV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedGenericResidualPolicyReceiptV1,
        VerifiedGenericResidualTypedSourceMapV1,
    ) {
        (self.receipt, self.map)
    }
}

impl VerifiedGenericResidualPolicyReceiptV1 {
    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }
}

pub(crate) fn issue_generic_residual_policy_demand_v1(
    map: VerifiedGenericResidualTypedSourceMapV1,
    schedule: FrozenLoopRouteScheduleV1,
) -> Result<VerifiedGenericResidualPolicyDemandV1, GenericResidualPolicyDemandRejectV1> {
    let evaluation = evaluate_frozen_loop_route_schedule_v1(&schedule, map.root_frame_key());
    match evaluation {
        LoopRoutePolicyEvaluationV1::Qualified(qualified) => {
            let (_facts, winner) = qualified.into_parts();
            seal_generic_residual_policy_demand_v1(map, winner)
        }
        LoopRoutePolicyEvaluationV1::Blocked(reason) => {
            Err(GenericResidualPolicyDemandRejectV1::PolicyBlocked(reason))
        }
        LoopRoutePolicyEvaluationV1::Exhausted => {
            Err(GenericResidualPolicyDemandRejectV1::Exhausted)
        }
    }
}

fn seal_generic_residual_policy_demand_v1(
    map: VerifiedGenericResidualTypedSourceMapV1,
    winner: VerifiedLoopPolicyWinnerV1,
) -> Result<VerifiedGenericResidualPolicyDemandV1, GenericResidualPolicyDemandRejectV1> {
    if !winner.frame_key().matches(map.root_frame_key()) {
        return Err(GenericResidualPolicyDemandRejectV1::ExecutionFrameMismatch);
    }
    let actual = winner.into_raw_cursor();
    let expected = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::GenericLoopV1)
        .expect("canonical route order contains GenericLoopV1");
    if actual != expected {
        return Err(GenericResidualPolicyDemandRejectV1::WrongWinnerCursor { expected, actual });
    }
    let frame_key = map.root_frame_key().clone();
    Ok(VerifiedGenericResidualPolicyDemandV1 {
        receipt: VerifiedGenericResidualPolicyReceiptV1 {
            frame_key,
            _seal: GenericResidualPolicySealV1,
        },
        map,
    })
}
