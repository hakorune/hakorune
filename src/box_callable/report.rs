//! Box callable registry report vocabulary.

pub const BOX_CALLABLE_REGISTRY_ENABLED: &str = "box_callable_registry_enabled";
pub const BOX_CALLABLE_REGISTRY_TRUTH_OWNER: &str = "box_callable_registry_truth_owner";
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
pub const BOX_CALLABLE_PROVIDER_SOURCE_STORED: &str = "box_callable_provider_source_stored";
pub const STRING_SURFACE_CATALOG_PROVIDER_ROWS: &str = "string_surface_catalog_provider_rows";
pub const ARRAY_SURFACE_CATALOG_PROVIDER_ROWS: &str = "array_surface_catalog_provider_rows";
pub const MAP_SURFACE_CATALOG_PROVIDER_ROWS: &str = "map_surface_catalog_provider_rows";
pub const BUFFER_SURFACE_CATALOG_REQUIRED_BEFORE_PROVIDER_ROWS: &str =
    "buffer_surface_catalog_required_before_provider_rows";
pub const BUFFER_SURFACE_CATALOG_EXISTS: &str = "buffer_surface_catalog_exists";
pub const BUFFER_SURFACE_CATALOG_VISIBLE_METHODS_NAMED: &str =
    "buffer_surface_catalog_visible_methods_named";
pub const BUFFER_PROVIDER_ROWS_NOT_ADDED_BEFORE_CATALOG: &str =
    "buffer_provider_rows_not_added_before_catalog";
pub const BUFFER_STORAGE_MECHANICS_OWNER: &str = "buffer_storage_mechanics_owner";
pub const BUFFER_SURFACE_CATALOG_PROVIDER_ROWS: &str = "buffer_surface_catalog_provider_rows";
pub const BUFFER_VM_HANDLER_DISPATCH_OWNER: &str = "buffer_vm_handler_dispatch_owner";
pub const BUFFER_VISIBLE_SEMANTICS_CHANGED: &str = "buffer_visible_semantics_changed";
pub const TYPE_REGISTRY_EXECUTION_TRUTH_OWNER: &str = "type_registry_execution_truth_owner";
pub const TYPE_REGISTRY_SLOT_VOCABULARY_PROVIDER: &str = "type_registry_slot_vocabulary_provider";
pub const TYPE_REGISTRY_DISPATCH_BEHAVIOR_OWNER: &str = "type_registry_dispatch_behavior_owner";
pub const VM_DISPATCH_BY_SLOT_BEHAVIOR_OWNER: &str = "vm_dispatch_by_slot_behavior_owner";
pub const WASM_DISPATCH_BY_SLOT_BEHAVIOR_OWNER: &str = "wasm_dispatch_by_slot_behavior_owner";
pub const ROUTE_PLAN_USES_REGISTRY_ENTRY_TARGET: &str = "route_plan_uses_registry_entry_target";
pub const ROUTE_PLAN_USES_PROVIDER_SOURCE_AS_EXECUTION_ROUTE: &str =
    "route_plan_uses_provider_source_as_execution_route";
pub const ROUTE_PLAN_SEMANTIC_DATA_ONLY: &str = "route_plan_semantic_data_only";
pub const ROUTE_PLAN_EXECUTABLE_POINTER_COUNT: &str = "route_plan_executable_pointer_count";
pub const RUNTIME_INVOKE_BOUNDARY_EXECUTABLE_POINTER_OWNER: &str =
    "runtime_invoke_boundary_executable_pointer_owner";
pub const PROVIDER_EXECUTABLE_POINTER_COUNT: &str = "provider_executable_pointer_count";
pub const CATALOG_EXECUTABLE_POINTER_COUNT: &str = "catalog_executable_pointer_count";
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
pub const DUPLICATE_CALLABLE_TRUTH_COUNT: &str = "duplicate_callable_truth_count";
pub const PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY: &str = "plugin_loader_callable_provider_only";
pub const TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY: &str = "type_registry_callable_provider_only";
pub const TYPE_ABI_CATALOG_PROJECTION_ONLY: &str = "type_abi_catalog_projection_only";
pub const TYPEABI_CATALOG_EXECUTION_ROUTE_COUNT: &str = "typeabi_catalog_execution_route_count";
pub const METHOD_RESOLVER_DERIVES_FROM_ROUTE_PLAN_COUNT: &str =
    "method_resolver_derives_from_route_plan_count";
