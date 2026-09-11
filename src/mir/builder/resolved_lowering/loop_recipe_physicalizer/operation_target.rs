//! Exact logical-to-physical target receipt for one prepared operation.
//!
//! The receipt is a private proof object, not a CFG owner. The segment
//! dispatcher issues it from the segment receipt; leaves consume it instead
//! of recomputing placement independently.

use super::segment_topology::LoopPhysicalSegmentBlockReceiptV1;
use super::topology::{LoopPhysicalBlockRoleV1, ReadyLoopEntryV1};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopPhysicalSegmentKeyV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::BasicBlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct VerifiedLoopOperationTargetBlockV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    loop_key: LoopNodeKeyV1,
    logical_block: LoopBlockKeyV1,
    role: LoopPhysicalBlockRoleV1,
    preheader: BasicBlockId,
    physical_block: BasicBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationTargetRejectV1 {
    EntryOwnerMismatch,
    ReceiptOwnerMismatch,
    PreheaderMismatch,
    SegmentPlacementMissing(LoopPhysicalSegmentKeyV1),
    TargetFunctionMissing,
    PreheaderMissing(BasicBlockId),
    TargetBlockMissing(BasicBlockId),
    TargetBlockTerminated(BasicBlockId),
}

impl VerifiedLoopOperationTargetBlockV1 {
    pub(super) fn issue_for_segment(
        owner: FunctionOwnerIdV1,
        item: LoopItemKeyV1,
        segment: LoopPhysicalSegmentKeyV1,
        entry: &ReadyLoopEntryV1,
        segment_receipt: &LoopPhysicalSegmentBlockReceiptV1,
    ) -> Result<Self, LoopOperationTargetRejectV1> {
        if entry.owner() != owner {
            return Err(LoopOperationTargetRejectV1::EntryOwnerMismatch);
        }
        if segment_receipt.owner() != owner {
            return Err(LoopOperationTargetRejectV1::ReceiptOwnerMismatch);
        }
        if segment_receipt.preheader() != entry.preheader() {
            return Err(LoopOperationTargetRejectV1::PreheaderMismatch);
        }
        let physical_block = segment_receipt.lookup(segment).ok_or(
            LoopOperationTargetRejectV1::SegmentPlacementMissing(segment),
        )?;
        let role = segment_receipt.role(segment).ok_or(
            LoopOperationTargetRejectV1::SegmentPlacementMissing(segment),
        )?;
        Ok(Self {
            owner,
            item,
            loop_key: segment.loop_key(),
            logical_block: segment.block(),
            role,
            preheader: entry.preheader(),
            physical_block,
        })
    }

    pub(super) fn validate_function(
        self,
        builder: &MirBuilder,
    ) -> Result<(), LoopOperationTargetRejectV1> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(LoopOperationTargetRejectV1::TargetFunctionMissing)?;
        if function.get_block(self.preheader).is_none() {
            return Err(LoopOperationTargetRejectV1::PreheaderMissing(
                self.preheader,
            ));
        }
        let target = function.get_block(self.physical_block).ok_or(
            LoopOperationTargetRejectV1::TargetBlockMissing(self.physical_block),
        )?;
        if target.terminator.is_some() {
            return Err(LoopOperationTargetRejectV1::TargetBlockTerminated(
                self.physical_block,
            ));
        }
        Ok(())
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn loop_key(self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(super) const fn logical_block(self) -> LoopBlockKeyV1 {
        self.logical_block
    }

    pub(super) const fn role(self) -> LoopPhysicalBlockRoleV1 {
        self.role
    }

    pub(super) const fn preheader(self) -> BasicBlockId {
        self.preheader
    }

    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
}
