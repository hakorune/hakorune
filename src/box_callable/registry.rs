//! In-memory Box callable registry.

use std::collections::HashMap;

use super::model::{
    BoxCallableEntry, BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BoxCallableRegistry {
    entries: HashMap<BoxCallableKey, BoxCallableEntry>,
}

impl BoxCallableRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        key: BoxCallableKey,
        target: BoxCallableTarget,
    ) -> Option<BoxCallableTarget> {
        self.insert_with_source(key, BoxCallableSource::Manual, target)
    }

    pub fn insert_with_source(
        &mut self,
        key: BoxCallableKey,
        source: BoxCallableSource,
        target: BoxCallableTarget,
    ) -> Option<BoxCallableTarget> {
        self.entries
            .insert(key, BoxCallableEntry::new(source, target))
            .map(|entry| entry.target)
    }

    pub fn get(&self, key: &BoxCallableKey) -> Option<&BoxCallableTarget> {
        self.entries.get(key).map(|entry| &entry.target)
    }

    pub fn get_entry(&self, key: &BoxCallableKey) -> Option<&BoxCallableEntry> {
        self.entries.get(key)
    }

    pub fn get_source(&self, key: &BoxCallableKey) -> Option<BoxCallableSource> {
        self.entries.get(key).map(|entry| entry.source)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BoxCallableKey, &BoxCallableTarget)> {
        self.entries.iter().map(|(key, entry)| (key, &entry.target))
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = (&BoxCallableKey, &BoxCallableEntry)> {
        self.entries.iter()
    }

    pub fn query_by_role(
        &self,
        role: BoxCallableRole,
    ) -> impl Iterator<Item = (&BoxCallableKey, &BoxCallableTarget)> {
        self.entries
            .iter()
            .filter(move |(key, _entry)| key.role == role)
            .map(|(key, entry)| (key, &entry.target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_callable::model::{
        BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
    };

    #[test]
    fn registry_stores_callable_key_to_typed_target() {
        let key = BoxCallableKey::new("ArrayBox", BoxCallableRole::Method, "length", 0);
        let target = BoxCallableTarget::InternalSlot { slot: 200 };
        let mut registry = BoxCallableRegistry::new();

        let old = registry.insert(key.clone(), target.clone());

        assert!(old.is_none());
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.get(&key), Some(&target));
        assert_eq!(
            registry.get(&key).unwrap().id_space(),
            "internal_vtable_slot"
        );
        assert_eq!(registry.get_source(&key), Some(BoxCallableSource::Manual));
    }

    #[test]
    fn registry_keeps_plugin_method_id_space_separate_from_internal_slot() {
        let internal = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run_internal", 0);
        let plugin = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run_plugin", 0);
        let mut registry = BoxCallableRegistry::new();

        registry.insert(
            internal.clone(),
            BoxCallableTarget::InternalSlot { slot: 7 },
        );
        registry.insert(
            plugin.clone(),
            BoxCallableTarget::PluginMethod {
                type_id: 42,
                method_id: 7,
                returns_result: false,
            },
        );

        assert_eq!(
            registry.get(&internal).unwrap().id_space(),
            "internal_vtable_slot"
        );
        assert_eq!(
            registry.get(&plugin).unwrap().id_space(),
            "plugin_typebox_method_id"
        );
    }

    #[test]
    fn registry_can_store_provider_source_next_to_target() {
        let key = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1);
        let mut registry = BoxCallableRegistry::new();

        registry.insert_with_source(
            key.clone(),
            BoxCallableSource::TypeRegistry,
            BoxCallableTarget::InternalSlot { slot: 309 },
        );

        let entry = registry.get_entry(&key).unwrap();
        assert_eq!(entry.source, BoxCallableSource::TypeRegistry);
        assert_eq!(entry.source.as_str(), "type_registry");
        assert_eq!(entry.target.id_space(), "internal_vtable_slot");
    }

    #[test]
    fn registry_can_query_lifecycle_roles() {
        let birth = BoxCallableKey::new("DemoBox", BoxCallableRole::Birth, "birth", 0);
        let method = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 0);
        let mut registry = BoxCallableRegistry::new();

        registry.insert(
            birth,
            BoxCallableTarget::PluginLifecycle {
                type_id: 42,
                birth_id: Some(1),
                fini_id: Some(2),
            },
        );
        registry.insert(
            method,
            BoxCallableTarget::PluginMethod {
                type_id: 42,
                method_id: 3,
                returns_result: true,
            },
        );

        let lifecycle: Vec<_> = registry.query_by_role(BoxCallableRole::Birth).collect();
        assert_eq!(lifecycle.len(), 1);
        assert_eq!(lifecycle[0].1.id_space(), "plugin_lifecycle_method_id");
    }
}
