//! In-memory Box callable registry.

use std::collections::HashMap;

use super::model::{BoxCallableKey, BoxCallableRole, BoxCallableTarget};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BoxCallableRegistry {
    entries: HashMap<BoxCallableKey, BoxCallableTarget>,
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
        self.entries.insert(key, target)
    }

    pub fn get(&self, key: &BoxCallableKey) -> Option<&BoxCallableTarget> {
        self.entries.get(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BoxCallableKey, &BoxCallableTarget)> {
        self.entries.iter()
    }

    pub fn query_by_role(
        &self,
        role: BoxCallableRole,
    ) -> impl Iterator<Item = (&BoxCallableKey, &BoxCallableTarget)> {
        self.entries
            .iter()
            .filter(move |(key, _target)| key.role == role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_callable::model::{BoxCallableKey, BoxCallableRole, BoxCallableTarget};

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
