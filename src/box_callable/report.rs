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
pub const BOX_CALLABLE_BUILTIN_SEED_SOURCE: &str = "box_callable_builtin_seed_source";

pub const ID_SPACE_INTERNAL_VTABLE_SLOT: &str = "internal_vtable_slot";
pub const ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID: &str = "plugin_typebox_method_id";
pub const TRUTH_SOURCE_TYPE_REGISTRY: &str = "type_registry";

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
}
