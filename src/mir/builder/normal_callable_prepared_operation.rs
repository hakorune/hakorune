//! One-shot callable ingress to the neutral full-operation preflight.
//!
//! This module is the only consumer of `PreparedCallableLoopIngressV1` for
//! the current row. It retains the existing source receipt and callable
//! boundary contracts, while the common `PreparedLoopOperationProgramV1`
//! remains the sole full-demand owner. No Builder or physical identity is
//! created here.

use super::normal_callable_semantic_source::{
    PreparedCallableLoopIngressV1, VerifiedNormalCallableSourceIngressReceiptV1,
};
use crate::mir::compiler::callable_single_loop_operation_effect::{
    issue_callable_operation_effect_parts_v1, CallableOperationEffectAdapterRejectV1,
};
use crate::mir::compiler::callable_single_loop_recipe_coseal::{
    VerifiedCallablePreludeV1, VerifiedCallableTailV1, VerifiedLoopInputRelationV1,
};
use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedLoopOperationPhysicalDemandV1, VerifiedLoopSemanticContextV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedCallableLoopOperationRejectV1 {
    SourceOwnerMismatch,
    SourceOriginMismatch,
    SourceKindMismatch,
    SourceLoopSiteMismatch,
    SourceFrameMismatch,
    SourceScopeRegionMismatch,
    InputOwnerMismatch,
    PreludeOwnerMismatch,
    TailOwnerMismatch,
    OperationEffect(CallableOperationEffectAdapterRejectV1),
    Demand(LoopOperationPhysicalDemandRejectV1),
}

/// Thin profile transport after complete Builder-free operation preflight.
/// The common program owns Recipe/JoinSig/operation/effect/continuation; this
/// wrapper retains only the callable source and boundary contracts for the
/// later physical row.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableLoopOperationProgramV1<'source> {
    source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
    input: VerifiedLoopInputRelationV1,
    operation: PreparedLoopOperationProgramV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}

impl<'source> PreparedCallableLoopIngressV1<'source> {
    /// Consume one prepared ingress and run the existing complete neutral
    /// operation/effect preflight exactly once. This is intentionally the
    /// only full-demand entry for the callable profile.
    pub(super) fn prepare_full_demand(
        self,
    ) -> Result<
        PreparedCallableLoopOperationProgramV1<'source>,
        PreparedCallableLoopOperationRejectV1,
    > {
        let (source, logical) = self.into_parts();
        let parts = issue_callable_operation_effect_parts_v1(logical)
            .map_err(PreparedCallableLoopOperationRejectV1::OperationEffect)?;
        let (operation_effect, input, context, continuation, prelude, tail) = parts.into_parts();
        verify_source_context(&source, &context)?;
        if input.source_binding().owner() != source.owner() {
            return Err(PreparedCallableLoopOperationRejectV1::InputOwnerMismatch);
        }
        if prelude.owner() != source.owner() {
            return Err(PreparedCallableLoopOperationRejectV1::PreludeOwnerMismatch);
        }
        if tail.owner() != source.owner() {
            return Err(PreparedCallableLoopOperationRejectV1::TailOwnerMismatch);
        }
        let demand =
            VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)
                .map_err(PreparedCallableLoopOperationRejectV1::Demand)?;
        let operation = demand
            .prepare_all()
            .map_err(PreparedCallableLoopOperationRejectV1::Demand)?;
        Ok(PreparedCallableLoopOperationProgramV1 {
            source,
            input,
            operation,
            prelude,
            tail,
        })
    }
}

impl<'source> PreparedCallableLoopOperationProgramV1<'source> {
    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedNormalCallableSourceIngressReceiptV1<'source>,
        VerifiedLoopInputRelationV1,
        PreparedLoopOperationProgramV1,
        VerifiedCallablePreludeV1,
        VerifiedCallableTailV1,
    ) {
        (
            self.source,
            self.input,
            self.operation,
            self.prelude,
            self.tail,
        )
    }
}

fn verify_source_context(
    source: &VerifiedNormalCallableSourceIngressReceiptV1<'_>,
    context: &VerifiedLoopSemanticContextV1,
) -> Result<(), PreparedCallableLoopOperationRejectV1> {
    if source.owner() != context.owner() || source.ledger().owner() != context.owner() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceOwnerMismatch);
    }
    if source.ledger().function_origin() != context.origin() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceOriginMismatch);
    }
    if source.ledger().source_kind() != context.source_kind() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceKindMismatch);
    }
    let membership = source
        .ledger()
        .only_loop_site()
        .map_err(|_| PreparedCallableLoopOperationRejectV1::SourceLoopSiteMismatch)?;
    if membership.source().site() != context.loop_site() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceLoopSiteMismatch);
    }
    if membership.frame() != context.frame() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceFrameMismatch);
    }
    if membership.scope_region() != context.scope_region() {
        return Err(PreparedCallableLoopOperationRejectV1::SourceScopeRegionMismatch);
    }
    Ok(())
}

#[cfg(test)]
#[path = "normal_callable_prepared_operation_tests.rs"]
mod tests;
