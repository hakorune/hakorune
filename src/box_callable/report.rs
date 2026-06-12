//! Box callable registry report vocabulary.

pub const BOX_CALLABLE_REGISTRY_ENABLED: &str = "box_callable_registry_enabled";
pub const BOX_CALLABLE_COMMON_KEY_ENABLED: &str = "box_callable_common_key_enabled";
pub const METHOD_SLOT_ID_SPACE: &str = "method_slot_id_space";
pub const PLUGIN_METHOD_ID_SPACE: &str = "plugin_method_id_space";
pub const LIFECYCLE_ID_SPACE: &str = "lifecycle_id_space";
pub const ID_SPACE_MIXED_COUNT: &str = "id_space_mixed_count";
pub const SLOT_COMPARED_TO_METHOD_ID_COUNT: &str = "slot_compared_to_method_id_count";
pub const PLUGIN_METHOD_ID_USED_AS_INTERNAL_SLOT_COUNT: &str =
    "plugin_method_id_used_as_internal_slot_count";
pub const INTERNAL_SLOT_USED_AS_PLUGIN_METHOD_ID_COUNT: &str =
    "internal_slot_used_as_plugin_method_id_count";
pub const BOX_CALLABLE_TRUTH_SOURCE_INTERNAL_SLOT: &str =
    "box_callable_truth_source[internal_slot]";
pub const BOX_CALLABLE_TRUTH_SOURCE_PLUGIN_METHOD: &str =
    "box_callable_truth_source[plugin_method]";
pub const BOX_CALLABLE_TRUTH_SOURCE_LIFECYCLE: &str = "box_callable_truth_source[lifecycle]";
pub const BOX_CALLABLE_BUILTIN_SEED_SOURCE: &str = "box_callable_builtin_seed_source";
pub const BOX_CALLABLE_PLUGIN_SEED_SOURCE: &str = "box_callable_plugin_seed_source";
pub const METHOD_CALL_ROUTE_PLAN_EXISTS: &str = "method_call_route_plan_exists";
pub const NEWBOX_ROUTE_PLAN_EXISTS: &str = "newbox_route_plan_exists";
pub const DROPBOX_ROUTE_PLAN_EXISTS: &str = "dropbox_route_plan_exists";
pub const ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT: &str = "route_plan_type_abi_hot_lookup_count";
pub const NEWBOX_PLAN_DERIVES_FROM_REGISTRY_COUNT: &str = "newbox_plan_derives_from_registry_count";
pub const DROPBOX_PLAN_DERIVES_FROM_HANDLE_ROUTE_COUNT: &str =
    "dropbox_plan_derives_from_handle_route_count";
pub const LIFECYCLE_SELECTED_ROUTE_RERESOLVE_HOT_COUNT: &str =
    "lifecycle_selected_route_reresolve_hot_count";
pub const FALLBACK_AFTER_SELECTED_LIFECYCLE_ROUTE_COUNT: &str =
    "fallback_after_selected_lifecycle_route_count";
pub const METHOD_CALL_PLAN_DERIVES_FROM_REGISTRY_COUNT: &str =
    "method_call_plan_derives_from_registry_count";
pub const SLOW_DYNAMIC_ROUTE_EXPLICIT_COUNT: &str = "slow_dynamic_route_explicit_count";
pub const FALLBACK_AFTER_SELECTED_METHOD_ROUTE_COUNT: &str =
    "fallback_after_selected_method_route_count";

pub const ID_SPACE_INTERNAL_VTABLE_SLOT: &str = "internal_vtable_slot";
pub const ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID: &str = "plugin_typebox_method_id";
pub const ID_SPACE_PLUGIN_LIFECYCLE_METHOD_ID: &str = "plugin_lifecycle_method_id";
pub const TRUTH_SOURCE_TYPE_REGISTRY: &str = "type_registry";
pub const TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER: &str = "plugin_loader_provider";

pub const BOXCALL_001_ROWS: &[(&str, &str)] = &[
    (BOX_CALLABLE_REGISTRY_ENABLED, "1"),
    (BOX_CALLABLE_COMMON_KEY_ENABLED, "1"),
    (METHOD_SLOT_ID_SPACE, ID_SPACE_INTERNAL_VTABLE_SLOT),
    (PLUGIN_METHOD_ID_SPACE, ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID),
    (LIFECYCLE_ID_SPACE, "plugin_lifecycle_method_id"),
    (ID_SPACE_MIXED_COUNT, "0"),
    (SLOT_COMPARED_TO_METHOD_ID_COUNT, "0"),
    (PLUGIN_METHOD_ID_USED_AS_INTERNAL_SLOT_COUNT, "0"),
    (INTERNAL_SLOT_USED_AS_PLUGIN_METHOD_ID_COUNT, "0"),
];

