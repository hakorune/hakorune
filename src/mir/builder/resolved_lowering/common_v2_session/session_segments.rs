//! Segment and physical-target projections for the common-V2 session.
//!
//! The parent session remains the sole owner of the canonical SSA session and
//! its one-shot state. These impls only move structural allocation/projection
//! helpers behind a private child module.

use super::{
    CommonV2CanonicalSessionRefV1, ConditionBlockPhysicalTargetRefV1, ConditionBlockTargetRejectV1,
    SharedSegmentScopeRejectV1,
};
use crate::mir::builder::resolved_lowering::common_v2_after_block_allocation::{
    allocate_after_block, issue_after_allocation_plan, AfterBlockAllocationRejectV1,
    PreparedAfterBlockViewV1,
};
use crate::mir::builder::resolved_lowering::common_v2_if_continuation_target::{
    issue_if_continuation_target, IfContinuationPhysicalTargetRefV1,
    IfContinuationPhysicalTargetRejectV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::issue_v2_segment_allocation_plan;

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    pub(in crate::mir::builder) fn allocate_v2_segment_blocks(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<
        super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
        super::super::common_v2_segment_block_allocation::SegmentBlockAllocationRejectV1,
    > {
        let plan = issue_v2_segment_allocation_plan(self.envelope).map_err(
            super::super::common_v2_segment_block_allocation::SegmentBlockAllocationRejectV1::Plan,
        )?;
        super::super::common_v2_segment_block_allocation::allocate_v2_segment_blocks(
            &mut self.session,
            builder,
            plan,
        )
    }

    /// Open one private, one-shot segment scope for this canonical session.
    pub(in crate::mir::builder) fn with_shared_segment_scope<R>(
        &mut self,
        builder: &mut MirBuilder,
        callback: impl FnOnce(
            &mut Self,
            &mut MirBuilder,
            super::super::common_v2_segment_block_allocation::CommonV2SharedSegmentScopeV1,
        ) -> Result<R, String>,
    ) -> Result<R, SharedSegmentScopeRejectV1> {
        let receipt = self
            .allocate_v2_segment_blocks(builder)
            .map_err(|error| SharedSegmentScopeRejectV1::Allocation(format!("{error:?}")))?;
        let scope =
            super::super::common_v2_segment_block_allocation::CommonV2SharedSegmentScopeV1::new(
                receipt,
            );
        callback(self, builder, scope).map_err(SharedSegmentScopeRejectV1::Callback)
    }

    /// Reserve one unpublished physical target for the exact source
    /// fallthrough item. No edge or instruction API escapes this callback.
    pub(in crate::mir::builder) fn with_if_continuation_target<R>(
        &mut self,
        builder: &mut MirBuilder,
        segment_receipt: &super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
        callback: impl for<'target> FnOnce(
            &mut MirBuilder,
            IfContinuationPhysicalTargetRefV1<'target>,
        ) -> Result<R, String>,
    ) -> Result<R, IfContinuationPhysicalTargetRejectV1> {
        if self.if_continuation_target_issued {
            return Err(IfContinuationPhysicalTargetRejectV1::AlreadyIssued);
        }
        self.if_continuation_target_issued = true;
        issue_if_continuation_target(
            &mut self.session,
            self.envelope,
            segment_receipt,
            builder,
            callback,
        )
    }

    /// Lend exactly one source condition-block target from the existing
    /// segment receipt. This is projection-only and cannot issue a ValueId.
    pub(in crate::mir::builder) fn with_condition_block_target<R>(
        &mut self,
        builder: &mut MirBuilder,
        callback: impl for<'target> FnOnce(
            &mut MirBuilder,
            ConditionBlockPhysicalTargetRefV1<'target>,
        ) -> Result<R, String>,
    ) -> Result<R, ConditionBlockTargetRejectV1> {
        let receipt = self
            .allocate_v2_segment_blocks(builder)
            .map_err(|error| ConditionBlockTargetRejectV1::Allocation(format!("{error:?}")))?;
        let target = self.condition_block_target_from_receipt(&receipt)?;
        callback(builder, target).map_err(ConditionBlockTargetRejectV1::Callback)
    }

    pub(in crate::mir::builder::resolved_lowering::common_v2_session) fn condition_block_target_from_receipt<
        'target,
    >(
        &'target self,
        receipt: &'target super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
    ) -> Result<ConditionBlockPhysicalTargetRefV1<'target>, ConditionBlockTargetRejectV1> {
        let owner = self.session.owner();
        if receipt.owner() != owner || self.envelope.owner() != owner {
            return Err(ConditionBlockTargetRejectV1::OwnerMismatch);
        }
        let producer = self.envelope.condition_producer();
        let logical_block = producer.condition_block();
        let Some(layout_segment) = self.envelope.layout().segment_for_block(logical_block) else {
            return Err(ConditionBlockTargetRejectV1::LayoutMismatch);
        };
        if layout_segment.loop_key() != producer.loop_key() {
            return Err(ConditionBlockTargetRejectV1::LayoutMismatch);
        }
        let mut rows = receipt
            .rows()
            .iter()
            .filter(|row| row.logical_block() == logical_block);
        let Some(row) = rows.next() else {
            return Err(ConditionBlockTargetRejectV1::MissingConditionRow);
        };
        if rows.next().is_some()
            || row.loop_key() != layout_segment.loop_key()
            || row.split_ordinal() != layout_segment.split_ordinal()
        {
            return Err(ConditionBlockTargetRejectV1::LayoutMismatch);
        }
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| ConditionBlockTargetRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner {
            return Err(ConditionBlockTargetRejectV1::OwnerMismatch);
        }
        Ok(ConditionBlockPhysicalTargetRefV1 {
            owner,
            logical_block,
            physical_block: row.physical_block(),
            stamp,
            _row: row,
        })
    }

    pub(in crate::mir::builder) fn allocate_v2_after_block<'session>(
        &'session mut self,
        builder: &mut MirBuilder,
        segment_receipt: &super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
    ) -> Result<PreparedAfterBlockViewV1<'session>, AfterBlockAllocationRejectV1> {
        let plan = issue_after_allocation_plan(
            &mut self.after_allocation_state,
            &self.session,
            self.envelope,
            segment_receipt,
        )?;
        allocate_after_block(
            &mut self.after_allocation_state,
            &mut self.session,
            builder,
            plan,
        )
    }
}
