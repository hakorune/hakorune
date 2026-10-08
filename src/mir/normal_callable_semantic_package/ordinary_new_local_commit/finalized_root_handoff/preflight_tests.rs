//! Whole real module finishing and failure atomicity at the original collector seam.
use super::*;
use crate::mir::builder::{
    CompletedNormalDefaultRootCatalogLifecycleV1, SelectedNormalCallableKeyV1,
};
use crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1;

fn completed(text: &str) -> CompletedNormalDefaultRootCatalogLifecycleV1 {
    let (completed, original) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    drop(original);
    completed
}

fn source(received: bool, opaque: bool, nullable: bool, zero: bool) -> String {
    let formal = if zero {
        ""
    } else if opaque {
        "size"
    } else {
        "size: i64"
    };
    let actual = if zero { "" } else { "7" };
    let make = if nullable {
        format!(
            "if {} > 0 {{ return new Token() }} return null",
            if zero { "1" } else { "size" }
        )
    } else {
        "return new Token()".into()
    };
    let tail = if received {
        format!("local item = maker.make({actual}) return item")
    } else {
        format!("return maker.make({actual})")
    };
    // Both ordinary children really lower; Received child Plain is an independent obligation.
    format!("box Spare {{}} box Token {{}} box Maker {{ make({formal}) {{ {make} }} direct() {{ local spare = new Spare() return me.make({actual}) }} received() {{ local spare = new Spare() local item = me.make({actual}) return item }} }} static box Main {{ main() {{ local maker = new Maker() local spare = new Spare() {tail} }} }}")
}

type Fixture = (
    String,
    crate::mir::MirModule,
    Rc<OrdinaryNewClaimLedgerV1>,
    BTreeSet<CanonicalSameModuleCallableKeyV1>,
    Option<VerifiedCallableResultContractCohortV1>,
);

fn fixture(received: bool) -> Fixture {
    completed(&source(received, false, false, false))
        .document_preflight_parts_for_test(true)
        .expect("typed Direct/Received document finishing")
}

fn snapshot(ledger: &OrdinaryNewClaimLedgerV1) -> (String, String) {
    (
        format!("{:?}", ledger.root_exits.borrow()),
        format!("{:?}", ledger.root_local_call_bindings.borrow()),
    )
}

#[test]
fn original_main_and_real_children_document_handoff_survive_contraction() {
    for received in [false, true] {
        for (opaque, zero) in [(false, false), (true, false), (false, true)] {
            for nullable in [false, true] {
                let label =
                    format!("received={received},opaque={opaque},zero={zero},nullable={nullable}");
                let (completed, ledger) =
                    crate::mir::builder::lexical_call_projection_document_completion_fixture(
                        &source(received, opaque, nullable, zero),
                    );
                let original = Rc::downgrade(&ledger);
                let child_key = SelectedNormalCallableKeyV1::Cataloged(
                    CanonicalSameModuleCallableKeyV1::test_instance_box_method(
                        "Maker", "received", 0,
                    ),
                );
                let (key, module, same, keys, cohort) = completed
                    .document_preflight_parts_for_test(true)
                    .unwrap_or_else(|error| panic!("{label}: {error}"));
                assert!(Rc::ptr_eq(&same, &ledger));
                for function in module.functions.values() {
                    crate::mir::verification::MirVerifier::new_strict()
                        .verify_function(function)
                        .unwrap_or_else(|error| panic!("{label}: {error:?}"));
                }
                let child = cohort
                    .as_ref()
                    .unwrap()
                    .completed_result(&child_key)
                    .unwrap();
                let before = child.completion() as *const _;
                let child_owner = child.owner();
                assert_ne!(Some(child_owner), ledger.root_owner());
                let handoff = ledger
                    .seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort)
                    .unwrap_or_else(|error| panic!("{label}: {error}"));
                assert_eq!(
                    handoff.root_result(),
                    None,
                    "Object Value ABI remains pending"
                );
                let retained = handoff.root_source().unwrap();
                assert_eq!(retained.owner(), ledger.root_owner().unwrap());
                assert_eq!(
                    handoff
                        .callables()
                        .unwrap()
                        .completed_result(&child_key)
                        .unwrap()
                        .completion() as *const _,
                    before
                );
                assert!(
                    ledger
                        .root_exits
                        .borrow()
                        .iter()
                        .any(|((owner, _), progress)| *owner == child_owner
                            && matches!(
                                progress,
                                RootHomeExitProgress::Emitted {
                                    entry: RootHomeExitEntry::Plain { .. },
                                    ..
                                }
                            )),
                    "real Received child remains original Emitted Plain"
                );
                retained
                    .validate_finalized_root_cleanup_v1(&module)
                    .unwrap();
                assert!(ledger
                    .seal_finalized_root_birth_handoff(key, &module, &keys, None)
                    .unwrap_err()
                    .contains("artifact-root-already-finalized"));
                assert!(ledger
                    .validate_no_pending_object_returns_v1()
                    .unwrap_err()
                    .contains("object-return-handoff-unavailable"));
                drop(same);
                drop(ledger);
                assert!(original.upgrade().is_some());
            }
        }
    }
}

