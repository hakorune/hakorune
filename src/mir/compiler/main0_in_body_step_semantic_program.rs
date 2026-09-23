//! Main0 in-body-step semantic-program demand issuance.
//!
//! The issuer consumes the complete source-backed Main0 in-body-step product
//! once and keeps the operation/effect, context, JoinSig continuation, tail,
//! and control receipts together until the next consumer; no caller can
//! re-pair those rows through separate arguments. It mirrors
//! `main0_continue_semantic_program.rs` for the second bounded profile.

use super::main0_in_body_step_recipe_coseal::{
    VerifiedMain0InBodyStepControlSourceV1, VerifiedMain0InBodyStepRecipeProductV1,
    VerifiedMain0InBodyStepTailV1,
};
use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedLoopContinuationContractV1, VerifiedLoopInitializedLocalInputSourceSetV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationPhysicalDemandV1,
    VerifiedLoopSemanticContextV1,
};

/// One Main0 in-body-step semantic-program parent. The co-seal already
/// issued every row; this wrapper is the single demand-construction point
/// so the prepared program cannot be re-paired from independent pieces.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedMain0InBodyStepSemanticProgramV1 {
    operation_effect: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    tail: VerifiedMain0InBodyStepTailV1,
    control: VerifiedMain0InBodyStepControlSourceV1,
}

impl VerifiedMain0InBodyStepSemanticProgramV1 {
    /// Consume the complete semantic parent into one source-free prepared
    /// demand parent. Demand construction remains here so callers cannot
    /// re-pair operation/effect/context/continuation rows.
    pub(in crate::mir) fn into_prepared_operation_demand(
        self,
    ) -> Result<PreparedMain0InBodyStepOperationDemandV1, LoopOperationPhysicalDemandRejectV1>
    {
        let Self {
            operation_effect,
            input,
            context,
            continuation,
            tail,
            control,
        } = self;
        let operation =
            VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)?
                .prepare_all()?;
        Ok(PreparedMain0InBodyStepOperationDemandV1 {
            input,
            operation,
            tail,
            control,
        })
    }
}

/// Source-free, one-shot prepared demand for the Main0 in-body-step
/// profile. It is an aggregate of already-issued rows, not a new semantic
/// authority; the only consumer is the prepared Main0 operation handoff
/// (plus test probes).
#[derive(Debug)]
pub(in crate::mir) struct PreparedMain0InBodyStepOperationDemandV1 {
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    operation: PreparedLoopOperationProgramV1,
    tail: VerifiedMain0InBodyStepTailV1,
    control: VerifiedMain0InBodyStepControlSourceV1,
}

impl PreparedMain0InBodyStepOperationDemandV1 {
    /// Lend all prepared rows in one one-shot callback. No tuple or
    /// independent field getter is exposed, so a consumer cannot re-pair
    /// rows from separate semantic parents.
    pub(in crate::mir) fn consume<R>(
        self,
        consumer: impl FnOnce(
            VerifiedLoopInitializedLocalInputSourceSetV1,
            PreparedLoopOperationProgramV1,
            VerifiedMain0InBodyStepTailV1,
            VerifiedMain0InBodyStepControlSourceV1,
        ) -> R,
    ) -> R {
        consumer(self.input, self.operation, self.tail, self.control)
    }
}

/// Canonical Main0 in-body-step issuer. The complete recipe/co-seal product
/// is the only input; separate context/Core/continuation arguments are
/// rejected by construction because this API has no such shape.
pub(in crate::mir) fn issue_main0_in_body_step_semantic_program_v1(
    product: VerifiedMain0InBodyStepRecipeProductV1,
) -> VerifiedMain0InBodyStepSemanticProgramV1 {
    let (operation_effect, input, context, continuation, tail, control) = product.into_parts();
    VerifiedMain0InBodyStepSemanticProgramV1 {
        operation_effect,
        input,
        context,
        continuation,
        tail,
        control,
    }
}
