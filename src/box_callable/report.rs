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

pub const ID_SPACE_INTERNAL_VTABLE_SLOT: &str = "internal_vtable_slot";
pub const ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID: &str = "plugin_typebox_method_id";

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
}
