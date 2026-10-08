use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};

fn package(
    received: bool,
    opaque: bool,
    nullable: bool,
    zero: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    package_with_caller(received, opaque, nullable, zero, false)
}

fn package_with_caller(
    received: bool,
    opaque: bool,
    nullable: bool,
    zero: bool,
    root: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if zero {
        ""
    } else if opaque {
        "size"
    } else {
        "size: i64"
    };
    let actual = if zero { "" } else { "7" };
    let condition = if zero { "1" } else { "size" };
    let body = if nullable {
        format!("if {condition} > 0 {{ return new Token() }} return null")
    } else {
        "return new Token()".into()
    };
    let receiver = if root { "maker" } else { "me" };
    let tail = if received {
        format!("local item = {receiver}.make({actual}) return item")
    } else {
        format!("return {receiver}.make({actual})")
    };
    let relay = if root {
        String::new()
    } else {
        format!("relay() {{ {tail} }}")
    };
    let main = if root {
        format!("local maker = new Maker() local spare = new Spare() {tail}")
    } else {
        "return 0".into()
    };
    issue_with_brand_catalog(&format!("box Spare {{}} box Token {{}} box Maker {{ make({formal}) {{ {body} }} {relay} }} static box Main {{ main() {{ {main} }} }}")).unwrap()
}

#[test]
fn original_main_object_packet_matrix_borrows_same_executable_storage_and_affine_take() {
    for received in [false, true] {
        for (opaque, zero) in [(false, false), (true, false), (false, true)] {
            for nullable in [false, true] {
                let package = package_with_caller(received, opaque, nullable, zero, true);
                let ledger = &package.ordinary_new_claim_ledger;
                let root = ledger.root_completion.as_ref().unwrap().as_ref().unwrap();
                let owner = root.owner();
                assert!(Rc::ptr_eq(
                    ledger.completion_index[&owner].as_ref().unwrap(),
                    root
                ));
                assert!(Rc::ptr_eq(
                    &ledger.terminal_relation_index[&owner],
                    &ledger.terminal_relation
                ));
                let main = package
                    .declaration_catalog()
                    .source_backed_app_main()
                    .unwrap();
                assert!(package
                    .selected
                    .batch_slot(
                        &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                            main.catalog_key().clone()
                        )
                    )
                    .is_none());
                let slots = ledger.lexical_instance_calls.borrow();
                let (site, ready) = slots
                    .iter()
                    .find_map(|(site, slot)| match slot {
                        LexicalInstanceCallDispositionSlotV1::Ready(row)
                            if site.owner() == owner =>
                        {
                            Some((site.clone(), row))
                        }
                        _ => None,
                    })
                    .expect("original Main Ready packet");
                assert!(!ready.source_target().is_self_receiver());
                assert_eq!(
                    ready.source_target().object_return_sources().unwrap(),
                    ledger
                        .callable_result_classes
                        .qualifications_at_call(&site)
                        .as_ref()
                );
                assert!(ledger
                    .borrowed_call_actuals_v1(ready)
                    .unwrap_err()
                    .contains("disposition-not-owned-and-taken"));
                assert!(ledger
                    .object_packet_arguments_v1(ready)
                    .unwrap_err()
                    .contains("disposition-not-owned-and-taken"));
                assert!(ledger
                    .object_packet_teardown_v1(ready)
                    .unwrap_err()
                    .contains("disposition-not-owned-and-taken"));
                drop(slots);
                let row = ledger
                    .take_lexical_instance_call(owner, site.site())
                    .unwrap()
                    .unwrap();
                let original = ledger.borrowed_formal_actuals[&site].as_ref().unwrap();
                original.require_executable_v1().unwrap();
                let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
                let (checked_actuals, checked_arguments) =
                    row.checked_object_packet_inputs_v1(ledger).unwrap();
                let emitted_arguments = ledger.object_packet_arguments_v1(&row).unwrap();
                assert!(ledger
                    .object_packet_teardown_v1(&row)
                    .unwrap()
                    .owned_children()
                    .is_none());
                assert_eq!(actuals.as_ptr(), original.opaque_actuals.as_ptr());
                assert_eq!(checked_actuals.as_ptr(), original.opaque_actuals.as_ptr());
                assert_eq!(
                    checked_arguments.as_ptr(),
                    original.ordered_arguments.as_ptr()
                );
                assert_eq!(
                    emitted_arguments.as_ptr(),
                    original.ordered_arguments.as_ptr()
                );
                assert_eq!(actuals.len(), usize::from(opaque));
                assert_eq!(checked_arguments.len(), usize::from(!zero));
                assert_eq!(
                    row.result(),
                    Some(if nullable {
                        InvokeCallResultKind::NullableHandle
                    } else {
                        InvokeCallResultKind::Handle
                    })
                );
                assert!(ledger
                    .take_lexical_instance_call(owner, site.site())
                    .unwrap_err()
                    .contains("already-taken"));
                assert_eq!(ledger.validate_no_pending_object_returns_v1().unwrap_err(), "[freeze:contract][ordinary-new/local-commit/object-return-handoff-unavailable]");
            }
        }
    }
}
fn take(package: &VerifiedNormalCallableSemanticPackageV1) -> LexicalInstanceCallDispositionRowV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger
        .lexical_instance_calls
        .borrow()
        .iter()
        .find_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source_target().has_object_source_requirement() =>
            {
                Some(site.clone())
            }
            _ => None,
        })
        .unwrap();
    ledger
        .take_lexical_instance_call(site.owner(), site.site())
        .unwrap()
        .unwrap()
}

