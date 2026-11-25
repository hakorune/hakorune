//! MIR-side unified member property naming policy.
//!
//! Parser-side synthetic method AST emission lives in
//! `parser/declarations/box_def/members/property_emit.rs`. This module owns the
//! MIR builder's view of those names for registration and read lowering.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PropertyKind {
    Computed,
    Once,
    BirthOnce,
}

impl PropertyKind {
    pub(crate) fn from_getter_method_name(method_name: &str) -> Option<(Self, String)> {
        if let Some(rest) = method_name.strip_prefix("__get_once_") {
            Some((Self::Once, rest.to_string()))
        } else if let Some(rest) = method_name.strip_prefix("__get_birth_") {
            Some((Self::BirthOnce, rest.to_string()))
        } else {
            method_name
                .strip_prefix("__get_")
                .map(|rest| (Self::Computed, rest.to_string()))
        }
    }

    pub(crate) fn getter_method_name(&self, prop_name: &str) -> String {
        match self {
            Self::Computed => format!("__get_{}", prop_name),
            Self::Once => format!("__get_once_{}", prop_name),
            Self::BirthOnce => format!("__get_birth_{}", prop_name),
        }
    }
}

/// MIR-side property getter registry.
///
/// The parser emits synthetic getter methods. Declaration lowering registers
/// those methods here, and field-read lowering asks this registry whether a
/// source property name should lower as a getter call.
#[derive(Debug, Default)]
pub(crate) struct PropertyRegistry {
    getters_by_box: HashMap<String, HashMap<String, PropertyKind>>,
}

impl PropertyRegistry {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn getter_method_name(&self, box_name: &str, prop_name: &str) -> Option<String> {
        self.getters_by_box
            .get(box_name)
            .and_then(|props| props.get(prop_name))
            .map(|kind| kind.getter_method_name(prop_name))
    }

    pub(crate) fn register_getter_method(&mut self, box_name: String, method_name: &str) -> bool {
        let Some((kind, prop_name)) = PropertyKind::from_getter_method_name(method_name) else {
            return false;
        };
        self.register_getter(box_name, prop_name, kind);
        true
    }

    fn register_getter(&mut self, box_name: String, prop_name: String, kind: PropertyKind) {
        self.getters_by_box
            .entry(box_name)
            .or_default()
            .insert(prop_name, kind);
    }
}
