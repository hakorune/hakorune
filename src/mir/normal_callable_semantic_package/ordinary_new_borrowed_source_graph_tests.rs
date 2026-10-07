//! Original rows and callee vetoes share one immutable inventory.
use super::super::super::borrowed_formal_uses::{borrow_call_sources_v1, BorrowedCallSourceLoanV1};
use super::*;
use std::rc::Rc;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
fn package(source: &str) -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        source,
    )
    .unwrap()
}
const INSTANCE: &str = "box Heap { lookup(p) { return 0 } unused(q) { return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }";
const STATIC: &str = "static box Layout { class_id(p) { local alias: i64 = p return 0 } } static box Main { main() { local k = Layout.class_id(7) return 0 } }";

#[test]
fn source_graph_retains_raw_no_incoming_and_projects_final_callee_only() {
    let package = package(INSTANCE);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let inventory = &source.source_incoming;
    assert_eq!(inventory.owners.len(), 2);
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory
        .observations
        .iter()
        .any(|(_, row)| matches!(row, Err(BorrowedIncomingDraftErrorV1::NoIncoming(_)))));
    assert!(inventory.project(&inventory.owners).is_err());
    let final_owners = source.definitions.keys().copied().collect();
    let projected = inventory.project(&final_owners).unwrap();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].call, source.incoming[0].call);
    assert!(matches!(
        projected[0].source,
        BorrowedIncomingSourceV1::Instance(_)
    ));
    assert_eq!(
        projected[0].source.require_instance().unwrap(),
        source.incoming[0].source.require_instance().unwrap()
    );
}

#[test]
fn source_graph_selected_callee_keeps_every_caller_veto_and_global_batch_fault() {
    let package = package(INSTANCE);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let row = source.incoming[0].clone();
    let owners = std::collections::BTreeSet::from([row.callee]);
    for error in [
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(row.call.clone()),
        BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(row.call.clone()),
        BorrowedIncomingDraftErrorV1::CallIdentity(row.call.clone()),
        BorrowedIncomingDraftErrorV1::NoIncoming(row.callee),
    ] {
        let inventory = BorrowedIncomingInventoryV1 {
            unsupported_static_spelling: Default::default(),
            unsupported_static_context: Default::default(),
            owners: owners.clone(),
            static_observations: BTreeMap::new(),
            observations: vec![
                (Some(row.callee), Ok(row.clone())),
                (Some(row.callee), Err(error.clone())),
            ],
        };
        assert_eq!(inventory.project(&owners).unwrap_err(), error);
        assert!(inventory.project(&Default::default()).unwrap().is_empty());
    }
    let inventory = BorrowedIncomingInventoryV1 {
        unsupported_static_spelling: Default::default(),
        unsupported_static_context: Default::default(),
        owners: owners.clone(),
        static_observations: BTreeMap::new(),
        observations: vec![(None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))],
    };
    assert_eq!(inventory.vetoed_owners(), inventory.owners);
    for selected in [owners, Default::default()] {
        assert_eq!(
            inventory.project(&selected).unwrap_err(),
            BorrowedIncomingDraftErrorV1::BatchLoan
        );
    }
}

#[test]
fn source_graph_static_owner_preserves_rc_and_refuses_instance_authority() {
    let package = package(STATIC);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = source
        .source_incoming
        .static_observations()
        .values()
        .next()
        .unwrap()
        .as_ref()
        .unwrap();
    let owned = BorrowedIncomingSourceV1::QualifiedStatic(Rc::clone(original));
    assert_eq!(owned.call_site(), original.call_site());
    assert_eq!(owned.target(), original.target());
    assert_eq!(owned.argument_sites(), original.argument_sites());
    assert!(
        matches!(owned.clone(), BorrowedIncomingSourceV1::QualifiedStatic(row) if Rc::ptr_eq(&row, original))
    );
    assert!(
        matches!(owned.as_loan(), BorrowedCallSourceLoanV1::QualifiedStatic(row) if std::ptr::eq(row, original.as_ref()))
    );
    assert!(owned
        .require_instance()
        .unwrap_err()
        .contains("instance-source-required"));
    assert!(owned.object_return_source().is_none());
    assert!(owned.object_source_forwards().is_none());
    assert!(owned.object_producer_dependencies().is_none());
    assert!(owned.stored_receiver().is_none());
    assert!(source.definitions.is_empty());
    assert!(source.incoming.is_empty());
}

