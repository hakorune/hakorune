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
pub const TYPE_ABI_CATALOG_ENABLED: &str = "type_abi_catalog_enabled";
pub const TYPE_ABI_CATALOG_IS_TRUTH: &str = "type_abi_catalog_is_truth";
pub const TYPE_ABI_EXISTING_REFRESH_PRESERVED: &str = "type_abi_existing_refresh_preserved";
pub const TYPE_ABI_REFRESH_TRUTH_TRAIT_ENABLED: &str = "type_abi_refresh_truth_trait_enabled";
pub const TYPE_ABI_CATALOG_ENTRY_COUNT: &str = "type_abi_catalog_entry_count";
pub const TYPE_ABI_CATALOG_QUERY_COUNT: &str = "type_abi_catalog_query_count";
pub const TYPE_ABI_CATALOG_CROSS_DOMAIN_QUERY_COUNT: &str =
    "type_abi_catalog_cross_domain_query_count";
pub const TYPE_ABI_CATALOG_HOT_LOOKUP_COUNT: &str = "type_abi_catalog_hot_lookup_count";
pub const TYPE_ABI_PACK_FROM_CATALOG_COUNT: &str = "type_abi_pack_from_catalog_count";
pub const TYPE_ABI_PACK_USED_BY_PLANNER_COUNT: &str = "type_abi_pack_used_by_planner_count";
pub const DOMAIN_PLANNER_OWN_TRUTH_READ_COUNT: &str = "domain_planner_own_truth_read_count";
pub const DOMAIN_PLANNER_CATALOG_QUERY_COUNT: &str = "domain_planner_catalog_query_count";
pub const GENERIC_TYPEABI_GENERATE_PLANS_COUNT: &str = "generic_typeabi_generate_plans_count";
pub const BOX_DESCRIPTOR_MODE: &str = "box_descriptor_mode";
pub const BOX_DESCRIPTOR_MODE_PROJECTION_OVER_BOX_CALLABLE_REGISTRY: &str =
    "projection_over_box_callable_registry";
pub const BOX_DESCRIPTOR_PACK_IS_TRUTH: &str = "box_descriptor_pack_is_truth";
pub const BOX_DESCRIPTOR_HOT_LOOKUP_COUNT: &str = "box_descriptor_hot_lookup_count";
pub const BOX_DESCRIPTOR_CATALOG_IS_EXECUTION_REGISTRY: &str =
    "box_descriptor_catalog_is_execution_registry";

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

