//! Caller-zero physical placement for one verified If fallthrough target.
//!
//! This module owns only the source/segment parity checks and one unpublished
//! block reservation.  It does not emit an edge, terminator, operation,
//! Return, or PHI; the enclosing physical-entry transaction remains the sole
//! rollback owner.

use crate::mir::compiler::common_v2_physical_function_skeleton::PhysicalFunctionEntryCohortStampV1;
use crate::mir::loop_recipe_contract::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1,
    LoopJoinNextItemV1, PreparedLoopControlPlacementV2, PreparedLoopV2PreSessionEnvelopeV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirBuilder};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::common_v2_segment_block_allocation::{
    PreparedSegmentBlockReceiptV1, SegmentBlockReceiptRowV1,
};

/// A callback-scoped mechanical target for the explicit fallthrough item.
///
/// The target is intentionally not a semantic receipt.  Its physical block
/// is unpublished and tied to the source segment row and entry stamp by
/// borrows that cannot outlive the callback.
#[derive(Debug)]
pub(in crate::mir::builder) struct IfContinuationPhysicalTargetRefV1<'target> {
    owner: FunctionOwnerIdV1,
    if_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    continuation: LoopJoinNextItemV1,
    source_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    source_split_ordinal: u32,
    physical_block: BasicBlockId,
    stamp: &'target PhysicalFunctionEntryCohortStampV1,
    _row: &'target SegmentBlockReceiptRowV1,
}

impl IfContinuationPhysicalTargetRefV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn if_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.if_item
    }

    pub(in crate::mir::builder) const fn continuation(&self) -> LoopJoinNextItemV1 {
        self.continuation
    }

    pub(in crate::mir::builder) const fn source_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.source_block
    }

    pub(in crate::mir::builder) const fn source_split_ordinal(&self) -> u32 {
        self.source_split_ordinal
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn stamp_owner(&self) -> FunctionOwnerIdV1 {
        self.stamp.owner()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum IfContinuationPhysicalTargetRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    MissingPhysicalEntryStamp,
    BranchMissing,
    BranchDuplicate,
    IfPlacementMissing,
    IfPlacementDuplicate,
    BranchRelation,
    SegmentMissing,
    SegmentRowMissing,
    SegmentRowDuplicate,
    SegmentRowMismatch,
    IfItemMissing,
    ContinuationItemMissing,
    ContinuationNotStrict,
    ContinuationIsControl,
    Allocation(String),
    Callback(String),
}

pub(in crate::mir::builder) fn issue_if_continuation_target<R>(
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    envelope: &PreparedLoopV2PreSessionEnvelopeV1<'_, '_>,
    segment_receipt: &PreparedSegmentBlockReceiptV1,
    builder: &mut MirBuilder,
    callback: impl for<'target> FnOnce(
        &mut MirBuilder,
        IfContinuationPhysicalTargetRefV1<'target>,
    ) -> Result<R, String>,
) -> Result<R, IfContinuationPhysicalTargetRejectV1> {
    let owner = session.owner();
    if envelope.owner() != owner || segment_receipt.owner() != owner {
        return Err(IfContinuationPhysicalTargetRejectV1::OwnerMismatch);
    }
    let stamp_owner = session
        .physical_entry_stamp()
        .map_err(|_| IfContinuationPhysicalTargetRejectV1::MissingPhysicalEntryStamp)?
        .owner();
    if stamp_owner != owner {
        return Err(IfContinuationPhysicalTargetRejectV1::OwnerMismatch);
    }

    let branches = envelope.control().transfer().branches();
    let [branch] = branches else {
        return Err(if branches.is_empty() {
            IfContinuationPhysicalTargetRejectV1::BranchMissing
        } else {
            IfContinuationPhysicalTargetRejectV1::BranchDuplicate
        });
    };
    let mut if_rows = envelope
        .control()
        .rows()
        .iter()
        .filter_map(|row| match row {
            PreparedLoopControlPlacementV2::If {
                item,
                block,
                condition,
                then_block,
                else_block,
            } if *item == branch.if_item => {
                Some((*item, *block, *condition, *then_block, *else_block))
            }
            _ => None,
        });
    let Some((if_item, if_block, condition, then_block, else_block)) = if_rows.next() else {
        return Err(IfContinuationPhysicalTargetRejectV1::IfPlacementMissing);
    };
    if if_rows.next().is_some() {
        return Err(IfContinuationPhysicalTargetRejectV1::IfPlacementDuplicate);
    }

    let producer = envelope.condition_producer();
    let continuation = match (branch.then_arm, branch.else_arm) {
        (
            LoopJoinBranchArmTransferRefV2::Exit(exit),
            LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. },
        ) if exit.role == LoopJoinEdgeRoleV1::Return
            && exit.target == LoopJoinBranchExitTargetV2::FunctionExit =>
        {
            continuation
        }
        _ => return Err(IfContinuationPhysicalTargetRejectV1::BranchRelation),
    };
    if branch.owner_loop != producer.loop_key()
        || branch.if_item != if_item
        || branch.condition != condition
        || then_block == if_block
        || else_block.is_some()
        || !envelope.layout().has_block(then_block)
        || continuation.block != if_block
    {
        return Err(IfContinuationPhysicalTargetRejectV1::BranchRelation);
    }

    let segment = envelope
        .layout()
        .segment_for_block(if_block)
        .ok_or(IfContinuationPhysicalTargetRejectV1::SegmentMissing)?;
    if segment.loop_key() != branch.owner_loop {
        return Err(IfContinuationPhysicalTargetRejectV1::SegmentRowMismatch);
    }
    let Some(if_position) = segment.items().iter().position(|item| *item == if_item) else {
        return Err(IfContinuationPhysicalTargetRejectV1::IfItemMissing);
    };
    let Some(continuation_position) = segment
        .items()
        .iter()
        .position(|item| *item == continuation.item)
    else {
        return Err(IfContinuationPhysicalTargetRejectV1::ContinuationItemMissing);
    };
    if continuation_position <= if_position {
        return Err(IfContinuationPhysicalTargetRejectV1::ContinuationNotStrict);
    }
    if envelope
        .control()
        .rows()
        .iter()
        .any(|row| row.item() == continuation.item)
    {
        return Err(IfContinuationPhysicalTargetRejectV1::ContinuationIsControl);
    }

    let mut rows = segment_receipt
        .rows()
        .iter()
        .filter(|row| row.logical_block() == if_block);
    let row = rows
        .next()
        .ok_or(IfContinuationPhysicalTargetRejectV1::SegmentRowMissing)?;
    if rows.next().is_some() {
        return Err(IfContinuationPhysicalTargetRejectV1::SegmentRowDuplicate);
    }
    if row.loop_key() != segment.loop_key() || row.split_ordinal() != segment.split_ordinal() {
        return Err(IfContinuationPhysicalTargetRejectV1::SegmentRowMismatch);
    }

    let physical_block = session
        .create_unpublished_block(builder)
        .map_err(IfContinuationPhysicalTargetRejectV1::Allocation)?;
    let stamp = session
        .physical_entry_stamp()
        .map_err(|_| IfContinuationPhysicalTargetRejectV1::MissingPhysicalEntryStamp)?;
    let target = IfContinuationPhysicalTargetRefV1 {
        owner,
        if_item,
        continuation,
        source_block: if_block,
        source_split_ordinal: segment.split_ordinal(),
        physical_block,
        stamp,
        _row: row,
    };
    callback(builder, target).map_err(IfContinuationPhysicalTargetRejectV1::Callback)
}
