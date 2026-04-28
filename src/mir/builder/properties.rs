//! MIR-side unified member property naming policy.
//!
//! Parser-side synthetic method AST emission lives in
//! `parser/declarations/box_def/members/property_emit.rs`. This module owns the
//! MIR builder's view of those names for registration and read lowering.

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
