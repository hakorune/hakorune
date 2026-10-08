use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use std::rc::Rc;

fn package(
    received: bool,
    opaque: bool,
    nullable: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    let relay = if received {
        "local item = me.make(7) return item"
    } else {
        "return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ {body} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn rearm_pending(ledger: &OrdinaryNewClaimLedgerV1) -> Vec<OwnedExprSiteV1> {
    let mut slots = ledger.lexical_instance_calls.borrow_mut();
    let mut sites = Vec::new();
    for (site, slot) in slots.iter_mut() {
        if let LexicalInstanceCallDispositionSlotV1::Ready(row) = slot {
            if row.source.has_object_source_requirement() {
                let source = row.source.clone();
                *slot = LexicalInstanceCallDispositionSlotV1::SourcePending(source);
                sites.push(site.clone());
            }
        }
    }
    assert!(!sites.is_empty());
    sites
}

fn receiver_source_identity(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (
    OwnedExprSiteV1,
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
) {
    package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow()
        .iter()
        .find_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row) => {
                Some((site.clone(), row.target().clone()))
            }
            _ => None,
        })
        .unwrap()
}

#[test]
fn object_receiver_selection_refuses_foreign_locator_before_affine_take() {
    let package = package(true, false, false);
    let (site, key) = receiver_source_identity(&package);
    let wrong = package
        .batch()
        .declarations()
        .find_map(
            |row| match package.selected.key_for_batch_slot(row.batch_slot()) {
                Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "relay" => {
                    Some(key)
                }
                _ => None,
            },
        )
        .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), wrong)
        .unwrap_err()
        .contains("source-identity"));
    assert!(matches!(
        ledger.lexical_instance_calls.borrow().get(&site),
        Some(LexicalInstanceCallDispositionSlotV1::Ready(_))
    ));
    assert!(ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), &key)
        .unwrap()
        .is_some());
}

#[test]
fn object_receiver_selection_keeps_pending_and_duplicate_source_failures() {
    let package = package(true, false, false);
    let (site, key) = receiver_source_identity(&package);
    rearm_pending(&package.ordinary_new_claim_ledger);
    let error = package
        .ordinary_new_claim_ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), &key)
        .unwrap_err();
    assert!(error.contains("source-pending"));
    assert!(error.contains(&format!("{site:?}")));
    assert!(error.contains(&format!("{key:?}")));
    assert!(matches!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_calls
            .borrow()
            .get(&site),
        Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_))
    ));
    let mut package = self::package(true, false, false);
    let (site, key) = receiver_source_identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap()
        .source_incoming
        .duplicate_object_row_for_test(&site);
    assert!(ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), &key)
        .unwrap_err()
        .contains("source-duplicate"));
    assert!(matches!(
        ledger.lexical_instance_calls.borrow().get(&site),
        Some(LexicalInstanceCallDispositionSlotV1::Ready(_))
    ));
}

fn legacy_nullable_receiver_package() -> VerifiedNormalCallableSemanticPackageV1 {
    issue_with_brand_catalog("box Probe { init { v } birth(v) { me.v = v } fetch(flag: i64) { if flag == 0 { return null } return new Probe(7) } run(flag: i64) { local h = me.fetch(flag) return 0 } } static box Main { main() { return 0 } }").unwrap()
}

#[test]
fn object_receiver_selection_preserves_legacy_nullable_without_new_ingress_demand() {
    let mut package = legacy_nullable_receiver_package();
    let (site, key) = receiver_source_identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert!(
        matches!(ledger.lexical_instance_calls.borrow().get(&site), Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if !row.source.has_object_source_requirement())
    );
    ledger.borrowed_formal_source = Some(Err("unrelated-input-refusal".into()));
    assert!(ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), &key)
        .unwrap()
        .is_none());
    assert!(ledger.nullable_call_source(&site).is_some());
    assert!(matches!(
        ledger.lexical_instance_calls.borrow().get(&site),
        Some(LexicalInstanceCallDispositionSlotV1::Ready(_))
    ));
}