#[test]
fn source_graph_common_loan_refuses_instance_collision_and_equal_reissued_static() {
    let static_package = package(STATIC);
    let source = static_package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = source
        .source_incoming
        .static_observations()
        .values()
        .next()
        .unwrap()
        .as_ref()
        .unwrap();
    let instance_package = package(INSTANCE);
    let instance = instance_package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming[0]
        .source
        .require_instance()
        .unwrap();
    let collision = BTreeMap::from([(original.call_site().clone(), instance)]);
    assert!(borrow_call_sources_v1(&collision, [original.as_ref()])
        .unwrap_err()
        .contains("conflicting-source-call"));
    let main = super::super::super::borrow_app_main_source_v1(
        static_package.batch(),
        static_package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    let reissued = static_package
        .batch()
        .with_lowering_input(main.batch_slot(), |input| {
            let (_, call) = input.function().method_calls().next().unwrap();
            static_package
                .source_static_claims_for_test
                .incoming_source(
                    main.catalog_key(),
                    original.call_site(),
                    call,
                    &static_package.selected,
                    &static_package.parameter_contracts,
                    Some(&main),
                )
                .unwrap()
                .unwrap()
                .retain()
        })
        .unwrap();
    assert!(!Rc::ptr_eq(original, &reissued));
    assert!(
        borrow_call_sources_v1(&BTreeMap::new(), [original.as_ref(), reissued.as_ref()]).is_err()
    );
    assert_eq!(
        borrow_call_sources_v1(&BTreeMap::new(), [original.as_ref(), original.as_ref()])
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn source_graph_original_scan_retains_one_exact_caller_and_missing_sibling_veto() {
    let package = package("box Heap { lookup(p) { return 0 } } static box Main { main() { local heap = new Heap() local a = heap.lookup(7) local b = heap.lookup(8) return 0 } }");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.incoming.len(), 2);
    let row = &source.incoming[0];
    let omitted = &source.incoming[1];
    let calls = BTreeMap::from([(row.call.clone(), row.source.require_instance().unwrap())]);
    let scopes = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &source.definitions,
        &package.parameter_contracts,
        &calls,
        &scopes,
        None,
    )
    .unwrap();
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory.observations.iter().any(|(owner, observation)| *owner == Some(omitted.callee) && matches!(observation, Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(site)) if site == &omitted.call)));
    assert_eq!(
        inventory
            .project(&source.definitions.keys().copied().collect())
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(omitted.call.clone())
    );
}

#[test]
fn source_graph_partial_scan_preserves_global_loan_fault_before_orphan_check() {
    let package = package("static box Layout { class_id(p) { return 0 } } box Heap { lookup(p) { local k = Layout.class_id(p) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.static_arguments.len(), 1);
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &source.static_arguments,
        main: None,
    };
    let mut inventory = BorrowedIncomingInventoryV1 {
        unsupported_static_spelling: Default::default(),
        unsupported_static_context: Default::default(),
        owners: Default::default(),
        static_observations: BTreeMap::new(),
        observations: vec![(None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))],
    };
    inventory
        .corroborate_scan_completeness(&Default::default(), Some(&context))
        .unwrap();
    assert_eq!(
        inventory.project(&Default::default()).unwrap_err(),
        BorrowedIncomingDraftErrorV1::BatchLoan
    );
    inventory.observations.clear();
    assert_eq!(
        inventory
            .corroborate_scan_completeness(&Default::default(), Some(&context))
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::SourceIdentity
    );
}

#[test]
fn source_graph_final_instance_correspondence_rejects_owned_source_mutation() {
    let mut package = package(INSTANCE);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let prepared = ledger.lexical_instance_calls.borrow().values().filter_map(|slot| {
        match slot {
            crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
            _ => None,
        }
    }).collect::<Vec<_>>();
    assert!(!prepared.is_empty(), "original issued affine targets");
    let source = ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    source.corroborate_source_targets(&prepared).unwrap();
    source.incoming[0].source.instance_mut().target_batch_slot += 1;
    assert!(source
        .corroborate_source_targets(&prepared)
        .unwrap_err()
        .contains("final-incoming-drift"));
}