pub const SINGLETON_BIRTH_DERIVES_FROM_LIFECYCLE_PLAN_COUNT: &str =
    "singleton_birth_derives_from_lifecycle_plan_count";
pub const PLUGIN_LOADER_REGISTRY_SNAPSHOT_ENTRYPOINT_COUNT: &str =
    "plugin_loader_registry_snapshot_entrypoint_count";
pub const PLUGIN_LOADER_PROVIDER_SNAPSHOT_ONLY: &str = "plugin_loader_provider_snapshot_only";
pub const PLUGIN_CALLABLE_EXPORT_CONTAINS_FN_POINTER_COUNT: &str =
    "plugin_callable_export_contains_fn_pointer_count";
pub const TYPEBOX_ABI_V2_CHANGED: &str = "typebox_abi_v2_changed";
pub const PLUGIN_LIFECYCLE_SNAPSHOT_FILTERED_COUNT: &str =
    "plugin_lifecycle_snapshot_filtered_count";
pub const METHOD_PLAN_DIRECT_PROVIDER_SEED_COUNT: &str = "method_plan_direct_provider_seed_count";
pub const LIFECYCLE_PLAN_DIRECT_PROVIDER_SEED_COUNT: &str =
    "lifecycle_plan_direct_provider_seed_count";
pub const REGISTRY_SNAPSHOT_CACHE_REQUIRED_COUNT: &str = "registry_snapshot_cache_required_count";
pub const RUNTIME_INVOKE_BOUNDARY_MODULE_COUNT: &str = "runtime_invoke_boundary_module_count";
pub const ROUTE_RESOLVER_INVOKE_CONTRACT_COUNT: &str = "route_resolver_invoke_contract_count";
pub const RUNTIME_INVOKE_BOUNDARY_DERIVES_FN_POINTER_COUNT: &str =
    "runtime_invoke_boundary_derives_fn_pointer_count";
pub const CALLABLE_ROUTE_TRUTH_FROM_INVOKE_BOUNDARY_COUNT: &str =
    "callable_route_truth_from_invoke_boundary_count";
pub const RUNTIME_INVOKE_BOUNDARY_OWNS_METHOD_ID_COUNT: &str =
    "runtime_invoke_boundary_owns_method_id_count";
pub const RUNTIME_INVOKE_BOUNDARY_OWNS_LIFECYCLE_ID_COUNT: &str =
    "runtime_invoke_boundary_owns_lifecycle_id_count";
pub const RUNTIME_INVOKE_BOUNDARY_TYPEABI_LOOKUP_COUNT: &str =
    "runtime_invoke_boundary_typeabi_lookup_count";
pub const RUNTIME_INVOKE_BOUNDARY_FUNCTION_POINTER_BINDING_COUNT: &str =
    "runtime_invoke_boundary_function_pointer_binding_count";
pub const PLUGIN_CATALOG_PROJECTION_CHAIN_DOCUMENTED: &str =
    "plugin_catalog_projection_chain_documented";
pub const PLUGIN_LOADER_TO_TYPEABI_DIRECT_TRUTH_COUNT: &str =
    "plugin_loader_to_typeabi_direct_truth_count";
pub const TYPE_ABI_CATALOG_AS_PLUGIN_ROUTE_TRUTH_COUNT: &str =
    "type_abi_catalog_as_plugin_route_truth_count";
pub const PLUGIN_SNAPSHOT_CATALOG_PROJECTION_HELPER_COUNT: &str =
    "plugin_snapshot_catalog_projection_helper_count";
pub const PLUGIN_SNAPSHOT_CATALOG_READS_LOADER_DIRECTLY: &str =
    "plugin_snapshot_catalog_reads_loader_directly";
