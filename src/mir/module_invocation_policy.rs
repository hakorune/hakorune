//! ROOT0-DRAIN0-POLICY0: neutral module-invocation policy SSOT.
//!
//! This product carries only route policy.  Concrete function identities,
//! receipts, and physical draft rows remain owned by the selected source and
//! collector phases.  Raw is included so the existing legacy route keeps its
//! policy vocabulary while canonical source continuations use the same SSOT.

use super::module_invocation_identity::ModuleInvocationFamilyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationInventoryAuthorityV1 {
    RawExpansionReceipts,
    CanonicalResolvedOwner,
    CanonicalCallableCatalog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationRootPolicyV1 {
    RequiredMain,
    ExactCanonicalOwner,
    ExactCallableCatalog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationConditionPolicyV1 {
    RawSourceSelected,
    Forbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationFallbackPolicyV1 {
    Forbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModuleInvocationPolicyV1 {
    family: ModuleInvocationFamilyV1,
    inventory_authority: InvocationInventoryAuthorityV1,
    root_policy: InvocationRootPolicyV1,
    condition_policy: InvocationConditionPolicyV1,
    fallback_policy: InvocationFallbackPolicyV1,
}

impl ModuleInvocationPolicyV1 {
    /// Derive the only policy allowed for a route family.
    pub(crate) const fn policy_for_family(family: ModuleInvocationFamilyV1) -> Self {
        let (inventory_authority, root_policy, condition_policy) = match family {
            ModuleInvocationFamilyV1::Raw => (
                InvocationInventoryAuthorityV1::RawExpansionReceipts,
                InvocationRootPolicyV1::RequiredMain,
                InvocationConditionPolicyV1::RawSourceSelected,
            ),
            ModuleInvocationFamilyV1::CanonicalAPlus
            | ModuleInvocationFamilyV1::BindingSsaTrivial => (
                InvocationInventoryAuthorityV1::CanonicalResolvedOwner,
                InvocationRootPolicyV1::ExactCanonicalOwner,
                InvocationConditionPolicyV1::Forbidden,
            ),
            ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive => (
                InvocationInventoryAuthorityV1::CanonicalCallableCatalog,
                InvocationRootPolicyV1::ExactCallableCatalog,
                InvocationConditionPolicyV1::Forbidden,
            ),
        };
        Self {
            family,
            inventory_authority,
            root_policy,
            condition_policy,
            fallback_policy: InvocationFallbackPolicyV1::Forbidden,
        }
    }

    pub(crate) const fn family(self) -> ModuleInvocationFamilyV1 {
        self.family
    }

    pub(crate) const fn inventory_authority(self) -> InvocationInventoryAuthorityV1 {
        self.inventory_authority
    }

    pub(crate) const fn root_policy(self) -> InvocationRootPolicyV1 {
        self.root_policy
    }

    pub(crate) const fn condition_policy(self) -> InvocationConditionPolicyV1 {
        self.condition_policy
    }

    pub(crate) const fn fallback_policy(self) -> InvocationFallbackPolicyV1 {
        self.fallback_policy
    }
}
