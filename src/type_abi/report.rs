//! Stable report vocabulary for the Type ABI view surface.

pub const TYPE_ABI_MODE: &str = "type_abi_mode";
pub const TYPE_ABI_MODE_VIEW_OVER_EXISTING_TRUTH: &str = "view_over_existing_truth";
pub const TYPE_ABI_PACK_IS_TRUTH: &str = "type_abi_pack_is_truth";
pub const TYPE_ABI_NEW_DUPLICATE_DESCRIPTOR_COUNT: &str = "type_abi_new_duplicate_descriptor_count";
pub const TYPE_ABI_C_API_FUNCTION_COUNT: &str = "type_abi_c_api_function_count";
pub const TYPE_ABI_HOT_LOOKUP_COUNT: &str = "type_abi_hot_lookup_count";
pub const TYPE_ABI_QUERY_HOT_PATH_COUNT: &str = "type_abi_query_hot_path_count";
pub const TYPE_ABI_DEBUG_LOOKUP_COUNT: &str = "type_abi_debug_lookup_count";
pub const TYPE_ABI_QUERY_PHASE: &str = "type_abi_query_phase";
pub const TYPE_ABI_VIEW_ADAPTER_COUNT: &str = "type_abi_view_adapter_count";
pub const TYPE_ABI_PACK_GENERATED_COUNT: &str = "type_abi_pack_generated_count";
pub const TYPE_ABI_PACK_SOURCE_HASH: &str = "type_abi_pack_source_hash";
pub const TYPE_ABI_PACK_ENTRY_COUNT: &str = "type_abi_pack_entry_count";
pub const TYPE_ABI_PACK_SCHEMA_VERSION: &str = "type_abi_pack_schema_version";

pub const TYPE_ABI_QUERY_PHASE_PLANNING: &str = "planning";
pub const TYPE_ABI_QUERY_PHASE_REFLECTION: &str = "reflection";
pub const TYPE_ABI_QUERY_PHASE_DEBUG: &str = "debug";
pub const TYPE_ABI_QUERY_PHASE_HOT: &str = "hot";

pub const PLAN_ENVELOPE_STAMP_ENABLED: &str = "plan_envelope_stamp_enabled";
pub const PLAN_STAMP_MODE: &str = "plan_stamp_mode";
pub const PLAN_STAMP_MODE_COMPILE_SESSION_EPOCH: &str = "compile_session_epoch";
pub const PLAN_STAMP_DOMAIN_EPOCH_ENABLED: &str = "plan_stamp_domain_epoch_enabled";
pub const PLAN_STAMP_TRUTH_HASH_ENABLED: &str = "plan_stamp_truth_hash_enabled";
pub const PLAN_STAMP_HOT_LOOP_CHECK_COUNT: &str = "plan_stamp_hot_loop_check_count";
pub const PLAN_STAMP_DEBUG_CHECK_COUNT: &str = "plan_stamp_debug_check_count";
pub const PLAN_STALE_DETECTED_COUNT: &str = "plan_stale_detected_count";
pub const PLAN_REGENERATED_COUNT: &str = "plan_regenerated_count";
pub const PLAN_FALLBACK_DUE_TO_STALE_COUNT: &str = "plan_fallback_due_to_stale_count";

pub const TYPE_ABI_DOMAIN_CALL_TRUTH_SOURCE: &str = "type_abi_domain[call].truth_source";
pub const TYPE_ABI_DOMAIN_FIELD_TRUTH_SOURCE: &str = "type_abi_domain[field].truth_source";
pub const TYPE_ABI_DOMAIN_MEMORY_TRUTH_SOURCE: &str = "type_abi_domain[memory].truth_source";
pub const TYPE_ABI_DOMAIN_STRING_TRUTH_SOURCE: &str = "type_abi_domain[string].truth_source";
pub const TYPE_ABI_DOMAIN_GUI_TRUTH_SOURCE: &str = "type_abi_domain[gui].truth_source";

pub const TRUTH_SOURCE_TYPE_REGISTRY: &str = "type_registry";
pub const TRUTH_SOURCE_TYPED_OBJECT_PLAN: &str = "typed_object_plan";
pub const TRUTH_SOURCE_FASTMEM_ACCESS_PLAN: &str = "fastmem_access_plan";
pub const TRUTH_SOURCE_STRING_KERNEL_PLAN: &str = "string_kernel_plan";
pub const TRUTH_SOURCE_GUI_DOMAIN: &str = "gui_domain";

