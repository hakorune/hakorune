//! BoxDescriptor compatibility aliases.
//!
//! The current implementation still lives under `type_abi` for compatibility.
//! These aliases expose the intended descriptor naming without moving files.

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
    use crate::runtime::type_box_abi::MethodEntry;
    use crate::type_abi::box_descriptor::{
        build_box_descriptor_pack, BoxDescriptorCatalog, BoxDescriptorPack, BoxDescriptorTag,
        BoxDescriptorView,
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
}