pub const BOXCALL_002_ROWS: &[(&str, &str)] = &[
    (
        BOX_CALLABLE_TRUTH_SOURCE_INTERNAL_SLOT,
        TRUTH_SOURCE_TYPE_REGISTRY,
    ),
    (BOX_CALLABLE_BUILTIN_SEED_SOURCE, TRUTH_SOURCE_TYPE_REGISTRY),
    (METHOD_SLOT_ID_SPACE, ID_SPACE_INTERNAL_VTABLE_SLOT),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_003_ROWS: &[(&str, &str)] = &[
    (
        BOX_CALLABLE_TRUTH_SOURCE_PLUGIN_METHOD,
        TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER,
    ),
    (
        BOX_CALLABLE_TRUTH_SOURCE_LIFECYCLE,
        TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER,
    ),
    (
        BOX_CALLABLE_PLUGIN_SEED_SOURCE,
        TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER,
    ),
    (PLUGIN_METHOD_ID_SPACE, ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID),
    (LIFECYCLE_ID_SPACE, ID_SPACE_PLUGIN_LIFECYCLE_METHOD_ID),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_005_ROWS: &[(&str, &str)] = &[
    (METHOD_CALL_ROUTE_PLAN_EXISTS, "1"),
    (NEWBOX_ROUTE_PLAN_EXISTS, "1"),
    (DROPBOX_ROUTE_PLAN_EXISTS, "1"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_006_ROWS: &[(&str, &str)] = &[
    (NEWBOX_PLAN_DERIVES_FROM_REGISTRY_COUNT, "1"),
    (DROPBOX_PLAN_DERIVES_FROM_HANDLE_ROUTE_COUNT, "1"),
    (LIFECYCLE_SELECTED_ROUTE_RERESOLVE_HOT_COUNT, "0"),
    (FALLBACK_AFTER_SELECTED_LIFECYCLE_ROUTE_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
];

pub const BOXCALL_007_ROWS: &[(&str, &str)] = &[
    (METHOD_CALL_PLAN_DERIVES_FROM_REGISTRY_COUNT, "1"),
    (PLUGIN_METHOD_ID_SPACE, ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID),
    (METHOD_SLOT_ID_SPACE, ID_SPACE_INTERNAL_VTABLE_SLOT),
    (SLOW_DYNAMIC_ROUTE_EXPLICIT_COUNT, "1"),
    (FALLBACK_AFTER_SELECTED_METHOD_ROUTE_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxcall_rows_enable_registry_without_mixing_id_spaces() {
        let rows = BOXCALL_001_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[BOX_CALLABLE_REGISTRY_ENABLED], "1");
        assert_eq!(rows[BOX_CALLABLE_COMMON_KEY_ENABLED], "1");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
        assert_eq!(rows[SLOT_COMPARED_TO_METHOD_ID_COUNT], "0");
        assert_eq!(rows[PLUGIN_METHOD_ID_USED_AS_INTERNAL_SLOT_COUNT], "0");
        assert_eq!(rows[INTERNAL_SLOT_USED_AS_PLUGIN_METHOD_ID_COUNT], "0");
    }

    #[test]
    fn boxcall_002_rows_mark_type_registry_as_internal_slot_seed_source() {
        let rows = BOXCALL_002_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            rows[BOX_CALLABLE_TRUTH_SOURCE_INTERNAL_SLOT],
            TRUTH_SOURCE_TYPE_REGISTRY
        );
        assert_eq!(rows[BOX_CALLABLE_BUILTIN_SEED_SOURCE], "type_registry");
        assert_eq!(rows[METHOD_SLOT_ID_SPACE], ID_SPACE_INTERNAL_VTABLE_SLOT);
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_003_rows_mark_plugin_loader_as_plugin_seed_source() {
        let rows = BOXCALL_003_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            rows[BOX_CALLABLE_TRUTH_SOURCE_PLUGIN_METHOD],
            TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER
        );
        assert_eq!(
            rows[BOX_CALLABLE_TRUTH_SOURCE_LIFECYCLE],
            TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER
        );
        assert_eq!(
            rows[BOX_CALLABLE_PLUGIN_SEED_SOURCE],
            TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER
        );
        assert_eq!(
            rows[PLUGIN_METHOD_ID_SPACE],
            ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID
        );
        assert_eq!(
            rows[LIFECYCLE_ID_SPACE],
            ID_SPACE_PLUGIN_LIFECYCLE_METHOD_ID
        );
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_005_rows_enable_route_plan_vocabulary_without_type_abi_hot_lookup() {
        let rows = BOXCALL_005_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[METHOD_CALL_ROUTE_PLAN_EXISTS], "1");
        assert_eq!(rows[NEWBOX_ROUTE_PLAN_EXISTS], "1");
        assert_eq!(rows[DROPBOX_ROUTE_PLAN_EXISTS], "1");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_006_rows_keep_lifecycle_execution_on_route_plan_seam() {
        let rows = BOXCALL_006_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[NEWBOX_PLAN_DERIVES_FROM_REGISTRY_COUNT], "1");
        assert_eq!(rows[DROPBOX_PLAN_DERIVES_FROM_HANDLE_ROUTE_COUNT], "1");
        assert_eq!(rows[LIFECYCLE_SELECTED_ROUTE_RERESOLVE_HOT_COUNT], "0");
        assert_eq!(rows[FALLBACK_AFTER_SELECTED_LIFECYCLE_ROUTE_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
    }

    #[test]
    fn boxcall_007_rows_keep_method_calls_on_registry_route_plan_seam() {
        let rows = BOXCALL_007_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[METHOD_CALL_PLAN_DERIVES_FROM_REGISTRY_COUNT], "1");
        assert_eq!(
            rows[PLUGIN_METHOD_ID_SPACE],
            ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID
        );
        assert_eq!(rows[METHOD_SLOT_ID_SPACE], ID_SPACE_INTERNAL_VTABLE_SLOT);
        assert_eq!(rows[SLOW_DYNAMIC_ROUTE_EXPLICIT_COUNT], "1");
        assert_eq!(rows[FALLBACK_AFTER_SELECTED_METHOD_ROUTE_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }
}
