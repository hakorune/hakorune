//! Typed transport projection for the selected Dynamic V2 AOT call sites.
//!
//! This module co-seals already-issued admission and physical receipt facts.
//! It does not select a provider, inspect MIR, or issue a semantic result.

use crate::abi::text_scan_aot_export_facts::{TextScanAotEntryIdV1, TextScanValueLaneV1};
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64LaneV1, APrimeI64PhysicalReceiptRejectV1, APrimeI64PhysicalReceiptV1,
};
use crate::mir::checked_callout::{
    CheckedCallOutEntryIdV1, CheckedCallOutSiteIdV1, CheckedCallOutSitePlanPairV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

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
}

/// Co-seal the existing AOT admission and canonical physical receipt.
///
/// The function deliberately borrows both sources and returns only an owned
/// transport projection.  No source/Recipe/Core row is reconstructed here.
pub(crate) fn project_dynamic_v2_aot_call_metadata(
    admission: &PreparedAotExecutableAdmissionV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    site_plans: &CheckedCallOutSitePlanPairV1,
) -> Result<DynamicV2AotCallMetadataProjectionV1, DynamicV2AotCallMetadataRejectV1> {
    receipt
        .validate()
        .map_err(DynamicV2AotCallMetadataRejectV1::InvalidReceipt)?;
    let substring = project_call(
        admission,
        receipt,
        site_plans,
        DynamicV2AotCallRoleV1::Substring,
    )?;
    let index_of = project_call(
        admission,
        receipt,
        site_plans,
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
        calls: [substring, index_of],
    })
}

fn project_call(
    admission: &PreparedAotExecutableAdmissionV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    site_plans: &CheckedCallOutSitePlanPairV1,
    role: DynamicV2AotCallRoleV1,
) -> Result<DynamicV2AotCallSiteProjectionV1, DynamicV2AotCallMetadataRejectV1> {
    let entry = admission.entry_for(role.admitted_role());
    if entry.entry() != role.expected_entry() {
        return Err(DynamicV2AotCallMetadataRejectV1::AdmissionEntryMismatch);
    }
    let site_id = site_plans
        .site_id_for_entry(CheckedCallOutEntryIdV1(entry.entry() as u32))
        .ok_or(DynamicV2AotCallMetadataRejectV1::MissingSitePlan)?;
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
    })
}

