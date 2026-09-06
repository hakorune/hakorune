use super::*;
use crate::mir::resolved_semantics::{
    FunctionOwnerIssuerV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn test_site() -> OwnedExprSiteV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let owner = issuer.issue().expect("owner");
    OwnedExprSiteV1::new(
        owner,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ])),
    )
}

fn claim(site: OwnedExprSiteV1, arity: usize) -> OrdinaryNewAdmissionClaimV1 {
    let package =
        super::super::brand_catalog_tests::issue_with_brand_catalog("box Page {}").unwrap();
    let box_source = package
        .batch()
        .ordinary_box_coverage()
        .row_for("Page")
        .unwrap()
        .unwrap()
        .clone();
    let construction = package
        .instance_constructors
        .construction_for(&box_source, 0)
        .unwrap()
        .clone();
    let (object, destruction) = package
        .instance_constructors
        .destruction_for(&box_source)
        .unwrap();
    let destination = BindingRefV1::new(site.owner(), hakorune_mir_core::BindingId::new(0));
    let declaration = SourceBindingSiteV1::Local {
        statement: crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(
            crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
            ]),
        ),
        ordinal: 0,
    };
    OrdinaryNewAdmissionClaimV1 {
        site,
        box_source,
        class: "Page".into(),
        arity,
        constructor: OrdinaryNewConstructorDispositionV1::NoBirthZero,
        destination,
        declaration,
        home_prefix: Err(HomePrefixUnavailableV1::SourceMismatch),
        construction,
        object,
        destruction,
    }
}

#[test]
fn exact_claim_is_consumed_once() {
    let site = test_site();
    let ledger = OrdinaryNewClaimLedgerV1::issue(
        vec![claim(site.clone(), 0)].into_boxed_slice(),
        vec!["Page".into()].into_boxed_slice(),
    );

    let taken = ledger
        .try_take(&site, "Page", 0)
        .expect("exact claim should be available")
        .expect("ordinary class should return a claim");
    assert_eq!(taken.class(), "Page");
    assert_eq!(taken.arity(), 0);
    assert_eq!(
        ledger
            .local_commits
            .borrow()
            .get(&site)
            .unwrap()
            .construction(),
        taken.construction()
    );
    assert!(taken
        .construction()
        .as_ref()
        .unwrap()
        .reclaims_unpublished_outer_storage());
    assert!(!ledger.is_empty(), "target take is not local completion");
    assert!(
        !ledger.prepare_new_emission(&taken).unwrap(),
        "unavailable prefix stays fenced"
    );
    let SourceBindingSiteV1::Local { statement, ordinal } = &taken.declaration else {
        panic!("local claim");
    };
    ledger
        .complete_new_expression(&site, "Page", crate::mir::ValueId(0))
        .unwrap();
    assert!(!ledger.is_empty(), "whole New is not local installation");
    ledger
        .complete_local_installation(
            site.owner(),
            statement.node(),
            &[(
                taken.destination,
                *ordinal,
                crate::mir::ValueId(0),
                crate::mir::ValueId(1),
            )],
        )
        .unwrap();
    assert!(
        ledger.is_empty(),
        "ValueId zero is a valid completed initializer"
    );
    assert_eq!(
        ledger.try_take(&site, "Page", 0),
        Err(OrdinaryNewClaimTakeErrorV1::Unavailable)
    );
    use crate::mir::function::{
        RootOrdinaryNewObservation as O, RootOrdinaryNewUnavailable as U,
    };
    let physical = crate::mir::MirFunction::new(
        crate::mir::FunctionSignature {
            name: "observation_only".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::CONTROL,
        },
        crate::mir::BasicBlockId::new(0),
    );
    ledger.register_new_root(site.owner()).unwrap();
    assert_eq!(
        ledger.validate_finalized_new_root(&physical).unwrap(),
        O::Unavailable(U::CompletionMissing)
    );
    let mut ledger = ledger;
    ledger.root_completion = Some(Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::BodyLengthOverflow));
    *ledger.root_validation.borrow_mut() =
        local_commit::RootNewValidation::Pending(site.owner());
    assert_eq!(
        ledger.validate_finalized_new_root(&physical).unwrap(),
        O::Unavailable(U::CompletionRejected)
    );
    let empty = OrdinaryNewClaimLedgerV1::issue(Box::new([]), Box::new([]));
    assert_eq!(
        empty.validate_finalized_new_root(&physical).unwrap(),
        O::NotIssued
    );
    empty.register_new_root(site.owner()).unwrap();
    assert_eq!(
        empty.validate_finalized_new_root(&physical).unwrap(),
        O::NoSelectedLocalNew
    );
}

