//! Length-specific projections for the common-V2 canonical session.
//!
//! These methods borrow the one parent session owner. They issue no second
//! session, source meaning, or physical authority; the split is structural
//! only so the session façade remains below the source-size limit.

use super::{
    CommonV2CanonicalSessionRefV1, ConditionBlockTargetRejectV1,
    LengthCallMaterializationCanaryRejectV1, LengthCallMaterializationCanaryV1,
    LengthCallTargetPlanRejectV1, LengthReceiverPhysicalOperandRefV1,
    LengthReceiverPhysicalOperandRejectV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::loop_recipe_contract::{
    issue_s6c_v2_string_len_call_target_plan_v1, LoopValueClassV2,
    PreparedLoopV2ConditionOperandKindV1, PreparedLoopV2StringLenCallTargetPlanV1,
    S6CLogicalCallRoleV1,
};
use crate::mir::resolved_semantics::ResolvedLoopPlacementV1;
use std::marker::PhantomData;

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Issue the source-backed StringLen target plan once. This remains
    /// Builder-neutral: no callee, ValueId, or MIR instruction is created.
    pub(in crate::mir) fn issue_length_call_target_plan(
        &mut self,
    ) -> Result<PreparedLoopV2StringLenCallTargetPlanV1, LengthCallTargetPlanRejectV1> {
        if self.length_target_plan_issued {
            return Err(LengthCallTargetPlanRejectV1::AlreadyIssued);
        }
        let owner = self.session.owner();
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| LengthCallTargetPlanRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner || self.envelope.owner() != owner {
            return Err(LengthCallTargetPlanRejectV1::OwnerMismatch);
        }
        let plan =
            issue_s6c_v2_string_len_call_target_plan_v1(self.envelope.condition_operands(), owner)
                .map_err(LengthCallTargetPlanRejectV1::SourcePlan)?;
        if plan.owner() != owner
            || plan.block() != self.envelope.condition_operands().condition_block()
        {
            return Err(LengthCallTargetPlanRejectV1::OwnerMismatch);
        }
        self.length_target_plan_issued = true;
        Ok(plan)
    }

    /// Consume the source Length relation once without opening physical call
    /// lowering. The later materializer may consume this canary, but no other
    /// session or legacy CallSlot emitter may use it as an authority.
    pub(in crate::mir) fn issue_length_call_materialization_canary(
        &mut self,
    ) -> Result<LengthCallMaterializationCanaryV1<'_>, LengthCallMaterializationCanaryRejectV1>
    {
        if self.length_call_canary_issued {
            return Err(LengthCallMaterializationCanaryRejectV1::AlreadyIssued);
        }
        let owner = self.session.owner();
        let producer = self.envelope.condition_producer();
        let inventory = self.envelope.condition_operands();
        if self.envelope.owner() != owner || producer.owner() != owner || inventory.owner() != owner
        {
            return Err(LengthCallMaterializationCanaryRejectV1::OwnerMismatch);
        }
        let [left, right] = inventory.rows();
        if left.block() != producer.condition_block()
            || right.block() != producer.condition_block()
            || left.class() != LoopValueClassV2::I64
            || right.class() != LoopValueClassV2::I64
            || right.value() != producer.right()
        {
            return Err(LengthCallMaterializationCanaryRejectV1::ProducerMismatch);
        }
        let source = match right.kind() {
            PreparedLoopV2ConditionOperandKindV1::LengthCall { source } => source,
            PreparedLoopV2ConditionOperandKindV1::ReadBinding { .. } => {
                return Err(LengthCallMaterializationCanaryRejectV1::OperandInventoryMismatch)
            }
        };
        if source.owner() != owner
            || source.role() != S6CLogicalCallRoleV1::Length
            || source.operation() != CoreMethodOp::StringLen
            || source.placement() != ResolvedLoopPlacementV1::Condition
            || source.arity() != 0
            || !source.arguments().is_empty()
        {
            return Err(LengthCallMaterializationCanaryRejectV1::LengthSourceShapeMismatch);
        }
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| LengthCallMaterializationCanaryRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner {
            return Err(LengthCallMaterializationCanaryRejectV1::OwnerMismatch);
        }
        self.length_call_canary_issued = true;
        Ok(LengthCallMaterializationCanaryV1 {
            owner,
            condition_block: right.block(),
            call_item: right.item(),
            result: right.value(),
            stamp,
        })
    }

    /// Lend one source-backed Length receiver operand without opening the
    /// Length Call. The canonical identity/SSA seam remains the sole issuer.
    pub(in crate::mir::builder) fn with_length_receiver_operand<R>(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        receipt: &super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
        callback: impl for<'target> FnOnce(
            &mut crate::mir::builder::MirBuilder,
            LengthReceiverPhysicalOperandRefV1<'target>,
        ) -> Result<R, String>,
    ) -> Result<R, LengthReceiverPhysicalOperandRejectV1> {
        if self.length_receiver_operand_issued {
            return Err(LengthReceiverPhysicalOperandRejectV1::AlreadyIssued);
        }
        let owner = self.session.owner();
        if receipt.owner() != owner || self.envelope.owner() != owner {
            return Err(LengthReceiverPhysicalOperandRejectV1::OwnerMismatch);
        }
        let producer = self.envelope.condition_producer();
        let logical_block = producer.condition_block();
        let Some(layout_segment) = self.envelope.layout().segment_for_block(logical_block) else {
            return Err(LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                ConditionBlockTargetRejectV1::LayoutMismatch,
            ));
        };
        if layout_segment.loop_key() != producer.loop_key() {
            return Err(LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                ConditionBlockTargetRejectV1::LayoutMismatch,
            ));
        }
        let mut rows = receipt
            .rows()
            .iter()
            .filter(|row| row.logical_block() == logical_block);
        let Some(row) = rows.next() else {
            return Err(LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                ConditionBlockTargetRejectV1::MissingConditionRow,
            ));
        };
        if rows.next().is_some()
            || row.loop_key() != layout_segment.loop_key()
            || row.split_ordinal() != layout_segment.split_ordinal()
        {
            return Err(LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                ConditionBlockTargetRejectV1::LayoutMismatch,
            ));
        }
        let stamp_owner = {
            let stamp = self.session.physical_entry_stamp().map_err(|_| {
                LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                    ConditionBlockTargetRejectV1::MissingPhysicalEntryStamp,
                )
            })?;
            stamp.owner()
        };
        if stamp_owner != owner {
            return Err(LengthReceiverPhysicalOperandRejectV1::OwnerMismatch);
        }
        let inventory = self.envelope.condition_operands();
        let [_, right] = inventory.rows();
        let source = match right.kind() {
            PreparedLoopV2ConditionOperandKindV1::LengthCall { source } => source,
            PreparedLoopV2ConditionOperandKindV1::ReadBinding { .. } => {
                return Err(LengthReceiverPhysicalOperandRejectV1::SourceShapeMismatch)
            }
        };
        if source.owner() != owner
            || source.role() != S6CLogicalCallRoleV1::Length
            || source.operation() != CoreMethodOp::StringLen
            || source.placement() != ResolvedLoopPlacementV1::Condition
            || source.arity() != 0
            || !source.arguments().is_empty()
        {
            return Err(LengthReceiverPhysicalOperandRejectV1::SourceShapeMismatch);
        }
        let binding = source
            .receiver_binding()
            .ok_or(LengthReceiverPhysicalOperandRejectV1::MissingReceiverBinding)?;
        let read = self
            .session
            .identity
            .read_entry_receipt(
                builder,
                &mut self.session.phis,
                row.physical_block(),
                binding,
            )
            .map_err(LengthReceiverPhysicalOperandRejectV1::Read)?;
        if read.owner() != owner
            || read.binding() != binding
            || read.physical_block() != row.physical_block()
        {
            return Err(LengthReceiverPhysicalOperandRejectV1::OwnerMismatch);
        }
        let stamp = self.session.physical_entry_stamp().map_err(|_| {
            LengthReceiverPhysicalOperandRejectV1::ConditionTarget(
                ConditionBlockTargetRejectV1::MissingPhysicalEntryStamp,
            )
        })?;
        self.length_receiver_operand_issued = true;
        let view = LengthReceiverPhysicalOperandRefV1 {
            owner,
            binding,
            read,
            row,
            stamp,
            _receipt: PhantomData,
        };
        callback(builder, view).map_err(LengthReceiverPhysicalOperandRejectV1::Callback)
    }
}
