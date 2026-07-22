//! CUT0-I0-ID0-S0 fixtures for the disconnected identity/token vocabulary.

use super::module_invocation_identity::{
    ModuleInvocationFamilyV1, ModuleInvocationIdentityErrorV1, ModuleInvocationTokenV1,
    TestInvocationPreflightFactoryV1,
};
use super::module_invocation_route_matrix::InvocationRootFamilyV1;

const FAMILIES: [ModuleInvocationFamilyV1; 5] = [
    InvocationRootFamilyV1::Raw,
    InvocationRootFamilyV1::CanonicalAPlus,
    InvocationRootFamilyV1::BindingSsaTrivial,
    InvocationRootFamilyV1::BindingSsaAcyclic,
    InvocationRootFamilyV1::BindingSsaRecursive,
];

#[test]
fn one_factory_mints_each_existing_family_once() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let tokens = FAMILIES
        .into_iter()
        .map(|family| factory.mint(family).unwrap())
        .collect::<Vec<ModuleInvocationTokenV1>>();
    assert_eq!(tokens.len(), 5);
    for (token, family) in tokens.iter().zip(FAMILIES) {
        assert_eq!(token.family(), family);
        assert!(token.id().ordinal() > 0);
    }
}

#[test]
fn invocation_ids_are_distinct_and_not_clone_authority() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let first = factory
        .mint(InvocationRootFamilyV1::Raw)
        .unwrap();
    let second = factory
        .mint(InvocationRootFamilyV1::BindingSsaRecursive)
        .unwrap();
    assert_ne!(first.id().ordinal(), second.id().ordinal());
    assert!(!first.id().same(second.id()));
}

#[test]
fn foreign_source_family_is_rejected_before_token_creation() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let error = factory
        .mint_with_source(
            InvocationRootFamilyV1::Raw,
            InvocationRootFamilyV1::BindingSsaRecursive,
        )
        .unwrap_err();
    assert_eq!(
        error,
        ModuleInvocationIdentityErrorV1::FamilySourceMismatch {
            family: InvocationRootFamilyV1::Raw,
            source_family: InvocationRootFamilyV1::BindingSsaRecursive,
        }
    );
    let token = factory
        .mint(InvocationRootFamilyV1::Raw)
        .expect("rejected foreign source must not consume the issuer ordinal");
    assert_eq!(token.id().ordinal(), 1);
}

#[test]
fn token_family_is_not_a_boolean_route_selector() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory
        .mint(InvocationRootFamilyV1::CanonicalAPlus)
        .unwrap();
    assert_eq!(token.family(), InvocationRootFamilyV1::CanonicalAPlus);
}
