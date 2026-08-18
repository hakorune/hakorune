//! Source-backed target realization for the common-V2 StringSubstring CallSlot.
//!
//! This is a Builder-free, physical-ID-free bridge.  It carries the already
//! verified S6C source target together with the exact logical CallSlot and the
//! checked TextScan export facts.  It does not issue a MIR call, a lease, or a
//! runtime handle.

use super::common_v2_issuers::PreparedLoopV2PreSessionEnvelopeV1;
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_scan_with_init_joinir::{S6CLogicalCallInputRefV1, S6CLogicalCallRoleV1};
use super::s6c_scan_with_init_joinir_output_rows::{S6CLogicalCallArgsV1, S6CLogicalItemV1};
use super::schema_v2::{LoopOperationExecutionClassV2, LoopOperationV2, LoopValueClassV2};
use crate::abi::text_scan_aot_export_facts::{
    TextScanAotEntryIdV1, TextScanAotExportFactV1, TextScanCallOutParameterV1,
    TextScanCallParameterTypeV1, TextScanCallTransportReturnV1, TextScanLeaseCapabilityV1,
    TextScanValueLaneV1, TEXT_SCAN_AOT_EXPORT_FACTS_V1, TEXT_SCAN_CALL_ABI_REVISION_V1,
    TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2, TEXT_SCAN_SYMBOL_SUBSTRING_V1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    CoreMethodEffectV1, CoreMethodResultKindV1, CORE_METHOD_MANIFEST_BRAND_V2,
};
use crate::mir::resolved_semantics::{
    CoreMethodHomeAbiProfileV1, CoreMethodHomeExecutionPolicyV1, CoreMethodHomeParameterRelationV1,
    CoreMethodHomeReceiverRelationV1, CoreMethodHomeResultRelationV1, CoreMethodHomeSchemaV1,
    FunctionOwnerIdV1, ResolvedLoopPlacementV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SubstringCallTargetPlanRejectV1 {
    ForeignOwner,
    MissingOperation,
    DuplicateOperation,
    SourceShapeMismatch,
    RecipeShapeMismatch,
    TargetBrandMismatch,
    TargetShapeMismatch,
    ProviderShapeMismatch,
}

/// One-shot logical target plan for the future checked Substring materializer.
/// No physical ID, Call, lease token, or runtime handle can escape this plan.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2SubstringCallTargetPlanV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    result: LoopValueKeyV1,
    target_brand: crate::mir::resolved_semantics::CoreMethodTargetBrandV1,
    manifest_brand: crate::mir::core_method_result_kind::CoreMethodManifestBrandV2,
    abi_profile: CoreMethodHomeAbiProfileV1,
    receiver: CoreMethodHomeReceiverRelationV1,
    result_relation: CoreMethodHomeResultRelationV1,
    effect: CoreMethodEffectV1,
    execution_policy: CoreMethodHomeExecutionPolicyV1,
    box_name: &'static str,
    method_name: &'static str,
    provider: &'static TextScanAotExportFactV1,
}

impl PreparedLoopV2SubstringCallTargetPlanV1 {
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
    ) -> crate::mir::core_method_result_kind::CoreMethodManifestBrandV2 {
        self.manifest_brand
    }

    pub(crate) const fn abi_profile(&self) -> CoreMethodHomeAbiProfileV1 {
        self.abi_profile
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

    pub(crate) const fn provider(&self) -> &'static TextScanAotExportFactV1 {
        self.provider
    }
}

