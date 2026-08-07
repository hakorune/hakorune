//! Caller-zero adapters for the full callable Loop physical canary.
//!
//! This module is test-only while production selection is parked.  It contains
//! no semantic owner: it only moves existing prepared capabilities into the
//! canonical lowering session and records the missing handoff contracts.

#![cfg(test)]

use super::callable_single_loop_recipe_coseal::VerifiedCallableTailV1;
use super::loop_physical_prepare::{
    PreparedCallableLoopPhysicalizationV1, VerifiedCallableFunctionLoweringInputV1,
    VerifiedCallablePreludeCapabilityV1, VerifiedCallableTerminalCompatibilityV1,
    VerifiedLoopPhysicalDemandV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;

pub(crate) type CallableLoopCanaryParts<'a> = (
    VerifiedCallableFunctionLoweringInputV1<'a>,
    VerifiedLoopPhysicalDemandV1,
    VerifiedCallablePreludeCapabilityV1,
    VerifiedCallableTailV1,
    VerifiedCallableTerminalCompatibilityV1,
    VerifiedFunctionCompletionV1,
);

/// Move the prepared product into the canary exactly once.  In particular,
/// Completion is not cloned or re-verified on the physical path.
pub(crate) fn into_canary_parts<'a>(
    prepared: PreparedCallableLoopPhysicalizationV1<'a>,
) -> CallableLoopCanaryParts<'a> {
    let PreparedCallableLoopPhysicalizationV1 {
        input,
        demand,
        prelude,
        tail,
        terminal,
        completion,
    } = prepared;
    (input, demand, prelude, tail, terminal, completion)
}