#[test]
fn object_receiver_selection_does_not_hide_duplicate_original_legacy_sources() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { make(flag) { if flag > 0 { return new Token() } return null } run() { local item = me.make(7) return 0 } } static box Main { main() { return 0 } }").unwrap();
    let (site, key) = receiver_source_identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert!(
        matches!(ledger.lexical_instance_calls.borrow().get(&site), Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if !row.source.has_object_source_requirement())
    );
    ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap()
        .source_incoming
        .duplicate_object_row_for_test(&site);
    assert!(ledger
        .take_receiver_object_packet_v1(site.owner(), site.site(), &key)
        .unwrap_err()
        .contains("source-duplicate"));
}

#[test]
fn object_slot_production_closes_typed_opaque_direct_received_handle_nullable() {
    for received in [false, true] {
        for opaque in [false, true] {
            for nullable in [false, true] {
                let package = package(received, opaque, nullable);
                let ledger = &package.ordinary_new_claim_ledger;
                let slots = ledger.lexical_instance_calls.borrow();
                let rows: Vec<_> = slots
                    .values()
                    .filter_map(|slot| match slot {
                        LexicalInstanceCallDispositionSlotV1::Ready(row)
                            if row.source.has_object_source_requirement() =>
                        {
                            Some(row)
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(rows.len(), 1);
                assert_eq!(
                    rows[0].result(),
                    Some(if nullable {
                        InvokeCallResultKind::NullableHandle
                    } else {
                        InvokeCallResultKind::Handle
                    })
                );
                assert!(ledger.lexical_instance_call_covered(
                    rows[0].call_site().owner(),
                    rows[0].call_site().site()
                ));
                assert!(ledger.validate_no_pending_object_returns_v1().is_err());
            }
        }
    }
}

#[test]
fn object_slot_pending_take_preserves_source_and_ready_is_affine() {
    let package = package(false, false, false);
    let ledger = &package.ordinary_new_claim_ledger;
    let sites = rearm_pending(ledger);
    let site = &sites[0];
    for _ in 0..2 {
        assert!(ledger
            .take_lexical_instance_call(site.owner(), site.site())
            .unwrap_err()
            .contains("source-pending"));
        assert!(!ledger.lexical_instance_call_covered(site.owner(), site.site()));
        assert!(
            matches!(ledger.lexical_instance_calls.borrow().get(site), Some(LexicalInstanceCallDispositionSlotV1::SourcePending(source)) if source.call_site() == site)
        );
    }
    let mut package = package;
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .finish_object_lexical_slots_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts,
        )
        .unwrap();
    assert!(ledger
        .take_lexical_instance_call(site.owner(), site.site())
        .unwrap()
        .is_some());
    assert!(ledger
        .take_lexical_instance_call(site.owner(), site.site())
        .unwrap_err()
        .contains("already-taken"));
}

#[test]
fn object_slot_missing_input_stays_pending_and_foreign_signature_still_refuses() {
    let foreign = package(false, false, false);
    let mut package = package(false, false, false);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let sites = rearm_pending(ledger);
    ledger.borrowed_formal_actuals.remove(&sites[0]);
    ledger
        .finish_object_lexical_slots_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts,
        )
        .unwrap();
    assert!(matches!(
        ledger.lexical_instance_calls.borrow().get(&sites[0]),
        Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_))
    ));
    assert!(ledger
        .finish_object_lexical_slots_v1(
            &package.selected,
            &package.parameter_contracts,
            &foreign.physical_signature,
            &package.result_contracts
        )
        .unwrap_err()
        .contains("signature-identity"));
}

