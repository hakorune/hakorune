//! Caller-zero contracts for the post-Recipe callable Loop boundary.
//!
//! This module is deliberately test-only while the physical selector is
//! parked.  It joins existing resolver, Recipe, ABI, and completion products
//! without opening a Builder session.  The only new fact is the relation that
//! those products may be executed together; no source meaning is re-resolved.

#![cfg(test)]

use std::ptr;

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::VerifiedLoopPhysicalBoundaryV1;
use crate::mir::resolved_control_flow::{
    DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, RegionId, ResolvedCallableRefV1, VerifiedCallableHeaderV1,
    VerifiedCallableIndexV1,
};

use super::callable_single_loop_prelude_arguments::{
    PreludeArgumentRejectV1, VerifiedCallablePreludeArgumentListV1,
};
use super::callable_single_loop_recipe_coseal::{
    VerifiedCallablePreludeV1, VerifiedCallableSingleLoopRecipeProductV1, VerifiedCallableTailV1,
    VerifiedLoopRecipeCoSealV1,
};
use super::callable_single_loop_source_shapes::SourceReceiverShapeV1;
use super::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalPrepareRejectReasonV1 {
    MissingCallableIndex,
    MissingCallableHeader,
    ForeignCallableIndex,
    ForeignCallableHeader,
    OwnerHeaderMismatch,
    HeaderIndexMismatch,
    MissingPreludeTarget,
    PreludeTargetHeaderMissing,
    PreludeOwnerMismatch,
    PreludeReceiverMismatch,
    PreludeArityMismatch,
    PreludeResultAbiUnsupported,
    TerminalOwnerMismatch,
    TerminalTargetMismatch,
    TerminalSiteMismatch,
    TerminalNotValue,
    TerminalBindingMismatch,
    TerminalAbiMismatch,
    DeclaredResultAbiUnsupported,
    DeclaredResultAbiMismatch,
    PreludeArgument(PreludeArgumentRejectV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalPrepareRejectV1 {
    NoSafeSlice(LoopPhysicalPrepareRejectReasonV1),
}

/// A borrowed input is branded only when its catalog and header are the exact
/// objects already attached to the resolved input.  The brand is intentionally
/// not `Clone`, even though the underlying view is `Copy`.
#[derive(Debug)]
pub(crate) struct VerifiedCallableFunctionLoweringInputV1<'a> {
    input: ResolvedFunctionLoweringInputV1<'a>,
    index: &'a VerifiedCallableIndexV1,
    header: &'a VerifiedCallableHeaderV1,
}

impl<'a> VerifiedCallableFunctionLoweringInputV1<'a> {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'a>,
        index: &'a VerifiedCallableIndexV1,
        header: &'a VerifiedCallableHeaderV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let Some(attached_index) = input.callable_index() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingCallableIndex,
            ));
        };
        if !ptr::eq(attached_index, index) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableIndex,
            ));
        }
        let Some(attached_header) = input.callable_header() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingCallableHeader,
            ));
        };
        if !ptr::eq(attached_header, header) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableHeader,
            ));
        }
        if input.owner() != header.callable().owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::OwnerHeaderMismatch,
            ));
        }
        let indexed_header = index
            .header_for_callable(header.callable())
            .map_err(|_| no_safe_slice(LoopPhysicalPrepareRejectReasonV1::HeaderIndexMismatch))?;
        if !ptr::eq(indexed_header, header) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::HeaderIndexMismatch,
            ));
        }
        Ok(Self {
            input,
            index,
            header,
        })
    }

    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        self.input
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    pub(crate) const fn index(&self) -> &'a VerifiedCallableIndexV1 {
        self.index
    }

    pub(crate) const fn header(&self) -> &'a VerifiedCallableHeaderV1 {
        self.header
    }
}

/// The common Loop demand owns the co-sealed logical product.  It does not
/// duplicate topology, create physical IDs, or retain the source AST.
#[derive(Debug)]
pub(crate) struct VerifiedLoopPhysicalDemandV1 {
    co_seal: VerifiedLoopRecipeCoSealV1,
}

impl VerifiedLoopPhysicalDemandV1 {
    pub(crate) fn issue(co_seal: VerifiedLoopRecipeCoSealV1) -> Self {
        Self { co_seal }
    }

    pub(crate) fn co_seal(&self) -> &VerifiedLoopRecipeCoSealV1 {
        &self.co_seal
    }

