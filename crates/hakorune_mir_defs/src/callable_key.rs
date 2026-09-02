//! Backend-neutral identity for a source-cataloged same-module callable.
//!
//! The key is issued by the source/catalog owner and is carried through
//! physical publication.  Its projections are one-way; no consumer may
//! reconstruct source meaning from a physical symbol or display string.

use crate::global_target::CanonicalGlobalTargetV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SameModuleCallableNamespaceV1 {
    /// A top-level/free static function in the same published module.
    ///
    /// The owner component is intentionally empty for this namespace.  The
    /// function name and arity are still part of the exact source-issued key;
    /// the empty owner must never be interpreted as a wildcard by consumers.
    FreeFunction,
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalSameModuleCallableKeyV1 {
    namespace: SameModuleCallableNamespaceV1,
    owner: Box<str>,
    name: Box<str>,
    arity: u32,
}

impl CanonicalSameModuleCallableKeyV1 {
    pub fn free_function(name: &str, arity: u32) -> Self {
        Self {
            namespace: SameModuleCallableNamespaceV1::FreeFunction,
            owner: "".into(),
            name: name.into(),
            arity,
        }
    }

    pub fn static_box_method(owner: &str, name: &str, arity: u32) -> Self {
        Self {
            namespace: SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner: owner.into(),
            name: name.into(),
            arity,
        }
    }

    pub fn instance_box_method(owner: &str, name: &str, arity: u32) -> Self {
        Self {
            namespace: SameModuleCallableNamespaceV1::InstanceBoxMethod,
            owner: owner.into(),
            name: name.into(),
            arity,
        }
    }

    pub const fn namespace(&self) -> SameModuleCallableNamespaceV1 {
        self.namespace
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn arity(&self) -> u32 {
        self.arity
    }

    /// Physical symbol projection.  This is never an authority for rebuilding
    /// the key; it is only emitted after the key has already been selected.
    pub fn mir_symbol_projection(&self) -> String {
        match self.namespace {
            SameModuleCallableNamespaceV1::FreeFunction => {
                format!("{}/{}", self.name, self.arity)
            }
            SameModuleCallableNamespaceV1::StaticBoxMethod
            | SameModuleCallableNamespaceV1::InstanceBoxMethod => {
                format!("{}.{}/{}", self.owner, self.name, self.arity)
            }
        }
    }

    /// Project an already-selected static declaration into the structural
    /// global carrier.  Catalog/session authority remains with the caller.
    pub fn canonical_global_target_v1(&self) -> Result<CanonicalGlobalTargetV1, String> {
        match self.namespace {
            SameModuleCallableNamespaceV1::FreeFunction => {
                CanonicalGlobalTargetV1::new_free_function(self.name.clone(), self.arity)
                    .map_err(|error| format!("invalid catalog global target: {error:?}"))
            }
            SameModuleCallableNamespaceV1::StaticBoxMethod => {
                CanonicalGlobalTargetV1::new_static_box_method(
                    self.owner.clone(),
                    self.name.clone(),
                    self.arity,
                )
                .map_err(|error| format!("invalid catalog global target: {error:?}"))
            }
            SameModuleCallableNamespaceV1::InstanceBoxMethod => {
                Err("instance methods do not have a global target".to_owned())
            }
        }
    }

    /// Hidden compatibility constructors for existing root-crate fixtures.
    #[doc(hidden)]
    pub fn test_static_box_method(owner: &str, name: &str, arity: usize) -> Self {
        Self::static_box_method(owner, name, arity as u32)
    }

    /// Hidden compatibility constructors for existing root-crate fixtures.
    #[doc(hidden)]
    pub fn test_instance_box_method(owner: &str, name: &str, arity: usize) -> Self {
        Self::instance_box_method(owner, name, arity as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
    use crate::global_target::{CanonicalGlobalTargetV1, CanonicalSameModuleGlobalTargetV1};

    #[test]
    fn free_function_key_projects_to_unqualified_symbol_and_global_target() {
        let key = CanonicalSameModuleCallableKeyV1::free_function("helper", 2);
        assert_eq!(key.namespace(), SameModuleCallableNamespaceV1::FreeFunction);
        assert_eq!(key.owner(), "");
        assert_eq!(key.name(), "helper");
        assert_eq!(key.arity(), 2);
        assert_eq!(key.mir_symbol_projection(), "helper/2");
        assert_eq!(
            key.canonical_global_target_v1().expect("free target"),
            CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction {
                name: "helper".into(),
                arity: 2,
            })
        );
    }

    #[test]
    fn free_function_key_does_not_reuse_static_owner_projection() {
        let free = CanonicalSameModuleCallableKeyV1::free_function("helper", 0);
        let static_method = CanonicalSameModuleCallableKeyV1::static_box_method("Box", "helper", 0);
        assert_ne!(free, static_method);
        assert_ne!(
            free.mir_symbol_projection(),
            static_method.mir_symbol_projection()
        );
    }
}
