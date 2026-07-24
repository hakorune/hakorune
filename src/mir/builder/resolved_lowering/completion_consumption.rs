//! Exact completion consumption and post-Lower canonical finalization.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirBuilder, MirFunction, MirInstruction, ValueId};

#[derive(Debug)]
pub(super) struct ResolvedFunctionCompletionConsumptionV1 {
    completion: VerifiedFunctionCompletionV1,
    explicit_consumed: bool,
    explicit_operand: Option<ReturnOperandWitnessV1>,
}

/// Temporal witness minted only after every current canonical Lower finish.
///
/// The future SSA-I1 finish slots before this witness without changing the
/// finalizer API. Raw pre-Builder completion products cannot finalize a draft.
#[derive(Debug)]
pub(super) struct ReadyFunctionCompletionV1 {
    completion: VerifiedFunctionCompletionV1,
    explicit_operand: Option<ReturnOperandWitnessV1>,
}

impl ReadyFunctionCompletionV1 {
    pub(super) fn explicit_operand(&self) -> Option<ReturnOperandWitnessV1> {
        self.explicit_operand
    }
}

/// Builder-side evidence for the one explicit source exit accepted by F1.
///
/// The source completion contract decides whether the operand is a return;
/// this witness records only the exact already-lowered physical operand and
/// block so draft sealing never rediscovers it by scanning MIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReturnOperandWitnessV1 {
    block: BasicBlockId,
    value: ValueId,
}

impl ReturnOperandWitnessV1 {
    pub(super) fn new(block: BasicBlockId, value: ValueId) -> Self {
        Self { block, value }
    }

    pub(super) fn block(self) -> BasicBlockId {
        self.block
    }

    pub(super) fn value(self) -> ValueId {
        self.value
    }
}

impl ResolvedFunctionCompletionConsumptionV1 {
    pub(super) fn new(
        expected_owner: FunctionOwnerIdV1,
        completion: VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        if completion.owner() != expected_owner {
            return Err("[freeze:contract][canonical_completion/owner_mismatch]".to_string());
        }
        if completion.unreachable_suffix_count() != 0 {
            return Err("[freeze:contract][canonical_completion/unreachable_suffix]".to_string());
        }
        if !completion.cleanup().crossed_scopes().is_empty() {
            return Err("[freeze:contract][canonical_completion/e0_cleanup_not_empty]".to_string());
        }
        Ok(Self {
            completion,
            explicit_consumed: false,
            explicit_operand: None,
        })
    }

    pub(super) fn claim_explicit_return(
        &mut self,
        site: &SourceStmtSiteV1,
        target_function: RegionId,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        if self.explicit_consumed {
            return Err("[freeze:contract][canonical_completion/explicit_reconsumed]".to_string());
        }
        if self.completion.explicit_site() != Some(site) {
            return Err(
                "[freeze:contract][canonical_completion/explicit_site_mismatch]".to_string(),
            );
        }
        if self.completion.target_function() != target_function {
            return Err("[freeze:contract][canonical_completion/target_mismatch]".to_string());
        }
        self.explicit_consumed = true;
        self.explicit_operand = Some(ReturnOperandWitnessV1::new(block, value));
        Ok(())
    }

    pub(super) fn finish(
        self,
        root_body: &SourceBodySiteV1,
        root_body_end: u32,
        target_function: RegionId,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        if self.completion.target_function() != target_function {
            return Err(
                "[freeze:contract][canonical_completion/finish_target_mismatch]".to_string(),
            );
        }
        if self.completion.explicit_site().is_some() != self.explicit_consumed {
            return Err("[freeze:contract][canonical_completion/consumption_mismatch]".to_string());
        }
        if self.completion.explicit_site().is_some() != self.explicit_operand.is_some() {
            return Err(
                "[freeze:contract][canonical_completion/operand_witness_missing]".to_string(),
            );
        }
        if let Some((expected_body, expected_end)) = self.completion.implicit_body_end() {
            if expected_body != root_body || expected_end != root_body_end {
                return Err(
                    "[freeze:contract][canonical_completion/implicit_body_mismatch]".to_string(),
                );
            }
        }
        Ok(ReadyFunctionCompletionV1 {
            completion: self.completion,
            explicit_operand: self.explicit_operand,
        })
    }
}

