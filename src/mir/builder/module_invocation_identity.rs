//! CUT0-I0-ID0-S0: one invocation identity and five-family token vocabulary.
//!
//! This row is intentionally disconnected. It brands no production shell,
//! collector, receipt, or compiler ingress yet; ID0-P0 will replace the
//! opaque source witnesses with real preflight plans and thread the identity
//! through those owners. The existing route matrix remains the family SSOT.

use std::num::NonZeroU64;

use super::module_invocation_route_matrix::InvocationRootFamilyV1;

pub(in crate::mir::builder) type ModuleInvocationFamilyV1 = InvocationRootFamilyV1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::mir::builder) struct ModuleInvocationIdV1 {
    ordinal: NonZeroU64,
    _seal: ModuleInvocationIdSealV1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ModuleInvocationIdSealV1;

impl ModuleInvocationIdV1 {
    pub(in crate::mir::builder) fn same(&self, other: &Self) -> bool {
        self == other
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn ordinal(&self) -> u64 {
        self.ordinal.get()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleInvocationIdentityErrorV1 {
    OrdinalExhausted,
    FamilySourceMismatch {
        family: ModuleInvocationFamilyV1,
        source_family: ModuleInvocationFamilyV1,
    },
}

impl std::fmt::Display for ModuleInvocationIdentityErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][invocation_identity] {self:?}")
    }
}

impl std::error::Error for ModuleInvocationIdentityErrorV1 {}

/// Private source brands for ID0-S0. ID0-P0 replaces them with actual plans.
#[derive(Debug, PartialEq, Eq)]
enum SealedInvocationSourceWitnessV1 {
    Raw,
    CanonicalAPlus,
    BindingSsaTrivial,
    BindingSsaAcyclic,
    BindingSsaRecursive,
}

impl SealedInvocationSourceWitnessV1 {
    fn family(&self) -> ModuleInvocationFamilyV1 {
        match self {
            Self::Raw => InvocationRootFamilyV1::Raw,
            Self::CanonicalAPlus => InvocationRootFamilyV1::CanonicalAPlus,
            Self::BindingSsaTrivial => InvocationRootFamilyV1::BindingSsaTrivial,
            Self::BindingSsaAcyclic => InvocationRootFamilyV1::BindingSsaAcyclic,
            Self::BindingSsaRecursive => InvocationRootFamilyV1::BindingSsaRecursive,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleInvocationTokenKindV1 {
    Raw {
        id: ModuleInvocationIdV1,
        source: SealedInvocationSourceWitnessV1,
    },
    CanonicalAPlus {
        id: ModuleInvocationIdV1,
        source: SealedInvocationSourceWitnessV1,
    },
    BindingSsaTrivial {
        id: ModuleInvocationIdV1,
        source: SealedInvocationSourceWitnessV1,
    },
    BindingSsaAcyclic {
        id: ModuleInvocationIdV1,
        source: SealedInvocationSourceWitnessV1,
    },
    BindingSsaRecursive {
        id: ModuleInvocationIdV1,
        source: SealedInvocationSourceWitnessV1,
    },
}

/// Sealed wrapper around the five private token variants. Sibling modules can
/// inspect the brand but cannot construct a foreign source/family pairing.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct ModuleInvocationTokenV1 {
    kind: ModuleInvocationTokenKindV1,
}

impl ModuleInvocationTokenV1 {
    pub(in crate::mir::builder) fn id(&self) -> &ModuleInvocationIdV1 {
        match &self.kind {
            ModuleInvocationTokenKindV1::Raw { id, .. }
            | ModuleInvocationTokenKindV1::CanonicalAPlus { id, .. }
            | ModuleInvocationTokenKindV1::BindingSsaTrivial { id, .. }
            | ModuleInvocationTokenKindV1::BindingSsaAcyclic { id, .. }
            | ModuleInvocationTokenKindV1::BindingSsaRecursive { id, .. } => id,
        }
    }

    pub(in crate::mir::builder) const fn family(&self) -> ModuleInvocationFamilyV1 {
        match self.kind {
            ModuleInvocationTokenKindV1::Raw { .. } => InvocationRootFamilyV1::Raw,
            ModuleInvocationTokenKindV1::CanonicalAPlus { .. } => InvocationRootFamilyV1::CanonicalAPlus,
            ModuleInvocationTokenKindV1::BindingSsaTrivial { .. } => InvocationRootFamilyV1::BindingSsaTrivial,
            ModuleInvocationTokenKindV1::BindingSsaAcyclic { .. } => InvocationRootFamilyV1::BindingSsaAcyclic,
            ModuleInvocationTokenKindV1::BindingSsaRecursive { .. } => InvocationRootFamilyV1::BindingSsaRecursive,
        }
    }

    #[cfg(test)]
    fn from_test_preflight(
        id: ModuleInvocationIdV1,
        family: ModuleInvocationFamilyV1,
        source: SealedInvocationSourceWitnessV1,
    ) -> Result<Self, ModuleInvocationIdentityErrorV1> {
        if source.family() != family {
            return Err(ModuleInvocationIdentityErrorV1::FamilySourceMismatch {
                family,
                source_family: source.family(),
            });
        }
        let kind = match family {
            InvocationRootFamilyV1::Raw => ModuleInvocationTokenKindV1::Raw { id, source },
            InvocationRootFamilyV1::CanonicalAPlus => {
                ModuleInvocationTokenKindV1::CanonicalAPlus { id, source }
            }
            InvocationRootFamilyV1::BindingSsaTrivial => {
                ModuleInvocationTokenKindV1::BindingSsaTrivial { id, source }
            }
            InvocationRootFamilyV1::BindingSsaAcyclic => {
                ModuleInvocationTokenKindV1::BindingSsaAcyclic { id, source }
            }
            InvocationRootFamilyV1::BindingSsaRecursive => {
                ModuleInvocationTokenKindV1::BindingSsaRecursive { id, source }
            }
        };
        Ok(Self { kind })
    }
}

#[cfg(test)]
pub(in crate::mir::builder) struct TestInvocationPreflightFactoryV1 {
    next: u64,
}

#[cfg(test)]
impl TestInvocationPreflightFactoryV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self { next: 1 }
    }

    pub(in crate::mir::builder) fn mint(
        &mut self,
        family: ModuleInvocationFamilyV1,
    ) -> Result<ModuleInvocationTokenV1, ModuleInvocationIdentityErrorV1> {
        self.mint_with_source(family, family)
    }

    pub(in crate::mir::builder) fn mint_with_source(
        &mut self,
        family: ModuleInvocationFamilyV1,
        source_family: ModuleInvocationFamilyV1,
    ) -> Result<ModuleInvocationTokenV1, ModuleInvocationIdentityErrorV1> {
        if family != source_family {
            return Err(ModuleInvocationIdentityErrorV1::FamilySourceMismatch {
                family,
                source_family,
            });
        }
        let ordinal = NonZeroU64::new(self.next)
            .ok_or(ModuleInvocationIdentityErrorV1::OrdinalExhausted)?;
        self.next = self
            .next
            .checked_add(1)
            .ok_or(ModuleInvocationIdentityErrorV1::OrdinalExhausted)?;
        ModuleInvocationTokenV1::from_test_preflight(
            ModuleInvocationIdV1 {
                ordinal,
                _seal: ModuleInvocationIdSealV1,
            },
            family,
            source_witness(source_family),
        )
    }
}

#[cfg(test)]
fn source_witness(family: ModuleInvocationFamilyV1) -> SealedInvocationSourceWitnessV1 {
    match family {
        InvocationRootFamilyV1::Raw => SealedInvocationSourceWitnessV1::Raw,
        InvocationRootFamilyV1::CanonicalAPlus => SealedInvocationSourceWitnessV1::CanonicalAPlus,
        InvocationRootFamilyV1::BindingSsaTrivial => SealedInvocationSourceWitnessV1::BindingSsaTrivial,
        InvocationRootFamilyV1::BindingSsaAcyclic => SealedInvocationSourceWitnessV1::BindingSsaAcyclic,
        InvocationRootFamilyV1::BindingSsaRecursive => SealedInvocationSourceWitnessV1::BindingSsaRecursive,
    }
}
