//! Shared Loop physical services and entry transport for segment lowering.
//!
//! Segment allocation owns physical block placement. Leaf operation emission
//! is kept in `operation_emitter`; this module owns no placement, operation,
//! or value publication authority.

use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::BasicBlockId;

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct ReadyLoopEntryV1 {
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
}

impl ReadyLoopEntryV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn new(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
    ) -> Self {
        Self { owner, preheader }
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
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
    cfg: crate::mir::builder::resolved_lowering::canonical_ssa::SharedFunctionControlV1,
}

impl<'a> LoopPhysicalServicesV1<'a> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'a mut MirBuilder,
        cfg: crate::mir::builder::resolved_lowering::canonical_ssa::SharedFunctionControlV1,
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
            .borrow_mut()
            .cfg
            .create_block(function, block)
            .map_err(|error| LoopPhysicalizerRejectV1::BlockAllocation(error.to_string()))?;
        Ok(block)
    }
}