#[test]
fn late_child_entry_or_projection_failure_leaves_root_calls_and_local_pool_unconsumed() {
    for received_child in [false, true] {
        for foreign_projection in [false, true] {
            let (key, module, ledger, keys, cohort) = fixture(false);
            let owner = cohort
                .as_ref()
                .unwrap()
                .completed_result(&SelectedNormalCallableKeyV1::Cataloged(
                    CanonicalSameModuleCallableKeyV1::test_instance_box_method(
                        "Maker",
                        if received_child { "received" } else { "direct" },
                        0,
                    ),
                ))
                .unwrap()
                .owner();
            if foreign_projection {
                let other = cohort
                    .as_ref()
                    .unwrap()
                    .completed_result(&SelectedNormalCallableKeyV1::Cataloged(
                        CanonicalSameModuleCallableKeyV1::test_instance_box_method(
                            "Maker",
                            if received_child { "direct" } else { "received" },
                            0,
                        ),
                    ))
                    .unwrap()
                    .owner();
                let mut states = ledger.child_physical_validation.borrow_mut();
                let ChildPhysicalValidation::FinishingChecked { symbol, projection } =
                    states.remove(&other).unwrap()
                else {
                    panic!("finished sibling")
                };
                let ChildPhysicalValidation::FinishingChecked {
                    projection: selected,
                    ..
                } = states.get_mut(&owner).unwrap()
                else {
                    panic!("finished selected child")
                };
                let original = std::mem::replace(selected, projection);
                states.insert(
                    other,
                    ChildPhysicalValidation::FinishingChecked {
                        symbol,
                        projection: original,
                    },
                );
            } else {
                let exit = ledger.completion_for_owner(owner).unwrap().explicit_sites()[0].clone();
                ledger.root_exits.borrow_mut().remove(&(owner, exit));
            }
            let before = snapshot(&ledger);
            assert!(ledger
                .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
                .is_err());
            assert_eq!(snapshot(&ledger), before);
            assert!(matches!(
                *ledger.root_validation.borrow(),
                RootNewValidation::FinishingChecked { .. }
            ));
        }
    }
}

#[test]
fn late_birth_and_identity_failures_precede_root_batch_take() {
    for identity in [false, true] {
        let birth_source = source(false, false, false, false).replace(
            "box Spare {}",
            "box Spare { tag: i64 birth() { me.tag = 0 } }",
        );
        let (key, module, mut ledger, mut keys, cohort) = completed(&birth_source)
            .document_preflight_parts_for_test(true)
            .expect("real Birth document finishing");
        if identity {
            Rc::get_mut(&mut ledger).unwrap().app_main_identity = None;
        } else {
            assert!(keys.pop_last().is_some());
        }
        let before = snapshot(&ledger);
        let error = ledger
            .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
            .unwrap_err();
        assert!(
            error.contains(if identity {
                "artifact-root-identity-unavailable"
            } else {
                "artifact-birth-construction-missing"
            }),
            "{error}"
        );
        assert_eq!(snapshot(&ledger), before);
    }
}

