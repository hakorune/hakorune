//! Builtin Box callable provider backed by `runtime::type_registry`.

use crate::runtime::type_box_abi::{MethodEntry, TypeBox};
use crate::runtime::type_registry;

use super::super::{BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget};

pub const TRUTH_SOURCE_TYPE_REGISTRY: &str = "type_registry";

const KNOWN_BUILTIN_TYPEBOXES: &[&str] = &[
    "MapBox",
    "ArrayBox",
    "StringBox",
    "ConsoleBox",
    "InstanceBox",
    "String",
    "Array",
];

pub fn seed_typebox_methods(registry: &mut BoxCallableRegistry, type_box: &TypeBox) -> usize {
    type_box
        .methods
        .iter()
        .map(|entry| seed_method_entry(registry, type_box.type_name, entry))
        .sum()
}

pub fn seed_typebox_by_name(registry: &mut BoxCallableRegistry, type_name: &str) -> Option<usize> {
    let type_box = type_registry::resolve_typebox_by_name(type_name)?;
    Some(seed_typebox_methods(registry, type_box))
}

pub fn seed_known_builtin_typeboxes(registry: &mut BoxCallableRegistry) -> usize {
    KNOWN_BUILTIN_TYPEBOXES
        .iter()
        .filter_map(|type_name| seed_typebox_by_name(registry, type_name))
        .sum()
}

fn seed_method_entry(
    registry: &mut BoxCallableRegistry,
    type_name: &'static str,
    entry: &MethodEntry,
) -> usize {
    let key = BoxCallableKey::new(type_name, BoxCallableRole::Method, entry.name, entry.arity);
    let target = BoxCallableTarget::InternalSlot { slot: entry.slot };
    registry.insert(key, target);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_stringbox_method_entry_as_internal_slot() {
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_typebox_by_name(&mut registry, "StringBox");

        let key = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1);
        assert!(seeded.unwrap() > 0);
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::InternalSlot { slot: 309 })
        );
        assert_eq!(
            registry.get(&key).unwrap().id_space(),
            "internal_vtable_slot"
        );
    }

    #[test]
    fn seeds_primitive_string_separately_from_stringbox() {
        let mut registry = BoxCallableRegistry::new();

        seed_typebox_by_name(&mut registry, "String").unwrap();
        seed_typebox_by_name(&mut registry, "StringBox").unwrap();

        let primitive = BoxCallableKey::new("String", BoxCallableRole::Method, "contains", 1);
        let boxed = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1);
        assert_eq!(
            registry.get(&primitive),
            Some(&BoxCallableTarget::InternalSlot { slot: 309 })
        );
        assert_eq!(
            registry.get(&boxed),
            Some(&BoxCallableTarget::InternalSlot { slot: 309 })
        );
        assert_ne!(primitive, boxed);
    }

    #[test]
    fn unknown_typebox_does_not_seed() {
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_typebox_by_name(&mut registry, "MissingBox");

        assert_eq!(seeded, None);
        assert!(registry.is_empty());
    }

    #[test]
    fn seeds_known_builtin_typeboxes() {
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_known_builtin_typeboxes(&mut registry);

        assert!(seeded > 0);
        assert!(!registry.is_empty());
        assert!(registry
            .query_by_role(BoxCallableRole::Method)
            .all(|(_key, target)| target.id_space() == "internal_vtable_slot"));
    }
}