pub const TYPEABI_VIEW_001_BASELINE: &[(&str, &str)] = &[
    (TYPE_ABI_MODE, TYPE_ABI_MODE_VIEW_OVER_EXISTING_TRUTH),
    (TYPE_ABI_PACK_IS_TRUTH, "0"),
    (TYPE_ABI_NEW_DUPLICATE_DESCRIPTOR_COUNT, "0"),
    (TYPE_ABI_C_API_FUNCTION_COUNT, "0"),
    (TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (TYPE_ABI_QUERY_HOT_PATH_COUNT, "0"),
    (PLAN_ENVELOPE_STAMP_ENABLED, "0"),
    (PLAN_STAMP_HOT_LOOP_CHECK_COUNT, "0"),
];

pub const TYPEABI_VIEW_002_METHOD_ENTRY_ROWS: &[(&str, &str)] = &[
    (TYPE_ABI_VIEW_ADAPTER_COUNT, "1"),
    (
        TYPE_ABI_DOMAIN_CALL_TRUTH_SOURCE,
        TRUTH_SOURCE_TYPE_REGISTRY,
    ),
    (TYPE_ABI_PACK_IS_TRUTH, "0"),
    (TYPE_ABI_NEW_DUPLICATE_DESCRIPTOR_COUNT, "0"),
    (TYPE_ABI_C_API_FUNCTION_COUNT, "0"),
    (TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (TYPE_ABI_QUERY_HOT_PATH_COUNT, "0"),
];

pub const TYPEABI_VIEW_003_IN_MEMORY_QUERY_ROWS: &[(&str, &str)] = &[
    (TYPE_ABI_QUERY_PHASE, TYPE_ABI_QUERY_PHASE_PLANNING),
    (TYPE_ABI_PACK_GENERATED_COUNT, "0"),
    (TYPE_ABI_C_API_FUNCTION_COUNT, "0"),
    (TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (TYPE_ABI_QUERY_HOT_PATH_COUNT, "0"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_report_keeps_type_abi_cold_and_view_only() {
        let rows = TYPEABI_VIEW_001_BASELINE
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_MODE], TYPE_ABI_MODE_VIEW_OVER_EXISTING_TRUTH);
        assert_eq!(rows[TYPE_ABI_PACK_IS_TRUTH], "0");
        assert_eq!(rows[TYPE_ABI_C_API_FUNCTION_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_QUERY_HOT_PATH_COUNT], "0");
    }

    #[test]
    fn domain_truth_source_names_match_the_ssot() {
        assert_eq!(TRUTH_SOURCE_TYPE_REGISTRY, "type_registry");
        assert_eq!(TRUTH_SOURCE_TYPED_OBJECT_PLAN, "typed_object_plan");
        assert_eq!(TRUTH_SOURCE_FASTMEM_ACCESS_PLAN, "fastmem_access_plan");
        assert_eq!(TRUTH_SOURCE_STRING_KERNEL_PLAN, "string_kernel_plan");
        assert_eq!(TRUTH_SOURCE_GUI_DOMAIN, "gui_domain");
    }

    #[test]
    fn method_entry_rows_keep_call_truth_in_type_registry() {
        let rows = TYPEABI_VIEW_002_METHOD_ENTRY_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_VIEW_ADAPTER_COUNT], "1");
        assert_eq!(
            rows[TYPE_ABI_DOMAIN_CALL_TRUTH_SOURCE],
            TRUTH_SOURCE_TYPE_REGISTRY
        );
        assert_eq!(rows[TYPE_ABI_NEW_DUPLICATE_DESCRIPTOR_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_C_API_FUNCTION_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_HOT_LOOKUP_COUNT], "0");
    }

    #[test]
    fn in_memory_query_rows_do_not_generate_pack_or_c_api() {
        let rows = TYPEABI_VIEW_003_IN_MEMORY_QUERY_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_QUERY_PHASE], TYPE_ABI_QUERY_PHASE_PLANNING);
        assert_eq!(rows[TYPE_ABI_PACK_GENERATED_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_C_API_FUNCTION_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_QUERY_HOT_PATH_COUNT], "0");
    }
}