#[test]
fn object_packet_taken_lends_same_typed_opaque_zero_direct_received_storage() {
    for received in [false, true] {
        for nullable in [false, true] {
            for (opaque, zero) in [(false, false), (true, false), (false, true)] {
                let package = package(received, opaque, nullable, zero);
                let row = take(&package);
                let ledger = &package.ordinary_new_claim_ledger;
                let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
                let original = &ledger.borrowed_formal_actuals[row.call_site()]
                    .as_ref()
                    .unwrap()
                    .opaque_actuals;
                assert_eq!(actuals.as_ptr(), original.as_ptr());
                assert_eq!(actuals.len(), usize::from(opaque));
                if received {
                    let arguments = ledger.receiver_object_packet_arguments_v1(&row).unwrap();
                    let original = &ledger.borrowed_formal_actuals[row.call_site()]
                        .as_ref()
                        .unwrap()
                        .ordered_arguments;
                    assert_eq!(arguments.as_ptr(), original.as_ptr());
                    assert_eq!(arguments.len(), usize::from(!zero));
                    assert!(ledger
                        .take_receiver_object_packet_v1(
                            row.call_site().owner(),
                            row.call_site().site(),
                            row.target()
                        )
                        .unwrap_err()
                        .contains("already-taken"));
                }
                assert_eq!(
                    row.result(),
                    Some(if nullable {
                        InvokeCallResultKind::NullableHandle
                    } else {
                        InvokeCallResultKind::Handle
                    })
                );
                assert!(ledger
                    .take_lexical_instance_call(row.call_site().owner(), row.call_site().site())
                    .unwrap_err()
                    .contains("already-taken"));
                if received {
                    let mut row = row;
                    let descriptor = ledger.object_packet_teardown_v1(&row).unwrap().clone();
                    row.object_packet.as_mut().unwrap().teardown =
                        ObjectReturnTeardownAvailabilityV1::Verified {
                            descriptor: descriptor.clone(),
                            nullable: !nullable,
                        };
                    let before = format!("{:?}", ledger.local_commits.borrow());
                    assert!(ledger
                        .begin_object_packet_call_emission(&row)
                        .unwrap_err()
                        .contains("teardown-kind-drift"));
                    assert_eq!(format!("{:?}", ledger.local_commits.borrow()), before);
                    row.object_packet.as_mut().unwrap().teardown =
                        ObjectReturnTeardownAvailabilityV1::Verified {
                            descriptor,
                            nullable,
                        };
                    ledger.begin_object_packet_call_emission(&row).unwrap();
                    let before = format!("{:?}", ledger.local_commits.borrow());
                    assert!(ledger
                        .begin_object_packet_call_emission(&row)
                        .unwrap_err()
                        .contains("duplicate-emission"));
                    assert_eq!(format!("{:?}", ledger.local_commits.borrow()), before);
                }
                assert!(ledger.validate_no_pending_object_returns_v1().is_err());
            }
        }
    }
}

#[test]
fn object_packet_wrong_result_and_foreign_completion_refuse() {
    for opaque in [false, true] {
        let package = package(false, opaque, false, false);
        let row = take(&package).with_result_for_test(Some(InvokeCallResultKind::I64));
        assert!(package
            .ordinary_new_claim_ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err()
            .contains("result-mismatch"));
        let mut package = self::package(false, opaque, false, false);
        let foreign = self::package(false, opaque, false, false);
        let row = take(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let foreign_row = take(&foreign);
        ledger.completion_index.insert(
            row.callee_owner(),
            foreign.ordinary_new_claim_ledger.completion_index[&foreign_row.callee_owner()].clone(),
        );
        assert!(ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err()
            .contains("result-source-drift"));
    }
    let package = issue_with_brand_catalog("box Token { items: ArrayBox = new ArrayBox() birth() {} } box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { local item = me.make(7) return item } } static box Main { main() { return 0 } }").unwrap();
    let row = take(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let before = format!("{:?}", ledger.local_commits.borrow());
    assert!(ledger
        .object_packet_teardown_v1(&row)
        .unwrap_err()
        .contains("teardown-unavailable"));
    assert!(ledger
        .begin_object_packet_call_emission(&row)
        .unwrap_err()
        .contains("teardown-unavailable"));
    assert_eq!(format!("{:?}", ledger.local_commits.borrow()), before);
}

#[test]
fn object_packet_current_input_errors_precede_wrong_result() {
    for opaque in [false, true] {
        let mut package = package(false, opaque, false, false);
        let row = take(&package).with_result_for_test(Some(InvokeCallResultKind::I64));
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let saved = ledger
            .borrowed_formal_actuals
            .remove(row.call_site())
            .unwrap();
        assert!(ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err()
            .contains("actuals-missing"));
        ledger
            .borrowed_formal_actuals
            .insert(row.call_site().clone(), Err("original-input-error".into()));
        assert_eq!(
            ledger.borrowed_call_actuals_v1(&row).unwrap_err(),
            "original-input-error"
        );
        ledger
            .borrowed_formal_actuals
            .insert(row.call_site().clone(), saved);
        assert!(ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err()
            .contains("result-mismatch"));
    }
}

#[test]
fn object_packet_current_arguments_and_terminal_index_must_match_seal() {
    let mut package = package(false, false, false, false);
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let current = ledger
        .borrowed_formal_actuals
        .get_mut(row.call_site())
        .unwrap()
        .as_mut()
        .unwrap();
    current.ordered_arguments[0] =
        crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(8);
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("actuals-drift"));
    let mut package = self::package(false, false, false, false);
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let terminals = ledger.terminal_relation_index[&row.callee_owner()]
        .as_ref()
        .clone();
    ledger
        .terminal_relation_index
        .insert(row.callee_owner(), Rc::new(terminals));
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("result-source-drift"));
}

