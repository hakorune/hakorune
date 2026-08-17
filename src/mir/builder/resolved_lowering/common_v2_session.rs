//! Caller-zero opener for one common V2 canonical session.
//!
//! This is intentionally a thin transport wrapper.  The admission owns the
//! source/cohort checks; the canonical session owns the mutable CFG/SSA/PHI
//! state.  No operation or control placement is emitted here.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableParameterDescriptorV1;
use crate::mir::compiler::common_v2_physical_function_skeleton::PhysicalFunctionEntryCohortStampV1;
use crate::mir::compiler::common_v2_session_admission::LoopV2CanonicalSessionAdmissionRefV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::loop_recipe_contract::issue_v2_segment_allocation_plan;
use crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1;
use crate::mir::loop_recipe_contract::{
    issue_s6c_v2_string_len_call_target_plan_v1, LoopValueClassV2,
    PreparedLoopV2ConditionOperandKindV1, PreparedLoopV2StringLenCallTargetPlanV1,
    S6CLogicalCallRoleV1, StringLenCallTargetPlanRejectV1,
};
use crate::mir::resolved_semantics::ResolvedLoopPlacementV1;
use std::marker::PhantomData;

use super::canonical_ssa::{CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2};
use super::common_v2_after_block_allocation::{
    allocate_after_block, issue_after_allocation_plan, AfterBlockAllocationRejectV1,
    AfterBlockAllocationStateV1, PreparedAfterBlockViewV1,
};

#[path = "common_v2_length_call.rs"]
mod length_call;
pub(in crate::mir::builder) use length_call::{
    CanonicalLengthCallResultReceiptV1, LengthCallDirectEmitterRejectV1,
};

#[path = "common_v2_initial_index_seed.rs"]
mod initial_index_seed;
pub(in crate::mir::builder) use initial_index_seed::{
    CanonicalInitialIndexSeedReceiptV1, InitialIndexSeedMaterializationRejectV1,
};

/// A callback-scoped mechanical view of the physical block corresponding to
/// the source condition block.  The row and entry stamp are borrowed from the
/// same unpublished session, so this view cannot be re-paired with another
/// segment receipt or retained after the callback.
#[derive(Debug)]
pub(in crate::mir::builder) struct ConditionBlockPhysicalTargetRefV1<'target> {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    logical_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    physical_block: crate::mir::BasicBlockId,
    stamp: &'target PhysicalFunctionEntryCohortStampV1,
    _row: &'target super::common_v2_segment_block_allocation::SegmentBlockReceiptRowV1,
}

impl ConditionBlockPhysicalTargetRefV1<'_> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn logical_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.logical_block
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn stamp_owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.stamp.owner()
    }
}

/// A callback-scoped physical receiver view for the source Length call.  The
/// source binding and canonical read receipt are kept together with the
/// condition target so a raw receiver value cannot be re-paired with another
/// session or block after the callback returns.
#[derive(Debug)]
pub(in crate::mir::builder) struct LengthReceiverPhysicalOperandRefV1<'target> {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    read: CanonicalBindingReadReceiptV1,
    row: &'target super::common_v2_segment_block_allocation::SegmentBlockReceiptRowV1,
    stamp: &'target PhysicalFunctionEntryCohortStampV1,
    _receipt: PhantomData<
        &'target super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
    >,
}

