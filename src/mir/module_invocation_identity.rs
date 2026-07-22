//! CANON-BRIDGE0-IDKERNEL: shared module-invocation identity vocabulary.
//!
//! Issuance remains owned by `MirCompiler`; this module only owns the opaque
//! value types carried across compiler and Builder phase boundaries.  The
//! token is non-Clone and route-bearing.  Its brand is a copyable membership
//! witness, never a source-authority replacement.

use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ModuleInvocationFamilyV1 {
    Raw,
    CanonicalAPlus,
    BindingSsaTrivial,
    BindingSsaAcyclic,
    BindingSsaRecursive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ModuleInvocationBrandV1 {
    compiler_domain: NonZeroU64,
    invocation_ordinal: NonZeroU64,
}

impl ModuleInvocationBrandV1 {
    pub(crate) fn same(self, other: Self) -> bool {
        self == other
    }

    pub(crate) const fn compiler_domain(self) -> NonZeroU64 {
        self.compiler_domain
    }

    pub(crate) const fn invocation_ordinal(self) -> NonZeroU64 {
        self.invocation_ordinal
    }

    pub(crate) const fn ordinal(self) -> u64 {
        self.invocation_ordinal.get()
    }

    #[cfg(test)]
    pub(crate) const fn legacy_test() -> Self {
        Self {
            compiler_domain: NonZeroU64::new(1).expect("non-zero test domain"),
            invocation_ordinal: NonZeroU64::new(1).expect("non-zero test ordinal"),
        }
    }

    #[cfg(test)]
    pub(crate) const fn test_with_ordinal(ordinal: u64) -> Self {
        Self {
            compiler_domain: NonZeroU64::new(1).expect("non-zero test domain"),
            invocation_ordinal: NonZeroU64::new(ordinal).expect("non-zero test ordinal"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ModuleInvocationIdV1 {
    brand: ModuleInvocationBrandV1,
    _seal: ModuleInvocationIdSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ModuleInvocationIdSealV1;

impl ModuleInvocationIdV1 {
    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(crate) fn same(&self, other: &Self) -> bool {
        self.brand.same(other.brand)
    }

    #[cfg(test)]
    pub(crate) const fn ordinal(&self) -> u64 {
        self.brand.invocation_ordinal().get()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ModuleInvocationTokenV1 {
    id: ModuleInvocationIdV1,
    family: ModuleInvocationFamilyV1,
    _seal: ModuleInvocationTokenSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ModuleInvocationTokenSealV1;

impl ModuleInvocationTokenV1 {
    pub(crate) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.family
    }

    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.id.brand()
    }

    pub(crate) const fn id(&self) -> &ModuleInvocationIdV1 {
        &self.id
    }

    pub(crate) fn from_issued(
        compiler_domain: NonZeroU64,
        invocation_ordinal: NonZeroU64,
        family: ModuleInvocationFamilyV1,
    ) -> Self {
        Self {
            id: ModuleInvocationIdV1 {
                brand: ModuleInvocationBrandV1 {
                    compiler_domain,
                    invocation_ordinal,
                },
                _seal: ModuleInvocationIdSealV1,
            },
            family,
            _seal: ModuleInvocationTokenSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_test(
        invocation_ordinal: NonZeroU64,
        family: ModuleInvocationFamilyV1,
    ) -> Self {
        Self::from_issued(
            NonZeroU64::new(1).expect("non-zero test domain"),
            invocation_ordinal,
            family,
        )
    }
}
