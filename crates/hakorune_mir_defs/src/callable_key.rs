//! Backend-neutral identity for a source-cataloged same-module callable.
//!
//! The key is issued by the source/catalog owner and is carried through
//! physical publication.  Its projections are one-way; no consumer may
//! reconstruct source meaning from a physical symbol or display string.

use crate::global_target::CanonicalGlobalTargetV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SameModuleCallableNamespaceV1 {
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
        format!("{}.{}/{}", self.owner, self.name, self.arity)
    }

    /// Project an already-selected static declaration into the structural
    /// global carrier.  Catalog/session authority remains with the caller.
    pub fn canonical_global_target_v1(
        &self,
    ) -> Result<CanonicalGlobalTargetV1, String> {
        if self.namespace != SameModuleCallableNamespaceV1::StaticBoxMethod {
            return Err("only static box methods have a global target".to_owned());
        }
        CanonicalGlobalTargetV1::new_static_box_method(
            self.owner.clone(),
            self.name.clone(),
            self.arity,
        )
        .map_err(|error| format!("invalid catalog global target: {error:?}"))
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
