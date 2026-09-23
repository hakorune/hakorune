//! One-shot Main0 in-body-step ingress to the neutral full-operation
//! preflight.
//!
//! This module is the only consumer of `PreparedMain0InBodyStepLoopIngressV1`
//! for the current row. It retains the lent installed-source input and the
//! sealed Main0 in-body-step receipts, while the common
//! `PreparedLoopOperationProgramV1` remains the sole full-demand owner. No
//! Builder or physical identity is created here.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::main0_in_body_step_recipe_coseal::{
    VerifiedMain0InBodyStepControlSourceV1, VerifiedMain0InBodyStepRecipeProductV1,
    VerifiedMain0InBodyStepTailV1,
};
use crate::mir::compiler::main0_in_body_step_semantic_program::issue_main0_in_body_step_semantic_program_v1;
use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedLoopInitializedLocalInputSourceSetV1, VerifiedLoopSemanticContextV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedMain0InBodyStepLoopOperationRejectV1 {
    SourceLedgerUnavailable,
    SourceOwnerMismatch,
    SourceOriginMismatch,
    SourceKindMismatch,
    SourceLoopSiteMismatch,
    SourceFrameMismatch,
    SourceScopeRegionMismatch,
    InputOwnerMismatch,
    TailOwnerMismatch,
    ControlOwnerMismatch,
    Demand(LoopOperationPhysicalDemandRejectV1),
}

/// Pair of the lent installed App Main input and the selected Main0
/// in-body-step semantic product. Issued once inside the one-shot
/// source-loan callback; the input borrow cannot escape that scope, so the
/// full handoff is driven from `prepare_full_demand`.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedMain0InBodyStepLoopIngressV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    product: VerifiedMain0InBodyStepRecipeProductV1,
}

impl<'source> PreparedMain0InBodyStepLoopIngressV1<'source> {
    /// Issue the ingress inside the App Main source-loan callback. The
    /// caller must pass exactly the input lent by
    /// `with_app_main_root_lowering_input` together with the product
    /// selected from the same source observation.
    pub(in crate::mir::builder) fn issue(
        input: ResolvedFunctionLoweringInputV1<'source>,
        product: VerifiedMain0InBodyStepRecipeProductV1,
    ) -> Result<Self, PreparedMain0InBodyStepLoopOperationRejectV1> {
        if input.owner() != product.tail().owner() {
            return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceOwnerMismatch);
        }
        Ok(Self { input, product })
    }
}

/// Thin profile transport after complete Builder-free operation preflight.
/// The common program owns Recipe/JoinSig/operation/effect/continuation;
/// this wrapper retains only the lent source input and the Main0 boundary
/// contracts for the later physical row.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedMain0InBodyStepOperationProgramV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    locals: VerifiedLoopInitializedLocalInputSourceSetV1,
    operation: PreparedLoopOperationProgramV1,
    tail: VerifiedMain0InBodyStepTailV1,
    control: VerifiedMain0InBodyStepControlSourceV1,
}

impl<'source> PreparedMain0InBodyStepLoopIngressV1<'source> {
    /// Consume one prepared ingress and run the existing complete neutral
    /// operation/effect preflight exactly once. This is intentionally the
    /// only full-demand entry for the Main0 in-body-step profile.
    pub(in crate::mir::builder) fn prepare_full_demand(
        self,
    ) -> Result<
        PreparedMain0InBodyStepOperationProgramV1<'source>,
        PreparedMain0InBodyStepLoopOperationRejectV1,
    > {
        let Self { input, product } = self;
        let program = issue_main0_in_body_step_semantic_program_v1(product);
        let prepared = program
            .into_prepared_operation_demand()
            .map_err(PreparedMain0InBodyStepLoopOperationRejectV1::Demand)?;
        prepared.consume(|locals, operation, tail, control| {
            verify_source_context(&input, operation.demand().context())?;
            if locals.owner() != input.owner() {
                return Err(PreparedMain0InBodyStepLoopOperationRejectV1::InputOwnerMismatch);
            }
            if tail.owner() != input.owner() {
                return Err(PreparedMain0InBodyStepLoopOperationRejectV1::TailOwnerMismatch);
            }
            if control.owner() != input.owner() {
                return Err(PreparedMain0InBodyStepLoopOperationRejectV1::ControlOwnerMismatch);
            }
            Ok(PreparedMain0InBodyStepOperationProgramV1 {
                input,
                locals,
                operation,
                tail,
                control,
            })
        })
    }
}

impl<'source> PreparedMain0InBodyStepOperationProgramV1<'source> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'source>,
        VerifiedLoopInitializedLocalInputSourceSetV1,
        PreparedLoopOperationProgramV1,
        VerifiedMain0InBodyStepTailV1,
        VerifiedMain0InBodyStepControlSourceV1,
    ) {
        (
            self.input,
            self.locals,
            self.operation,
            self.tail,
            self.control,
        )
    }
}

fn verify_source_context(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    context: &VerifiedLoopSemanticContextV1,
) -> Result<(), PreparedMain0InBodyStepLoopOperationRejectV1> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| PreparedMain0InBodyStepLoopOperationRejectV1::SourceLedgerUnavailable)?;
    if input.owner() != context.owner() || ledger.owner() != context.owner() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceOwnerMismatch);
    }
    if ledger.function_origin() != context.origin() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceOriginMismatch);
    }
    if ledger.source_kind() != context.source_kind() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceKindMismatch);
    }
    let membership = ledger
        .only_loop_site()
        .map_err(|_| PreparedMain0InBodyStepLoopOperationRejectV1::SourceLoopSiteMismatch)?;
    if membership.source().site() != context.loop_site() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceLoopSiteMismatch);
    }
    if membership.frame() != context.frame() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceFrameMismatch);
    }
    if membership.scope_region() != context.scope_region() {
        return Err(PreparedMain0InBodyStepLoopOperationRejectV1::SourceScopeRegionMismatch);
    }
    Ok(())
}
