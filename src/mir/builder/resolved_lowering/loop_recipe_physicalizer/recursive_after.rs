//! R3 recursive Loop transfer closure.
//!
//! The segment layout and operation dispatch are already verified before this
//! module runs.  This module only binds those products to the canonical CFG,
//! identity SSA, and existing PHI transaction; it owns no second graph or
//! retry path.

use super::segment_dispatcher::CompletedLoopSegmentProgramV1;
use super::segment_topology::LoopPhysicalSegmentBlockReceiptV1;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopPhysicalTargetV1, LoopPhysicalTransferV1, LoopValueClassV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RecursiveAfterRejectV1 {
    OwnerMismatch,
    CurrentBlockMismatch {
        expected: BasicBlockId,
        found: BasicBlockId,
    },
    MissingSegment(BasicBlockId),
    MissingRootAfter(BasicBlockId),
    TargetMissing,
    ConditionMissing,
    ConditionOwnerMismatch,
    ConditionClassMismatch,
    ConditionTypeMismatch,
    ConditionPlacementMismatch {
        expected: BasicBlockId,
        found: BasicBlockId,
    },
    TargetFunctionMissing,
    TargetBlockTerminated(BasicBlockId),
    Edge(String),
    CfgSeal(String),
    IdentitySeal(String),
    SelectAfter(String),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadyCallableLoopProfileCloseV1 {
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
    after_predecessors: Box<[BasicBlockId]>,
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    write_count: usize,
    condition_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

impl ReadyCallableLoopProfileCloseV1 {
    pub(super) fn finish(
        self,
        owner: FunctionOwnerIdV1,
        terminal_block: BasicBlockId,
    ) -> Result<(), String> {
        if self.owner != owner || self.terminal_block != terminal_block {
            return Err("callable profile close owner/terminal mismatch".into());
        }
        if self.after_predecessors.len() != 1
            || (
                self.operation_count,
                self.pure_count,
                self.read_count,
                self.write_count,
            ) != (7, 4, 2, 1)
        {
            return Err("callable profile close coverage mismatch".into());
        }
        let _condition_key = self.condition_key;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadyLoopAfterContinuationV1 {
    owner: FunctionOwnerIdV1,
    root_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    root_after: BasicBlockId,
    predecessors: Box<[BasicBlockId]>,
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    write_count: usize,
    condition_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

impl ReadyLoopAfterContinuationV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn root_after(&self) -> BasicBlockId {
        self.root_after
    }

    pub(super) fn into_profile_close(self) -> ReadyCallableLoopProfileCloseV1 {
        ReadyCallableLoopProfileCloseV1 {
            owner: self.owner,
            terminal_block: self.root_after,
            after_predecessors: self.predecessors,
            operation_count: self.operation_count,
            pure_count: self.pure_count,
            read_count: self.read_count,
            write_count: self.write_count,
            condition_key: self.condition_key,
        }
    }
}

pub(super) struct PreparedRecursiveAfterV1 {
    program: CompletedLoopSegmentProgramV1,
    condition_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    condition_value: ValueId,
    counts: (usize, usize, usize, usize),
}

pub(super) fn prepare_recursive_after_v1(
    program: CompletedLoopSegmentProgramV1,
    builder: &MirBuilder,
) -> Result<PreparedRecursiveAfterV1, RecursiveAfterRejectV1> {
    let owner = program.layout.program().demand().context().owner();
    if program.segment_receipt.owner() != owner || program.entry.owner() != owner {
        return Err(RecursiveAfterRejectV1::OwnerMismatch);
    }
    let preheader = program.entry.preheader();
    let current = builder
        .function_state
        .current_block
        .ok_or(RecursiveAfterRejectV1::TargetFunctionMissing)?;
    if current != preheader {
        return Err(RecursiveAfterRejectV1::CurrentBlockMismatch {
            expected: preheader,
            found: current,
        });
    }
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or(RecursiveAfterRejectV1::TargetFunctionMissing)?;
    let root_after = program.segment_receipt.root_after();
    ensure_open_block(function, root_after)?;
    let condition = find_condition(&program)?;
    validate_condition(builder, &program.segment_receipt, condition)?;
    for segment in program.layout.segments() {
        let source = program
            .segment_receipt
            .lookup(segment.key())
            .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
        ensure_open_block(function, source)?;
        validate_transfer(&program.segment_receipt, segment.transfer())?;
    }
    let counts = dispatch_counts(&program);
    Ok(PreparedRecursiveAfterV1 {
        condition_key: condition.key(),
        condition_value: condition.physical_value(),
        counts,
        program,
    })
}

impl PreparedRecursiveAfterV1 {
    pub(super) fn emit_and_seal(
        self,
        builder: &mut MirBuilder,
        cfg: &mut CanonicalCfgSessionV1,
        identity: &mut ResolvedSsaIdentityStateV2<'_>,
        phis: &mut PhiTxn,
    ) -> Result<ReadyLoopAfterContinuationV1, RecursiveAfterRejectV1> {
        let owner = self.program.layout.program().demand().context().owner();
        let preheader = self.program.entry.preheader();
        let root_after = self.program.segment_receipt.root_after();
        let entry_block = self
            .program
            .segment_receipt
            .lookup(self.program.segment_receipt.entry_segment())
            .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
        {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or(RecursiveAfterRejectV1::TargetFunctionMissing)?;
            cfg.emit_jump(function, preheader, entry_block)
                .map_err(|error| RecursiveAfterRejectV1::Edge(error.to_string()))?;
            for segment in self.program.layout.segments() {
                let source = self
                    .program
                    .segment_receipt
                    .lookup(segment.key())
                    .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
                emit_transfer(
                    function,
                    cfg,
                    source,
                    segment.transfer(),
                    &self.program.segment_receipt,
                    self.condition_value,
                )?;
            }
        }
        seal_block(builder, cfg, identity, phis, preheader)?;
        for segment in self.program.layout.segments() {
            let block = self
                .program
                .segment_receipt
                .lookup(segment.key())
                .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
            seal_block(builder, cfg, identity, phis, block)?;
        }
        let after = seal_block(builder, cfg, identity, phis, root_after)?;
        cfg.select_block(builder, root_after)
            .map_err(|error| RecursiveAfterRejectV1::SelectAfter(error.to_string()))?;
        let root_loop = self
            .program
            .layout
            .program()
            .demand()
            .continuation()
            .after()
            .loop_key();
        Ok(ReadyLoopAfterContinuationV1 {
            owner,
            root_loop,
            root_after,
            predecessors: after.predecessors().to_vec().into_boxed_slice(),
            operation_count: self.counts.0,
            pure_count: self.counts.1,
            read_count: self.counts.2,
            write_count: self.counts.3,
            condition_key: self.condition_key,
        })
    }
}

fn find_condition(
    program: &CompletedLoopSegmentProgramV1,
) -> Result<super::operation_ledger::LoopOperationValueReceiptV1, RecursiveAfterRejectV1> {
    let recipe = program
        .layout
        .program()
        .demand()
        .operation_effect()
        .core()
        .recipe()
        .as_recipe();
    let condition = recipe
        .loops
        .iter()
        .find_map(|row| match row.condition {
            crate::mir::loop_recipe_contract::LoopConditionV1::Predicate { value, .. } => {
                Some(value)
            }
            crate::mir::loop_recipe_contract::LoopConditionV1::Always => None,
        })
        .ok_or(RecursiveAfterRejectV1::ConditionMissing)?;
    if !program.dispatch.contains_result(condition) {
        return Err(RecursiveAfterRejectV1::ConditionMissing);
    }
    program
        .values
        .receipt(condition)
        .ok_or(RecursiveAfterRejectV1::ConditionMissing)
}

fn validate_condition(
    builder: &MirBuilder,
    receipt: &LoopPhysicalSegmentBlockReceiptV1,
    condition: super::operation_ledger::LoopOperationValueReceiptV1,
) -> Result<(), RecursiveAfterRejectV1> {
    if condition.owner() != receipt.owner() {
        return Err(RecursiveAfterRejectV1::ConditionOwnerMismatch);
    }
    if condition.class() != LoopValueClassV1::Bool {
        return Err(RecursiveAfterRejectV1::ConditionClassMismatch);
    }
    if builder
        .function_state
        .type_ctx
        .get_type(condition.physical_value())
        != Some(&MirType::Bool)
    {
        return Err(RecursiveAfterRejectV1::ConditionTypeMismatch);
    }
    let expected = receipt
        .lookup(receipt.entry_segment())
        .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
    if condition.physical_block() != expected {
        return Err(RecursiveAfterRejectV1::ConditionPlacementMismatch {
            expected,
            found: condition.physical_block(),
        });
    }
    Ok(())
}

fn validate_transfer(
    receipt: &LoopPhysicalSegmentBlockReceiptV1,
    transfer: LoopPhysicalTransferV1,
) -> Result<(), RecursiveAfterRejectV1> {
    let target = match transfer {
        LoopPhysicalTransferV1::Jump { target } => target,
        LoopPhysicalTransferV1::Predicate {
            on_true, on_false, ..
        } => {
            receipt
                .lookup(on_true)
                .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
            on_false
        }
        LoopPhysicalTransferV1::OpenNestedLoop { entry, .. } => {
            receipt
                .lookup(entry)
                .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
            return Ok(());
        }
    };
    if let LoopPhysicalTargetV1::Segment(segment) = target {
        receipt
            .lookup(segment)
            .ok_or(RecursiveAfterRejectV1::TargetMissing)?;
    }
    Ok(())
}

fn emit_transfer(
    function: &mut crate::mir::MirFunction,
    cfg: &CanonicalCfgSessionV1,
    source: BasicBlockId,
    transfer: LoopPhysicalTransferV1,
    receipt: &LoopPhysicalSegmentBlockReceiptV1,
    condition: ValueId,
) -> Result<(), RecursiveAfterRejectV1> {
    let target = |target: LoopPhysicalTargetV1| -> Result<BasicBlockId, RecursiveAfterRejectV1> {
        match target {
            LoopPhysicalTargetV1::Segment(segment) => receipt
                .lookup(segment)
                .ok_or(RecursiveAfterRejectV1::TargetMissing),
            LoopPhysicalTargetV1::OpenRootAfter => Ok(receipt.root_after()),
        }
    };
    match transfer {
        LoopPhysicalTransferV1::Jump { target: next } => cfg
            .emit_jump(function, source, target(next)?)
            .map_err(|error| RecursiveAfterRejectV1::Edge(error.to_string())),
        LoopPhysicalTransferV1::Predicate {
            on_true, on_false, ..
        } => cfg
            .emit_branch(
                function,
                source,
                condition,
                receipt
                    .lookup(on_true)
                    .ok_or(RecursiveAfterRejectV1::TargetMissing)?,
                target(on_false)?,
            )
            .map_err(|error| RecursiveAfterRejectV1::Edge(error.to_string())),
        LoopPhysicalTransferV1::OpenNestedLoop { entry, .. } => cfg
            .emit_jump(
                function,
                source,
                receipt
                    .lookup(entry)
                    .ok_or(RecursiveAfterRejectV1::TargetMissing)?,
            )
            .map_err(|error| RecursiveAfterRejectV1::Edge(error.to_string())),
    }
}

fn dispatch_counts(program: &CompletedLoopSegmentProgramV1) -> (usize, usize, usize, usize) {
    let mut counts = (0, 0, 0, 0);
    for receipt in program.dispatch.receipts() {
        match receipt {
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Pure(_) => counts.1 += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Read(_) => counts.2 += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Write(_) => counts.3 += 1,
        }
    }
    counts.0 = program.dispatch.operation_count();
    counts
}

fn ensure_open_block(
    function: &crate::mir::MirFunction,
    block: BasicBlockId,
) -> Result<(), RecursiveAfterRejectV1> {
    let target = function
        .get_block(block)
        .ok_or(RecursiveAfterRejectV1::TargetFunctionMissing)?;
    if target.terminator.is_some() {
        return Err(RecursiveAfterRejectV1::TargetBlockTerminated(block));
    }
    Ok(())
}

fn seal_block(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    identity: &mut ResolvedSsaIdentityStateV2<'_>,
    phis: &mut PhiTxn,
    block: BasicBlockId,
) -> Result<
    crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1,
    RecursiveAfterRejectV1,
> {
    let witness = {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(RecursiveAfterRejectV1::TargetFunctionMissing)?;
        cfg.seal_block(function, block)
            .map_err(|error| RecursiveAfterRejectV1::CfgSeal(error.to_string()))?
    };
    identity
        .seal_block(builder, phis, block, &witness)
        .map_err(RecursiveAfterRejectV1::IdentitySeal)?;
    Ok(witness)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::segment_topology::{
        LoopPhysicalSegmentBlockReceiptV1, LoopPhysicalSegmentBlockRowV1,
    };
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::topology::LoopPhysicalBlockRoleV1;
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;

    fn receipt() -> (
        LoopPhysicalSegmentBlockReceiptV1,
        crate::mir::loop_recipe_contract::LoopPhysicalSegmentKeyV1,
        crate::mir::loop_recipe_contract::LoopPhysicalSegmentKeyV1,
    ) {
        let (effect, context, continuation) = callable_operation_demand_parts_for_test();
        let layout = VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("callable demand")
            .prepare_all()
            .expect("callable program")
            .prepare_physical_layout()
            .expect("callable layout");
        let owner = layout.program().demand().context().owner();
        let first = layout.segments()[0].key();
        let missing = layout.segments()[1].key();
        let receipt = LoopPhysicalSegmentBlockReceiptV1::issue_with_boundary(
            owner,
            BasicBlockId::new(0),
            first,
            BasicBlockId::new(2),
            &[first],
            vec![LoopPhysicalSegmentBlockRowV1::new(
                first,
                LoopPhysicalBlockRoleV1::Header,
                BasicBlockId::new(1),
            )],
        )
        .expect("exact receipt");
        (receipt, first, missing)
    }

    #[test]
    fn recursive_after_rejects_transfer_to_missing_segment() {
        let (receipt, _first, missing) = receipt();
        let error = validate_transfer(
            &receipt,
            LoopPhysicalTransferV1::Jump {
                target: LoopPhysicalTargetV1::Segment(missing),
            },
        )
        .expect_err("missing transfer target must reject");
        assert_eq!(error, RecursiveAfterRejectV1::TargetMissing);
    }
}