impl LengthReceiverPhysicalOperandRefV1<'_> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn binding(
        &self,
    ) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.row.physical_block()
    }

    pub(in crate::mir::builder) const fn physical_value(&self) -> crate::mir::ValueId {
        self.read.physical_value()
    }

    pub(in crate::mir::builder) const fn stamp_owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.stamp.owner()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionBlockTargetRejectV1 {
    Allocation(String),
    MissingPhysicalEntryStamp,
    OwnerMismatch,
    LayoutMismatch,
    MissingConditionRow,
    DuplicateConditionRow,
    Callback(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LengthReceiverPhysicalOperandRejectV1 {
    AlreadyIssued,
    ConditionTarget(ConditionBlockTargetRejectV1),
    OwnerMismatch,
    MissingReceiverBinding,
    SourceShapeMismatch,
    Read(String),
    Callback(String),
}

/// One callback-scoped session plus the exact envelope it consumed.  The
/// envelope is retained as a sibling view so a later physicalizer cannot
/// reacquire a second Port loan.
pub(in crate::mir) struct CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    session: CanonicalSsaFunctionSessionV2<'source>,
    envelope: &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope>,
    after_allocation_state: AfterBlockAllocationStateV1,
    length_call_canary_issued: bool,
    length_target_plan_issued: bool,
    length_receiver_operand_issued: bool,
    length_call_direct_issued: bool,
    initial_index_seed_issued: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum LengthCallMaterializationCanaryRejectV1 {
    AlreadyIssued,
    MissingPhysicalEntryStamp,
    OwnerMismatch,
    ProducerMismatch,
    OperandInventoryMismatch,
    LengthSourceShapeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum LengthCallTargetPlanRejectV1 {
    AlreadyIssued,
    MissingPhysicalEntryStamp,
    OwnerMismatch,
    SourcePlan(StringLenCallTargetPlanRejectV1),
}

/// Builder-neutral, one-shot evidence that the source Length result reached
/// the same canonical session. It deliberately carries no ValueId or type.
#[derive(Debug)]
pub(in crate::mir) struct LengthCallMaterializationCanaryV1<'session> {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    condition_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    call_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    stamp: &'session PhysicalFunctionEntryCohortStampV1,
}

impl LengthCallMaterializationCanaryV1<'_> {
    pub(in crate::mir) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn condition_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.condition_block
    }

    pub(in crate::mir) const fn call_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.call_item
    }

    pub(in crate::mir) const fn result(&self) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.result
    }

    pub(in crate::mir) const fn stamp_owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.stamp.owner()
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    pub(in crate::mir) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.session.owner()
    }

    pub(in crate::mir) const fn completion_is_implicit(&self) -> bool {
        self.session.completion_is_implicit()
    }

    pub(in crate::mir) const fn envelope(
        &self,
    ) -> &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope> {
        self.envelope
    }

    pub(in crate::mir) fn adopt_physical_entry_lanes(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        descriptors: &[PhysicalCallableParameterDescriptorV1],
    ) -> Result<(), String> {
        self.session
            .adopt_physical_entry_lanes(builder, descriptors)
    }

    pub(in crate::mir) fn attach_physical_entry_stamp(
        &mut self,
        stamp: PhysicalFunctionEntryCohortStampV1,
    ) -> Result<(), String> {
        self.session.attach_physical_entry_stamp(stamp)
    }

    pub(in crate::mir) fn physical_entry_stamp(
        &self,
    ) -> Result<&PhysicalFunctionEntryCohortStampV1, String> {
        self.session.physical_entry_stamp()
    }

    /// Issue the source-backed StringLen target plan once.  This is still
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
        let [left, right] = inventory.rows() else {
            return Err(LengthCallMaterializationCanaryRejectV1::OperandInventoryMismatch);
        };
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

    pub(in crate::mir::builder) fn allocate_v2_segment_blocks(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
    ) -> Result<
        super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
        super::common_v2_segment_block_allocation::SegmentBlockAllocationRejectV1,
    > {
        let plan = issue_v2_segment_allocation_plan(self.envelope).map_err(
            super::common_v2_segment_block_allocation::SegmentBlockAllocationRejectV1::Plan,
        )?;
        super::common_v2_segment_block_allocation::allocate_v2_segment_blocks(
            &mut self.session,
            builder,
            plan,
        )
    }

    /// Allocate the existing source segments and lend exactly one physical
    /// condition-block target.  This is deliberately a projection-only seam:
    /// it does not issue a ValueId, Call, edge, or terminator, and the target
    /// cannot escape the callback that owns the segment receipt.
    pub(in crate::mir::builder) fn with_condition_block_target<R>(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        callback: impl for<'target> FnOnce(
            &mut crate::mir::builder::MirBuilder,
            ConditionBlockPhysicalTargetRefV1<'target>,
        ) -> Result<R, String>,
    ) -> Result<R, ConditionBlockTargetRejectV1> {
        let receipt = self
            .allocate_v2_segment_blocks(builder)
            .map_err(|error| ConditionBlockTargetRejectV1::Allocation(format!("{error:?}")))?;
        let target = self.condition_block_target_from_receipt(&receipt)?;
        callback(builder, target).map_err(ConditionBlockTargetRejectV1::Callback)
    }

    fn condition_block_target_from_receipt<'target>(
        &'target self,
        receipt: &'target super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
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
        if rows.next().is_some() {
            return Err(ConditionBlockTargetRejectV1::DuplicateConditionRow);
        }
        if row.loop_key() != layout_segment.loop_key()
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
        let target = ConditionBlockPhysicalTargetRefV1 {
            owner,
            logical_block,
            physical_block: row.physical_block(),
            stamp,
            _row: row,
        };
        Ok(target)
    }

    /// Lend one source-backed Length receiver operand without opening the
    /// Length Call. The source resolver relation is projected mechanically;
    /// the canonical identity/SSA seam remains the sole physical read issuer.
    pub(in crate::mir::builder) fn with_length_receiver_operand<R>(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        receipt: &super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
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
        let [_, right] = inventory.rows() else {
            return Err(LengthReceiverPhysicalOperandRejectV1::SourceShapeMismatch);
        };
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

    pub(in crate::mir::builder) fn allocate_v2_after_block<'session>(
        &'session mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        segment_receipt: &super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
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

    #[cfg(test)]
    pub(in crate::mir) fn physical_entry_sidecar_row_count(&self) -> usize {
        self.session.physical_entry_sidecar_row_count()
    }
}

/// Consume one common admission and open one canonical session owner for the
/// duration of the nested callback.  The caller-zero canary deliberately
/// exposes no lowerer, DraftSeal, or physical placement API yet.
pub(in crate::mir) fn with_common_v2_canonical_session<R>(
    admission: LoopV2CanonicalSessionAdmissionRefV1<'_, '_, '_>,
    callback: impl for<'source, 'envelope> FnOnce(
        &mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    ) -> R,
) -> Result<R, String> {
    admission.consume_for_canonical_session(|parts| {
        let envelope = parts.envelope();
        let session = CanonicalSsaFunctionSessionV2::new_common_v2(parts)?;
        let mut common = CommonV2CanonicalSessionRefV1 {
            session,
            envelope,
            after_allocation_state: AfterBlockAllocationStateV1::Available,
            length_call_canary_issued: false,
            length_target_plan_issued: false,
            length_receiver_operand_issued: false,
            length_call_direct_issued: false,
            initial_index_seed_issued: false,
        };
        Ok(callback(&mut common))
    })
}
