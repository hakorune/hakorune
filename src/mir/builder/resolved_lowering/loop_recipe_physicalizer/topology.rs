//! Shared Loop physical services and entry transport for segment lowering.
//!
//! Segment allocation owns physical block placement. Leaf operation emission
//! is kept in `operation_emitter`; this module owns no placement, operation,
//! or value publication authority.

use super::operation_target::VerifiedLoopOperationTargetBlockV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::{
    CanonicalCfgSessionV1, CanonicalOpenInstructionTargetErrorV1,
    VerifiedCanonicalOpenInstructionTargetV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReadyLoopEntryRowV1 {
    key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    binding: BindingRefV1,
    value: ValueId,
}

impl ReadyLoopEntryRowV1 {
    pub(super) const fn new(
        key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Self {
        Self {
            key,
            binding,
            value,
        }
    }

    pub(super) const fn value(self) -> ValueId {
        self.value
    }

    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct ReadyLoopEntryV1 {
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
    pub(super) rows: Box<[ReadyLoopEntryRowV1]>,
}

impl ReadyLoopEntryV1 {
    pub(super) fn from_rows(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        rows: Vec<ReadyLoopEntryRowV1>,
    ) -> Self {
        Self {
            owner,
            preheader,
            rows: rows.into_boxed_slice(),
        }
    }

    #[cfg(test)]
    pub(super) fn new_for_test(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        rows: Vec<ReadyLoopEntryRowV1>,
    ) -> Self {
        Self::from_rows(owner, preheader, rows)
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
    }

    pub(super) fn contains_binding(&self, binding: BindingRefV1) -> bool {
        self.rows.iter().any(|row| row.binding == binding)
    }
}

/// Mechanical bridge from canonical identity receipts to segment allocation.
/// The rows must already have been read from the live canonical preheader;
/// this constructor performs no source lookup.
pub(in crate::mir::builder::resolved_lowering) fn ready_loop_entry_from_canonical_rows(
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
    rows: Vec<(
        crate::mir::loop_recipe_contract::LoopValueKeyV1,
        BindingRefV1,
        ValueId,
    )>,
) -> ReadyLoopEntryV1 {
    ReadyLoopEntryV1 {
        owner,
        preheader,
        rows: rows
            .into_iter()
            .map(|(key, binding, value)| ReadyLoopEntryRowV1::new(key, binding, value))
            .collect(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder::resolved_lowering) enum LoopPhysicalBlockRoleV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum LoopPhysicalizerRejectV1 {
    MissingFunction,
    BlockAllocation(String),
}

/// Borrowed canonical services. This is a service bundle, not a second
/// physical/SSA owner; callers pass the existing session's CFG service.
pub(in crate::mir::builder::resolved_lowering) struct LoopPhysicalServicesV1<'a> {
    pub(super) builder: &'a mut MirBuilder,
    cfg: &'a mut CanonicalCfgSessionV1,
}

impl<'a> LoopPhysicalServicesV1<'a> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'a mut MirBuilder,
        cfg: &'a mut CanonicalCfgSessionV1,
    ) -> Self {
        Self { builder, cfg }
    }

    pub(in crate::mir::builder::resolved_lowering) fn allocate_block(
        &mut self,
    ) -> Result<BasicBlockId, LoopPhysicalizerRejectV1> {
        let block = self.builder.next_block_id();
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizerRejectV1::MissingFunction)?;
        self.cfg
            .create_block(function, block)
            .map_err(|error| LoopPhysicalizerRejectV1::BlockAllocation(error.to_string()))?;
        Ok(block)
    }

    /// Co-seal the existing Loop target receipt with the CFG session that
    /// created its physical block. This is a narrow open-target proof; it
    /// does not issue operands, dominance, or instruction meaning.
    pub(in crate::mir::builder::resolved_lowering) fn prepare_open_instruction_target(
        &self,
        target: &VerifiedLoopOperationTargetBlockV1,
    ) -> Result<VerifiedCanonicalOpenInstructionTargetV1, CanonicalOpenInstructionTargetErrorV1>
    {
        let function = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(CanonicalOpenInstructionTargetErrorV1::FunctionMissing)?;
        self.cfg.prepare_created_open_instruction_target(
            function,
            target.owner(),
            target.physical_block(),
        )
    }
}
