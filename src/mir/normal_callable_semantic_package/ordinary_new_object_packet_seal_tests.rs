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
                    .borrowed_object_call_packet_v1(ready)
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
                let packet = ledger.borrowed_object_call_packet_v1(&row).unwrap();
                assert_eq!(packet.arguments().as_ptr(), original.ordered_arguments.as_ptr());
                assert_eq!(Some(packet.result()), row.result());
                for value in [crate::mir::ValueId(71), crate::mir::ValueId(93)] {
                    let object = packet.teardown().object();
                    let expected = if nullable {
                        crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { object, value }
                    } else {
                        crate::mir::instruction::InvokeOperation::HomeRelease { object, value }
                    };
                    assert_eq!(packet.cancellation_operations_v1(value), [expected],
                        "pure End projection keeps original nullable kind and supplied result coordinate");
                }
                assert!(std::ptr::eq(packet.teardown(), row.object_packet.as_ref().unwrap()
                    .teardown.descriptor().unwrap().0));
                assert!(ledger
                    .borrowed_object_call_packet_v1(&row)
                    .unwrap()
                    .teardown()
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
                    let descriptor = ledger.borrowed_object_call_packet_v1(&row).unwrap().teardown().clone();
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
        .borrowed_object_call_packet_v1(&row)
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
    let package = self::package(false, false, false, false);
    let mut row = take(&package);
    let seal = row.object_packet.as_mut().unwrap();
    let saved = std::mem::take(&mut seal.field_supports);
    assert!(!saved.is_empty(), "even support-free Fresh exits retain original census");
    assert!(package.ordinary_new_claim_ledger.borrowed_call_actuals_v1(&row)
        .unwrap_err().contains("field-support-source-drift"));
    row.object_packet.as_mut().unwrap().field_supports = saved;
    package.ordinary_new_claim_ledger.borrowed_call_actuals_v1(&row).unwrap();
    let mut package = self::package(false, false, false, false);
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
    for expression in ["local item = me.relay()", "me.relay()", "local arr = new ArrayBox() arr.push(me.make(7)) me.consume(arr.get(0))", "local arr = new ArrayBox() arr.push(me.make(0)) me.consume(arr.get(0))", "local arr = new ArrayBox() arr.push(me.make(7)) Sink.consume(arr.get(0))"] {
        let source = "box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } ignored() { CANDIDATE return 0 } } static box Main { main() { return 0 } }".replace("CANDIDATE", expression);
        let array = expression.contains("ArrayBox");
        let static_array = expression.contains("Sink.consume");
        let nonnull = expression.contains("make(0)");
        let source = if array { source.replace("ignored()", "consume(value) { if value == null { return 0 } return 1 } ignored()") } else { source };
        let source = if static_array { format!("{source} static box Sink {{ consume(value) {{ if value == null {{ return 0 }} return 1 }} }}") }
            else if array { source.replace("relay() { return me.make(7) } ", "") } else { source };
        let source = if nonnull { source.replace("if size > 0 { return new Token() } return null", "return new Token()") } else { source };
        let package = issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        if static_array {
            let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let call = ingress.source_incoming.exact_rows().find(|call|
                call.source.target().namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                    && call.source.target().name() == "consume").unwrap();
            let read = OwnedExprSiteV1::new(call.call.owner(), call.arguments[0].1.clone());
            let loan = ingress.array_element_source_v1(&read).unwrap().unwrap();
            assert_eq!(loan.producers().len(), 1, "preserve the original Static Get source without granting execution");
            let producer = &loan.producers()[0].1;
            assert!(std::ptr::eq(ingress.original_array_producer_source_v1(producer.source()).unwrap().unwrap(), producer));
            continue;
        }
        let site = ledger.lexical_instance_calls.borrow().iter()
            .find_map(|(site, slot)| match slot {
                LexicalInstanceCallDispositionSlotV1::Ready(row)
                    if if array {
                        ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()
                            .original_array_producer_source_v1(row.source_target()).unwrap().is_some()
                    } else { row.source_target().object_producer_dependencies().is_some() } =>
                { Some(site.clone()) }
                _ => None,
            });
        if expression == "me.relay()" {
            // A generic expression statement has no completed packet route.
            // Lending the joined view must not silently select that family.
            // A future Append consumer needs its own original source relation.
            assert!(site.is_none());
            continue;
        }
        let site = site.expect("original source-selected producer packet");
        let row = ledger.take_lexical_instance_call(site.owner(), site.site())
            .unwrap().unwrap();
        assert!(row.source_target().object_return_sources().is_none());
        if array {
            assert!(row.source_target().object_producer_dependencies().is_none(),
                "original Append source must select the leaf without forging a Return dependency");
            let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            assert!(!ingress.source_incoming.has_object_input_callee_v1(row.source_target()));
            assert_eq!(ledger.checked_object_slot_observation_v1(row.source_target(), None).unwrap(), Some(false));
            let wrong = if nonnull { InvokeCallResultKind::NullableHandle } else { InvokeCallResultKind::Handle };
            assert!(ledger.checked_object_slot_observation_v1(row.source_target(), Some(wrong))
                .unwrap_err().contains("array-result-observation"));
            assert!(matches!(ledger.selected_original_instance_input_kind_v1(ingress, row.source_target().callee_owner()).unwrap(),
                Some(super::super::super::borrowed_formal_actuals::object_input_finish::OriginalInstanceInputKindV1::SourceInstance)));
        }
        let result = if nonnull { InvokeCallResultKind::Handle } else { InvokeCallResultKind::NullableHandle };
        assert_eq!(row.result(), Some(result));
        assert!(ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap().is_empty(),
            "literal I64 actuals do not create borrowed-handle payloads");
        let packet = ledger.borrowed_object_call_packet_v1(&row).unwrap();
        assert_eq!(packet.arguments().len(), usize::from(array));
        assert_eq!(packet.result(), result);
        assert!(packet.teardown().owned_children().is_none());
    }
}