    pub(crate) fn into_co_seal(self) -> VerifiedLoopRecipeCoSealV1 {
        self.co_seal
    }

    pub(crate) fn into_physical_boundary(self) -> VerifiedLoopPhysicalBoundaryV1 {
        self.co_seal.into_physical_boundary()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallablePreludeCapabilityV1 {
    owner: FunctionOwnerIdV1,
    site: crate::mir::resolved_semantics::SourceExprSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    target: ResolvedCallableRefV1,
    receiver: SourceReceiverShapeV1,
    arity: u32,
    result_abi: ExactTrivialReturnAbiV1,
    arguments: VerifiedCallablePreludeArgumentListV1,
}

impl VerifiedCallablePreludeCapabilityV1 {
    pub(crate) fn issue(
        branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
        prelude: &VerifiedCallablePreludeV1,
        expected_receiver: SourceReceiverShapeV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let Some(target) = prelude.direct_callable() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingPreludeTarget,
            ));
        };
        if prelude.owner() != branded.owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeOwnerMismatch,
            ));
        }
        if prelude.call().receiver() != expected_receiver {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeReceiverMismatch,
            ));
        }
        let header = branded.index().header_for_callable(target).map_err(|_| {
            no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeTargetHeaderMissing)
        })?;
        if prelude.call().argument_count() as usize != header.signature().arity() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeArityMismatch,
            ));
        }
        let result_abi =
            ExactTrivialReturnAbiV1::classify(header.signature().result().source_type_name())
                .ok_or_else(|| {
                    no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeResultAbiUnsupported)
                })?;
        let arguments =
            VerifiedCallablePreludeArgumentListV1::issue(branded.input(), prelude, header)
                .map_err(|reason| {
                    no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeArgument(reason))
                })?;
        Ok(Self {
            owner: prelude.owner(),
            site: prelude.site().clone(),
            binding: prelude.binding(),
            target,
            receiver: prelude.call().receiver(),
            arity: prelude.call().argument_count(),
            result_abi,
            arguments,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(crate) const fn target(&self) -> ResolvedCallableRefV1 {
        self.target
    }

    pub(crate) const fn receiver(&self) -> SourceReceiverShapeV1 {
        self.receiver
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) const fn result_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.result_abi
    }

    pub(crate) fn arguments(&self) -> &VerifiedCallablePreludeArgumentListV1 {
        &self.arguments
    }

    #[allow(dead_code)]
    pub(crate) fn site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        &self.site
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableTerminalCompatibilityV1 {
    owner: FunctionOwnerIdV1,
    callable_target: ResolvedCallableRefV1,
    target_function: RegionId,
    statement: crate::mir::resolved_semantics::SourceStmtSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedCallableTerminalCompatibilityV1 {
    pub(crate) fn issue(
        branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
        prelude: &VerifiedCallablePreludeCapabilityV1,
        tail: &VerifiedCallableTailV1,
        completion: &VerifiedFunctionCompletionV1,
        abi: ExactTrivialReturnAbiV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        if completion.owner() != branded.owner() || tail.owner() != branded.owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalOwnerMismatch,
            ));
        }
        let expected_target = branded
            .input()
            .function()
            .lowering_roots()
            .function_pair()
            .region();
        if completion.target_function() != expected_target {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalTargetMismatch,
            ));
        }
        if completion.explicit_site() != Some(tail.statement()) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalSiteMismatch,
            ));
        }
        if !completion.returns_value() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalNotValue,
            ));
        }
        if prelude.binding() != tail.binding() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalBindingMismatch,
            ));
        }
        if prelude.result_abi() != abi {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalAbiMismatch,
            ));
        }
        match completion.function_exit_contract().declared_result() {
            DeclaredFunctionResultContractV1::Annotated(name)
                if name.as_ref() == abi.source_type_name() => {}
            _ => {
                return Err(no_safe_slice(
                    LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch,
                ))
            }
        }
        Ok(Self {
            owner: branded.owner(),
            callable_target: prelude.target(),
            target_function: completion.target_function(),
            statement: tail.statement().clone(),
            binding: tail.binding(),
            abi,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn target_function(&self) -> RegionId {
        self.target_function
    }

    pub(crate) const fn callable_target(&self) -> ResolvedCallableRefV1 {
        self.callable_target
    }

    pub(crate) const fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }

    #[allow(dead_code)]
    pub(crate) fn statement(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.statement
    }
}

