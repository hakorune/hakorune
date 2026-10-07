//! Passive source requirements preserve the original issuer and result authority.
use super::*;

const SOURCE: &str = "box Item { value: i64 birth() { me.value = 5 } }
    box Leaf { flag: i64 birth() { me.flag = 0 }
        read(p: Item) { if p == null { return 7 } return p.value } }
    box Parent { child: Leaf birth() { me.child = new Leaf() }
        run(p: Item) { return me.child.read(p) } }
    static box Main { main() { local parent = new Parent() local item = new Item()
        return parent.run(item) } }";

#[test]
fn lexical_source_requirement_keeps_original_lexical_and_stored_defaults() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(SOURCE)
        .expect("original source-issued rows");
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let mut stored = 0;
    let mut lexical = 0;
    for incoming in &source.incoming {
        let target = &incoming.source;
        assert!(matches!(
            target.result_requirement,
            LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
        ));
        assert!(!target.has_object_source_requirement());
        assert!(target.object_return_source().is_none());
        assert!(target.object_producer_dependencies().is_none());
        assert!(target.object_source_forwards().is_none());
        if target.stored_receiver().is_some() {
            stored += 1;
        } else {
            lexical += 1;
        }
    }
    assert!(
        stored > 0 && lexical > 0,
        "both original constructors exercised"
    );
}

#[test]
fn lexical_source_requirement_borrows_original_witnesses_without_arming_a_call() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(SOURCE)
        .expect("original source-issued rows");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let mut target = source
        .incoming
        .iter()
        .find(|row| row.source.stored_receiver().is_some())
        .unwrap()
        .source
        .clone();
    let original = target.clone();
    let facts = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_result_facts_for_test(
        "box Token { birth() { } } box Door { make() { return new Token() }
         relay() { return me.make() } } static box Main { main() { return 0 } }",
    );
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", "relay", 0);
    let exit = &facts.outcomes(&key).unwrap()[0];
    let qualification = facts.object_return_qualification(exit.site()).unwrap();
    target.result_requirement = LexicalCallSourceResultRequirementV1::ObjectProducerDependency(
        vec![qualification.clone()].into_boxed_slice(),
    );
    assert!(target.has_object_source_requirement());
    assert!(target.object_return_source().is_none());
    assert!(target.object_source_forwards().is_none());
    assert_eq!(
        target.object_producer_dependencies().unwrap(),
        &[qualification.clone()]
    );
    target.result_requirement = LexicalCallSourceResultRequirementV1::ObjectReturnSource {
        qualification: qualification.clone(),
        forwards: Box::new([]),
    };
    let loan = target.object_return_source().unwrap();
    assert_eq!(loan, &qualification);
    for (actual, original) in loan.witnesses().iter().zip(exit.witnesses()) {
        assert!(std::rc::Rc::ptr_eq(actual, original));
    }
    assert!(target.object_producer_dependencies().is_none());
    assert_eq!(target.object_source_forwards().unwrap().len(), 0);
    assert_eq!(target.call_site(), original.call_site());
    assert_eq!(target.receiver_site(), original.receiver_site());
    assert_eq!(target.target(), original.target());
    assert_eq!(target.argument_sites(), original.argument_sites());
    // Only this test-local passive model changes; no ledger disposition is replaced.
    assert_eq!(
        &source
            .incoming
            .iter()
            .find(|row| row.source.stored_receiver().is_some())
            .unwrap()
            .source,
        &original
    );
}
