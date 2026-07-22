//! ROOT0-DRAIN0-POLICY0 focused proof for the neutral policy SSOT.

use super::super::module_invocation_identity::ModuleInvocationFamilyV1;
use super::super::module_invocation_policy::{
    InvocationConditionPolicyV1, InvocationFallbackPolicyV1, InvocationInventoryAuthorityV1,
    InvocationRootPolicyV1, ModuleInvocationPolicyV1,
};

#[test]
fn policy_ssot_keeps_raw_and_canonical_laws_distinct() {
    let raw = ModuleInvocationPolicyV1::policy_for_family(ModuleInvocationFamilyV1::Raw);
    assert_eq!(
        raw.inventory_authority(),
        InvocationInventoryAuthorityV1::RawExpansionReceipts
    );
    assert_eq!(raw.root_policy(), InvocationRootPolicyV1::RequiredMain);
    assert_eq!(
        raw.condition_policy(),
        InvocationConditionPolicyV1::RawSourceSelected
    );
    assert_eq!(
        raw.fallback_policy(),
        InvocationFallbackPolicyV1::Forbidden
    );

    for family in [
        ModuleInvocationFamilyV1::CanonicalAPlus,
        ModuleInvocationFamilyV1::BindingSsaTrivial,
    ] {
        let policy = ModuleInvocationPolicyV1::policy_for_family(family);
        assert_eq!(policy.family(), family);
        assert_eq!(
            policy.inventory_authority(),
            InvocationInventoryAuthorityV1::CanonicalResolvedOwner
        );
        assert_eq!(
            policy.root_policy(),
            InvocationRootPolicyV1::ExactCanonicalOwner
        );
        assert_eq!(
            policy.condition_policy(),
            InvocationConditionPolicyV1::Forbidden
        );
        assert_eq!(
            policy.fallback_policy(),
            InvocationFallbackPolicyV1::Forbidden
        );
    }
}

#[test]
fn policy_ssot_groups_callable_families_without_concrete_rows() {
    for family in [
        ModuleInvocationFamilyV1::BindingSsaAcyclic,
        ModuleInvocationFamilyV1::BindingSsaRecursive,
    ] {
        let policy = ModuleInvocationPolicyV1::policy_for_family(family);
        assert_eq!(policy.family(), family);
        assert_eq!(
            policy.inventory_authority(),
            InvocationInventoryAuthorityV1::CanonicalCallableCatalog
        );
        assert_eq!(
            policy.root_policy(),
            InvocationRootPolicyV1::ExactCallableCatalog
        );
        assert_eq!(
            policy.condition_policy(),
            InvocationConditionPolicyV1::Forbidden
        );
        assert_eq!(
            policy.fallback_policy(),
            InvocationFallbackPolicyV1::Forbidden
        );
    }
}
