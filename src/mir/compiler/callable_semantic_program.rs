//! Callable-first semantic-program co-seal.
//!
//! The issuer consumes the complete source-backed Callable parent once.  It
//! deliberately keeps the operation/effect, context, JoinSig continuation,
//! and Callable boundary rows together until the next consumer; no caller can
//! re-pair those rows through separate arguments.

use super::callable_single_loop_operation_effect::{
    issue_callable_operation_effect_parts_v1, CallableOperationEffectAdapterRejectV1,
};
use super::callable_single_loop_recipe_coseal::{
    VerifiedCallablePreludeV1, VerifiedCallableSingleLoopRecipeProductV1, VerifiedCallableTailV1,
};
use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedLoopContinuationContractV1, VerifiedLoopInitializedLocalInputSourceSetV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationPhysicalDemandV1,
    VerifiedLoopSemanticContextV1,
};

/// One Callable-first semantic-program parent.  This is a profile adapter,
/// not the repository-wide G0 issuer; the generic promotion remains a later
/// decision after all admitted families have a same-parent source producer.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableSemanticProgramV1 {
    operation_effect: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}
impl VerifiedCallableSemanticProgramV1 {
    /// Consume the complete semantic parent into one source-free prepared
    /// demand parent.  Demand construction remains here so callers cannot
    /// re-pair operation/effect/context/continuation rows.
    pub(in crate::mir) fn into_prepared_operation_demand(
        self,
    ) -> Result<PreparedCallableOperationDemandV1, LoopOperationPhysicalDemandRejectV1> {
        let Self {
            operation_effect,
            input,
            context,
            continuation,
            prelude,
            tail,
        } = self;
        let operation = VerifiedLoopOperationPhysicalDemandV1::issue(
            context,
            operation_effect,
            continuation,
        )?
        .prepare_all()?;
        Ok(PreparedCallableOperationDemandV1 {
            input,
            operation,
            prelude,
            tail,
        })
    }
}

/// Source-free, one-shot prepared demand for the Callable profile.  It is an
/// aggregate of already-issued rows, not a new semantic authority; the only
/// consumer is the prepared Callable operation handoff (plus its test probe).
#[derive(Debug)]
pub(in crate::mir) struct PreparedCallableOperationDemandV1 {
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    operation: PreparedLoopOperationProgramV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}

impl PreparedCallableOperationDemandV1 {
    /// Lend all prepared rows in one one-shot callback.  No tuple or
    /// independent field getter is exposed, so a consumer cannot re-pair
    /// rows from separate semantic parents.
    pub(in crate::mir) fn consume<R>(
        self,
        consumer: impl FnOnce(
            VerifiedLoopInitializedLocalInputSourceSetV1,
            PreparedLoopOperationProgramV1,
            VerifiedCallablePreludeV1,
            VerifiedCallableTailV1,
        ) -> R,
    ) -> R {
        consumer(self.input, self.operation, self.prelude, self.tail)
    }
}

/// Canonical Callable-first issuer.  The complete recipe/co-seal parent is
/// the only input; separate context/Core/continuation arguments are rejected
/// by construction because this API has no such shape.
pub(in crate::mir) fn issue_callable_semantic_program_v1(
    product: VerifiedCallableSingleLoopRecipeProductV1,
) -> Result<VerifiedCallableSemanticProgramV1, CallableOperationEffectAdapterRejectV1> {
    let parts = issue_callable_operation_effect_parts_v1(product)?;
    let (operation_effect, input, context, continuation, prelude, tail) = parts.into_parts();
    Ok(VerifiedCallableSemanticProgramV1 {
        operation_effect,
        input,
        context,
        continuation,
        prelude,
        tail,
    })
}
