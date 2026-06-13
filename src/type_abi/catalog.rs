//! Thin in-memory Type ABI catalog.
//!
//! The catalog stores entry headers and indexes for cross-domain planning
//! queries. It is not semantic truth and does not depend on TypeAbiPack.

use std::collections::HashMap;

use super::{TypeAbiEntryHeader, TypeAbiTag, TypeAbiView};

/// Builder for a catalog assembled after existing domain refresh has run.
///
/// The builder is intentionally small: it only publishes read-only views into
/// the catalog. It does not own MIR metadata refresh, canonicalization, or plan
/// generation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TypeAbiCatalogBuilder {
    catalog: TypeAbiCatalog,
}

impl TypeAbiCatalogBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish<V: TypeAbiView + ?Sized>(&mut self, view: &V) -> usize {
        self.catalog.publish(view)
    }

    pub fn publish_header(&mut self, header: TypeAbiEntryHeader) -> usize {
        self.catalog.publish_header(header)
    }

    pub fn finish(self) -> TypeAbiCatalog {
        self.catalog
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TypeAbiCatalog {
    entries: Vec<TypeAbiEntryHeader>,
    by_tag_id: HashMap<(TypeAbiTag, u32), usize>,
    by_tag_name: HashMap<(TypeAbiTag, String), usize>,
}

impl TypeAbiCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder_from_refreshed_world() -> TypeAbiCatalogBuilder {
        TypeAbiCatalogBuilder::new()
    }

    pub fn from_refreshed_views(views: &[&dyn TypeAbiView]) -> Self {
        let mut builder = Self::builder_from_refreshed_world();
        for view in views {
            builder.publish(*view);
        }
        builder.finish()
    }

    pub fn publish<V: TypeAbiView + ?Sized>(&mut self, view: &V) -> usize {
        self.publish_header(TypeAbiEntryHeader::from_view(view))
    }

    pub fn publish_header(&mut self, header: TypeAbiEntryHeader) -> usize {
        let index = self.entries.len();

        self.by_tag_id.insert((header.tag, header.id), index);
        if let Some(name) = header.name.as_ref() {
            self.by_tag_name.insert((header.tag, name.clone()), index);
        }
        self.entries.push(header);

        index
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[TypeAbiEntryHeader] {
        &self.entries
    }

    pub fn query_by_tag(&self, tag: TypeAbiTag) -> Vec<&TypeAbiEntryHeader> {
        self.entries
            .iter()
            .filter(|entry| entry.tag == tag)
            .collect()
    }

    pub fn get_by_tag_id(&self, tag: TypeAbiTag, id: u32) -> Option<&TypeAbiEntryHeader> {
        self.by_tag_id
            .get(&(tag, id))
            .and_then(|index| self.entries.get(*index))
    }

    pub fn get_by_tag_name(&self, tag: TypeAbiTag, name: &str) -> Option<&TypeAbiEntryHeader> {
        self.by_tag_name
            .get(&(tag, name.to_string()))
            .and_then(|index| self.entries.get(*index))
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::type_box_abi::MethodEntry;

    use super::*;
    use crate::type_abi::method_entry::TYPE_ABI_METHOD_ENTRY_SCHEMA_V0;

    #[test]
    fn catalog_stores_headers_and_indexes_without_pack() {
        let entry = MethodEntry {
            name: "len",
            arity: 0,
            slot: 200,
        };
        let mut catalog = TypeAbiCatalog::new();

        let index = catalog.publish(&entry);

        assert_eq!(index, 0);
        assert_eq!(catalog.len(), 1);
        assert!(!catalog.is_empty());

        let by_id = catalog.get_by_tag_id(TypeAbiTag::Method, 200).unwrap();
        assert_eq!(by_id.name.as_deref(), Some("len"));
        assert_eq!(by_id.payload_schema, TYPE_ABI_METHOD_ENTRY_SCHEMA_V0);

        let by_name = catalog.get_by_tag_name(TypeAbiTag::Method, "len").unwrap();
        assert_eq!(by_name.id, 200);
    }

    #[test]
    fn catalog_query_by_tag_returns_only_matching_headers() {
        let method = TypeAbiEntryHeader {
            tag: TypeAbiTag::Method,
            id: 1,
            name: Some("m".to_string()),
            payload_schema: 1,
        };
        let field = TypeAbiEntryHeader {
            tag: TypeAbiTag::Field,
            id: 2,
            name: Some("f".to_string()),
            payload_schema: 1,
        };
        let mut catalog = TypeAbiCatalog::new();

        catalog.publish_header(method);
        catalog.publish_header(field);

        let methods = catalog.query_by_tag(TypeAbiTag::Method);
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name.as_deref(), Some("m"));
    }

    #[test]
    fn catalog_builder_names_refreshed_world_boundary() {
        let entry = MethodEntry {
            name: "size",
            arity: 0,
            slot: 201,
        };
        let mut builder = TypeAbiCatalog::builder_from_refreshed_world();

        let index = builder.publish(&entry);
        let catalog = builder.finish();

        assert_eq!(index, 0);
        assert_eq!(catalog.len(), 1);
        let got = catalog.get_by_tag_name(TypeAbiTag::Method, "size").unwrap();
        assert_eq!(got.id, 201);
    }

    #[test]
    fn catalog_from_refreshed_views_does_not_require_pack() {
        let entry = MethodEntry {
            name: "contains",
            arity: 1,
            slot: 309,
        };
        let catalog = TypeAbiCatalog::from_refreshed_views(&[&entry as &dyn TypeAbiView]);

        assert_eq!(catalog.len(), 1);
        assert!(catalog.get_by_tag_id(TypeAbiTag::Method, 309).is_some());
    }
}
