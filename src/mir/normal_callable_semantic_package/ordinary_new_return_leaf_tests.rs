use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};

fn package() -> VerifiedNormalCallableSemanticPackageV1 {
    issue_with_brand_catalog("box Token {} static box Work { make(args) { local sibling = new Token() return new Token() } other(args) { return new Token() } } static box Main { main() { return 0 } }").unwrap()
}
fn fresh(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (CanonicalSameModuleCallableKeyV1, Rc<ResultOriginWitnessV1>) {
    let key = CanonicalSameModuleCallableKeyV1::static_box_method("Work", "make", 1);
    let witness = Rc::clone(
        &package
            .ordinary_new_claim_ledger
            .callable_result_classes
            .outcomes(&key)
            .unwrap()[0]
            .witnesses()[0],
    );
    (key, witness)
}

#[test]
fn exact_fresh_leaf_corroborates_without_consuming_claim_or_sibling_home() {
    for (text, fault_count, normal_count, relation_count) in [
        ("box Token {} static box Work { make(args) { local sibling = new Token() return new Token() } } static box Main { main() { return 0 } }", 1, 1, 0),
        ("box Token {} box Wrap { child: Token birth(child) { me.child = child } } static box Work { make(args) { local sibling = new Token() local child = new Token() return new Wrap(child) } } static box Main { main() { return 0 } }", 2, 1, 0),
        ("box Token {} box Wrap { n: i64 birth(n) { me.n = n } } static box Work { make(args) { local sibling = new Token() local n = 7 return new Wrap(n) } } static box Main { main() { return 0 } }", 1, 1, 0),
        ("box Token {} box Wrap { flag: i64 child: Token birth(flag, child) { me.flag = flag me.child = child } } static box Work { make(args) { local sibling = new Token() return new Wrap(1, null) } } static box Main { main() { return 0 } }", 1, 1, 1),
        ("box Token {} box Wrap { child: Token birth(child) { me.child = child } } static box Work { make(args: Token) { local sibling = new Token() return new Wrap(args) } } static box Main { main() { return 0 } }", 1, 1, 1),
    ] {
        let package = issue_with_brand_catalog(text).unwrap();
        let (key, witness) = fresh(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let claims = ledger.result_claims.borrow();
        let original = &claims[witness.site()];
        let teardown = ObjectReturnTeardownDescriptorV1::from_exact_claim(original);
        let arguments = original.core.argument_rows.as_ref().unwrap().clone();
        assert_eq!(arguments.iter().map(|argument| argument.child_relations().len()).sum::<usize>(),
            relation_count, "nonempty source loans are required for the erasure negative");
        assert_eq!(original.home_prefix().unwrap().prior_homes().len(), fault_count);
        assert_eq!(original.home_prefix().unwrap().normal_homes().len(), normal_count);
        let expected_normal = original.home_prefix().unwrap().prior_homes()[fault_count - normal_count..].to_vec();
        assert_eq!(original.home_prefix().unwrap().normal_homes(), expected_normal.as_slice(),
            "the original sibling remains; the moved child cannot replace it");
        let original_count = claims.len();
        drop(claims);
        for _ in 0..2 {
            assert_eq!(
                ledger
                    .checked_object_return_leaf_v1(&key, &witness)
                    .unwrap(),
                Some(VerifiedObjectReturnLeafV1::Fresh {
                    site: witness.site().clone(),
                    teardown: teardown.clone(),
                    arguments: arguments.clone(),
                })
            );
        }
        if arguments.iter().any(|argument| !argument.child_relations().is_empty()) {
            let mut claims = ledger.result_claims.borrow_mut();
            let rows = claims.get_mut(witness.site()).unwrap().core.argument_rows.as_mut().unwrap();
            let index = rows.iter().position(|argument| !argument.child_relations().is_empty()).unwrap();
            let original = rows[index].clone();
            rows[index].retain_child_relations(Box::new([]));
            drop(claims);
            assert!(ledger.checked_object_return_leaf_v1(&key, &witness).unwrap_err()
                .contains("leaf-child-relation-identity"), "erased loan cannot erase the original relation");
            ledger.result_claims.borrow_mut().get_mut(witness.site()).unwrap()
                .core.argument_rows.as_mut().unwrap()[index] = original;
            assert!(ledger.checked_object_return_leaf_v1(&key, &witness).unwrap().is_some());
        }
        let leaf = ledger.checked_object_return_leaf_v1(&key, &witness).unwrap().unwrap();
        let supports = ledger.object_return_leaf_supports_v1(&leaf).unwrap();
        assert_eq!(supports.len(), usize::from(text.contains("args: Token")));
        assert_eq!(leaf.requires_anchored_field_end_v1(), !supports.is_empty());
        let end_requires_residence = !supports.is_empty();
        for support in supports {
            assert_eq!(support.ordinal, 0);
            assert_eq!(support.formal.owner(), witness.site().owner());
            assert!(arguments.iter().flat_map(|row| row.child_relations()).any(|relation|
                Rc::ptr_eq(relation, &support.relation)
                    && relation.witnesses().iter().any(|witness| Rc::ptr_eq(witness, &support.witness))));
        }
        let end = super::super::return_handoff::reduce_object_return_teardown_v1(&[
            super::super::return_handoff::VerifiedObjectReturnAlternativeV1::Leaf(leaf),
        ]).unwrap();
        assert_eq!(end.descriptor().is_none(), end_requires_residence,
            "source-supported child cannot activate owning-only teardown");
        assert_eq!(ledger.result_claims.borrow().len(), original_count);
        assert_eq!(
            ledger.result_claims.borrow()[witness.site()]
                .home_prefix()
                .unwrap()
                .prior_homes()
                .len(),
            fault_count
        );
        assert_eq!(ledger.result_claims.borrow()[witness.site()].home_prefix().unwrap().normal_homes(), expected_normal.as_slice());
    }
}

#[test]
fn same_class_other_claim_cannot_replace_missing_exact_returned_leaf() {
    let mut package = package();
    let (key, witness) = fresh(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .result_claims
        .borrow_mut()
        .remove(witness.site())
        .unwrap();
    assert!(ledger
        .result_claims
        .borrow()
        .values()
        .any(|claim| claim.class() == "Token"));
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}

#[test]
fn foreign_facts_leaf_and_missing_completion_cannot_corroborate() {
    let foreign = package();
    let (_, foreign_witness) = fresh(&foreign);
    let mut package = package();
    let (key, witness) = fresh(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert!(ledger
        .checked_object_return_leaf_v1(&key, &foreign_witness)
        .unwrap_err()
        .contains("foreign-leaf"));
    assert!(ledger
        .completion_index
        .remove(&witness.site().owner())
        .unwrap()
        .is_ok());
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}

#[test]
fn unavailable_exact_prefix_cannot_borrow_another_successful_claim() {
    let package = package();
    let (key, witness) = fresh(&package);
    package.ordinary_new_claim_ledger.result_claims.borrow_mut().get_mut(witness.site()).unwrap().home_prefix = Err(
        crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::TerminalNotCovered,
    );
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}

#[test]
fn exact_null_leaf_retains_nullable_class_without_issuing_a_fresh_home() {
    let package = issue_with_brand_catalog("box Token {} box Maker { birth() {} make(size: i64) { if size == 0 { return null } return new Token() } } static box Main { main() { return 0 } }").unwrap();
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "make", 1);
    let ledger = &package.ordinary_new_claim_ledger;
    let witness = ledger
        .callable_result_classes
        .outcomes(&key)
        .unwrap()
        .iter()
        .flat_map(|row| row.witnesses())
        .find(|witness| matches!(witness.step(), ResultWitnessStepV1::NullLiteral))
        .unwrap();
    assert_eq!(
        ledger.checked_object_return_leaf_v1(&key, witness).unwrap(),
        Some(VerifiedObjectReturnLeafV1::Null)
    );
    assert!(!ledger.result_claims.borrow().contains_key(witness.site()));
}

#[test]
fn foreign_completion_cannot_be_borrowed_through_matching_map_key() {
    let mut package = package();
    let (key, witness) = fresh(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let foreign = ledger
        .completion_index
        .iter()
        .find(|(owner, _)| **owner != witness.site().owner())
        .map(|(_, row)| row.as_ref().unwrap().clone())
        .unwrap();
    ledger
        .completion_index
        .insert(witness.site().owner(), Ok(foreign));
    assert!(ledger
        .checked_object_return_leaf_v1(&key, &witness)
        .unwrap_err()
        .contains("leaf-completion-owner"));
}

#[test]
fn rejected_indexed_completion_cannot_retry_a_matching_root_completion() {
    let mut package = package();
    let (key, witness) = fresh(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = witness.site().owner();
    let original = ledger.completion_index[&owner].as_ref().unwrap().clone();
    ledger.root_completion = Some(Ok(original));
    ledger.completion_index.insert(
        owner,
        Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::OwnerClosureMismatch),
    );
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}

#[test]
fn another_claims_successful_prefix_cannot_replace_exact_returned_prefix() {
    let package = package();
    let (key, witness) = fresh(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let mut claims = ledger.result_claims.borrow_mut();
    let other_prefix = claims
        .iter()
        .find(|(site, claim)| *site != witness.site() && claim.home_prefix().is_ok())
        .map(|(_, claim)| claim.home_prefix().unwrap().clone())
        .unwrap();
    claims.get_mut(witness.site()).unwrap().home_prefix = Ok(other_prefix);
    drop(claims);
    assert!(ledger
        .checked_object_return_leaf_v1(&key, &witness)
        .unwrap_err()
        .contains("leaf-prefix-identity"));
}

#[test]
fn moved_original_terminal_cannot_match_another_exit_coordinate() {
    let mut package = package();
    let (key, witness) = fresh(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = witness.site().owner();
    let mut index = ledger.terminal_relation_index[&owner].as_ref().clone();
    let exact_exit = index
        .iter()
        .find(|(_, relation)| {
            matches!(relation,
        TerminalRelationV1::Value(row) if row.value_site() == witness.site().site())
        })
        .map(|(exit, _)| exit.clone())
        .unwrap();
    let original = index.remove(&exact_exit).unwrap();
    let other_exit = ledger
        .terminal_relation_index
        .iter()
        .filter(|(other, _)| **other != owner)
        .flat_map(|(_, rows)| rows.keys())
        .find(|exit| **exit != exact_exit)
        .unwrap()
        .clone();
    index.insert(other_exit, original);
    ledger.terminal_relation_index.insert(owner, Rc::new(index));
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}

#[test]
fn exact_null_source_completion_without_cleanup_cannot_corroborate() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { birth() {} make(size: i64) { if size == 0 { return null } return new Token() } } static box Main { main() { return 0 } }").unwrap();
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "make", 1);
    let witness = package
        .ordinary_new_claim_ledger
        .callable_result_classes
        .outcomes(&key)
        .unwrap()
        .iter()
        .flat_map(|row| row.witnesses())
        .find(|row| matches!(row.step(), ResultWitnessStepV1::NullLiteral))
        .unwrap()
        .clone();
    let owner = witness.site().owner();
    let slot = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap()
        .batch_slot;
    // Same original canonical source, deliberately without the Home analysis.
    let source_only = package
        .batch
        .with_lowering_input(
            slot,
            crate::mir::resolved_control_flow::verify_function_completion_v1,
        )
        .unwrap()
        .unwrap();
    assert_eq!(source_only.owner(), owner);
    assert!(source_only.cleanup().root_flow().is_none());
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        Some(VerifiedObjectReturnLeafV1::Null)
    );
    ledger
        .completion_index
        .insert(owner, Ok(Rc::new(source_only)));
    assert_eq!(
        ledger
            .checked_object_return_leaf_v1(&key, &witness)
            .unwrap(),
        None
    );
}
