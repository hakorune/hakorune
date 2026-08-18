//! Caller-zero opener for one common V2 canonical session.
//!
//! This is intentionally a thin transport wrapper.  The admission owns the
//! source/cohort checks; the canonical session owns the mutable CFG/SSA/PHI
//! state.  No operation or control placement is emitted here.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableParameterDescriptorV1;
use crate::mir::compiler::common_v2_physical_function_skeleton::PhysicalFunctionEntryCohortStampV1;
use crate::mir::compiler::common_v2_session_admission::LoopV2CanonicalSessionAdmissionRefV1;
use crate::mir::loop_recipe_contract::{
    issue_s6c_v2_substring_call_target_plan_v1, PreparedLoopV2PreSessionEnvelopeV1,
    StringLenCallTargetPlanRejectV1, VerifiedS6CReturnSourceRecipeBindingV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use std::marker::PhantomData;

use super::canonical_ssa::{CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2};
use super::common_v2_after_block_allocation::AfterBlockAllocationStateV1;
use super::common_v2_s6c_substring_callout_admission::{
    issue_common_v2_s6c_substring_callout_admission_v1, CommonV2SubstringCallOutAdmissionRejectV1,
    PreparedCommonV2SubstringCallOutAdmissionV1,
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

#[path = "common_v2_return_read.rs"]
mod return_read;
pub(in crate::mir::builder) use return_read::{
    CommonV2ReturnReadPhysicalReceiptV1, ReturnReadPhysicalReceiptRejectV1,
};

#[path = "common_v2_condition_bool.rs"]
mod condition_bool;
pub(in crate::mir::builder) use condition_bool::{
    CanonicalConditionBoolResultReceiptV1, ConditionBoolMaterializationRejectV1,
    ConditionBoolReturnReadRejectV1,
};

#[path = "common_v2_s6c_operand_issuer.rs"]
mod s6c_operand_issuer;
pub(in crate::mir::builder) use s6c_operand_issuer::S6CTextEqOperandIssuerRejectV1;

#[path = "common_v2_s6c_text_eq_occurrence.rs"]
mod s6c_text_eq_occurrence;
pub(in crate::mir::builder) use s6c_text_eq_occurrence::S6CTextEqOccurrencePhysicalViewV1;
pub(in crate::mir::builder) use s6c_text_eq_occurrence::S6CTextEqOccurrenceViewRejectV1;

#[path = "common_v2_s6c_substring_v9_issuer.rs"]
mod s6c_substring_v9_issuer;
pub(in crate::mir::builder) use s6c_substring_v9_issuer::{
    CommonV2SubstringV9IssuerRejectV1, CommonV2SubstringV9MaterializationV1,
};

#[path = "common_v2_s6c_substring_callout_materializer.rs"]
mod s6c_substring_callout_materializer;
pub(in crate::mir::builder) use s6c_substring_callout_materializer::{
    CommonV2SubstringCallOutExactTextCoSealRefV1, CommonV2SubstringCallOutMirMaterializerRejectV1,
};

#[path = "common_v2_session_length.rs"]
mod session_length;

#[path = "common_v2_session_segments.rs"]
mod session_segments;

#[path = "common_v2_s6c_scalar_equality_leaf.rs"]
mod s6c_scalar_equality_leaf;
pub(in crate::mir::builder) use s6c_scalar_equality_leaf::{
    issue_common_v2_s6c_text_scalar_equality_leaf_v1,
    CommonV2S6CTextScalarEqualityLeafCapabilityV1, CommonV2S6CTextScalarEqualityLeafReceiptV1,
    CommonV2S6CTextScalarEqualityLeafRejectV1, CommonV2S6CTextScalarEqualityLeafShapeV1,
};

#[path = "common_v2_s6c_cursor_cfg.rs"]
mod s6c_cursor_cfg;
pub(in crate::mir::builder::resolved_lowering) use s6c_cursor_cfg::{
    CommonV2S6CCursorCfgReceiptV1, CommonV2S6CCursorCfgRejectV1,
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
    invocation_brand: ModuleInvocationBrandV1,
    after_allocation_state: AfterBlockAllocationStateV1,
    length_call_canary_issued: bool,
    length_target_plan_issued: bool,
    length_receiver_operand_issued: bool,
    length_call_direct_issued: bool,
    initial_index_seed_issued: bool,
    condition_bool_issued: bool,
    if_continuation_target_issued: bool,
    return_read_physical_issued: bool,
    s6c_text_eq_operands_issued: bool,
    s6c_text_eq_occurrence_issued: bool,
    s6c_substring_callout_admission_issued: bool,
    s6c_substring_callout_mir_issued: bool,
    s6c_scalar_equality_leaf_issued: bool,
    s6c_cursor_cfg_issued: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum SharedSegmentScopeRejectV1 {
    Allocation(String),
    Callback(String),
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

    pub(in crate::mir) fn return_source_binding(
        &self,
    ) -> &'envelope VerifiedS6CReturnSourceRecipeBindingV1 {
        self.envelope.return_source_binding()
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

    pub(in crate::mir::builder) const fn invocation_brand(&self) -> ModuleInvocationBrandV1 {
        self.invocation_brand
    }

    /// Admit the one common-V2 Substring provider/site plan without opening
    /// a callout effect. The session reserves this consumer before target or
    /// metadata work, so a failed attempt cannot be retried on the same draft.
    pub(in crate::mir::builder) fn with_s6c_substring_callout_admission<R>(
        &mut self,
        physical_effects: &crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1,
        callback: impl FnOnce(
            &mut Self,
            PreparedCommonV2SubstringCallOutAdmissionV1,
        ) -> Result<R, String>,
    ) -> Result<R, CommonV2SubstringCallOutAdmissionRejectV1> {
        if self.s6c_substring_callout_admission_issued {
            return Err(CommonV2SubstringCallOutAdmissionRejectV1::AlreadyIssued);
        }
        self.s6c_substring_callout_admission_issued = true;
        let target =
            issue_s6c_v2_substring_call_target_plan_v1(self.envelope, self.session.owner())
                .map_err(CommonV2SubstringCallOutAdmissionRejectV1::Target)?;
        let admission = issue_common_v2_s6c_substring_callout_admission_v1(
            target,
            physical_effects,
            self.invocation_brand,
        )?;
        callback(self, admission).map_err(CommonV2SubstringCallOutAdmissionRejectV1::Callback)
    }

    #[cfg(test)]
    pub(in crate::mir) fn physical_entry_sidecar_row_count(&self) -> usize {
        self.session.physical_entry_sidecar_row_count()
    }
}

/// Consume one common admission and open one canonical session owner for the
/// duration of the nested callback.  The caller-zero canary deliberately
/// exposes no lowerer, DraftSeal, or physical placement API yet.
pub(in crate::mir::builder) fn with_common_v2_canonical_session_branded<R>(
    admission: LoopV2CanonicalSessionAdmissionRefV1<'_, '_, '_>,
    invocation_brand: ModuleInvocationBrandV1,
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
            invocation_brand,
            after_allocation_state: AfterBlockAllocationStateV1::Available,
            length_call_canary_issued: false,
            length_target_plan_issued: false,
            length_receiver_operand_issued: false,
            length_call_direct_issued: false,
            initial_index_seed_issued: false,
            condition_bool_issued: false,
            if_continuation_target_issued: false,
            return_read_physical_issued: false,
            s6c_text_eq_operands_issued: false,
            s6c_text_eq_occurrence_issued: false,
            s6c_substring_callout_admission_issued: false,
            s6c_substring_callout_mir_issued: false,
            s6c_scalar_equality_leaf_issued: false,
            s6c_cursor_cfg_issued: false,
        };
        Ok(callback(&mut common))
    })
}

#[cfg(not(test))]
pub(in crate::mir) fn with_common_v2_canonical_session<R>(
    admission: LoopV2CanonicalSessionAdmissionRefV1<'_, '_, '_>,
    invocation_brand: ModuleInvocationBrandV1,
    callback: impl for<'source, 'envelope> FnOnce(
        &mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    ) -> R,
) -> Result<R, String> {
    with_common_v2_canonical_session_branded(admission, invocation_brand, callback)
}

#[cfg(test)]
pub(in crate::mir) fn with_common_v2_canonical_session<R>(
    admission: LoopV2CanonicalSessionAdmissionRefV1<'_, '_, '_>,
    callback: impl for<'source, 'envelope> FnOnce(
        &mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    ) -> R,
) -> Result<R, String> {
    with_common_v2_canonical_session_branded(
        admission,
        ModuleInvocationBrandV1::legacy_test(),
        callback,
    )
}
