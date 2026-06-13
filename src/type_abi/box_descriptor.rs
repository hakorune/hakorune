//! BoxDescriptor compatibility aliases.
//!
//! The current implementation still lives under `type_abi` for compatibility.
//! These aliases expose the intended descriptor naming without moving files.

pub use super::box_callable::{
    build_box_callable_registry_pack as build_box_descriptor_callable_pack,
    build_catalog_from_box_callable_registry_snapshot as build_box_descriptor_catalog_from_callable_registry,
    publish_box_callable_registry as publish_box_callable_descriptors,
    BoxCallableEntryView as BoxDescriptorBoxCallableView,
    TYPE_ABI_BOX_CALLABLE_SCHEMA_V0 as BOX_DESCRIPTOR_BOX_CALLABLE_SCHEMA_V0,
};
pub use super::catalog::TypeAbiCatalog as BoxDescriptorCatalog;
pub use super::pack::{
    build_type_abi_pack as build_box_descriptor_pack, TypeAbiPack as BoxDescriptorPack,
};
pub use super::{
    TypeAbiEntryHeader as BoxDescriptorEntryHeader, TypeAbiError as BoxDescriptorError,
    TypeAbiPayloadSink as BoxDescriptorPayloadSink, TypeAbiTag as BoxDescriptorTag,
    TypeAbiView as BoxDescriptorView,
};

#[cfg(test)]
mod tests {
    use crate::box_callable::{
        BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget,
    };
    use crate::runtime::type_box_abi::MethodEntry;
    use crate::type_abi::box_descriptor::{
        build_box_descriptor_callable_pack, build_box_descriptor_catalog_from_callable_registry,
        build_box_descriptor_pack, publish_box_callable_descriptors, BoxDescriptorBoxCallableView,
        BoxDescriptorCatalog, BoxDescriptorPack, BoxDescriptorTag, BoxDescriptorView,
        BOX_DESCRIPTOR_BOX_CALLABLE_SCHEMA_V0,
    };

    #[test]
    fn descriptor_aliases_preserve_existing_type_abi_behavior() {
        let entry = MethodEntry {
            name: "len",
            arity: 0,
            slot: 200,
        };
        let mut catalog = BoxDescriptorCatalog::new();

        catalog.publish(&entry);

        let got = catalog
            .get_by_tag_name(BoxDescriptorTag::Method, "len")
            .unwrap();
        assert_eq!(got.id, 200);

        let pack: BoxDescriptorPack =
            build_box_descriptor_pack(&[&entry as &dyn BoxDescriptorView]).unwrap();
        assert_eq!(pack.entry_count(), 1);
    }

    #[test]
    fn descriptor_aliases_cover_box_callable_projection() {
        let mut registry = BoxCallableRegistry::new();
        let key = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 2);
        let target = BoxCallableTarget::PluginMethod {
            type_id: 42,
            method_id: 7,
            returns_result: true,
        };
        let view = BoxDescriptorBoxCallableView::new(&key, &target);
        assert_eq!(view.payload_schema(), BOX_DESCRIPTOR_BOX_CALLABLE_SCHEMA_V0);

        registry.insert(key, target);
        let mut catalog = BoxDescriptorCatalog::new();
        assert_eq!(publish_box_callable_descriptors(&mut catalog, &registry), 1);
        assert_eq!(catalog.query_by_tag(BoxDescriptorTag::BoxCallable).len(), 1);

        let catalog = build_box_descriptor_catalog_from_callable_registry(&registry);
        assert_eq!(catalog.query_by_tag(BoxDescriptorTag::BoxCallable).len(), 1);

        let pack: BoxDescriptorPack = build_box_descriptor_callable_pack(&registry).unwrap();
        assert_eq!(pack.entry_count(), 1);
    }
}