pub const PLUGIN_CATALOG_ROUTEPLAN_CONSUMER_COUNT: &str = "plugin_catalog_routeplan_consumer_count";
pub const PLUGIN_CATALOG_HOT_PATH_CONSUMER_COUNT: &str = "plugin_catalog_hot_path_consumer_count";
pub const PLUGIN_CATALOG_TOOLING_CONSUMER_COUNT: &str = "plugin_catalog_tooling_consumer_count";
pub const REGISTRY_SNAPSHOT_CACHE_DEFAULT_ENABLED: &str = "registry_snapshot_cache_default_enabled";
pub const PLUGIN_CATALOG_TOOLING_EXAMPLE_COUNT: &str = "plugin_catalog_tooling_example_count";
pub const PLUGIN_CATALOG_SAMPLE_ENTRY_COUNT: &str = "plugin_catalog_sample_entry_count";
pub const PLUGIN_CATALOG_SAMPLE_METHOD_ENTRY_COUNT: &str =
    "plugin_catalog_sample_method_entry_count";
pub const PLUGIN_CATALOG_SAMPLE_LIFECYCLE_ENTRY_COUNT: &str =
    "plugin_catalog_sample_lifecycle_entry_count";
pub const PLUGIN_CATALOG_SAMPLE_ROUTEPLAN_CONSUMER_COUNT: &str =
    "plugin_catalog_sample_routeplan_consumer_count";
pub const PLUGIN_CATALOG_SAMPLE_HOT_PATH_CONSUMER_COUNT: &str =
    "plugin_catalog_sample_hot_path_consumer_count";
pub const PLUGIN_CATALOG_SAMPLE_EXECUTES_PLUGIN_LOADER_COUNT: &str =
    "plugin_catalog_sample_executes_plugin_loader_count";
pub const BOXCALL_CONTRACT_SPLIT_REQUIRED_COUNT: &str = "boxcall_contract_split_required_count";
pub const BOXCALL_SAMPLE_SUBCOMMAND_REQUIRED_COUNT: &str =
    "boxcall_sample_subcommand_required_count";
pub const BOXCALL_CONTRACT_OPTIONAL_SAMPLE_FLAG_COUNT: &str =
    "boxcall_contract_optional_sample_flag_count";
pub const BOXCALL_FOUNDATION_CLOSEOUT_READY: &str = "boxcall_foundation_closeout_ready";
pub const PROVIDER_ROWS_COVER_BUILTIN_PLUGIN_SURFACE: &str =
    "provider_rows_cover_builtin_plugin_surface";
pub const BOXCALL_NEXT_LANE_REQUIRES_SELECTION: &str = "boxcall_next_lane_requires_selection";

