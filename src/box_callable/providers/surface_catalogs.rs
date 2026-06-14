//! Builtin collection callable provider backed by visible surface catalogs.
//!
//! This provider makes the existing String / Array / Map surface catalogs
//! visible as BoxCallable rows without routing execution through TypeAbiCatalog
//! or duplicating method descriptors.

use crate::boxes::{array::ARRAY_SURFACE_METHODS, basic::STRING_SURFACE_METHODS};
use crate::boxes::{buffer::BUFFER_SURFACE_METHODS, MAP_SURFACE_METHODS};

use super::super::{
    BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
};

pub const TRUTH_SOURCE_SURFACE_CATALOG: &str = "surface_catalog";

pub fn seed_known_surface_catalogs(registry: &mut BoxCallableRegistry) -> usize {
    seed_string_surface_catalog(registry, "StringBox")
        + seed_string_surface_catalog(registry, "String")
        + seed_array_surface_catalog(registry, "ArrayBox")
        + seed_array_surface_catalog(registry, "Array")
        + seed_map_surface_catalog(registry, "MapBox")
        + seed_buffer_surface_catalog(registry, "BufferBox")
}

pub fn seed_string_surface_catalog(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
) -> usize {
    let mut seeded = 0;
    for spec in STRING_SURFACE_METHODS {
        seeded += seed_surface_method(registry, type_name, spec.canonical, spec.arity, spec.slot);
        for alias in spec.aliases {
            seeded += seed_surface_method(registry, type_name, alias, spec.arity, spec.slot);
        }
    }
    seeded
}

pub fn seed_array_surface_catalog(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
) -> usize {
    let mut seeded = 0;
    for spec in ARRAY_SURFACE_METHODS {
        seeded += seed_surface_method(registry, type_name, spec.canonical, spec.arity, spec.slot);
        for alias in spec.aliases {
            seeded += seed_surface_method(registry, type_name, alias, spec.arity, spec.slot);
        }
    }
    seeded
}

pub fn seed_map_surface_catalog(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
) -> usize {
    let mut seeded = 0;
    for spec in MAP_SURFACE_METHODS {
        seeded += seed_surface_method(registry, type_name, spec.canonical, spec.arity, spec.slot);
        for alias in spec.aliases {
            seeded += seed_surface_method(registry, type_name, alias, spec.arity, spec.slot);
        }
    }
    seeded
}

pub fn seed_buffer_surface_catalog(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
) -> usize {
    let mut seeded = 0;
    for spec in BUFFER_SURFACE_METHODS {
        seeded += seed_surface_method(registry, type_name, spec.canonical, spec.arity, spec.slot);
        for alias in spec.aliases {
            seeded += seed_surface_method(registry, type_name, alias, spec.arity, spec.slot);
        }
    }
    seeded
}

fn seed_surface_method(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
    name: &'static str,
    arity: u8,
    slot: u16,
) -> usize {
    let key = BoxCallableKey::new(type_name, BoxCallableRole::Method, name, arity);
    let target = BoxCallableTarget::InternalSlot { slot };
    registry.insert_with_source(key, BoxCallableSource::SurfaceCatalog, target);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_string_surface_catalog_rows() {
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_string_surface_catalog(&mut registry, "StringBox");

        let key = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1);
        let alias = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "len", 0);
        assert!(seeded >= STRING_SURFACE_METHODS.len());
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::InternalSlot { slot: 309 })
        );
        assert_eq!(
            registry.get(&alias),
            Some(&BoxCallableTarget::InternalSlot { slot: 300 })
        );
        assert_eq!(
            registry.get_source(&key).unwrap().as_str(),
            TRUTH_SOURCE_SURFACE_CATALOG
        );
    }

    #[test]
    fn seeds_array_surface_catalog_rows() {
        let mut registry = BoxCallableRegistry::new();

        seed_array_surface_catalog(&mut registry, "ArrayBox");

        let key = BoxCallableKey::new("ArrayBox", BoxCallableRole::Method, "insert", 2);
        let alias = BoxCallableKey::new("ArrayBox", BoxCallableRole::Method, "size", 0);
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::InternalSlot { slot: 113 })
        );
        assert_eq!(
            registry.get(&alias),
            Some(&BoxCallableTarget::InternalSlot { slot: 102 })
        );
        assert_eq!(
            registry.get_source(&key).unwrap().as_str(),
            TRUTH_SOURCE_SURFACE_CATALOG
        );
    }

    #[test]
    fn seeds_map_surface_catalog_rows() {
        let mut registry = BoxCallableRegistry::new();

        seed_map_surface_catalog(&mut registry, "MapBox");

        let key = BoxCallableKey::new("MapBox", BoxCallableRole::Method, "delete", 1);
        let alias = BoxCallableKey::new("MapBox", BoxCallableRole::Method, "length", 0);
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::InternalSlot { slot: 205 })
        );
        assert_eq!(
            registry.get(&alias),
            Some(&BoxCallableTarget::InternalSlot { slot: 200 })
        );
        assert_eq!(
            registry.get_source(&key).unwrap().as_str(),
            TRUTH_SOURCE_SURFACE_CATALOG
        );
    }

    #[test]
    fn seeds_known_catalogs_including_buffer_rows() {
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_known_surface_catalogs(&mut registry);

        assert!(seeded > 0);
        assert!(registry
            .iter_entries()
            .all(|(_key, entry)| entry.source == BoxCallableSource::SurfaceCatalog));
        assert!(registry
            .iter_entries()
            .all(|(_key, entry)| entry.target.id_space() == "internal_vtable_slot"));
        assert!(registry
            .iter_entries()
            .any(|(key, _entry)| key.box_key.as_str() == "BufferBox"));
    }

    #[test]
    fn seeds_buffer_surface_catalog_rows() {
        let mut registry = BoxCallableRegistry::new();

        seed_buffer_surface_catalog(&mut registry, "BufferBox");

        let key = BoxCallableKey::new("BufferBox", BoxCallableRole::Method, "readAll", 0);
        let alias = BoxCallableKey::new("BufferBox", BoxCallableRole::Method, "len", 0);
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::InternalSlot { slot: 502 })
        );
        assert_eq!(
            registry.get(&alias),
            Some(&BoxCallableTarget::InternalSlot { slot: 504 })
        );
        assert_eq!(
            registry.get_source(&key).unwrap().as_str(),
            TRUTH_SOURCE_SURFACE_CATALOG
        );
        assert!(registry
            .iter_entries()
            .all(|(_key, entry)| entry.source == BoxCallableSource::SurfaceCatalog));
    }
}