pub(super) fn emit_canonical_explicit_return(
    builder: &mut MirBuilder,
    value: ValueId,
) -> Result<(), String> {
    verify_canonical_return_state(builder)?;
    if builder.is_current_block_terminated() {
        return Err(
            "[freeze:contract][canonical_completion/return_block_already_terminated]".to_string(),
        );
    }
    builder.emit_instruction(MirInstruction::Return { value: Some(value) })?;
    Ok(())
}

pub(super) fn finalize_ready_function_completion(
    builder: &mut MirBuilder,
    ready: ReadyFunctionCompletionV1,
) -> Result<MirFunction, String> {
    verify_canonical_return_state(builder)?;
    let completion = ready.completion;
    let current_block = builder.function_state.current_block.ok_or_else(|| {
        "[freeze:contract][canonical_completion/current_block_missing]".to_string()
    })?;
    let explicit_return = current_terminator_is_return(builder, current_block)?;
    if completion.is_implicit_void() {
        if explicit_return || builder.is_current_block_terminated() {
            return Err(
                "[freeze:contract][canonical_completion/implicit_already_terminated]".to_string(),
            );
        }
        let value = crate::mir::builder::emission::constant::emit_void(builder)?;
        emit_canonical_explicit_return(builder, value)?;
    } else if !explicit_return {
        return Err("[freeze:contract][canonical_completion/explicit_return_missing]".to_string());
    }

    let returns_value = completion.returns_value();
    let draft = builder.finalize_function_draft(returns_value)?;
    if !matches!(
        draft
            .get_block(current_block)
            .and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::Return { .. })
    ) {
        return Err("[freeze:contract][canonical_completion/final_return_missing]".to_string());
    }
    Ok(draft)
}

/// Finalizes the SSA-I1-T draft after its return block and whole CFG were
/// already emitted and sealed by the function-owned SSA transaction.
pub(super) fn finalize_preterminated_function_completion(
    builder: &mut MirBuilder,
    ready: ReadyFunctionCompletionV1,
) -> Result<MirFunction, String> {
    verify_canonical_return_state(builder)?;
    let current_block = builder.function_state.current_block.ok_or_else(|| {
        "[freeze:contract][canonical_completion/current_block_missing]".to_string()
    })?;
    if !current_terminator_is_return(builder, current_block)? {
        return Err(
            "[freeze:contract][canonical_completion/preterminated_return_missing]".to_string(),
        );
    }
    let returns_value = ready.completion.returns_value();
    let draft = builder.finalize_function_draft(returns_value)?;
    if !matches!(
        draft
            .get_block(current_block)
            .and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::Return { .. })
    ) {
        return Err("[freeze:contract][canonical_completion/final_return_missing]".to_string());
    }
    Ok(draft)
}

fn verify_canonical_return_state(builder: &MirBuilder) -> Result<(), String> {
    if builder.function_state.return_defer_active
        || builder.function_state.return_defer_slot.is_some()
        || builder.function_state.return_defer_target.is_some()
        || builder.function_state.return_deferred_emitted
        || builder.function_state.in_cleanup_block
    {
        return Err(
            "[freeze:contract][canonical_completion/legacy_return_state_active]".to_string(),
        );
    }
    Ok(())
}

fn current_terminator_is_return(builder: &MirBuilder, block: BasicBlockId) -> Result<bool, String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][canonical_completion/function_missing]".to_string())?;
    let block = function
        .get_block(block)
        .ok_or_else(|| "[freeze:contract][canonical_completion/block_missing]".to_string())?;
    Ok(matches!(
        block.terminator.as_ref(),
        Some(MirInstruction::Return { .. })
    ))
}
