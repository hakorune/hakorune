//! Packet selection is final incoming + actuals + original Completion, not inventory alone.
use super::*;
const TEXT: &str = "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }";
fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        TEXT,
    )
    .unwrap()
}
#[test]
fn static_packet_selection_retains_exact_final_source_and_sealed_routes() {
    let package = package();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let mut count = 0;
    for row in &source.incoming {
        let super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &row.source
        else {
            continue;
        };
        let selected = ledger
            .selected_static_local_source_v1(&row.call)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(original, &selected));
        assert!(ledger
            .has_routed_static_local_for_owner_v1(row.call.owner())
            .unwrap());
        assert_eq!(
            ledger
                .borrowed_static_packet_actuals_v1(original)
                .unwrap()
                .unwrap()
                .len(),
            1
        );
        count += 1;
    }
    assert_eq!(count, 2);
    let zero = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { word() { return 8 } run() { local a = me.word() local b = me.word() return 0 } } static box Main { main() { return 0 } }",
    ).unwrap();
    let ledger = &zero.ordinary_new_claim_ledger;
    let originals: Vec<_> = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .filter_map(|row| match &row.source {
            super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                original,
            ) if original.current_owner_source().is_some() => Some(original),
            _ => None,
        })
        .collect();
    assert_eq!(originals.len(), 2);
    for original in originals {
        assert!(Rc::ptr_eq(
            original,
            &ledger
                .selected_static_local_source_v1(original.call_site())
                .unwrap()
                .unwrap()
        ));
        assert!(ledger
            .borrowed_static_packet_actuals_v1(original)
            .unwrap()
            .unwrap()
            .is_empty());
        assert!(ledger
            .has_routed_static_local_for_owner_v1(original.call_site().owner())
            .unwrap());
    }
}
#[test]
fn static_packet_selection_refuses_missing_and_rejected_callee_completion() {
    for rejected in [false, true] {
        let mut package = package();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let row = source
            .incoming
            .iter()
            .find(|row| {
                matches!(
                    row.source,
                    super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(_)
                )
            })
            .unwrap();
        let site = row.call.clone();
        let owner = row.callee;
        if rejected {
            ledger.completion_index.insert(owner, Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::OwnerClosureMismatch));
        } else {
            ledger.completion_index.remove(&owner);
        }
        assert!(ledger
            .selected_static_local_source_v1(&site)
            .unwrap_err()
            .contains("callee-completion-missing"));
    }
}

#[test]
fn static_packet_physical_selector_preserves_original_error_demand_scope() {
    let mut package = package();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let selected = ledger
        .borrowed_static_source_sites
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone();
    let sibling = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming
        .iter()
        .find(|row| {
            row.source.as_loan().target().namespace()
                == crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod
        })
        .unwrap()
        .call
        .clone();
    ledger.borrowed_formal_source = Some(Err("physical-ingress-sentinel".into()));
    assert!(ledger
        .selected_static_local_source_v1(&sibling)
        .unwrap()
        .is_none());
    assert_eq!(
        ledger
            .selected_static_local_source_v1(&selected)
            .unwrap_err(),
        "physical-ingress-sentinel"
    );
    ledger.co_seal_static_local_routes_v1().unwrap();
    assert!(!ledger
        .has_routed_static_local_for_owner_v1(selected.owner())
        .unwrap());
    ledger.borrowed_static_source_sites = Some(Ok(std::collections::BTreeSet::new()));
    assert!(ledger
        .selected_static_local_source_v1(&selected)
        .unwrap()
        .is_none());
    ledger.co_seal_static_local_routes_v1().unwrap();
    assert!(!ledger
        .has_routed_static_local_for_owner_v1(selected.owner())
        .unwrap());
    ledger.borrowed_static_source_sites = Some(Err("physical-selection-sentinel".into()));
    assert_eq!(
        ledger
            .selected_static_local_source_v1(&selected)
            .unwrap_err(),
        "physical-selection-sentinel"
    );
}
#[test]
fn static_packet_lender_rejects_same_shape_scalar_actual_drift() {
    let text = "static box Layout { pick(p, q: i64) { return q } } static box Main { main() { local a = Layout.pick(8, 9) return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let row = &ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming[0];
    let site = row.call.clone();
    let actuals = ledger
        .borrowed_formal_actuals
        .get_mut(&site)
        .unwrap()
        .as_mut()
        .unwrap();
    assert!(matches!(
        actuals.ordered_arguments[1],
        crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(9)
    ));
    actuals.ordered_arguments[1] =
        crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(10);
    assert!(ledger
        .selected_static_local_source_v1(&site)
        .unwrap_err()
        .contains("ordered-arguments-drift"));
}

#[test]
fn static_packet_route_preflight_refuses_partial_marking_on_missing_completion() {
    let mut package = package();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let owner = source
        .incoming
        .iter()
        .find(|row| {
            matches!(
                row.source,
                super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(_)
            )
        })
        .unwrap()
        .callee;
    ledger.lifecycle_local_call_sites.borrow_mut().clear();
    ledger.completion_index.remove(&owner);
    assert!(ledger
        .co_seal_static_local_routes_v1()
        .unwrap_err()
        .contains("callee-completion-missing"));
    assert!(ledger.lifecycle_local_call_sites.borrow().is_empty());
}

#[test]
fn static_zero_packet_selection_keeps_pending_demands_and_refuses_before_publication() {
    const ZERO: &str = "static box Layout { word() { return 8 } run() { local a = me.word() local b = Layout.word() return 0 } } static box Main { main() { return 0 } }";
    for mutation in 0..6 {
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(ZERO).unwrap();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let calls: Vec<_> = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .source_incoming
            .exact_rows()
            .filter_map(|row| match &row.source {
                super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                    original,
                ) if original.target().name() == "word" => Some(Rc::clone(original)),
                _ => None,
            })
            .collect();
        assert_eq!(calls.len(), 2);
        ledger.lifecycle_local_call_sites.borrow_mut().clear();
        match mutation {
            0 => {
                ledger
                    .completion_index
                    .remove(&calls[0].call_site().owner());
            }
            1 => {
                ledger.borrowed_formal_actuals.remove(calls[1].call_site());
            }
            2 => {
                ledger.borrowed_formal_actuals.insert(
                    calls[1].call_site().clone(),
                    Err("original-packet-sibling-refusal".into()),
                );
            }
            3 => {
                ledger.completion_index.remove(&calls[0].callee_owner());
            }
            4 => {
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .source_incoming
                    .remove_exact_row_for_test(calls[1].call_site());
            }
            5 => {
                ledger.borrowed_formal_source = None;
            }
            _ => unreachable!(),
        }
        ledger.select_static_zero_local_routes_v1();
        ledger.co_seal_static_local_routes_v1().unwrap();
        assert!(
            ledger.lifecycle_local_call_sites.borrow().is_empty(),
            "no partial routes: {mutation}"
        );
        for call in &calls {
            assert!(ledger
                .borrowed_static_source_sites
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .contains(call.call_site()));
            let error = ledger
                .selected_static_local_source_v1(call.call_site())
                .unwrap_err();
            if mutation == 2 {
                assert_eq!(error, "original-packet-sibling-refusal");
            }
        }
    }
}
