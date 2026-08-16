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
    LoopValueClassV2, PreparedLoopV2ConditionOperandKindV1, S6CLogicalCallRoleV1,
};
use crate::mir::resolved_semantics::ResolvedLoopPlacementV1;

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::common_v2_after_block_allocation::{
    allocate_after_block, issue_after_allocation_plan, AfterBlockAllocationRejectV1,
    AfterBlockAllocationStateV1, PreparedAfterBlockViewV1,
};

/// One callback-scoped session plus the exact envelope it consumed.  The
/// envelope is retained as a sibling view so a later physicalizer cannot
/// reacquire a second Port loan.
pub(in crate::mir) struct CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    session: CanonicalSsaFunctionSessionV2<'source>,
    envelope: &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope>,
    after_allocation_state: AfterBlockAllocationStateV1,
    length_call_canary_issued: bool,
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
        };
        Ok(callback(&mut common))
    })
}
