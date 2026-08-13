//! Typed transport projection for the selected Dynamic V2 AOT call sites.
//!
//! This module co-seals already-issued admission and physical receipt facts.
//! It does not select a provider, inspect MIR, or issue a semantic result.

use crate::abi::text_scan_aot_export_facts::{TextScanAotEntryIdV1, TextScanValueLaneV1};
#[cfg(test)]
use crate::abi::text_scan_aot_export_facts::{
    TextScanCallAbiFactV1, TextScanCallOutParameterV1, TextScanCallParameterTypeV1,
    TextScanCallTransportReturnV1, TextScanLeaseCapabilityV1, TEXT_SCAN_CALL_ABI_REVISION_V1,
    TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2, TEXT_SCAN_SYMBOL_INDEX_OF_V1,
    TEXT_SCAN_SYMBOL_SUBSTRING_V1,
};
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64LaneV1, APrimeI64PhysicalReceiptRejectV1, APrimeI64PhysicalReceiptV1,
};
use crate::mir::checked_callout::{
    CheckedCallOutEntryIdV1, CheckedCallOutNormalShapeV1, CheckedCallOutPlanTableV1,
    CheckedCallOutSiteIdV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{EffectMask, MirFunction, MirType, ValueId};

use super::admitted_registry::TextScanAdmittedRoleV1;
use super::aot_admission::{PreparedAotExecutableAdmissionV1, TextScanEntryContractV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicV2AotCallMetadataRejectV1 {
    InvalidReceipt(APrimeI64PhysicalReceiptRejectV1),
    MissingCallRole,
    DuplicateCallRole,
    MissingCallSite,
    CallSiteRoleMismatch,
    AdmissionEntryMismatch,
    MissingSitePlan,
    FunctionSignatureMismatch,
    FormalLaneMismatch,
    FormalValueMismatch,
    MissingNormalResult,
    DuplicateNormalResult,
    ReceiverLaneMismatch,
    ArgumentLaneMismatch,
    ResultLaneMismatch,
    ArityMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DynamicV2AotCallRoleV1 {
    Substring,
    IndexOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicV2AotFormalRoleV1 {
    Src,
    Pos,
    End,
    PredChars,
}

impl DynamicV2AotFormalRoleV1 {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Src => "src",
            Self::Pos => "pos",
            Self::End => "end",
            Self::PredChars => "pred_chars",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DynamicV2AotFormalProjectionV1 {
    role: DynamicV2AotFormalRoleV1,
    value_id: ValueId,
    lane: APrimeI64LaneV1,
}

impl DynamicV2AotFormalProjectionV1 {
    pub(crate) const fn new(
        role: DynamicV2AotFormalRoleV1,
        value_id: ValueId,
        lane: APrimeI64LaneV1,
    ) -> Self {
        Self {
            role,
            value_id,
            lane,
        }
    }

    pub(crate) const fn role(self) -> DynamicV2AotFormalRoleV1 {
        self.role
    }

    pub(crate) const fn value_id(self) -> ValueId {
        self.value_id
    }

    pub(crate) const fn lane(self) -> APrimeI64LaneV1 {
        self.lane
    }
}

impl DynamicV2AotCallRoleV1 {
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Substring => "substring",
            Self::IndexOf => "index_of",
        }
    }

    const fn expected_entry(&self) -> TextScanAotEntryIdV1 {
        match self {
            Self::Substring => TextScanAotEntryIdV1::Substring,
            Self::IndexOf => TextScanAotEntryIdV1::IndexOf,
        }
    }

    const fn admitted_role(&self) -> TextScanAdmittedRoleV1 {
        match self {
            Self::Substring => TextScanAdmittedRoleV1::TextSliceRange,
            Self::IndexOf => TextScanAdmittedRoleV1::TextFindNeedle,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DynamicV2AotCallSiteProjectionV1 {
    role: DynamicV2AotCallRoleV1,
    site_id: CheckedCallOutSiteIdV1,
    entry: TextScanEntryContractV1,
    normal_shape: CheckedCallOutNormalShapeV1,
    outcome_slot: crate::mir::checked_callout::CheckedCallOutOutcomeSlotIdV1,
    normal_result_dst: ValueId,
    effects: EffectMask,
}

impl DynamicV2AotCallSiteProjectionV1 {
    pub(crate) fn role(&self) -> &DynamicV2AotCallRoleV1 {
        &self.role
    }

    pub(crate) const fn site_id(&self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }

    pub(crate) const fn entry(&self) -> TextScanEntryContractV1 {
        self.entry
    }

    pub(crate) const fn normal_shape(&self) -> CheckedCallOutNormalShapeV1 {
        self.normal_shape
    }

    pub(crate) const fn outcome_slot(
        &self,
    ) -> crate::mir::checked_callout::CheckedCallOutOutcomeSlotIdV1 {
        self.outcome_slot
    }

    pub(crate) const fn normal_result_dst(&self) -> ValueId {
        self.normal_result_dst
    }

    pub(crate) const fn effects(&self) -> EffectMask {
        self.effects
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DynamicV2AotCallMetadataProjectionV1 {
    schema_version: u32,
    contract_id: &'static str,
    profile: u32,
    abi_revision: u32,
    wire_revision: u32,
    registry_generation: u64,
    plan_stamp: ModuleInvocationBrandV1,
    formal_parameters: [DynamicV2AotFormalProjectionV1; 4],
    return_lane: APrimeI64LaneV1,
    function_effects: EffectMask,
    calls: [DynamicV2AotCallSiteProjectionV1; 2],
}

impl DynamicV2AotCallMetadataProjectionV1 {
    pub(crate) const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub(crate) const fn contract_id(&self) -> &'static str {
        self.contract_id
    }

    pub(crate) const fn profile(&self) -> u32 {
        self.profile
    }

    pub(crate) const fn abi_revision(&self) -> u32 {
        self.abi_revision
    }

    pub(crate) const fn wire_revision(&self) -> u32 {
        self.wire_revision
    }

    pub(crate) const fn registry_generation(&self) -> u64 {
        self.registry_generation
    }

    pub(crate) const fn plan_stamp(&self) -> ModuleInvocationBrandV1 {
        self.plan_stamp
    }

    pub(crate) fn calls(&self) -> &[DynamicV2AotCallSiteProjectionV1; 2] {
        &self.calls
    }

    pub(crate) const fn formal_parameters(&self) -> &[DynamicV2AotFormalProjectionV1; 4] {
        &self.formal_parameters
    }

    pub(crate) const fn return_lane(&self) -> APrimeI64LaneV1 {
        self.return_lane
    }

    pub(crate) const fn function_effects(&self) -> EffectMask {
        self.function_effects
    }

    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        let substring_abi = TextScanCallAbiFactV1 {
            entry: TextScanAotEntryIdV1::Substring,
            logical_arity: 2,
            abi_revision: TEXT_SCAN_CALL_ABI_REVISION_V1,
            out_wire_revision: TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2,
            transport_return: TextScanCallTransportReturnV1::U32,
            out_parameter: TextScanCallOutParameterV1::Required,
            parameter_types: &[
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::OutPointer,
            ],
        };
        let index_of_abi = TextScanCallAbiFactV1 {
            entry: TextScanAotEntryIdV1::IndexOf,
            logical_arity: 1,
            abi_revision: TEXT_SCAN_CALL_ABI_REVISION_V1,
            out_wire_revision: TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2,
            transport_return: TextScanCallTransportReturnV1::U32,
            out_parameter: TextScanCallOutParameterV1::Required,
            parameter_types: &[
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::OutPointer,
            ],
        };
        let substring = TextScanEntryContractV1::from_fact(
            TextScanAotEntryIdV1::Substring,
            TEXT_SCAN_SYMBOL_SUBSTRING_V1,
            2,
            TextScanValueLaneV1::HostHandle,
            &[
                TextScanValueLaneV1::ImmediateI64,
                TextScanValueLaneV1::ImmediateI64,
            ],
            TextScanValueLaneV1::HostHandle,
            TextScanLeaseCapabilityV1::EndAuthorized,
            substring_abi,
        );
        let index_of = TextScanEntryContractV1::from_fact(
            TextScanAotEntryIdV1::IndexOf,
            TEXT_SCAN_SYMBOL_INDEX_OF_V1,
            1,
            TextScanValueLaneV1::HostHandle,
            &[TextScanValueLaneV1::HostHandle],
            TextScanValueLaneV1::ImmediateI64,
            TextScanLeaseCapabilityV1::None,
            index_of_abi,
        );
        Self {
            schema_version: 2,
            contract_id: "hako.text.scan@1",
            profile: 1,
            abi_revision: 1,
            wire_revision: TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2,
            registry_generation: 7,
            plan_stamp: ModuleInvocationBrandV1::test_with_ordinal(7),
            formal_parameters: [
                DynamicV2AotFormalProjectionV1::new(
                    DynamicV2AotFormalRoleV1::Src,
                    ValueId::new(0),
                    APrimeI64LaneV1::OpaqueHandle,
                ),
                DynamicV2AotFormalProjectionV1::new(
                    DynamicV2AotFormalRoleV1::Pos,
                    ValueId::new(1),
                    APrimeI64LaneV1::ImmediateI64,
                ),
                DynamicV2AotFormalProjectionV1::new(
                    DynamicV2AotFormalRoleV1::End,
                    ValueId::new(2),
                    APrimeI64LaneV1::ImmediateI64,
                ),
                DynamicV2AotFormalProjectionV1::new(
                    DynamicV2AotFormalRoleV1::PredChars,
                    ValueId::new(3),
                    APrimeI64LaneV1::OpaqueHandle,
                ),
            ],
            return_lane: APrimeI64LaneV1::ImmediateI64,
            function_effects: EffectMask::READ,
            calls: [
                DynamicV2AotCallSiteProjectionV1 {
                    role: DynamicV2AotCallRoleV1::Substring,
                    site_id: CheckedCallOutSiteIdV1(0),
                    entry: substring,
                    normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                        lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(0),
                    },
                    outcome_slot: crate::mir::checked_callout::CheckedCallOutOutcomeSlotIdV1(0),
                    normal_result_dst: ValueId::new(20),
                    effects: EffectMask::READ,
                },
                DynamicV2AotCallSiteProjectionV1 {
                    role: DynamicV2AotCallRoleV1::IndexOf,
                    site_id: CheckedCallOutSiteIdV1(1),
                    entry: index_of,
                    normal_shape: CheckedCallOutNormalShapeV1::ImmediateI64,
                    outcome_slot: crate::mir::checked_callout::CheckedCallOutOutcomeSlotIdV1(1),
                    normal_result_dst: ValueId::new(21),
                    effects: EffectMask::READ,
                },
            ],
        }
    }
}

/// Co-seal the existing AOT admission and canonical physical receipt.
///
/// The function deliberately borrows both sources and returns only an owned
/// transport projection.  No source/Recipe/Core row is reconstructed here.
pub(crate) fn project_dynamic_v2_aot_call_metadata(
    admission: &PreparedAotExecutableAdmissionV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    site_plans: &CheckedCallOutPlanTableV1,
    function: &MirFunction,
    formal_parameters: [DynamicV2AotFormalProjectionV1; 4],
    expected_effects: EffectMask,
) -> Result<DynamicV2AotCallMetadataProjectionV1, DynamicV2AotCallMetadataRejectV1> {
    receipt
        .validate()
        .map_err(DynamicV2AotCallMetadataRejectV1::InvalidReceipt)?;
    validate_function_transport(function, formal_parameters, expected_effects)?;
    let substring = project_call(
        admission,
        receipt,
        site_plans,
        function,
        DynamicV2AotCallRoleV1::Substring,
    )?;
    let index_of = project_call(
        admission,
        receipt,
        site_plans,
        function,
        DynamicV2AotCallRoleV1::IndexOf,
    )?;
    Ok(DynamicV2AotCallMetadataProjectionV1 {
        schema_version: 2,
        contract_id: admission.contract_id(),
        profile: admission.profile(),
        abi_revision: admission.abi_revision(),
        wire_revision: substring.entry().call_abi().out_wire_revision,
        registry_generation: admission.registry_generation(),
        plan_stamp: admission.plan_stamp(),
        formal_parameters,
        return_lane: APrimeI64LaneV1::ImmediateI64,
        function_effects: function.signature.effects,
        calls: [substring, index_of],
    })
}

fn validate_function_transport(
    function: &MirFunction,
    formal_parameters: [DynamicV2AotFormalProjectionV1; 4],
    expected_effects: EffectMask,
) -> Result<(), DynamicV2AotCallMetadataRejectV1> {
    if function.params.len() != 4
        || function.signature.params.len() != 4
        || function.signature.return_type != MirType::Integer
        || function.signature.effects != expected_effects
    {
        return Err(DynamicV2AotCallMetadataRejectV1::FunctionSignatureMismatch);
    }
    let expected_roles = [
        DynamicV2AotFormalRoleV1::Src,
        DynamicV2AotFormalRoleV1::Pos,
        DynamicV2AotFormalRoleV1::End,
        DynamicV2AotFormalRoleV1::PredChars,
    ];
    for (index, row) in formal_parameters.iter().copied().enumerate() {
        if row.role != expected_roles[index] || function.params[index] != row.value_id {
            return Err(if row.role != expected_roles[index] {
                DynamicV2AotCallMetadataRejectV1::FormalLaneMismatch
            } else {
                DynamicV2AotCallMetadataRejectV1::FormalValueMismatch
            });
        }
        if index == 1 || index == 2 {
            if row.lane != APrimeI64LaneV1::ImmediateI64
                || function.signature.params[index] != MirType::Integer
            {
                return Err(DynamicV2AotCallMetadataRejectV1::FormalLaneMismatch);
            }
        } else if row.lane != APrimeI64LaneV1::OpaqueHandle {
            return Err(DynamicV2AotCallMetadataRejectV1::FormalLaneMismatch);
        }
    }
    Ok(())
}

fn project_call(
    admission: &PreparedAotExecutableAdmissionV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    site_plans: &CheckedCallOutPlanTableV1,
    function: &MirFunction,
    role: DynamicV2AotCallRoleV1,
) -> Result<DynamicV2AotCallSiteProjectionV1, DynamicV2AotCallMetadataRejectV1> {
    let entry = admission.entry_for(role.admitted_role());
    if entry.entry() != role.expected_entry() {
        return Err(DynamicV2AotCallMetadataRejectV1::AdmissionEntryMismatch);
    }
    let entry_id = CheckedCallOutEntryIdV1(entry.entry() as u32);
    let plan = site_plans
        .plan_for_entry(entry_id)
        .ok_or(DynamicV2AotCallMetadataRejectV1::MissingSitePlan)?;
    if site_plans.len() != 2
        || plan.call_abi_revision() != 1
        || plan.wire_revision() != 2
        || plan.plan_stamp() != admission.plan_stamp()
        || match role {
            DynamicV2AotCallRoleV1::Substring => !matches!(
                plan.normal_shape(),
                CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. }
            ),
            DynamicV2AotCallRoleV1::IndexOf => !matches!(
                plan.normal_shape(),
                CheckedCallOutNormalShapeV1::ImmediateI64
            ),
        }
    {
        return Err(DynamicV2AotCallMetadataRejectV1::MissingSitePlan);
    }
    let site_id = plan.site_id();
    let normal_result_dst = normal_result_for_site(function, site_id)?;
    let edge = receipt
        .call_edge(site_id)
        .ok_or(DynamicV2AotCallMetadataRejectV1::MissingCallSite)?;
    if edge.role != role.as_str() {
        return Err(DynamicV2AotCallMetadataRejectV1::CallSiteRoleMismatch);
    }
    if entry.arity() as usize != edge.arguments.len()
        || entry.call_abi().logical_arity as usize != edge.arguments.len()
    {
        return Err(DynamicV2AotCallMetadataRejectV1::ArityMismatch);
    }
    if lane_from_text(entry.receiver_lane()) != edge.receiver_lane {
        return Err(DynamicV2AotCallMetadataRejectV1::ReceiverLaneMismatch);
    }
    if entry
        .argument_lanes()
        .iter()
        .zip(edge.arguments.iter())
        .any(|(expected, actual)| lane_from_text(*expected) != actual.lane)
    {
        return Err(DynamicV2AotCallMetadataRejectV1::ArgumentLaneMismatch);
    }
    if lane_from_text(entry.result_lane()) != edge.result_lane {
        return Err(DynamicV2AotCallMetadataRejectV1::ResultLaneMismatch);
    }
    Ok(DynamicV2AotCallSiteProjectionV1 {
        role,
        site_id,
        entry,
        normal_shape: plan.normal_shape(),
        outcome_slot: plan.outcome_slot(),
        normal_result_dst,
        effects: plan.effects(),
    })
}

fn normal_result_for_site(
    function: &MirFunction,
    site_id: CheckedCallOutSiteIdV1,
) -> Result<ValueId, DynamicV2AotCallMetadataRejectV1> {
    let mut normal_landing = None;
    for block in function.blocks.values() {
        if let Some(crate::mir::MirInstruction::CheckedCallOut {
            site_id: observed,
            normal_landing: landing,
            ..
        }) = block.terminator.as_ref()
        {
            if *observed == site_id {
                normal_landing = Some(*landing);
                break;
            }
        }
    }
    let landing = normal_landing.ok_or(DynamicV2AotCallMetadataRejectV1::MissingNormalResult)?;
    let block = function
        .get_block(landing)
        .ok_or(DynamicV2AotCallMetadataRejectV1::MissingNormalResult)?;
    let mut result = None;
    for instruction in &block.instructions {
        if let crate::mir::MirInstruction::CheckedCallOutNormalResult {
            site_id: observed,
            dst,
        } = instruction
        {
            if *observed == site_id {
                if result.replace(*dst).is_some() {
                    return Err(DynamicV2AotCallMetadataRejectV1::DuplicateNormalResult);
                }
            }
        }
    }
    result.ok_or(DynamicV2AotCallMetadataRejectV1::MissingNormalResult)
}

fn lane_from_text(lane: TextScanValueLaneV1) -> APrimeI64LaneV1 {
    match lane {
        TextScanValueLaneV1::HostHandle => APrimeI64LaneV1::OpaqueHandle,
        TextScanValueLaneV1::ImmediateI64 => APrimeI64LaneV1::ImmediateI64,
    }
}

#[cfg(test)]
#[path = "call_metadata_tests.rs"]
mod tests;