#[test]
fn object_packet_typed_sibling_missing_or_changed_refuses_taken_lender() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { return new Token() } relay() { return me.make(7) } ignored() { local item = me.make(8) return 0 } } static box Main { main() { return 0 } }").unwrap();
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let sibling = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .find(|call| call.callee == row.callee_owner() && call.call != *row.call_site())
        .unwrap()
        .call
        .clone();
    let saved = ledger.borrowed_formal_actuals.remove(&sibling).unwrap();
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("actuals-missing"));
    assert!(ledger
        .object_packet_arguments_v1(&row)
        .unwrap_err()
        .contains("actuals-missing"));
    ledger
        .borrowed_formal_actuals
        .insert(sibling.clone(), saved);
    ledger
        .borrowed_formal_actuals
        .get_mut(&sibling)
        .unwrap()
        .as_mut()
        .unwrap()
        .ordered_arguments[0] =
        crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(9);
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("actuals-drift"));
    assert!(ledger
        .object_packet_arguments_v1(&row)
        .unwrap_err()
        .contains("actuals-drift"));
}

#[test]
fn object_packet_source_only_zero_arguments_never_lend_empty_executable() {
    use super::super::super::borrowed_formal_actuals::PreparedBorrowedCallActualsV1;
    let mut package = package(false, false, false, true);
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source_only = PreparedBorrowedCallActualsV1::object_source_for_test(
        ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap(),
        &package.parameter_contracts,
        row.call_site(),
        &[],
    );
    assert!(source_only.opaque_actuals.is_empty());
    assert!(source_only.ordered_arguments.is_empty());
    ledger
        .borrowed_formal_actuals
        .insert(row.call_site().clone(), Ok(source_only));
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("source-only-object-actuals"));
    assert!(ledger
        .object_packet_arguments_v1(&row)
        .unwrap_err()
        .contains("source-only-object-actuals"));
}

#[test]
fn object_packet_extra_original_incoming_refuses() {
    for opaque in [false, true] {
        let mut package = package(false, opaque, false, false);
        let row = take(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let ingress = ledger
            .borrowed_formal_source
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap();
        if opaque {
            let mut calls = ingress.incoming.to_vec();
            calls.push(calls[0].clone());
            ingress.incoming = calls.into_boxed_slice();
            assert!(ledger
                .borrowed_call_actuals_v1(&row)
                .unwrap_err()
                .contains("incoming-identity"));
        } else {
            ingress
                .source_incoming
                .duplicate_object_row_for_test(row.call_site());
            assert!(ledger.borrowed_call_actuals_v1(&row).is_err());
        }
    }
}

#[test]
fn object_packet_ready_row_cannot_lend_before_affine_take() {
    let package = package(false, false, false, false);
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    let row = slots
        .values()
        .find_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source_target().has_object_source_requirement() =>
            {
                Some(row)
            }
            _ => None,
        })
        .unwrap();
    assert!(ledger
        .borrowed_call_actuals_v1(row)
        .unwrap_err()
        .contains("disposition-not-owned-and-taken"));
}

#[test]
fn object_packet_opaque_original_raw_duplicate_refuses_unchanged_final_incoming() {
    let mut package = package(false, true, false, false);
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap()
        .source_incoming
        .duplicate_object_row_for_test(row.call_site());
    assert!(ledger.borrowed_call_actuals_v1(&row).is_err());
}

#[test]
fn object_packet_unqualified_nested_producer_lends_completed_zero_inputs() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } ignored() { local item = me.relay() return 0 } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger
        .lexical_instance_calls
        .borrow()
        .iter()
        .find_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source_target().object_producer_dependencies().is_some() =>
            {
                Some(site.clone())
            }
            _ => None,
        })
        .unwrap();
    let row = ledger
        .take_lexical_instance_call(site.owner(), site.site())
        .unwrap()
        .unwrap();
    assert!(row.source_target().object_return_sources().is_none());
    assert_eq!(row.result(), Some(InvokeCallResultKind::NullableHandle));
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap()
        .unwrap()
        .is_empty());
}
