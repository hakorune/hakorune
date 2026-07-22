//! CUT0-I0-ID0-P0 fixtures for same-brand owner-chain propagation.

use super::module_invocation_identity::{
    ModuleInvocationFamilyV1, TestInvocationPreflightFactoryV1,
};
use super::module_invocation_owner_chain::{
    advance_to_prepared_commit, BrandedCollectorV1, BrandedShellV1, CollectedInvocationDraftSetV1,
    InvocationBranded, InvocationBrandedReceiptV1, InvocationBrandErrorV1,
    InvocationDraftSourceProofV1, InvocationReceiptKindV1, ModuleBuilderInvocationSessionV1,
};
use super::module_invocation_route_matrix::InvocationRootFamilyV1;

#[test]
fn one_source_brand_survives_session_collection_and_prepared_commit() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = InvocationDraftSourceProofV1::from_token(
        factory
            .mint(InvocationRootFamilyV1::BindingSsaAcyclic)
            .unwrap(),
    );
    let session = ModuleBuilderInvocationSessionV1::from_source(&source);
    assert_eq!(session.family(), ModuleInvocationFamilyV1::BindingSsaAcyclic);
    let brand = source.brand();
    let shell = BrandedShellV1::<()>::from_test(brand, ());
    let collector = InvocationBranded::from_test(brand, ());
    let receipt = InvocationBrandedReceiptV1::from_test(
        brand,
        InvocationReceiptKindV1::CallableBatch,
    );
    let collected = CollectedInvocationDraftSetV1::from_parts(
        source,
        shell,
        collector,
        vec![receipt],
    )
    .unwrap();
    assert_eq!(collected.brand(), brand);
    assert_eq!(collected.receipt_count(), 1);
    let final_source = InvocationDraftSourceProofV1::from_token(
        factory
            .mint(InvocationRootFamilyV1::BindingSsaAcyclic)
            .unwrap(),
    );
    let final_brand = final_source.brand();
    let prepared = advance_to_prepared_commit(
        final_source,
        BrandedShellV1::<()>::from_test(final_brand, ()),
        InvocationBranded::from_test(final_brand, ()),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(prepared.brand(), final_brand);
}

#[test]
fn source_and_collector_foreign_pair_fails_before_co_seal() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = InvocationDraftSourceProofV1::from_token(
        factory.mint(InvocationRootFamilyV1::Raw).unwrap(),
    );
    let foreign = factory
        .mint(InvocationRootFamilyV1::CanonicalAPlus)
        .unwrap()
        .brand();
    assert_eq!(
        CollectedInvocationDraftSetV1::from_parts(
            source,
            BrandedShellV1::<()>::from_test(foreign, ()),
            InvocationBranded::<()>::from_test(foreign, ()),
            Vec::new(),
        )
        .unwrap_err(),
        InvocationBrandErrorV1::ForeignOwner {
            expected: 1,
            actual: 2,
        }
    );
}

#[test]
fn foreign_receipt_and_wrong_kind_fail_before_co_seal() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = InvocationDraftSourceProofV1::from_token(
        factory.mint(InvocationRootFamilyV1::Raw).unwrap(),
    );
    let foreign = factory
        .mint(InvocationRootFamilyV1::BindingSsaRecursive)
        .unwrap()
        .brand();
    let error = CollectedInvocationDraftSetV1::from_parts(
        source,
        BrandedShellV1::<()>::from_test(foreign, ()),
        BrandedCollectorV1::<()>::from_test(foreign, ()),
        Vec::new(),
    )
    .unwrap_err();
    assert!(matches!(error, InvocationBrandErrorV1::ForeignOwner { .. }));

    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = InvocationDraftSourceProofV1::from_token(
        factory.mint(InvocationRootFamilyV1::Raw).unwrap(),
    );
    let brand = source.brand();
    let error = CollectedInvocationDraftSetV1::from_parts(
        source,
        BrandedShellV1::<()>::from_test(brand, ()),
        InvocationBranded::from_test(brand, ()),
        vec![InvocationBrandedReceiptV1::from_test(
            brand,
            InvocationReceiptKindV1::CallableBatch,
        )],
    )
    .unwrap_err();
    assert!(matches!(
        error,
        InvocationBrandErrorV1::ReceiptKindMismatch {
            family: InvocationRootFamilyV1::Raw,
            kind: InvocationReceiptKindV1::CallableBatch,
        }
    ));
}

#[test]
fn foreign_shell_and_collector_fail_before_co_seal() {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = InvocationDraftSourceProofV1::from_token(
        factory.mint(InvocationRootFamilyV1::Raw).unwrap(),
    );
    let source_brand = source.brand();
    let foreign = factory
        .mint(InvocationRootFamilyV1::CanonicalAPlus)
        .unwrap()
        .brand();
    let error = CollectedInvocationDraftSetV1::from_parts(
        source,
        BrandedShellV1::<()>::from_test(source_brand, ()),
        BrandedCollectorV1::<()>::from_test(foreign, ()),
        Vec::new(),
    )
    .unwrap_err();
    assert!(matches!(error, InvocationBrandErrorV1::ForeignOwner { .. }));
}
