//! One-shot TextScan provider admission over retained semantic rows.

use std::num::NonZeroU64;

use crate::abi::text_scan_aot_export_facts::{
    TextScanAotEntryIdV1, TextScanAotExportFactV1, TextScanCallOutParameterV1,
    TextScanCallTransportReturnV1, TextScanLeaseCapabilityV1, TextScanValueLaneV1,
    TEXT_SCAN_ABI_REVISION_V1, TEXT_SCAN_AOT_EXPORT_FACTS_V1,
    TEXT_SCAN_CALL_ABI_REVISION_V1, TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2,
    TextScanCallParameterTypeV1, TEXT_SCAN_CONTRACT_ID_V1,
    TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED_V1, TEXT_SCAN_SYMBOL_INDEX_OF_V1,
    TEXT_SCAN_SYMBOL_SUBSTRING_V1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    CoreMethodEffectV1, CoreMethodResultKindV1, CoreMethodContractResultRowV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

use super::admitted_registry::AdmittedTextScanRegistryV1;
use super::aot_admission::{build, PreparedAotExecutableAdmissionV1, TextScanEntryContractV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderAdmissionRejectV1 {
    MissingCoreRow,
    CoreRowMismatch,
    MissingExport,
    ExportMismatch,
    AliasMissing,
    AliasConflict,
    RegistryCollision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanAliasProjectionV1 {
    substring_slot: u16,
    index_of_slot: u16,
}

impl TextScanAliasProjectionV1 {
    /// Borrow the runtime type-registry vocabulary; this does not select a
    /// provider and does not retain a mutable registry reference.
    pub(crate) fn from_type_registry() -> Result<Self, ProviderAdmissionRejectV1> {
        let slots = ["String", "StringBox"]
            .map(|type_name| {
                (
                    crate::runtime::type_registry::resolve_slot_by_name(
                        type_name,
                        "substring",
                        2,
                    ),
                    crate::runtime::type_registry::resolve_slot_by_name(
                        type_name,
                        "indexOf",
                        1,
                    ),
                )
            });
        let Some(substring_slot) = slots[0].0 else {
            return Err(ProviderAdmissionRejectV1::AliasMissing);
        };
        let Some(index_of_slot) = slots[0].1 else {
            return Err(ProviderAdmissionRejectV1::AliasMissing);
        };
        if slots.iter().any(|(substring, index_of)| {
            *substring != Some(substring_slot) || *index_of != Some(index_of_slot)
        }) {
            return Err(ProviderAdmissionRejectV1::AliasConflict);
        }
        Ok(Self {
            substring_slot,
            index_of_slot,
        })
    }
}

pub(crate) struct ProviderAdmissionSealV1;

impl ProviderAdmissionSealV1 {
    /// Consume retained generated Core rows into one symbolic AOT product.
    /// The rows are borrowed only for validation and are not copied into the
    /// resulting admission.
    pub(crate) fn consume_text_scan(
        substring_core: &CoreMethodContractResultRowV1,
        index_of_core: &CoreMethodContractResultRowV1,
        aliases: TextScanAliasProjectionV1,
        registry_generation: NonZeroU64,
        plan_stamp: ModuleInvocationBrandV1,
    ) -> Result<PreparedAotExecutableAdmissionV1, ProviderAdmissionRejectV1> {
        validate_core_row(
            substring_core,
            CoreMethodOp::StringSubstring,
            2,
            CoreMethodResultKindV1::StringValue,
        )?;
        validate_core_row(
            index_of_core,
            CoreMethodOp::StringIndexOf,
            1,
            CoreMethodResultKindV1::I64Value,
        )?;
        if substring_core as *const _ == index_of_core as *const _ {
            return Err(ProviderAdmissionRejectV1::CoreRowMismatch);
        }

        let substring = export_fact(TextScanAotEntryIdV1::Substring)?;
        let index_of = export_fact(TextScanAotEntryIdV1::IndexOf)?;
        let substring_entry = entry_contract(substring, TextScanAotEntryIdV1::Substring, 2)?;
        let index_of_entry = entry_contract(index_of, TextScanAotEntryIdV1::IndexOf, 1)?;
        let registry = AdmittedTextScanRegistryV1::new(
            aliases.substring_slot,
            aliases.index_of_slot,
            registry_generation.get(),
        )
        .map_err(|_| ProviderAdmissionRejectV1::RegistryCollision)?;
        Ok(build(
            TEXT_SCAN_CONTRACT_ID_V1,
            TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED_V1,
            TEXT_SCAN_ABI_REVISION_V1,
            "Text",
            ["String", "StringBox"],
            registry,
            substring_entry,
            index_of_entry,
            plan_stamp,
        ))
    }
}

fn validate_core_row(
    row: &CoreMethodContractResultRowV1,
    expected_op: CoreMethodOp,
    expected_arity: u32,
    expected_result: CoreMethodResultKindV1,
) -> Result<(), ProviderAdmissionRejectV1> {
    if row.receiver_box != "StringBox"
        || row.op != expected_op
        || !row.arities.contains(&expected_arity)
        || row.result_kind != expected_result
        || row.effect != CoreMethodEffectV1::PureRead
    {
        return Err(ProviderAdmissionRejectV1::CoreRowMismatch);
    }
    Ok(())
}

fn export_fact(
    entry: TextScanAotEntryIdV1,
) -> Result<&'static TextScanAotExportFactV1, ProviderAdmissionRejectV1> {
    let mut rows = TEXT_SCAN_AOT_EXPORT_FACTS_V1
        .iter()
        .filter(|fact| fact.entry == entry);
    let Some(row) = rows.next() else {
        return Err(ProviderAdmissionRejectV1::MissingExport);
    };
    if rows.next().is_some() {
        return Err(ProviderAdmissionRejectV1::ExportMismatch);
    }
    Ok(row)
}

fn entry_contract(
    fact: &'static TextScanAotExportFactV1,
    expected_entry: TextScanAotEntryIdV1,
    expected_arity: u32,
) -> Result<TextScanEntryContractV1, ProviderAdmissionRejectV1> {
    if fact.entry != expected_entry
        || fact.arity != expected_arity
        || fact.receiver_lane != TextScanValueLaneV1::HostHandle
        || fact.symbol != expected_symbol(expected_entry)
        || fact.call_abi.entry != expected_entry
        || fact.call_abi.logical_arity != expected_arity
        || fact.call_abi.abi_revision != TEXT_SCAN_CALL_ABI_REVISION_V1
        || fact.call_abi.out_wire_revision != TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2
        || fact.call_abi.transport_return != TextScanCallTransportReturnV1::U32
        || fact.call_abi.out_parameter != TextScanCallOutParameterV1::Required
        || fact.call_abi.parameter_types != expected_parameter_types(expected_entry)
    {
        return Err(ProviderAdmissionRejectV1::ExportMismatch);
    }
    if matches!(
        (expected_entry, fact.result_lane, fact.lease),
        (
            TextScanAotEntryIdV1::Substring,
            TextScanValueLaneV1::HostHandle,
            TextScanLeaseCapabilityV1::EndAuthorized
        ) | (
            TextScanAotEntryIdV1::IndexOf,
            TextScanValueLaneV1::ImmediateI64,
            TextScanLeaseCapabilityV1::None
        )
    ) {
        Ok(TextScanEntryContractV1::from_fact(
            fact.entry,
            fact.symbol,
            fact.arity,
            fact.receiver_lane,
            fact.argument_lanes,
            fact.result_lane,
            fact.lease,
            fact.call_abi,
        ))
    } else {
        Err(ProviderAdmissionRejectV1::ExportMismatch)
    }
}

fn expected_symbol(entry: TextScanAotEntryIdV1) -> &'static str {
    match entry {
        TextScanAotEntryIdV1::Substring => TEXT_SCAN_SYMBOL_SUBSTRING_V1,
        TextScanAotEntryIdV1::IndexOf => TEXT_SCAN_SYMBOL_INDEX_OF_V1,
    }
}

fn expected_parameter_types(
    entry: TextScanAotEntryIdV1,
) -> &'static [TextScanCallParameterTypeV1] {
    match entry {
        TextScanAotEntryIdV1::Substring => &[
            TextScanCallParameterTypeV1::U64,
            TextScanCallParameterTypeV1::I64,
            TextScanCallParameterTypeV1::I64,
            TextScanCallParameterTypeV1::OutPointer,
        ],
        TextScanAotEntryIdV1::IndexOf => &[
            TextScanCallParameterTypeV1::U64,
            TextScanCallParameterTypeV1::U64,
            TextScanCallParameterTypeV1::OutPointer,
        ],
    }
}