pub(crate) fn issue_s6c_v2_substring_call_target_plan_v1(
    envelope: &PreparedLoopV2PreSessionEnvelopeV1<'_, '_>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2SubstringCallTargetPlanV1, SubstringCallTargetPlanRejectV1> {
    if envelope.owner() != expected_owner {
        return Err(SubstringCallTargetPlanRejectV1::ForeignOwner);
    }
    let source: S6CLogicalCallInputRefV1<'_> = envelope.substring_source();
    let result = match source.recipe_row() {
        super::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2::CallSlot {
            result: Some(result),
            ..
        } => result,
        _ => return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch),
    };
    let mut operations = envelope.operations().rows().iter().filter(|row| {
        matches!(
            row.operation(),
            LoopOperationV2::CallSlot {
                result: Some(actual),
                ..
            } if *actual == result
        )
    });
    let operation = operations
        .next()
        .ok_or(SubstringCallTargetPlanRejectV1::MissingOperation)?;
    if operations.next().is_some() {
        return Err(SubstringCallTargetPlanRejectV1::DuplicateOperation);
    }
    if source.role() != S6CLogicalCallRoleV1::Substring
        || source.operation() != CoreMethodOp::StringSubstring
        || source.arity() != 2
        || source.placement() != ResolvedLoopPlacementV1::Body
        || source.arguments().len() != 2
        || source.receiver_binding().is_none()
    {
        return Err(SubstringCallTargetPlanRejectV1::SourceShapeMismatch);
    }

    let S6CLogicalItemV1::CallSlot(call) = operation.source() else {
        return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch);
    };
    let S6CLogicalCallArgsV1::Pair(args) = call.args else {
        return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch);
    };
    if call.role != S6CLogicalCallRoleV1::Substring
        || operation.item() != call.item
        || operation.block() != call.block
        || call.result_class != LoopValueClassV2::Text
    {
        return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch);
    }
    if !matches!(
        operation.operation(),
        LoopOperationV2::CallSlot {
            receiver: Some(receiver),
            args: operation_args,
            result: Some(result),
        } if *receiver == call.receiver
            && operation_args.as_slice() == args
            && *result == call.result
    ) || operation.execution()
        != (LoopOperationExecutionClassV2::ExternallyBoundOutcome {
            normal_result: Some(call.result),
        })
    {
        return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch);
    }
    match source.recipe_row() {
        super::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(receiver),
            args: recipe_args,
            result: Some(result),
        } if receiver == call.receiver && recipe_args == args && result == call.result => {}
        _ => return Err(SubstringCallTargetPlanRejectV1::RecipeShapeMismatch),
    }

    let target = source.target();
    let target_row = target.row().row();
    if target.manifest_brand() != CORE_METHOD_MANIFEST_BRAND_V2
        || target.row().brand() != target.manifest_brand()
    {
        return Err(SubstringCallTargetPlanRejectV1::TargetBrandMismatch);
    }
    if target.schema() != CoreMethodHomeSchemaV1::StringBoxText
        || target.receiver() != CoreMethodHomeReceiverRelationV1::StringBoxReceiver
        || target.parameters()
            != [
                CoreMethodHomeParameterRelationV1::I64Parameter,
                CoreMethodHomeParameterRelationV1::I64Parameter,
            ]
        || target.result() != CoreMethodHomeResultRelationV1::TextToCaller
        || target.abi_profile() != CoreMethodHomeAbiProfileV1::StringBoxTextV1
        || target_row.effect != CoreMethodEffectV1::PureRead
        || target.execution_policy() != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl
        || target_row.op != CoreMethodOp::StringSubstring
        || target.row().arity() != 2
        || target_row.receiver_box != "StringBox"
        || target_row.result_kind != CoreMethodResultKindV1::StringValue
        || target_row.canonical.is_empty()
    {
        return Err(SubstringCallTargetPlanRejectV1::TargetShapeMismatch);
    }

    let provider = substring_provider()?;
    Ok(PreparedLoopV2SubstringCallTargetPlanV1 {
        owner: expected_owner,
        item: call.item,
        block: call.block,
        result: call.result,
        target_brand: target.target_brand(),
        manifest_brand: target.manifest_brand(),
        abi_profile: target.abi_profile(),
        receiver: target.receiver(),
        result_relation: target.result(),
        effect: target_row.effect,
        execution_policy: target.execution_policy(),
        box_name: target_row.receiver_box,
        method_name: target_row.canonical,
        provider,
    })
}

fn substring_provider() -> Result<&'static TextScanAotExportFactV1, SubstringCallTargetPlanRejectV1>
{
    let mut rows = TEXT_SCAN_AOT_EXPORT_FACTS_V1
        .iter()
        .filter(|fact| fact.entry == TextScanAotEntryIdV1::Substring);
    let Some(fact) = rows.next() else {
        return Err(SubstringCallTargetPlanRejectV1::ProviderShapeMismatch);
    };
    if rows.next().is_some()
        || fact.symbol != TEXT_SCAN_SYMBOL_SUBSTRING_V1
        || fact.arity != 2
        || fact.receiver_lane != TextScanValueLaneV1::HostHandle
        || fact.argument_lanes
            != [
                TextScanValueLaneV1::ImmediateI64,
                TextScanValueLaneV1::ImmediateI64,
            ]
        || fact.result_lane != TextScanValueLaneV1::HostHandle
        || fact.lease != TextScanLeaseCapabilityV1::EndAuthorized
        || fact.call_abi.entry != TextScanAotEntryIdV1::Substring
        || fact.call_abi.logical_arity != 2
        || fact.call_abi.abi_revision != TEXT_SCAN_CALL_ABI_REVISION_V1
        || fact.call_abi.out_wire_revision != TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2
        || fact.call_abi.transport_return != TextScanCallTransportReturnV1::U32
        || fact.call_abi.out_parameter != TextScanCallOutParameterV1::Required
        || fact.call_abi.parameter_types
            != [
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::OutPointer,
            ]
    {
        return Err(SubstringCallTargetPlanRejectV1::ProviderShapeMismatch);
    }
    Ok(fact)
}