#[test]
fn pending_construction_retains_constructor_identity_after_target_take() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { value: i64\nbirth(value) { me.value = value } }
         static box Main { main() { local page = new Page(7)\nreturn 0 } }",
    )
    .unwrap();
    let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
    assert_eq!(claims.len(), 1);
    let claim = claims.values().next().unwrap();
    let site = claim.site().clone();
    let expected = claim
        .construction()
        .as_ref()
        .unwrap()
        .constructor()
        .unwrap()
        .clone();
    drop(claims);
    let ledger = package.ordinary_new_claim_ledger;
    drop(
        ledger
            .try_take(&site, "Page", 1)
            .unwrap()
            .unwrap()
            .constructor(),
    );
    let rows = ledger.local_commits.borrow();
    let plan = rows.get(&site).unwrap().construction().as_ref().unwrap();
    assert_eq!(plan.constructor(), Some(&expected));
    assert_eq!(plan.stores().len(), 1);
}

#[test]
fn nonordinary_class_does_not_consume_claim() {
    let site = test_site();
    let ledger = OrdinaryNewClaimLedgerV1::issue(
        vec![claim(site.clone(), 0)].into_boxed_slice(),
        vec!["Page".into()].into_boxed_slice(),
    );

    assert_eq!(ledger.try_take(&site, "Plugin", 0), Ok(None));
    assert!(!ledger.is_empty());
    assert!(ledger.try_take(&site, "Page", 0).unwrap().is_some());
    assert!(!ledger.is_empty(), "target take is not local completion");
}

#[test]
fn mismatched_shape_preserves_claim_for_the_correct_consumer() {
    let site = test_site();
    let ledger = OrdinaryNewClaimLedgerV1::issue(
        vec![claim(site.clone(), 0)].into_boxed_slice(),
        vec!["Page".into()].into_boxed_slice(),
    );

    assert_eq!(
        ledger.try_take(&site, "Page", 1),
        Err(OrdinaryNewClaimTakeErrorV1::Mismatch)
    );
    assert!(!ledger.is_empty());
    assert!(ledger.try_take(&site, "Page", 0).unwrap().is_some());
    assert!(!ledger.is_empty(), "target take is not local completion");
}

#[test]
fn ordinary_new_recipe_preserves_exact_birth_source() {
    let source_id = crate::parser::ConstructorSourceIdV1::test_new(7);
    let abi = InstanceConstructorAbiV1::issue(2).expect("constructor ABI");
    let target = CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 2);
    let recipe = VerifiedOrdinaryNewBirthRecipeV1 {
        source_id: source_id.clone(),
        target: target.clone(),
        effect: DeclaredInstanceCallSemanticEffectV1::OpaqueObservable,
        abi,
    };
    let OrdinaryNewConstructorDispositionV1::Birth(recipe) =
        OrdinaryNewConstructorDispositionV1::Birth(recipe)
    else {
        unreachable!()
    };
    assert!(recipe.source_id().same_as(&source_id));
    assert_eq!(recipe.abi(), abi);
    assert_eq!(
        recipe.effect,
        DeclaredInstanceCallSemanticEffectV1::OpaqueObservable
    );
    assert_eq!(
        recipe.physical_effect_mask(),
        crate::mir::canonical_direct_call::materialize_direct_call_effect_v1(
            crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1::ConservativeBarrier,
        ),
        "physical barrier bit policy must not drift; target issuers remain separate"
    );
    assert_eq!(recipe.target(), target);
}

#[test]
fn ordinary_new_recipe_target_is_selected_before_consumption() {
    let source_id = crate::parser::ConstructorSourceIdV1::test_new(8);
    let abi = InstanceConstructorAbiV1::issue(1).expect("constructor ABI");
    let target = CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 1);
    let recipe = VerifiedOrdinaryNewBirthRecipeV1 {
        source_id,
        target: target.clone(),
        effect: DeclaredInstanceCallSemanticEffectV1::OpaqueObservable,
        abi,
    };

    assert_eq!(&recipe.target, &target);
    assert_eq!(recipe.target(), target);
}