/// One pre-effect callable execution product.  Completion is moved into this
/// product exactly once and is not copied into the common Loop demand.
#[derive(Debug)]
pub(crate) struct PreparedCallableLoopPhysicalizationV1<'a> {
    pub(crate) input: VerifiedCallableFunctionLoweringInputV1<'a>,
    pub(crate) demand: VerifiedLoopPhysicalDemandV1,
    pub(crate) prelude: VerifiedCallablePreludeCapabilityV1,
    pub(crate) tail: VerifiedCallableTailV1,
    pub(crate) terminal: VerifiedCallableTerminalCompatibilityV1,
    pub(crate) completion: VerifiedFunctionCompletionV1,
}

impl<'a> PreparedCallableLoopPhysicalizationV1<'a> {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'a>,
        index: &'a VerifiedCallableIndexV1,
        header: &'a VerifiedCallableHeaderV1,
        product: VerifiedCallableSingleLoopRecipeProductV1,
        completion: VerifiedFunctionCompletionV1,
        // The profile supplies this already-verified source-call shape.  The
        // prepare layer never guesses a receiver kind from a callable name.
        expected_receiver: SourceReceiverShapeV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let input = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)?;
        let (co_seal, prelude, tail) = product.into_parts();
        let prelude_capability =
            VerifiedCallablePreludeCapabilityV1::issue(&input, &prelude, expected_receiver)?;
        let abi = declared_result_abi(&input, &completion)?;
        let terminal = VerifiedCallableTerminalCompatibilityV1::issue(
            &input,
            &prelude_capability,
            &tail,
            &completion,
            abi,
        )?;
        Ok(Self {
            input,
            demand: VerifiedLoopPhysicalDemandV1::issue(co_seal),
            prelude: prelude_capability,
            tail,
            terminal,
            completion,
        })
    }

    pub(crate) const fn input(&self) -> &VerifiedCallableFunctionLoweringInputV1<'a> {
        &self.input
    }

    pub(crate) fn demand(&self) -> &VerifiedLoopPhysicalDemandV1 {
        &self.demand
    }

    pub(crate) fn into_demand(self) -> VerifiedLoopPhysicalDemandV1 {
        self.demand
    }

    pub(crate) const fn prelude(&self) -> &VerifiedCallablePreludeCapabilityV1 {
        &self.prelude
    }

    pub(crate) fn tail(&self) -> &VerifiedCallableTailV1 {
        &self.tail
    }

    pub(crate) const fn terminal(&self) -> &VerifiedCallableTerminalCompatibilityV1 {
        &self.terminal
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }
}

pub(crate) fn issue_callable_loop_physicalization_v1<'a>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    index: &'a VerifiedCallableIndexV1,
    header: &'a VerifiedCallableHeaderV1,
    product: VerifiedCallableSingleLoopRecipeProductV1,
    completion: VerifiedFunctionCompletionV1,
    expected_receiver: SourceReceiverShapeV1,
) -> Result<PreparedCallableLoopPhysicalizationV1<'a>, LoopPhysicalPrepareRejectV1> {
    PreparedCallableLoopPhysicalizationV1::issue(
        input,
        index,
        header,
        product,
        completion,
        expected_receiver,
    )
}

fn declared_result_abi(
    branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<ExactTrivialReturnAbiV1, LoopPhysicalPrepareRejectV1> {
    let DeclaredFunctionResultContractV1::Annotated(name) =
        completion.function_exit_contract().declared_result()
    else {
        return Err(no_safe_slice(
            LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiUnsupported,
        ));
    };
    let completion_abi = ExactTrivialReturnAbiV1::classify(name).ok_or_else(|| {
        no_safe_slice(LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiUnsupported)
    })?;
    let header_abi =
        ExactTrivialReturnAbiV1::classify(branded.header().signature().result().source_type_name())
            .ok_or_else(|| {
                no_safe_slice(LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch)
            })?;
    if completion_abi != header_abi {
        return Err(no_safe_slice(
            LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch,
        ));
    }
    Ok(completion_abi)
}

fn no_safe_slice(reason: LoopPhysicalPrepareRejectReasonV1) -> LoopPhysicalPrepareRejectV1 {
    LoopPhysicalPrepareRejectV1::NoSafeSlice(reason)
}

#[cfg(test)]
#[path = "loop_physical_prepare_tests.rs"]
mod tests;
