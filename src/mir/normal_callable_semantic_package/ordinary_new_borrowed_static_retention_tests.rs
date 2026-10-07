//! Original package facts survive pruning without lending an executable entry.
use super::super::borrowed_formal_uses::{
    inventory_borrowed_incoming_calls_v1, StaticIncomingContextV1,
};
use super::*;
use std::rc::Rc;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
const SOURCE: &str = "static box Layout { class_id(a,b,c) { return 0 } } box Heap { lookup(size,other) { local k = Layout.class_id(size,other,3) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7,8) local x = Layout.class_id(1,2,3) return 0 } }";
fn package() -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        SOURCE,
    )
    .unwrap()
}

#[test]
fn static_retention_keeps_raw_opaque_facts_and_literal_sibling_after_pruning() {
    let package = package();
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(ingress.static_arguments.len(), 2);
    assert_eq!(ingress.source_incoming.static_observations().len(), 2);
    let facts: Vec<_> = ingress.static_arguments.values().collect();
    assert!(Rc::ptr_eq(
        facts[0].retained_call_source(),
        facts[1].retained_call_source()
    ));
    let source = facts[0].call_source();
    assert_eq!(source.argument_sites().len(), 3);
    assert_eq!(source.parameters().len(), 3);
    assert!(source.required_i64_arguments().is_empty());
    assert_eq!(facts[0].ordinal(), 0);
    assert_eq!(facts[1].ordinal(), 1);
    assert!(Rc::ptr_eq(
        ingress.source_incoming.static_observations()[source.call_site()]
            .as_ref()
            .unwrap(),
        facts[0].retained_call_source()
    ));
    assert!(
        !ingress
            .definitions
            .contains_key(&source.call_site().owner()),
        "raw caller removed by unchanged transport closure"
    );
    assert!(
        ingress.incoming.is_empty(),
        "no new final incoming transport"
    );
    assert!(
        ingress.definitions.is_empty(),
        "no Static seed or entry admission"
    );
    for fact in facts {
        assert_eq!(fact.binding().owner(), fact.call().owner());
        assert_eq!(fact.formal().owner(), fact.call().owner());
        assert_eq!(fact.use_site().owner(), fact.call().owner());
        assert_eq!(
            fact.target_formal(),
            source.parameters()[fact.ordinal() as usize].binding
        );
    }
}

#[test]
fn static_retention_main_literal_observation_uses_original_main_loan() {
    let package = package();
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let main_contract = package
        .parameter_contracts
        .iter()
        .find(|row| main.matches_contract(row))
        .unwrap();
    let observations: Vec<_> = ingress
        .source_incoming
        .static_observations()
        .iter()
        .filter(|(site, _)| site.owner() == main_contract.owner)
        .collect();
    assert_eq!(observations.len(), 1);
    let (site, observed) = observations[0];
    package
        .batch()
        .with_lowering_input(main.batch_slot(), |input| {
            let (_, call) = input
                .function()
                .method_calls()
                .find(|(key, _)| *key == site.site())
                .unwrap();
            let source = package
                .source_static_claims_for_test
                .incoming_source(
                    main.catalog_key(),
                    site,
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    Some(&main),
                )
                .unwrap()
                .unwrap();
            assert!(source.corroborates_retained(observed.as_ref().unwrap()));
        })
        .unwrap();
}

#[test]
fn static_retention_missing_main_authority_is_retained_error() {
    let package = package();
    let empty = BTreeMap::new();
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &empty,
        main: None,
    };
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &BTreeMap::new(),
        &Default::default(),
        Some(&context),
    )
    .unwrap();
    assert_eq!(inventory.static_observations().len(), 2);
    assert_eq!(
        inventory
            .static_observations()
            .values()
            .filter(|row| row.is_err())
            .count(),
        1
    );
    assert!(inventory
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().err())
        .all(|error| error.contains("caller-key-unavailable")));
    assert!(inventory.exact_rows().next().is_none());
}

#[test]
fn static_retention_orphan_fact_key_refuses_inventory() {
    let package = package();
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap();
    let ledger = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ledger.static_arguments.values().next().unwrap();
    let foreign = OwnedExprSiteV1::new(fact.target_formal().owner(), fact.call().site().clone());
    let mut arguments = BTreeMap::new();
    arguments.insert(
        (foreign, fact.ordinal()),
        QualifiedStaticArgumentSourceV1 {
            source: Rc::clone(fact.retained_call_source()),
            ordinal: fact.ordinal(),
            use_site: fact.use_site().clone(),
            binding: fact.binding(),
            formal: fact.formal(),
        },
    );
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &arguments,
        main: main.as_ref(),
    };
    assert!(inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &BTreeMap::new(),
        &Default::default(),
        Some(&context)
    )
    .is_err());
}

#[test]
fn static_retention_foreign_index_and_reissued_sibling_refuse() {
    let package = package();
    let foreign = self::package();
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap();
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let context = StaticIncomingContextV1 {
        claims: &foreign.source_static_claims_for_test,
        arguments: &ingress.static_arguments,
        main: main.as_ref(),
    };
    assert!(inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &BTreeMap::new(),
        &Default::default(),
        Some(&context)
    )
    .is_err());
    let mut arguments = BTreeMap::new();
    let facts: Vec<_> = ingress.static_arguments.values().collect();
    let fact = facts[1];
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == fact.call().owner())
        .unwrap();
    let caller =
        caller_key_for_function(&package.selected, contract.batch_slot, false, None).unwrap();
    let reissued = package
        .batch()
        .with_lowering_input(contract.batch_slot, |input| {
            let (_, source) = input
                .function()
                .method_calls()
                .find(|(site, _)| *site == fact.call().site())
                .unwrap();
            package
                .source_static_claims_for_test
                .incoming_source(
                    &caller,
                    fact.call(),
                    source,
                    &package.selected,
                    &package.parameter_contracts,
                    main.as_ref(),
                )
                .unwrap()
                .unwrap()
                .retain()
        })
        .unwrap();
    assert!(!Rc::ptr_eq(&reissued, fact.retained_call_source()));
    for fact in facts {
        arguments.insert(
            (fact.call().clone(), fact.ordinal()),
            QualifiedStaticArgumentSourceV1 {
                source: if fact.ordinal() == 1 {
                    Rc::clone(&reissued)
                } else {
                    Rc::clone(fact.retained_call_source())
                },
                ordinal: fact.ordinal(),
                use_site: fact.use_site().clone(),
                binding: fact.binding(),
                formal: fact.formal(),
            },
        );
    }
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &arguments,
        main: main.as_ref(),
    };
    assert!(inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &BTreeMap::new(),
        &Default::default(),
        Some(&context)
    )
    .is_err());
}

#[test]
fn static_retention_absent_claim_keeps_unavailable_observation() {
    let package = package();
    let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("static box Layout { class_id(a,b,c) { return 0 } } static box Main { main() { return 0 } }").unwrap();
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap();
    let arguments = BTreeMap::new();
    let context = StaticIncomingContextV1 {
        claims: &foreign.source_static_claims_for_test,
        arguments: &arguments,
        main: main.as_ref(),
    };
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &BTreeMap::new(),
        &Default::default(),
        Some(&context),
    )
    .unwrap();
    assert_eq!(inventory.static_observations().len(), 2);
    assert!(inventory
        .static_observations()
        .values()
        .all(|row| row.as_ref().unwrap_err().contains("claim-unavailable")));
    assert!(inventory.exact_rows().next().is_none());
    assert!(borrow_app_main_source_v1(package.batch(), None)
        .unwrap()
        .is_none());
}
