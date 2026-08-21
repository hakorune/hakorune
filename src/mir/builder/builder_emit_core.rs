//! Shared physical append core and strict Compare writer contract.
//!
//! The legacy emitter and the canonical prepared writer both reach the one
//! `append_instruction_core` mutation point. This child contains no route
//! selection or repair logic.

use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    ReservedCanonicalCompareDestinationV1, VerifiedCanonicalSameBlockIntegerOperandV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CanonicalCompareAppendProofV1 {
    target: BasicBlockId,
    instruction_index: usize,
    op: crate::mir::CompareOp,
    lhs: ValueId,
    rhs: ValueId,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CanonicalCompareDefinitionSourceV1 {
    owner: FunctionOwnerIdV1,
    target: BasicBlockId,
    physical_value: ValueId,
    proof: CanonicalCompareAppendProofV1,
}

impl CanonicalCompareDefinitionSourceV1 {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn target(&self) -> BasicBlockId {
        self.target
    }

    pub(in crate::mir::builder) const fn physical_value(&self) -> ValueId {
        self.physical_value
    }

    pub(in crate::mir::builder) const fn proof(&self) -> &CanonicalCompareAppendProofV1 {
        &self.proof
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalCompareAppendRejectV1 {
    FunctionMissing,
    TargetMissing(BasicBlockId),
    TargetOwnerMismatch,
    TargetSealed(BasicBlockId),
    TargetTerminated(BasicBlockId),
    OperandOwnerMismatch,
    OperandTargetMismatch,
    OperandDefinitionDrift,
    DestinationOwnerMismatch,
    DestinationAliasesOperand,
    DestinationAlreadyDefined,
    DestinationTypeAlreadyPublished,
}

pub(in crate::mir::builder) struct PreparedCanonicalCompareAppendV1<'builder> {
    builder: &'builder mut MirBuilder,
    target: VerifiedCanonicalOpenInstructionTargetV1,
    lhs: VerifiedCanonicalSameBlockIntegerOperandV1,
    rhs: VerifiedCanonicalSameBlockIntegerOperandV1,
    destination: ReservedCanonicalCompareDestinationV1,
    op: crate::mir::CompareOp,
    instruction_index: usize,
}

pub(super) fn append_instruction_core(
    function: &mut MirFunction,
    block_id: BasicBlockId,
    instruction: MirInstruction,
    span: crate::ast::Span,
) -> Result<(), String> {
    let block = function
        .get_block_mut(block_id)
        .ok_or_else(|| format!("Basic block {} does not exist", block_id))?;
    block.add_instruction_with_span(instruction, span);
    Ok(())
}

impl<'builder> PreparedCanonicalCompareAppendV1<'builder> {
    /// Commit one already-prepared Compare append. The shared append core is
    /// the only physical mutation point; prepared invariants make this commit
    /// infallible, so a violated invariant is an internal contract panic.
    pub(in crate::mir::builder) fn commit(self) -> CanonicalCompareDefinitionSourceV1 {
        let Self {
            builder,
            target,
            lhs,
            rhs,
            destination,
            op,
            instruction_index,
        } = self;
        let lhs_value = lhs.physical_value();
        let rhs_value = rhs.physical_value();
        let physical_value = destination.value();
        let instruction = MirInstruction::Compare {
            dst: physical_value,
            op,
            lhs: lhs_value,
            rhs: rhs_value,
        };
        let span = builder.metadata_ctx.current_span();
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .expect("prepared canonical Compare requires current function");
        append_instruction_core(function, target.block(), instruction, span)
            .expect("prepared canonical Compare target must remain appendable");
        CanonicalCompareDefinitionSourceV1 {
            owner: target.owner(),
            target: target.block(),
            physical_value,
            proof: CanonicalCompareAppendProofV1 {
                target: target.block(),
                instruction_index,
                op,
                lhs: lhs_value,
                rhs: rhs_value,
            },
        }
    }
}

impl MirBuilder {
    /// Prepare a strict Compare append from already-issued canonical
    /// witnesses. No block creation, current-block selection, type inference,
    /// LocalSSA repair, or PHI materialization is reachable from this path.
    pub(in crate::mir::builder) fn prepare_canonical_compare_append(
        &mut self,
        target: VerifiedCanonicalOpenInstructionTargetV1,
        lhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        rhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        destination: ReservedCanonicalCompareDestinationV1,
        op: crate::mir::CompareOp,
    ) -> Result<PreparedCanonicalCompareAppendV1<'_>, CanonicalCompareAppendRejectV1> {
        if target.owner() != lhs.owner()
            || target.owner() != rhs.owner()
            || target.owner() != destination.owner()
        {
            return Err(CanonicalCompareAppendRejectV1::OperandOwnerMismatch);
        }
        if lhs.target_block() != target.block()
            || rhs.target_block() != target.block()
            || lhs.definition_block() != target.block()
            || rhs.definition_block() != target.block()
        {
            return Err(CanonicalCompareAppendRejectV1::OperandTargetMismatch);
        }
        if lhs.physical_value() == destination.value()
            || rhs.physical_value() == destination.value()
        {
            return Err(CanonicalCompareAppendRejectV1::DestinationAliasesOperand);
        }

        let instruction_index = {
            let function = self
                .function_state
                .current_function
                .as_ref()
                .ok_or(CanonicalCompareAppendRejectV1::FunctionMissing)?;
            let block = function.get_block(target.block()).ok_or(
                CanonicalCompareAppendRejectV1::TargetMissing(target.block()),
            )?;
            if block.is_sealed() {
                return Err(CanonicalCompareAppendRejectV1::TargetSealed(target.block()));
            }
            if block.is_terminated() {
                return Err(CanonicalCompareAppendRejectV1::TargetTerminated(
                    target.block(),
                ));
            }
            if !definition_matches(
                block,
                lhs.physical_value(),
                lhs.definition_instruction_index(),
            ) || !definition_matches(
                block,
                rhs.physical_value(),
                rhs.definition_instruction_index(),
            ) {
                return Err(CanonicalCompareAppendRejectV1::OperandDefinitionDrift);
            }
            if function.params.contains(&destination.value())
                || function.blocks.values().any(|candidate| {
                    candidate
                        .instructions
                        .iter()
                        .any(|instruction| instruction.dst_value() == Some(destination.value()))
                })
            {
                return Err(CanonicalCompareAppendRejectV1::DestinationAlreadyDefined);
            }
            block.instructions.len()
        };
        if self
            .function_state
            .type_ctx
            .get_type(destination.value())
            .is_some()
        {
            return Err(CanonicalCompareAppendRejectV1::DestinationTypeAlreadyPublished);
        }
        Ok(PreparedCanonicalCompareAppendV1 {
            builder: self,
            target,
            lhs,
            rhs,
            destination,
            op,
            instruction_index,
        })
    }
}

fn definition_matches(block: &BasicBlock, value: ValueId, instruction_index: usize) -> bool {
    block
        .instructions
        .get(instruction_index)
        .and_then(MirInstruction::dst_value)
        == Some(value)
}
