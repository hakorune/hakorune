//! Source-backed target realization for the common-V2 StringLen CallSlot.
//!
//! This product is still physical-ID-free.  It fixes the target facts needed
//! by the later canonical session materializer without constructing a MIR
//! `Call`, selecting by spelling, or borrowing a legacy CallSlot emitter.

use super::common_v2_condition_operand_inventory::{
    PreparedLoopV2ConditionOperandInventoryV1, PreparedLoopV2ConditionOperandKindV1,
};
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_scan_with_init_joinir::S6CLogicalCallRoleV1;
use super::schema_v2::LoopValueClassV2;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{CoreMethodEffectV1, CORE_METHOD_MANIFEST_BRAND_V1};
use crate::mir::resolved_semantics::{
    CoreMethodHomeExecutionPolicyV1, CoreMethodHomeReceiverRelationV1,
    CoreMethodHomeResultRelationV1, CoreMethodHomeSchemaV1, FunctionOwnerIdV1,
    ResolvedLoopPlacementV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringLenCallTargetPlanRejectV1 {
    ForeignOwner,
    MissingLengthOperand,
    WrongOperandKind,
    SourceIdentityMismatch,
    SourceShapeMismatch,
    TargetBrandMismatch,
    TargetShapeMismatch,
}

/// Non-Clone source-backed target plan consumed by the future physical Call
/// materializer.  It deliberately contains no `ValueId`, `Callee`, or MIR.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2StringLenCallTargetPlanV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    result: LoopValueKeyV1,
    target_brand: crate::mir::resolved_semantics::CoreMethodTargetBrandV1,
    manifest_brand: crate::mir::core_method_result_kind::CoreMethodManifestBrandV1,
    receiver: CoreMethodHomeReceiverRelationV1,
    result_relation: CoreMethodHomeResultRelationV1,
    effect: CoreMethodEffectV1,
    execution_policy: CoreMethodHomeExecutionPolicyV1,
    box_name: &'static str,
    method_name: &'static str,
}

impl PreparedLoopV2StringLenCallTargetPlanV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(crate) const fn target_brand(
        &self,
    ) -> crate::mir::resolved_semantics::CoreMethodTargetBrandV1 {
        self.target_brand
    }

    pub(crate) const fn manifest_brand(
        &self,
    ) -> crate::mir::core_method_result_kind::CoreMethodManifestBrandV1 {
        self.manifest_brand
    }

    pub(crate) const fn receiver(&self) -> CoreMethodHomeReceiverRelationV1 {
        self.receiver
    }

    pub(crate) const fn result_relation(&self) -> CoreMethodHomeResultRelationV1 {
        self.result_relation
    }

    pub(crate) const fn effect(&self) -> CoreMethodEffectV1 {
        self.effect
    }

    pub(crate) const fn execution_policy(&self) -> CoreMethodHomeExecutionPolicyV1 {
        self.execution_policy
    }

    pub(crate) const fn box_name(&self) -> &'static str {
        self.box_name
    }

    pub(crate) const fn method_name(&self) -> &'static str {
        self.method_name
    }
}

pub(crate) fn issue_s6c_v2_string_len_call_target_plan_v1(
    inventory: &PreparedLoopV2ConditionOperandInventoryV1<'_>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2StringLenCallTargetPlanV1, StringLenCallTargetPlanRejectV1> {
    if inventory.owner() != expected_owner {
        return Err(StringLenCallTargetPlanRejectV1::ForeignOwner);
    }
    let length_source = inventory
        .rows()
        .iter()
        .find_map(|row| match row.kind() {
            PreparedLoopV2ConditionOperandKindV1::LengthCall { source } => Some((row, source)),
            PreparedLoopV2ConditionOperandKindV1::ReadBinding { .. } => None,
        })
        .ok_or(StringLenCallTargetPlanRejectV1::MissingLengthOperand)?;
    let (row, source) = length_source;
    if source.owner() != expected_owner {
        return Err(StringLenCallTargetPlanRejectV1::ForeignOwner);
    }
    if source.role() != S6CLogicalCallRoleV1::Length
        || source.operation() != CoreMethodOp::StringLen
        || source.arity() != 0
        || source.placement() != ResolvedLoopPlacementV1::Condition
    {
        return Err(StringLenCallTargetPlanRejectV1::SourceShapeMismatch);
    }
    if row.block() != inventory.condition_block() || row.class() != LoopValueClassV2::I64 {
        return Err(StringLenCallTargetPlanRejectV1::WrongOperandKind);
    }
    let target = source.target();
    let target_row = target.row().row();
    if target.manifest_brand() != CORE_METHOD_MANIFEST_BRAND_V1
        || target.row().brand() != target.manifest_brand()
    {
        return Err(StringLenCallTargetPlanRejectV1::TargetBrandMismatch);
    }
    if target.schema() != CoreMethodHomeSchemaV1::StringBoxText
        || target.receiver() != CoreMethodHomeReceiverRelationV1::StringBoxReceiver
        || target.parameters().len() != 0
        || target.result() != CoreMethodHomeResultRelationV1::I64ToCaller
        || target_row.effect != CoreMethodEffectV1::PureRead
        || target.execution_policy() != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl
        || target_row.op != CoreMethodOp::StringLen
        || target.row().arity() != 0
        || target_row.receiver_box != "StringBox"
        || target_row.result_kind
            != crate::mir::core_method_result_kind::CoreMethodResultKindV1::I64Value
        || target_row.canonical.is_empty()
    {
        return Err(StringLenCallTargetPlanRejectV1::TargetShapeMismatch);
    }
    let super::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2::CallSlot {
        receiver,
        args,
        result,
    } = source.recipe_row()
    else {
        return Err(StringLenCallTargetPlanRejectV1::SourceIdentityMismatch);
    };
    if receiver.is_none() || !args.is_empty() || result != Some(row.value()) {
        return Err(StringLenCallTargetPlanRejectV1::SourceIdentityMismatch);
    }
    Ok(PreparedLoopV2StringLenCallTargetPlanV1 {
        owner: expected_owner,
        item: row.item(),
        block: row.block(),
        result: row.value(),
        target_brand: target.target_brand(),
        manifest_brand: target.manifest_brand(),
        receiver: target.receiver(),
        result_relation: target.result(),
        effect: target_row.effect,
        execution_policy: target.execution_policy(),
        box_name: target_row.receiver_box,
        method_name: target_row.canonical,
    })
}