#[test]
fn object_packet_field_support_joins_original_callee_formal_and_actual_storage() {
    let package = issue_with_brand_catalog("box Token {} box Holder { child: Token birth(child) { me.child = child } }
        box Maker { make(marker: i64, h: Token) { return new Holder(h) }
            relay(h: Token) { return me.make(7, h) }
            nullRelay() { return me.make(7, null) } }
        static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let targets: Vec<_> = ledger.lexical_instance_calls.borrow().values().filter_map(|slot| match slot {
        LexicalInstanceCallDispositionSlotV1::Ready(row) if row.target().name() == "make" => Some(row.source_target().clone()),
        LexicalInstanceCallDispositionSlotV1::SourcePending(source) if source.target().name() == "make" => Some(source.clone()),
        _ => None,
    }).collect();
    assert_eq!(targets.len(), 2, "formal and Null inputs retain same callee census");
    for target in targets {
        let actuals = ledger.borrowed_formal_actuals[target.call_site()].as_ref().unwrap().opaque_actuals.as_ref();
        let checked = ledger.checked_object_callee_source_v1(&target, &package.result_contracts).unwrap().unwrap();
        let visit_actuals = |actuals: &[PreparedBorrowedFormalActualV1], visit: &mut dyn FnMut(
            &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectReturnFieldSupportV1,
            &PreparedBorrowedFormalActualV1) -> Result<(), String>| {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectCallFieldSupportRequirementV1::visit_actuals_v1(
                &checked.field_supports, &target, actuals, visit)
        };
        let mut seen = 0;
        visit_actuals(actuals, &mut |requirement, actual| {
            assert_eq!(requirement.ordinal, 1, "callee ordinal is not caller formal zero");
            assert!(std::ptr::eq(actual, &actuals[0]), "borrow original actual storage");
            seen += 1;
            Ok(())
        }).unwrap();
        assert!(seen > 0);
        assert!(checked.teardown.descriptor().is_none(), "source join alone cannot grant anchored End");
        assert!(visit_actuals(&[], &mut |_, _| Ok(())).is_err());
        let mut changed = actuals.to_vec();
        changed.push(changed[0].clone());
        assert!(visit_actuals(&changed, &mut |_, _| Ok(())).is_err());
        changed = actuals.to_vec();
        changed[0].ordinal = 0;
        assert!(visit_actuals(&changed, &mut |_, _| Ok(())).is_err());
        changed = actuals.to_vec();
        changed[0].formal = ledger.original_parameter_contracts.as_ref().unwrap().iter()
            .find(|contract| contract.owner == target.callee_owner()).unwrap().parameters[0].binding;
        assert!(visit_actuals(&changed, &mut |_, _| Ok(())).is_err());

    }
}
