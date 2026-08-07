//! Test-only callable Tail -> Completion handoff.
//!
//! The common Loop physicalizer stops at `ReadyLoopAfterContinuationV1`.
//! This seam belongs to the outer callable profile: it reads the exact Tail
//! binding through canonical identity, validates the declared trivial ABI,
//! and consumes the existing Completion/return ledgers once.

use super::continuation::{ReadyCallableLoopProfileCloseV1, ReadyLoopAfterContinuationV1};
use super::operation_type::ensure_provisional_value_class;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::callable_single_loop_recipe_coseal::VerifiedCallableTailV1;
use crate::mir::compiler::loop_physical_prepare::VerifiedCallableTerminalCompatibilityV1;
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::LoopValueClassV1;
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::{BasicBlockId, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableTailCompletionRejectV1 {
    OwnerMismatch,
    TailBindingMismatch,
    CurrentBlockMismatch {
        expected: BasicBlockId,
        found: BasicBlockId,
    },
    TailRead(String),
    TailPhysicalBlockMismatch,
    TailTypeMismatch {
        expected: MirType,
        found: Option<MirType>,
    },
    Completion(String),
    Identity(String),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadyCallableTailCompletionV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    block: BasicBlockId,
    value: ValueId,
    abi: ExactTrivialReturnAbiV1,
    profile_close: ReadyCallableLoopProfileCloseV1,
}

impl ReadyCallableTailCompletionV1 {
    pub(super) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn block(&self) -> BasicBlockId {
        self.block
    }

    pub(super) const fn value(&self) -> ValueId {
        self.value
    }

    pub(super) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }

    pub(super) fn into_profile_close(self) -> ReadyCallableLoopProfileCloseV1 {
        self.profile_close
    }
}

/// Consume the sealed After receipt exactly once at the callable boundary.
/// No new CFG/SSA owner is created and no source meaning is rediscovered.
pub(super) fn consume_callable_tail_completion_v1(
    ready: ReadyLoopAfterContinuationV1,
    tail: &VerifiedCallableTailV1,
    terminal: &VerifiedCallableTerminalCompatibilityV1,
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
) -> Result<ReadyCallableTailCompletionV1, CallableTailCompletionRejectV1> {
    let owner = ready.owner();
    if tail.owner() != owner || terminal.owner() != owner {
        return Err(CallableTailCompletionRejectV1::OwnerMismatch);
    }
    if terminal.binding() != tail.binding() {
        return Err(CallableTailCompletionRejectV1::TailBindingMismatch);
    }
    let after = ready.root_after();
    let current = builder.function_state.current_block.ok_or(
        CallableTailCompletionRejectV1::CurrentBlockMismatch {
            expected: after,
            found: BasicBlockId::new(u32::MAX),
        },
    )?;
    if current != after {
        return Err(CallableTailCompletionRejectV1::CurrentBlockMismatch {
            expected: after,
            found: current,
        });
    }

    session
        .identity
        .claim_variable_use_binding(tail.value_site(), tail.binding())
        .map_err(CallableTailCompletionRejectV1::Identity)?;
    let receipt = session
        .identity
        .read_entry_receipt(builder, &mut session.phis, after, tail.binding())
        .map_err(CallableTailCompletionRejectV1::TailRead)?;
    if receipt.owner() != owner || receipt.binding() != tail.binding() {
        return Err(CallableTailCompletionRejectV1::OwnerMismatch);
    }
    if receipt.physical_block() != after {
        return Err(CallableTailCompletionRejectV1::TailPhysicalBlockMismatch);
    }
    let value = receipt.physical_value();
    let expected = terminal.abi().mir_type();
    let found = builder.function_state.type_ctx.get_type(value).cloned();
    if found.as_ref() != Some(&expected) {
        if terminal.abi() != ExactTrivialReturnAbiV1::I64 {
            return Err(CallableTailCompletionRejectV1::TailTypeMismatch { expected, found });
        }
        ensure_provisional_value_class(builder, value, LoopValueClassV1::I64).map_err(|_| {
            CallableTailCompletionRejectV1::TailTypeMismatch {
                expected,
                found: builder.function_state.type_ctx.get_type(value).cloned(),
            }
        })?;
    }

    session
        .completion
        .claim_explicit_return(tail.statement(), terminal.target_function(), after, value)
        .map_err(CallableTailCompletionRejectV1::Completion)?;
    session
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(tail.statement().clone()))
        .map_err(CallableTailCompletionRejectV1::Identity)?;

    Ok(ReadyCallableTailCompletionV1 {
        owner,
        block: after,
        value,
        abi: terminal.abi(),
        profile_close: ready.into_profile_close(),
    })
}
