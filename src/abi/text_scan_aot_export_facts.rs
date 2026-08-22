//! Checked Rust projection of `include/nyrt_dynamic_text_scan_v1.h`.
//!
//! This is pre-link symbolic vocabulary only.  It is deliberately not a
//! ProviderAdmission, registry, RuntimeExecutablePlan, or runtime dispatcher.

pub(crate) const TEXT_SCAN_CONTRACT_ID_V1: &str = "hako.text.scan@1";
pub(crate) const TEXT_SCAN_ABI_REVISION_V1: u32 = 1;
pub(crate) const TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED_V1: u32 = 1;
pub(crate) const TEXT_SCAN_SUSPENSION_NON_SUSPENDING_V1: u32 = 0;
pub(crate) const TEXT_SCAN_CALL_ABI_REVISION_V1: u32 = 1;
pub const TEXT_SCAN_CALL_OK_V1: u32 = 0;
pub const TEXT_SCAN_CALL_INVALID_OUTPUT_V1: u32 = 1;
pub(crate) const TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2: u32 =
    super::dynamic_call_slot_wire::DYNAMIC_V2_WIRE_REVISION_V2;
pub(crate) const TEXT_SCAN_SYMBOL_SUBSTRING_V1: &str = "hako.text.scan.substring.v1";
pub(crate) const TEXT_SCAN_SYMBOL_INDEX_OF_V1: &str = "hako.text.scan.index_of.v1";

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanAotEntryIdV1 {
    Substring = 1,
    IndexOf = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanValueLaneV1 {
    HostHandle = 1,
    ImmediateI64 = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanLeaseCapabilityV1 {
    None = 0,
    EndAuthorized = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanCallTransportReturnV1 {
    U32 = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanCallOutParameterV1 {
    Required = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanCallParameterTypeV1 {
    U64 = 1,
    I64 = 2,
    OutPointer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanCallAbiFactV1 {
    pub(crate) entry: TextScanAotEntryIdV1,
    pub(crate) logical_arity: u32,
    pub(crate) abi_revision: u32,
    pub(crate) out_wire_revision: u32,
    pub(crate) transport_return: TextScanCallTransportReturnV1,
    pub(crate) out_parameter: TextScanCallOutParameterV1,
    pub(crate) parameter_types: &'static [TextScanCallParameterTypeV1],
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TextScanAotExportFactV1 {
    pub(crate) entry: TextScanAotEntryIdV1,
    pub(crate) symbol: &'static str,
    pub(crate) arity: u32,
    pub(crate) receiver_lane: TextScanValueLaneV1,
    pub(crate) argument_lanes: &'static [TextScanValueLaneV1],
    pub(crate) result_lane: TextScanValueLaneV1,
    pub(crate) lease: TextScanLeaseCapabilityV1,
    pub(crate) call_abi: TextScanCallAbiFactV1,
}

pub(crate) const TEXT_SCAN_AOT_EXPORT_FACTS_V1: &[TextScanAotExportFactV1] = &[
    TextScanAotExportFactV1 {
        entry: TextScanAotEntryIdV1::Substring,
        symbol: TEXT_SCAN_SYMBOL_SUBSTRING_V1,
        arity: 2,
        receiver_lane: TextScanValueLaneV1::HostHandle,
        argument_lanes: &[
            TextScanValueLaneV1::ImmediateI64,
            TextScanValueLaneV1::ImmediateI64,
        ],
        result_lane: TextScanValueLaneV1::HostHandle,
        lease: TextScanLeaseCapabilityV1::EndAuthorized,
        call_abi: TextScanCallAbiFactV1 {
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
        },
    },
    TextScanAotExportFactV1 {
        entry: TextScanAotEntryIdV1::IndexOf,
        symbol: TEXT_SCAN_SYMBOL_INDEX_OF_V1,
        arity: 1,
        receiver_lane: TextScanValueLaneV1::HostHandle,
        argument_lanes: &[TextScanValueLaneV1::HostHandle],
        result_lane: TextScanValueLaneV1::ImmediateI64,
        lease: TextScanLeaseCapabilityV1::None,
        call_abi: TextScanCallAbiFactV1 {
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
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_has_exact_two_symbolic_entries() {
        assert_eq!(TEXT_SCAN_AOT_EXPORT_FACTS_V1.len(), 2);
        assert_eq!(TEXT_SCAN_ABI_REVISION_V1, 1);
        assert_eq!(TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED_V1, 1);
        assert_eq!(TEXT_SCAN_CALL_ABI_REVISION_V1, 1);
        assert_eq!(TEXT_SCAN_CALL_OK_V1, 0);
        assert_eq!(TEXT_SCAN_CALL_INVALID_OUTPUT_V1, 1);
        assert_eq!(TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2, 2);
        assert_eq!(TEXT_SCAN_AOT_EXPORT_FACTS_V1[0].arity, 2);
        assert_eq!(TEXT_SCAN_AOT_EXPORT_FACTS_V1[1].arity, 1);
        assert_eq!(
            TEXT_SCAN_AOT_EXPORT_FACTS_V1
                .iter()
                .map(|fact| fact.receiver_lane)
                .collect::<Vec<_>>(),
            vec![TextScanValueLaneV1::HostHandle; 2]
        );
    }

    #[test]
    fn projection_does_not_relabel_result_or_effect_semantics() {
        assert!(!TEXT_SCAN_CONTRACT_ID_V1.is_empty());
        assert_eq!(TEXT_SCAN_SUSPENSION_NON_SUSPENDING_V1, 0);
    }

    #[test]
    fn call_abi_uses_checked_out_wire_and_logical_arity() {
        let [substring, index_of] = TEXT_SCAN_AOT_EXPORT_FACTS_V1 else {
            panic!("TextScan must expose exactly two entries");
        };
        assert_eq!(substring.call_abi.entry, substring.entry);
        assert_eq!(substring.call_abi.logical_arity, substring.arity);
        assert_eq!(
            substring.call_abi.transport_return,
            TextScanCallTransportReturnV1::U32
        );
        assert_eq!(
            substring.call_abi.out_parameter,
            TextScanCallOutParameterV1::Required
        );
        assert_eq!(index_of.call_abi.entry, index_of.entry);
        assert_eq!(index_of.call_abi.logical_arity, index_of.arity);
        assert_eq!(index_of.call_abi.out_wire_revision, 2);
        assert_eq!(
            substring.call_abi.parameter_types,
            &[
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::I64,
                TextScanCallParameterTypeV1::OutPointer,
            ]
        );
        assert_eq!(
            index_of.call_abi.parameter_types,
            &[
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::U64,
                TextScanCallParameterTypeV1::OutPointer,
            ]
        );
    }
}