pub const ID_SPACE_INTERNAL_VTABLE_SLOT: &str = "internal_vtable_slot";
pub const ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID: &str = "plugin_typebox_method_id";
pub const ID_SPACE_PLUGIN_LIFECYCLE_METHOD_ID: &str = "plugin_lifecycle_method_id";
pub const TRUTH_SOURCE_TYPE_REGISTRY: &str = "type_registry";
pub const TRUTH_SOURCE_SURFACE_CATALOG: &str = "surface_catalog";
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
    (BOX_CALLABLE_PROVIDER_SOURCE_STORED, "1"),
    (PLUGIN_METHOD_ID_SPACE, ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID),
    (LIFECYCLE_ID_SPACE, ID_SPACE_PLUGIN_LIFECYCLE_METHOD_ID),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_005_ROWS: &[(&str, &str)] = &[
    (METHOD_CALL_ROUTE_PLAN_EXISTS, "1"),
    (NEWBOX_ROUTE_PLAN_EXISTS, "1"),
    (DROPBOX_ROUTE_PLAN_EXISTS, "1"),
    (ROUTE_PLAN_SEMANTIC_DATA_ONLY, "1"),
    (ROUTE_PLAN_EXECUTABLE_POINTER_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_EXECUTABLE_POINTER_OWNER, "1"),
    (PROVIDER_EXECUTABLE_POINTER_COUNT, "0"),
    (CATALOG_EXECUTABLE_POINTER_COUNT, "0"),
    (ROUTE_PLAN_USES_REGISTRY_ENTRY_TARGET, "1"),
    (ROUTE_PLAN_USES_PROVIDER_SOURCE_AS_EXECUTION_ROUTE, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_CATALOG_001_ROWS: &[(&str, &str)] = &[
    (STRING_SURFACE_CATALOG_PROVIDER_ROWS, "1"),
    (ARRAY_SURFACE_CATALOG_PROVIDER_ROWS, "1"),
    (MAP_SURFACE_CATALOG_PROVIDER_ROWS, "1"),
    (BUFFER_SURFACE_CATALOG_REQUIRED_BEFORE_PROVIDER_ROWS, "1"),
    (BOX_CALLABLE_REGISTRY_ENABLED, "1"),
    (TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY, "1"),
    (TYPE_REGISTRY_EXECUTION_TRUTH_OWNER, "0"),
    (TYPE_ABI_CATALOG_PROJECTION_ONLY, "1"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BUFFER_CATALOG_001_ROWS: &[(&str, &str)] = &[
    (BUFFER_SURFACE_CATALOG_EXISTS, "1"),
    (BUFFER_SURFACE_CATALOG_VISIBLE_METHODS_NAMED, "1"),
    (BUFFER_PROVIDER_ROWS_NOT_ADDED_BEFORE_CATALOG, "1"),
    (BUFFER_STORAGE_MECHANICS_OWNER, "substrate"),
];

pub const BUFFER_PROVIDER_ROWS_001_ROWS: &[(&str, &str)] = &[
    (BUFFER_SURFACE_CATALOG_PROVIDER_ROWS, "1"),
    (BUFFER_VM_HANDLER_DISPATCH_OWNER, "1"),
    (BUFFER_VISIBLE_SEMANTICS_CHANGED, "0"),
    (TYPE_ABI_CATALOG_PROJECTION_ONLY, "1"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const TYPE_REGISTRY_PROVIDER_001_ROWS: &[(&str, &str)] = &[
    (TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY, "1"),
    (TYPE_REGISTRY_SLOT_VOCABULARY_PROVIDER, "1"),
    (TYPE_REGISTRY_EXECUTION_TRUTH_OWNER, "0"),
    (TYPE_REGISTRY_DISPATCH_BEHAVIOR_OWNER, "0"),
    (VM_DISPATCH_BY_SLOT_BEHAVIOR_OWNER, "1"),
    (WASM_DISPATCH_BY_SLOT_BEHAVIOR_OWNER, "1"),
    (BOX_CALLABLE_REGISTRY_ENABLED, "1"),
    (ROUTE_PLAN_SEMANTIC_DATA_ONLY, "1"),
];

pub const PLUGIN_PROVIDER_SNAPSHOT_001_ROWS: &[(&str, &str)] = &[
    (PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY, "1"),
    (PLUGIN_LOADER_PROVIDER_SNAPSHOT_ONLY, "1"),
    (PLUGIN_LOADER_REGISTRY_SNAPSHOT_ENTRYPOINT_COUNT, "1"),
    (PLUGIN_SNAPSHOT_CATALOG_PROJECTION_HELPER_COUNT, "1"),
    (PLUGIN_SNAPSHOT_CATALOG_READS_LOADER_DIRECTLY, "0"),
    (PLUGIN_CALLABLE_EXPORT_CONTAINS_FN_POINTER_COUNT, "0"),
    (TYPEBOX_ABI_V2_CHANGED, "0"),
    (PLUGIN_LIFECYCLE_SNAPSHOT_FILTERED_COUNT, "1"),
    (RUNTIME_INVOKE_BOUNDARY_EXECUTABLE_POINTER_OWNER, "1"),
];

pub const BOXCALL_FOUNDATION_CLOSEOUT_001_ROWS: &[(&str, &str)] = &[
    (BOXCALL_FOUNDATION_CLOSEOUT_READY, "1"),
    (BOX_CALLABLE_REGISTRY_ENABLED, "1"),
    (BOX_CALLABLE_REGISTRY_TRUTH_OWNER, "1"),
    (PROVIDER_ROWS_COVER_BUILTIN_PLUGIN_SURFACE, "1"),
    (ROUTE_PLAN_SEMANTIC_DATA_ONLY, "1"),
    (TYPEABI_CATALOG_EXECUTION_ROUTE_COUNT, "0"),
    (TYPE_ABI_CATALOG_PROJECTION_ONLY, "1"),
    (PLUGIN_LOADER_PROVIDER_SNAPSHOT_ONLY, "1"),
    (TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY, "1"),
    (BOXCALL_NEXT_LANE_REQUIRES_SELECTION, "1"),
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

pub const BOXCALL_008_ROWS: &[(&str, &str)] = &[
    (DUPLICATE_CALLABLE_TRUTH_COUNT, "0"),
    (PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY, "1"),
    (TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY, "1"),
    (TYPE_ABI_CATALOG_PROJECTION_ONLY, "1"),
    (METHOD_RESOLVER_DERIVES_FROM_ROUTE_PLAN_COUNT, "1"),
    (SINGLETON_BIRTH_DERIVES_FROM_LIFECYCLE_PLAN_COUNT, "1"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_009_ROWS: &[(&str, &str)] = &[
    (PLUGIN_LOADER_REGISTRY_SNAPSHOT_ENTRYPOINT_COUNT, "1"),
    (METHOD_PLAN_DIRECT_PROVIDER_SEED_COUNT, "0"),
    (LIFECYCLE_PLAN_DIRECT_PROVIDER_SEED_COUNT, "0"),
    (REGISTRY_SNAPSHOT_CACHE_REQUIRED_COUNT, "0"),
    (PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY, "1"),
    (DUPLICATE_CALLABLE_TRUTH_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const BOXCALL_010_ROWS: &[(&str, &str)] = &[
    (RUNTIME_INVOKE_BOUNDARY_MODULE_COUNT, "1"),
    (ROUTE_RESOLVER_INVOKE_CONTRACT_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_DERIVES_FN_POINTER_COUNT, "1"),
    (CALLABLE_ROUTE_TRUTH_FROM_INVOKE_BOUNDARY_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_OWNS_METHOD_ID_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_OWNS_LIFECYCLE_ID_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_TYPEABI_LOOKUP_COUNT, "0"),
    (RUNTIME_INVOKE_BOUNDARY_FUNCTION_POINTER_BINDING_COUNT, "1"),
    (DUPLICATE_CALLABLE_TRUTH_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const PLUGIN_CATALOG_000_005_ROWS: &[(&str, &str)] = &[
    (PLUGIN_CATALOG_PROJECTION_CHAIN_DOCUMENTED, "1"),
    (PLUGIN_LOADER_TO_TYPEABI_DIRECT_TRUTH_COUNT, "0"),
    (TYPE_ABI_CATALOG_AS_PLUGIN_ROUTE_TRUTH_COUNT, "0"),
    (PLUGIN_SNAPSHOT_CATALOG_PROJECTION_HELPER_COUNT, "1"),
    (PLUGIN_SNAPSHOT_CATALOG_READS_LOADER_DIRECTLY, "0"),
    (REGISTRY_SNAPSHOT_CACHE_REQUIRED_COUNT, "0"),
    (REGISTRY_SNAPSHOT_CACHE_DEFAULT_ENABLED, "0"),
    (PLUGIN_CATALOG_TOOLING_CONSUMER_COUNT, "1"),
    (PLUGIN_CATALOG_ROUTEPLAN_CONSUMER_COUNT, "0"),
    (PLUGIN_CATALOG_HOT_PATH_CONSUMER_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (ID_SPACE_MIXED_COUNT, "0"),
];

pub const PLUGIN_CATALOG_006_SAMPLE_ROWS: &[(&str, &str)] = &[
    (PLUGIN_CATALOG_TOOLING_EXAMPLE_COUNT, "1"),
    (PLUGIN_CATALOG_SAMPLE_ENTRY_COUNT, "3"),
    (PLUGIN_CATALOG_SAMPLE_METHOD_ENTRY_COUNT, "1"),
    (PLUGIN_CATALOG_SAMPLE_LIFECYCLE_ENTRY_COUNT, "2"),
    (PLUGIN_CATALOG_SAMPLE_ROUTEPLAN_CONSUMER_COUNT, "0"),
    (PLUGIN_CATALOG_SAMPLE_HOT_PATH_CONSUMER_COUNT, "0"),
    (PLUGIN_CATALOG_SAMPLE_EXECUTES_PLUGIN_LOADER_COUNT, "0"),
    (PLUGIN_LOADER_TO_TYPEABI_DIRECT_TRUTH_COUNT, "0"),
    (TYPE_ABI_CATALOG_AS_PLUGIN_ROUTE_TRUTH_COUNT, "0"),
    (ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT, "0"),
    (BOXCALL_CONTRACT_SPLIT_REQUIRED_COUNT, "0"),
    (BOXCALL_SAMPLE_SUBCOMMAND_REQUIRED_COUNT, "0"),
    (BOXCALL_CONTRACT_OPTIONAL_SAMPLE_FLAG_COUNT, "1"),
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

    #[test]
    fn boxcall_008_rows_retire_duplicate_callable_truth() {
        let rows = BOXCALL_008_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[DUPLICATE_CALLABLE_TRUTH_COUNT], "0");
        assert_eq!(rows[PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY], "1");
        assert_eq!(rows[TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY], "1");
        assert_eq!(rows[TYPE_ABI_CATALOG_PROJECTION_ONLY], "1");
        assert_eq!(rows[METHOD_RESOLVER_DERIVES_FROM_ROUTE_PLAN_COUNT], "1");
        assert_eq!(rows[SINGLETON_BIRTH_DERIVES_FROM_LIFECYCLE_PLAN_COUNT], "1");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_009_rows_keep_registry_snapshot_entrypoint_single() {
        let rows = BOXCALL_009_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[PLUGIN_LOADER_REGISTRY_SNAPSHOT_ENTRYPOINT_COUNT], "1");
        assert_eq!(rows[METHOD_PLAN_DIRECT_PROVIDER_SEED_COUNT], "0");
        assert_eq!(rows[LIFECYCLE_PLAN_DIRECT_PROVIDER_SEED_COUNT], "0");
        assert_eq!(rows[REGISTRY_SNAPSHOT_CACHE_REQUIRED_COUNT], "0");
        assert_eq!(rows[PLUGIN_LOADER_CALLABLE_PROVIDER_ONLY], "1");
        assert_eq!(rows[DUPLICATE_CALLABLE_TRUTH_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_010_rows_keep_invoke_boundary_out_of_route_truth() {
        let rows = BOXCALL_010_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[RUNTIME_INVOKE_BOUNDARY_MODULE_COUNT], "1");
        assert_eq!(rows[ROUTE_RESOLVER_INVOKE_CONTRACT_COUNT], "0");
        assert_eq!(rows[RUNTIME_INVOKE_BOUNDARY_DERIVES_FN_POINTER_COUNT], "1");
        assert_eq!(rows[CALLABLE_ROUTE_TRUTH_FROM_INVOKE_BOUNDARY_COUNT], "0");
        assert_eq!(rows[RUNTIME_INVOKE_BOUNDARY_OWNS_METHOD_ID_COUNT], "0");
        assert_eq!(rows[RUNTIME_INVOKE_BOUNDARY_OWNS_LIFECYCLE_ID_COUNT], "0");
        assert_eq!(rows[RUNTIME_INVOKE_BOUNDARY_TYPEABI_LOOKUP_COUNT], "0");
        assert_eq!(
            rows[RUNTIME_INVOKE_BOUNDARY_FUNCTION_POINTER_BINDING_COUNT],
            "1"
        );
        assert_eq!(rows[DUPLICATE_CALLABLE_TRUTH_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn boxcall_foundation_closeout_rows_match_taskboard_acceptance() {
        let rows = BOXCALL_FOUNDATION_CLOSEOUT_001_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[BOXCALL_FOUNDATION_CLOSEOUT_READY], "1");
        assert_eq!(rows[BOX_CALLABLE_REGISTRY_TRUTH_OWNER], "1");
        assert_eq!(rows[PROVIDER_ROWS_COVER_BUILTIN_PLUGIN_SURFACE], "1");
        assert_eq!(rows[ROUTE_PLAN_SEMANTIC_DATA_ONLY], "1");
        assert_eq!(rows[TYPEABI_CATALOG_EXECUTION_ROUTE_COUNT], "0");
        assert_eq!(rows[PLUGIN_LOADER_PROVIDER_SNAPSHOT_ONLY], "1");
        assert_eq!(rows[TYPE_REGISTRY_CALLABLE_PROVIDER_ONLY], "1");
        assert_eq!(rows[BOXCALL_NEXT_LANE_REQUIRES_SELECTION], "1");
    }

    #[test]
    fn plugin_catalog_rows_keep_projection_out_of_route_truth() {
        let rows = PLUGIN_CATALOG_000_005_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[PLUGIN_CATALOG_PROJECTION_CHAIN_DOCUMENTED], "1");
        assert_eq!(rows[PLUGIN_LOADER_TO_TYPEABI_DIRECT_TRUTH_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_CATALOG_AS_PLUGIN_ROUTE_TRUTH_COUNT], "0");
        assert_eq!(rows[PLUGIN_SNAPSHOT_CATALOG_PROJECTION_HELPER_COUNT], "1");
        assert_eq!(rows[PLUGIN_SNAPSHOT_CATALOG_READS_LOADER_DIRECTLY], "0");
        assert_eq!(rows[REGISTRY_SNAPSHOT_CACHE_REQUIRED_COUNT], "0");
        assert_eq!(rows[REGISTRY_SNAPSHOT_CACHE_DEFAULT_ENABLED], "0");
        assert_eq!(rows[PLUGIN_CATALOG_TOOLING_CONSUMER_COUNT], "1");
        assert_eq!(rows[PLUGIN_CATALOG_ROUTEPLAN_CONSUMER_COUNT], "0");
        assert_eq!(rows[PLUGIN_CATALOG_HOT_PATH_CONSUMER_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }

    #[test]
    fn plugin_catalog_006_rows_keep_sample_observation_only() {
        let rows = PLUGIN_CATALOG_006_SAMPLE_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[PLUGIN_CATALOG_TOOLING_EXAMPLE_COUNT], "1");
        assert_eq!(rows[PLUGIN_CATALOG_SAMPLE_ENTRY_COUNT], "3");
        assert_eq!(rows[PLUGIN_CATALOG_SAMPLE_METHOD_ENTRY_COUNT], "1");
        assert_eq!(rows[PLUGIN_CATALOG_SAMPLE_LIFECYCLE_ENTRY_COUNT], "2");
        assert_eq!(rows[PLUGIN_CATALOG_SAMPLE_ROUTEPLAN_CONSUMER_COUNT], "0");
        assert_eq!(rows[PLUGIN_CATALOG_SAMPLE_HOT_PATH_CONSUMER_COUNT], "0");
        assert_eq!(
            rows[PLUGIN_CATALOG_SAMPLE_EXECUTES_PLUGIN_LOADER_COUNT],
            "0"
        );
        assert_eq!(rows[PLUGIN_LOADER_TO_TYPEABI_DIRECT_TRUTH_COUNT], "0");
        assert_eq!(rows[TYPE_ABI_CATALOG_AS_PLUGIN_ROUTE_TRUTH_COUNT], "0");
        assert_eq!(rows[ROUTE_PLAN_TYPE_ABI_HOT_LOOKUP_COUNT], "0");
        assert_eq!(rows[BOXCALL_CONTRACT_SPLIT_REQUIRED_COUNT], "0");
        assert_eq!(rows[BOXCALL_SAMPLE_SUBCOMMAND_REQUIRED_COUNT], "0");
        assert_eq!(rows[BOXCALL_CONTRACT_OPTIONAL_SAMPLE_FLAG_COUNT], "1");
    }
}