#[test]
fn ordinary_new_constructor_disposition_accepts_zero_arity_without_birth() {
    let site = test_site();
    let disposition = no_birth_constructor_disposition(&site, "Page", 0)
        .expect("zero-arity allocation may omit birth");
    assert!(matches!(
        disposition,
        OrdinaryNewConstructorDispositionV1::NoBirthZero
    ));
}

#[test]
fn ordinary_new_constructor_disposition_rejects_nonzero_without_birth() {
    let site = test_site();
    let error = no_birth_constructor_disposition(&site, "Page", 1)
        .expect_err("nonzero allocation requires an exact birth row");
    assert!(matches!(
        error,
        OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
            class,
            arity: 1,
            ..
        } if class.as_ref() == "Page"
    ));
}

#[test]
fn ordinary_new_local_commit_rejects_drift_without_consuming_pending_installation() {
    use crate::mir::ValueId;
    let site = test_site();
    let claim = claim(site.clone(), 0);
    let destination = claim.destination;
    let SourceBindingSiteV1::Local { statement, ordinal } = claim.declaration.clone() else {
        panic!("local claim");
    };
    let ledger = OrdinaryNewClaimLedgerV1::issue(
        vec![claim].into_boxed_slice(),
        vec!["Page".into(), "Other".into()].into_boxed_slice(),
    );
    let good = (destination, ordinal, ValueId(8), ValueId(9));
    assert!(ledger
        .complete_local_installation(site.owner(), statement.node(), &[good])
        .unwrap_err()
        .contains("local-before-target-take"));
    assert!(ledger
        .complete_new_expression(&site, "Page", ValueId(8))
        .unwrap_err()
        .contains("expression-without-target-take"));
    let taken = ledger.try_take(&site, "Page", 0).unwrap().unwrap();
    assert!(ledger
        .complete_new_expression(&site, "Page", ValueId(8))
        .unwrap_err()
        .contains("expression-before-emission"));
    assert!(!ledger.prepare_new_emission(&taken).unwrap());
    assert!(ledger
        .complete_new_expression(&site, "Other", ValueId(8))
        .unwrap_err()
        .contains("expression-parent-mismatch"));
    assert!(ledger
        .complete_local_installation(site.owner(), statement.node(), &[good])
        .unwrap_err()
        .contains("local-initializer-mismatch"));
    ledger
        .complete_new_expression(&site, "Page", ValueId(8))
        .unwrap();
    assert!(ledger
        .complete_new_expression(&site, "Page", ValueId(8))
        .unwrap_err()
        .contains("duplicate-expression-completion"));
    for bad in [
        (destination, ordinal + 1, ValueId(8), ValueId(9)),
        (
            BindingRefV1::new(site.owner(), hakorune_mir_core::BindingId::new(1)),
            ordinal,
            ValueId(8),
            ValueId(9),
        ),
        (destination, ordinal, ValueId(7), ValueId(9)),
        (destination, ordinal, ValueId(8), ValueId(8)),
    ] {
        assert!(ledger
            .complete_local_installation(site.owner(), statement.node(), &[bad])
            .is_err());
        assert!(!ledger.is_empty());
    }
    let foreign = test_site().owner();
    assert!(ledger
        .complete_local_installation(foreign, statement.node(), &[good])
        .unwrap_err()
        .contains("foreign-or-duplicate-local"));
    assert!(ledger
        .complete_local_installation(site.owner(), statement.node(), &[good, good])
        .unwrap_err()
        .contains("foreign-or-duplicate-local"));
    let wrong_statement = SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::Body(1)]);
    ledger
        .complete_local_installation(site.owner(), &wrong_statement, &[good])
        .unwrap();
    assert!(
        !ledger.is_empty(),
        "unrelated local cannot discharge selected obligation"
    );
    ledger
        .complete_local_installation(site.owner(), statement.node(), &[good])
        .unwrap();
    assert!(ledger.is_empty());
    assert!(ledger
        .complete_local_installation(site.owner(), statement.node(), &[good])
        .unwrap_err()
        .contains("duplicate-local-installation"));
}
