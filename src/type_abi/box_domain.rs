//! Box Domain report vocabulary.
//!
//! Box Domain groups method slots, plugin method routes, lifecycle routes, and
//! NewBox/DropBox plan boundaries without merging their truth sources.

pub const BOX_DOMAIN_ENABLED: &str = "box_domain_enabled";
pub const METHOD_SLOT_TRUTH_SOURCE: &str = "method_slot_truth_source";
pub const PLUGIN_METHOD_ROUTE_TRUTH_SOURCE: &str = "plugin_method_route_truth_source";
pub const LIFECYCLE_ROUTE_TRUTH_SOURCE: &str = "lifecycle_route_truth_source";
pub const INVOKE_ROUTE_TRUTH_SOURCE: &str = "invoke_route_truth_source";
pub const METHOD_SLOT_ID_SPACE: &str = "method_slot_id_space";
pub const PLUGIN_METHOD_ID_SPACE: &str = "plugin_method_id_space";
pub const ID_SPACE_MIXED_COUNT: &str = "id_space_mixed_count";

pub const TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER: &str = "plugin_loader_route_resolver";
pub const ID_SPACE_INTERNAL_VTABLE_SLOT: &str = "internal_vtable_slot";
pub const ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID: &str = "plugin_typebox_method_id";

pub const TYPEABI_BOXDOMAIN_001_ROWS: &[(&str, &str)] = &[
    (BOX_DOMAIN_ENABLED, "1"),
    (
        METHOD_SLOT_TRUTH_SOURCE,
        crate::type_abi::report::TRUTH_SOURCE_TYPE_REGISTRY,
    ),
    (
        PLUGIN_METHOD_ROUTE_TRUTH_SOURCE,
        TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER,
    ),
    (
        LIFECYCLE_ROUTE_TRUTH_SOURCE,
        TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER,
    ),
    (
        INVOKE_ROUTE_TRUTH_SOURCE,
        TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER,
    ),
    (METHOD_SLOT_ID_SPACE, ID_SPACE_INTERNAL_VTABLE_SLOT),
    (PLUGIN_METHOD_ID_SPACE, ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID),
    (ID_SPACE_MIXED_COUNT, "0"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_domain_rows_keep_truth_sources_and_id_spaces_separate() {
        let rows = TYPEABI_BOXDOMAIN_001_ROWS
            .iter()
            .copied()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(rows[BOX_DOMAIN_ENABLED], "1");
        assert_eq!(
            rows[METHOD_SLOT_TRUTH_SOURCE],
            crate::type_abi::report::TRUTH_SOURCE_TYPE_REGISTRY
        );
        assert_eq!(
            rows[PLUGIN_METHOD_ROUTE_TRUTH_SOURCE],
            TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER
        );
        assert_eq!(
            rows[LIFECYCLE_ROUTE_TRUTH_SOURCE],
            TRUTH_SOURCE_PLUGIN_LOADER_ROUTE_RESOLVER
        );
        assert_eq!(rows[METHOD_SLOT_ID_SPACE], ID_SPACE_INTERNAL_VTABLE_SLOT);
        assert_eq!(
            rows[PLUGIN_METHOD_ID_SPACE],
            ID_SPACE_PLUGIN_TYPEBOX_METHOD_ID
        );
        assert_eq!(rows[ID_SPACE_MIXED_COUNT], "0");
    }
}
