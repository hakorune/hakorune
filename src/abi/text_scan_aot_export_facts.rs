//! Checked Rust projection of `include/nyrt_dynamic_text_scan_v1.h`.
//!
//! This is pre-link symbolic vocabulary only.  It is deliberately not a
//! ProviderAdmission, registry, RuntimeExecutablePlan, or runtime dispatcher.

pub(crate) const TEXT_SCAN_CONTRACT_ID_V1: &str = "hako.text.scan@1";
pub(crate) const TEXT_SCAN_ABI_REVISION_V1: u32 = 1;
pub(crate) const TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED_V1: u32 = 1;
pub(crate) const TEXT_SCAN_SUSPENSION_NON_SUSPENDING_V1: u32 = 0;

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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TextScanAotExportFactV1 {
    pub(crate) entry: TextScanAotEntryIdV1,
    pub(crate) symbol: &'static str,
    pub(crate) arity: u32,
    pub(crate) argument_lanes: &'static [TextScanValueLaneV1],
    pub(crate) result_lane: TextScanValueLaneV1,
    pub(crate) lease: TextScanLeaseCapabilityV1,
}

pub(crate) const TEXT_SCAN_AOT_EXPORT_FACTS_V1: &[TextScanAotExportFactV1] = &[
    TextScanAotExportFactV1 {
        entry: TextScanAotEntryIdV1::Substring,
        symbol: "hako.text.scan.substring.v1",
        arity: 2,
        argument_lanes: &[
            TextScanValueLaneV1::ImmediateI64,
            TextScanValueLaneV1::ImmediateI64,
        ],
        result_lane: TextScanValueLaneV1::HostHandle,
        lease: TextScanLeaseCapabilityV1::EndAuthorized,
    },
    TextScanAotExportFactV1 {
        entry: TextScanAotEntryIdV1::IndexOf,
        symbol: "hako.text.scan.index_of.v1",
        arity: 1,
        argument_lanes: &[TextScanValueLaneV1::HostHandle],
        result_lane: TextScanValueLaneV1::ImmediateI64,
        lease: TextScanLeaseCapabilityV1::None,
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
        assert_eq!(TEXT_SCAN_AOT_EXPORT_FACTS_V1[0].arity, 2);
        assert_eq!(TEXT_SCAN_AOT_EXPORT_FACTS_V1[1].arity, 1);
    }

    #[test]
    fn projection_does_not_relabel_result_or_effect_semantics() {
        assert!(!TEXT_SCAN_CONTRACT_ID_V1.is_empty());
        assert_eq!(TEXT_SCAN_SUSPENSION_NON_SUSPENDING_V1, 0);
    }
}