pub const TYPEABI_PACK_000_ROWS: &[(&str, &str)] = &[
    (TYPE_ABI_PACK_IS_TRUTH, "0"),
    (TYPE_ABI_PACK_GENERATED_COUNT, "1"),
    (TYPE_ABI_PACK_SCHEMA_VERSION, "1"),
    (TYPE_ABI_C_API_FUNCTION_COUNT, "0"),
    (TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (TYPE_ABI_QUERY_HOT_PATH_COUNT, "0"),
];

pub const TYPEABI_CATALOG_001_ROWS: &[(&str, &str)] = &[
    (TYPE_ABI_CATALOG_ENABLED, "1"),
    (TYPE_ABI_CATALOG_IS_TRUTH, "0"),
    (TYPE_ABI_CATALOG_HOT_LOOKUP_COUNT, "0"),
    (TYPE_ABI_PACK_USED_BY_PLANNER_COUNT, "0"),
    (GENERIC_TYPEABI_GENERATE_PLANS_COUNT, "0"),
];

pub const TYPEABI_CATALOG_CLEAN_000_ROWS: &[(&str, &str)] = &[
    (TYPE_ABI_CATALOG_ENABLED, "1"),
    (TYPE_ABI_CATALOG_IS_TRUTH, "0"),
    (TYPE_ABI_EXISTING_REFRESH_PRESERVED, "1"),
    (TYPE_ABI_REFRESH_TRUTH_TRAIT_ENABLED, "0"),
    (TYPE_ABI_PACK_USED_BY_PLANNER_COUNT, "0"),
    (GENERIC_TYPEABI_GENERATE_PLANS_COUNT, "0"),
    (TYPE_ABI_CATALOG_HOT_LOOKUP_COUNT, "0"),
];

pub const TYPEABI_NAMING_001_ROWS: &[(&str, &str)] = &[
    (
        BOX_DESCRIPTOR_MODE,
        BOX_DESCRIPTOR_MODE_PROJECTION_OVER_BOX_CALLABLE_REGISTRY,
    ),
    (TYPE_ABI_MODE, TYPE_ABI_MODE_VIEW_OVER_EXISTING_TRUTH),
    (BOX_DESCRIPTOR_PACK_IS_TRUTH, "0"),
    (BOX_DESCRIPTOR_HOT_LOOKUP_COUNT, "0"),
    (BOX_DESCRIPTOR_CATALOG_IS_EXECUTION_REGISTRY, "0"),
];

pub const BOXCALL_004_ROWS: &[(&str, &str)] = &[
    (
        BOX_DESCRIPTOR_MODE,
        BOX_DESCRIPTOR_MODE_PROJECTION_OVER_BOX_CALLABLE_REGISTRY,
    ),
    (TYPE_ABI_PACK_IS_TRUTH, "0"),
    (TYPE_ABI_PACK_USED_BY_PLANNER_COUNT, "0"),
    (TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (BOX_DESCRIPTOR_HOT_LOOKUP_COUNT, "0"),
    (BOX_DESCRIPTOR_CATALOG_IS_EXECUTION_REGISTRY, "0"),
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

    #[test]
    fn pack_rows_keep_snapshot_out_of_hot_path() {
        let rows = TYPEABI_PACK_000_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_PACK_IS_TRUTH], "0");
        assert_eq!(rows[TYPE_ABI_PACK_GENERATED_COUNT], "1");
        assert_eq!(rows[TYPE_ABI_PACK_SCHEMA_VERSION], "1");
        assert_eq!(rows[TYPE_ABI_C_API_FUNCTION_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_HOT_LOOKUP_COUNT], "0");
    }

    #[test]
    fn catalog_rows_keep_catalog_thin_and_out_of_hot_path() {
        let rows = TYPEABI_CATALOG_001_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_CATALOG_ENABLED], "1");
        assert_eq!(rows[TYPE_ABI_CATALOG_IS_TRUTH], "0");
        assert_eq!(rows[TYPE_ABI_CATALOG_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_PACK_USED_BY_PLANNER_COUNT], "0");
        assert_eq!(rows[GENERIC_TYPEABI_GENERATE_PLANS_COUNT], "0");
    }

    #[test]
    fn catalog_clean_rows_keep_refresh_outside_type_abi_domain() {
        let rows = TYPEABI_CATALOG_CLEAN_000_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[TYPE_ABI_CATALOG_ENABLED], "1");
        assert_eq!(rows[TYPE_ABI_CATALOG_IS_TRUTH], "0");
        assert_eq!(rows[TYPE_ABI_EXISTING_REFRESH_PRESERVED], "1");
        assert_eq!(rows[TYPE_ABI_REFRESH_TRUTH_TRAIT_ENABLED], "0");
        assert_eq!(rows[TYPE_ABI_PACK_USED_BY_PLANNER_COUNT], "0");
        assert_eq!(rows[GENERIC_TYPEABI_GENERATE_PLANS_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_CATALOG_HOT_LOOKUP_COUNT], "0");
    }

    #[test]
    fn box_descriptor_rows_prefer_new_projection_names() {
        let rows = TYPEABI_NAMING_001_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            rows[BOX_DESCRIPTOR_MODE],
            BOX_DESCRIPTOR_MODE_PROJECTION_OVER_BOX_CALLABLE_REGISTRY
        );
        assert_eq!(rows[TYPE_ABI_MODE], TYPE_ABI_MODE_VIEW_OVER_EXISTING_TRUTH);
        assert_eq!(rows[BOX_DESCRIPTOR_PACK_IS_TRUTH], "0");
        assert_eq!(rows[BOX_DESCRIPTOR_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[BOX_DESCRIPTOR_CATALOG_IS_EXECUTION_REGISTRY], "0");
    }

    #[test]
    fn boxcall_004_rows_keep_registry_projection_cold() {
        let rows = BOXCALL_004_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            rows[BOX_DESCRIPTOR_MODE],
            BOX_DESCRIPTOR_MODE_PROJECTION_OVER_BOX_CALLABLE_REGISTRY
        );
        assert_eq!(rows[TYPE_ABI_PACK_IS_TRUTH], "0");
        assert_eq!(rows[TYPE_ABI_PACK_USED_BY_PLANNER_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[BOX_DESCRIPTOR_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[BOX_DESCRIPTOR_CATALOG_IS_EXECUTION_REGISTRY], "0");
    }
}
