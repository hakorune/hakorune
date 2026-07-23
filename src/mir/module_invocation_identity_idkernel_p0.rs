//! Focused fixtures for the shared CANON-BRIDGE0 identity kernel.

use super::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};
use std::num::NonZeroU64;

#[test]
fn compiler_domain_separates_equal_local_ordinals() {
    let left = ModuleInvocationTokenV1::from_issued(
        NonZeroU64::new(11).unwrap(),
        NonZeroU64::new(1).unwrap(),
        ModuleInvocationFamilyV1::CanonicalAPlus,
    );
    let right = ModuleInvocationTokenV1::from_issued(
        NonZeroU64::new(12).unwrap(),
        NonZeroU64::new(1).unwrap(),
        ModuleInvocationFamilyV1::CanonicalAPlus,
    );

    assert!(!left.brand().same(right.brand()));
    assert_ne!(
        left.brand().compiler_domain(),
        right.brand().compiler_domain()
    );
    assert_eq!(
        left.brand().invocation_ordinal(),
        right.brand().invocation_ordinal()
    );
}

#[test]
fn local_ordinal_separates_invocations_in_one_domain() {
    let first = ModuleInvocationTokenV1::from_issued(
        NonZeroU64::new(11).unwrap(),
        NonZeroU64::new(1).unwrap(),
        ModuleInvocationFamilyV1::BindingSsaTrivial,
    );
    let second = ModuleInvocationTokenV1::from_issued(
        NonZeroU64::new(11).unwrap(),
        NonZeroU64::new(2).unwrap(),
        ModuleInvocationFamilyV1::BindingSsaTrivial,
    );

    assert!(!first.brand().same(second.brand()));
    assert_eq!(
        first.brand().compiler_domain(),
        second.brand().compiler_domain()
    );
    assert_eq!(first.brand().ordinal() + 1, second.brand().ordinal());
}

#[test]
fn token_preserves_route_family_without_identity_conversion() {
    let token = ModuleInvocationTokenV1::from_issued(
        NonZeroU64::new(11).unwrap(),
        NonZeroU64::new(3).unwrap(),
        ModuleInvocationFamilyV1::BindingSsaRecursive,
    );

    assert_eq!(
        token.family(),
        ModuleInvocationFamilyV1::BindingSsaRecursive
    );
    assert_eq!(token.id().brand(), token.brand());
}