fn lane_from_text(lane: TextScanValueLaneV1) -> APrimeI64LaneV1 {
    match lane {
        TextScanValueLaneV1::HostHandle => APrimeI64LaneV1::OpaqueHandle,
        TextScanValueLaneV1::ImmediateI64 => APrimeI64LaneV1::ImmediateI64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::a_prime_i64_physical_receipt::{
        APrimeI64BackendFamilyV1, APrimeI64CallArgumentReceiptV1, APrimeI64CallEdgeReceiptV1,
        APrimeI64ParameterReceiptV1, APrimeI64ReturnReceiptV1, A_PRIME_I64_FORMAL_PARAMETER_COUNT,
    };
    use crate::mir::checked_callout::{
        CheckedCallOutAdmittedSiteInputV1, CheckedCallOutLeaseSlotIdV1, CheckedCallOutNormalShapeV1,
    };
    use crate::mir::core_method_op::CoreMethodOp;
    use crate::mir::generated::core_method_contract_rows::CORE_METHOD_CONTRACT_RESULT_ROWS_V1;
    use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::{BasicBlockId, EffectMask, ValueId};

    #[test]
    fn projection_keeps_exact_two_typed_sites_and_stamp() {
        let admission = admission();
        let receipt = receipt();
        let site_plans = site_plans();
        let projection = project_dynamic_v2_aot_call_metadata(&admission, &receipt, &site_plans)
            .expect("valid admission/receipt projection");
        assert_eq!(projection.calls().len(), 2);
        assert_eq!(projection.calls()[0].role().as_str(), "substring");
        assert_eq!(projection.calls()[1].role().as_str(), "index_of");
        assert_eq!(
            projection.calls()[0].entry().entry(),
            TextScanAotEntryIdV1::Substring
        );
        assert_eq!(
            projection.calls()[1].entry().entry(),
            TextScanAotEntryIdV1::IndexOf
        );
        assert_eq!(projection.calls()[0].site_id().0, 0);
        assert_eq!(projection.calls()[1].site_id().0, 1);
        assert_eq!(projection.registry_generation(), 7);
        assert_eq!(
            projection.plan_stamp(),
            ModuleInvocationBrandV1::test_with_ordinal(7)
        );
    }

    #[test]
    fn malformed_receipt_is_rejected_before_projection() {
        let admission = admission();
        let error = APrimeI64PhysicalReceiptV1::seal_for_test(
            APrimeI64BackendFamilyV1::Llvm,
            A_PRIME_I64_FORMAL_PARAMETER_COUNT,
            vec![
                APrimeI64ParameterReceiptV1 {
                    role: "pos".into(),
                    formal_parameter_index: 0,
                    value_id: ValueId::new(2),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ParameterReceiptV1 {
                    role: "end".into(),
                    formal_parameter_index: 2,
                    value_id: ValueId::new(3),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![],
            vec![],
        )
        .expect_err("wrong formal lane must reject");
        assert!(matches!(
            error,
            APrimeI64PhysicalReceiptRejectV1::ParameterRoleIndexMismatch
        ));
        let _ = admission;
    }

    fn admission() -> PreparedAotExecutableAdmissionV1 {
        let substring = CORE_METHOD_CONTRACT_RESULT_ROWS_V1
            .iter()
            .find(|row| row.op == CoreMethodOp::StringSubstring)
            .expect("substring core row");
        let index_of = CORE_METHOD_CONTRACT_RESULT_ROWS_V1
            .iter()
            .find(|row| row.op == CoreMethodOp::StringIndexOf)
            .expect("indexOf core row");
        let aliases = super::super::seal::TextScanAliasProjectionV1::from_type_registry()
            .expect("type aliases");
        super::super::seal::ProviderAdmissionSealV1::consume_text_scan(
            substring,
            index_of,
            aliases,
            ModuleInvocationBrandV1::test_with_ordinal(7),
        )
        .expect("TextScan admission")
    }

    fn site_plans() -> CheckedCallOutSitePlanPairV1 {
        CheckedCallOutSitePlanPairV1::from_admitted(
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(1),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: CheckedCallOutLeaseSlotIdV1(0),
                },
                effects: EffectMask::READ,
            },
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(2),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::ImmediateI64,
                effects: EffectMask::READ,
            },
            ModuleInvocationBrandV1::test_with_ordinal(7),
        )
        .expect("valid CheckedCallOut site pair")
    }

    fn receipt() -> APrimeI64PhysicalReceiptV1 {
        APrimeI64PhysicalReceiptV1::seal_for_test(
            APrimeI64BackendFamilyV1::Llvm,
            A_PRIME_I64_FORMAL_PARAMETER_COUNT,
            vec![
                APrimeI64ParameterReceiptV1 {
                    role: "pos".into(),
                    formal_parameter_index: 1,
                    value_id: ValueId::new(2),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ParameterReceiptV1 {
                    role: "end".into(),
                    formal_parameter_index: 2,
                    value_id: ValueId::new(3),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![call("substring", 0), call("index_of", 1)],
            vec![
                APrimeI64ReturnReceiptV1 {
                    site: "inner".into(),
                    block: BasicBlockId::new(8),
                    value_id: ValueId::new(30),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ReturnReceiptV1 {
                    site: "outer".into(),
                    block: BasicBlockId::new(9),
                    value_id: ValueId::new(31),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
        )
        .expect("valid receipt")
    }

    fn call(role: &str, site_id: u32) -> APrimeI64CallEdgeReceiptV1 {
        APrimeI64CallEdgeReceiptV1 {
            site_id: CheckedCallOutSiteIdV1(site_id),
            role: role.into(),
            target_fingerprint: if role == "substring" {
                "substring/2".into()
            } else {
                "indexOf/1".into()
            },
            receiver_role: if role == "substring" {
                "src"
            } else {
                "pred_chars"
            }
            .into(),
            receiver_value_id: ValueId::new(if role == "substring" { 10 } else { 14 }),
            receiver_lane: APrimeI64LaneV1::OpaqueHandle,
            arguments: if role == "substring" {
                vec![
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 0,
                        role: "start".into(),
                        value_id: ValueId::new(12),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 1,
                        role: "end".into(),
                        value_id: ValueId::new(13),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                ]
            } else {
                vec![APrimeI64CallArgumentReceiptV1 {
                    ordinal: 0,
                    role: "ch".into(),
                    value_id: ValueId::new(20),
                    lane: APrimeI64LaneV1::OpaqueHandle,
                }]
            },
            result_value_id: ValueId::new(if role == "substring" { 20 } else { 21 }),
            result_lane: if role == "substring" {
                APrimeI64LaneV1::OpaqueHandle
            } else {
                APrimeI64LaneV1::ImmediateI64
            },
        }
    }
}