#[test]
fn received_source_missing_and_unregistered_exit_fail_before_move() {
    for foreign in [false, true] {
        let (key, module, mut ledger, keys, cohort) = fixture(false);
        let child_key = CanonicalSameModuleCallableKeyV1::test_instance_box_method(
            "Maker",
            if foreign { "direct" } else { "received" },
            0,
        );
        let child = cohort
            .as_ref()
            .unwrap()
            .completed_result(&SelectedNormalCallableKeyV1::Cataloged(child_key))
            .unwrap()
            .owner();
        if foreign {
            ledger.child_physical_validation.borrow_mut().remove(&child);
        } else {
            // Preserve the original Normal proof while removing its terminal index.
            Rc::get_mut(&mut ledger)
                .unwrap()
                .terminal_relation_index
                .remove(&child);
        }
        let before = snapshot(&ledger);
        let error = ledger
            .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
            .unwrap_err();
        assert!(
            error.contains(if foreign {
                "artifact-exit-owner-not-finished"
            } else {
                "artifact-object-return-source-unavailable"
            }),
            "{error}"
        );
        assert_eq!(snapshot(&ledger), before);
    }
}

#[test]
fn joint_mir_and_recorded_fault_omission_is_rejected_before_take() {
    let (key, mut module, ledger, keys, cohort) = fixture(false);
    let owner = ledger.root_owner().unwrap();
    let exit = ledger.completion_for_owner(owner).unwrap().explicit_sites()[0].clone();
    let removed = {
        let mut exits = ledger.root_exits.borrow_mut();
        let RootHomeExitProgress::Emitted {
            entry, bindings, ..
        } = exits.get_mut(&(owner, exit)).unwrap()
        else {
            panic!("original Emitted Root");
        };
        let RootHomeExitEntry::Call { projection, .. } = entry else {
            panic!("Direct Root Call");
        };
        let result = projection.1.dst_value().unwrap();
        let index = bindings
            .iter()
            .position(|(_, instruction)| {
                matches!(instruction,
            MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { value, .. }, .. }
                if *value == result)
            })
            .expect("post-success Fault owns result release");
        bindings.remove(index)
    };
    let MirInstruction::Invoke { normal_landing, .. } = removed.1 else {
        unreachable!();
    };
    module
        .functions
        .get_mut(&key)
        .unwrap()
        .blocks
        .get_mut(&removed.0)
        .unwrap()
        .terminator = Some(MirInstruction::Jump {
        target: normal_landing,
        edge_args: None,
    });
    let before = snapshot(&ledger);
    assert!(ledger
        .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
        .is_err());
    assert_eq!(
        snapshot(&ledger),
        before,
        "source order rejects even jointly omitted physical evidence"
    );
}

#[test]
fn named_array_owner_and_coverage_are_checked_on_borrowed_original_rows() {
    let text = "box Holder { init { items } birth() { me.items = new ArrayBox() } seed() { local items = me.items local i = 0 loop(i < 2) { items.push(i) i = i + 1 } } } static box Main { main() { local holder = new Holder() return 0 } }";
    let (key, module, ledger, keys, cohort) = completed(text)
        .document_preflight_parts_for_test(true)
        .unwrap();
    let cohort = cohort.unwrap();
    let rows = cohort.named_array_emissions();
    assert!(
        !rows.is_empty(),
        "real original cohort owes named-array writes"
    );
    let pointer = rows.as_ptr();
    let count = rows.len();
    let before = snapshot(&ledger);
    let checker = crate::mir::finalized_root_handoff::validate_named_array_handoff_inputs;
    checker(&module, Some(&cohort), rows).unwrap();
    assert!(checker(&module, None, rows)
        .unwrap_err()
        .contains("handoff-source-owner-mismatch"));
    let mut missing = module.clone();
    let marker = rows[0].marker();
    let symbol = missing
        .canonical_callable_definition_symbol(rows[0].caller())
        .unwrap()
        .to_owned();
    missing
        .functions
        .get_mut(&symbol)
        .unwrap()
        .metadata
        .named_array_write_obligations
        .retain(|row| row.write != marker.write);
    assert!(checker(&missing, Some(&cohort), rows).is_err());
    assert_eq!(cohort.named_array_emissions().as_ptr(), pointer);
    assert_eq!(cohort.named_array_emissions().len(), count);
    assert_eq!(snapshot(&ledger), before);
    let error = ledger
        .seal_finalized_root_birth_handoff(key, &missing, &keys, Some(cohort))
        .unwrap_err();
    assert!(error.contains("named-array"), "{error}");
    assert_eq!(snapshot(&ledger), before);
}