#[test]
fn object_slot_late_input_refusal_leaves_all_upgrades_and_routes_uninstalled() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { left(size: i64) { return new Token() } right(size: i64) { return new Token() } first() { return me.left(7) } second() { return me.right(8) } } static box Main { main() { return 0 } }").unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let sites = rearm_pending(ledger);
    assert_eq!(sites.len(), 2);
    let routes = ledger.lifecycle_local_call_sites.borrow().clone();
    ledger
        .borrowed_formal_actuals
        .insert(sites[1].clone(), Err("late-original-input-refusal".into()));
    assert_eq!(
        ledger
            .finish_object_lexical_slots_v1(
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
                &package.result_contracts
            )
            .unwrap_err(),
        "late-original-input-refusal"
    );
    for site in sites {
        assert!(matches!(
            ledger.lexical_instance_calls.borrow().get(&site),
            Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_))
        ));
    }
    assert_eq!(*ledger.lifecycle_local_call_sites.borrow(), routes);
}

#[test]
fn object_slot_nested_unqualified_producer_retains_dependencies_and_closes() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } ignored() { local item = me.relay() return 0 } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    let row = slots
        .values()
        .find_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source.object_producer_dependencies().is_some() =>
            {
                Some(row)
            }
            _ => None,
        })
        .unwrap();
    assert!(row.source.object_return_sources().is_none());
    assert_eq!(
        row.source.object_producer_dependencies(),
        ledger
            .callable_result_classes
            .object_return_dependencies(row.target(), row.callee_owner())
            .as_deref()
    );
    assert_eq!(row.result(), Some(InvokeCallResultKind::NullableHandle));
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
}

#[test]
fn object_slot_received_call_checks_all_original_return_qualifications() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { return new Token() } relay() { local item = me.make(7) if 1 > 0 { return item } return item } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    let row = slots
        .values()
        .find_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source.object_return_sources().is_some() =>
            {
                Some(row)
            }
            _ => None,
        })
        .unwrap();
    assert_eq!(row.source.object_return_sources().unwrap().len(), 2);
    assert_eq!(row.result(), Some(InvokeCallResultKind::Handle));
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
}

#[test]
fn object_slot_plural_drift_refuses_even_with_missing_input_or_completion() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { return new Token() } relay() { local item = me.make(7) if 1 > 0 { return item } return item } } static box Main { main() { return 0 } }").unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let sites = rearm_pending(ledger);
    let site = &sites[0];
    let mut slots = ledger.lexical_instance_calls.borrow_mut();
    let Some(LexicalInstanceCallDispositionSlotV1::SourcePending(source)) = slots.get_mut(site)
    else {
        panic!("pending")
    };
    let callee = source.callee_owner();
    let LexicalCallSourceResultRequirementV1::ObjectReturnSource { qualifications, .. } =
        &mut source.result_requirement
    else {
        panic!("qualified")
    };
    assert_eq!(qualifications.len(), 2);
    *qualifications = vec![qualifications[0].clone()].into_boxed_slice();
    drop(slots);
    ledger.borrowed_formal_actuals.remove(site);
    ledger.completion_index.remove(&callee);
    assert!(ledger
        .finish_object_lexical_slots_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts
        )
        .unwrap_err()
        .contains("caller-qualification-identity"));
    assert!(matches!(
        ledger.lexical_instance_calls.borrow().get(site),
        Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_))
    ));
}

#[test]
fn object_slot_formal_ordinal_and_binding_drift_refuse() {
    for binding in [false, true] {
        let mut package = package(false, false, false);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let sites = rearm_pending(ledger);
        let owner = match ledger
            .lexical_instance_calls
            .borrow()
            .get(&sites[0])
            .unwrap()
        {
            LexicalInstanceCallDispositionSlotV1::SourcePending(source) => source.callee_owner(),
            _ => panic!("pending"),
        };
        let contract = package
            .parameter_contracts
            .iter_mut()
            .find(|row| row.owner == owner)
            .unwrap();
        if binding {
            contract.parameters[0].binding = package
                .physical_signature
                .row(contract.batch_slot)
                .unwrap()
                .lanes()[0]
                .binding();
        } else {
            contract.parameters[0].ordinal = 1;
        }
        assert!(ledger
            .finish_object_lexical_slots_v1(
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
                &package.result_contracts
            )
            .unwrap_err()
            .contains("signature-geometry"));
    }
}
