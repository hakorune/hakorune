//! Original Main source joins original Completion/table indices before construction.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog as issue,
    VerifiedNormalCallableSemanticPackageV1 as Package,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    ObjectReturnAcquisitionV1, TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};

fn package(received: bool, opaque: bool, nullable: bool, zero: bool) -> Package {
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
    let body = if received {
        format!("local item = maker.make({actual}) return item")
    } else {
        format!("return maker.make({actual})")
    };
    issue(&format!("box Spare {{}} box Token {{}} box Maker {{ make({formal}) {{ {make} }} }} static box Main {{ main() {{ local maker = new Maker() local spare = new Spare() {body} }} }}")).unwrap()
}

fn identity(package: &Package) -> (FunctionOwnerIdV1, SourceStmtSiteV1) {
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion.as_ref().unwrap().as_ref().unwrap();
    assert_eq!(completion.explicit_sites().len(), 1);
    (completion.owner(), completion.explicit_sites()[0].clone())
}

fn original_child_seeds(package: &Package) -> VerifiedCallableResultContractBuilderV1 {
    let mut seeds = VerifiedCallableResultContractBuilderV1::new();
    for row in package.result_contracts.rows() {
        let declaration = package
            .batch()
            .declarations()
            .find(|decl| decl.batch_slot() == row.batch_slot())
            .unwrap();
        let original = Rc::clone(
            package.ordinary_new_claim_ledger.completion_index[&row.owner()]
                .as_ref()
                .unwrap(),
        );
        assert!(std::ptr::eq(original.as_ref(), row.borrow().completion()));
        seeds
            .push_completion(
                declaration,
                &package.selected,
                original,
                row.borrow().terminal_relations().clone(),
            )
            .unwrap();
    }
    seeds.finish()
}

#[test]
fn root_completion_index_direct_received_matrix_lends_same_original_source() {
    let mut missing = Vec::new();
    for received in [false, true] {
        for (opaque, zero) in [(false, false), (true, false), (false, true)] {
            for nullable in [false, true] {
                let package = package(received, opaque, nullable, zero);
                let (owner, exit) = identity(&package);
                let ledger = &package.ordinary_new_claim_ledger;
                let main = package.catalog.catalog().source_backed_app_main().unwrap();
                assert!(package
                    .selected
                    .batch_slot(
                        &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                            main.catalog_key().clone()
                        )
                    )
                    .is_none());
                assert!(Rc::ptr_eq(
                    ledger.completion_index[&owner].as_ref().unwrap(),
                    ledger.root_completion.as_ref().unwrap().as_ref().unwrap()
                ));
                assert!(Rc::ptr_eq(
                    &ledger.terminal_relation_index[&owner],
                    &ledger.terminal_relation
                ));
                let Some(terminal) = ledger.terminal_relation.get(&exit) else {
                    missing.push(format!(
                        "missing Root terminal: received={received} opaque={opaque} nullable={nullable} zero={zero}; flow={:#?}",
                        ledger.root_completion.as_ref().unwrap().as_ref().unwrap().cleanup().root_flow()
                    ));
                    continue;
                };
                let TerminalRelationV1::Value(value) = terminal else {
                    panic!("original root value")
                };
                let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
                    panic!("original root call")
                };
                assert_eq!(
                    ledger
                        .callable_result_classes
                        .object_return_qualification(obligation.qualification().value())
                        .as_ref(),
                    Some(obligation.qualification())
                );
                assert_eq!(obligation.qualification().key().owner(), "Maker");
                let projection = ledger
                    .normal_exit_projection_v1(owner, &exit)
                    .unwrap()
                    .expect("original Normal projection");
                assert_eq!(projection.fault_homes(), obligation.exit_homes());
                let super::normal_return::NormalReturnDispositionV1::Verified { proof } =
                    &ledger.normal_return_dispositions.as_ref().unwrap()[&(owner, exit.clone())]
                else {
                    panic!("original completed inputs/result")
                };
                assert_eq!(proof.acquisition().original(), obligation.as_ref());
                assert_eq!(
                    proof.acquisition().arguments().len(),
                    if zero { 0 } else { 1 }
                );
                if received {
                    let ObjectReturnAcquisitionV1::Received(call) = obligation.acquisition() else {
                        panic!("Received")
                    };
                    let (_, destination) = call.local_binding().unwrap();
                    assert_eq!(
                        projection
                            .fault_homes()
                            .iter()
                            .filter(|home| **home == destination)
                            .count(),
                        1
                    );
                    assert!(!projection.homes().contains(&destination));
                    assert!(ledger
                        .verified_direct_object_return_source_v1(owner, &exit)
                        .unwrap()
                        .is_none());
                } else {
                    assert_eq!(projection.homes(), projection.fault_homes());
                    assert!(ledger
                        .verified_direct_object_return_source_v1(owner, &exit)
                        .unwrap()
                        .is_some());
                }
                assert!(ledger.object_return_construction_ready_v1(owner).unwrap());
                let descriptor = ledger.checked_root_object_result_v1().unwrap().unwrap();
                assert_eq!(descriptor.owner, owner);
                assert_eq!(descriptor.class, "Token");
                assert_eq!(
                    descriptor.kind,
                    if nullable {
                        crate::mir::instruction::InvokeCallResultKind::NullableHandle
                    } else {
                        crate::mir::instruction::InvokeCallResultKind::Handle
                    }
                );
                assert_eq!(descriptor.terminals.len(), 1);
                assert!(std::ptr::eq(descriptor.terminals[0], terminal));

                assert!(ledger
                    .validate_no_pending_object_returns_v1()
                    .unwrap_err()
                    .contains("object-return-handoff-unavailable"));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
}

#[test]
fn root_completion_index_collision_and_late_identity_drift_are_atomic() {
    let foreign = package(false, false, false, false);
    for corruption in 0..6 {
        let mut package = package(false, false, false, false);
        let (owner, _) = identity(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        assert!(ledger.checked_root_object_result_v1().unwrap().is_some());
        match corruption {
            0 => {
                ledger.completion_index.remove(&owner);
            }
            1 => {
                ledger.normal_return_dispositions = None;
            }
            2 => {
                ledger
                    .terminal_relation_index
                    .insert(owner, Rc::new((*ledger.terminal_relation).clone()));
            }
            3 => {
                // Same contents do not replace the original successful Completion.
                let other = ledger
                    .completion_index
                    .iter()
                    .find(|(actual, _)| **actual != owner)
                    .unwrap()
                    .1
                    .clone();
                ledger.completion_index.insert(owner, other);
            }
            4 | 5 => {
                let extra = SourceStmtSiteV1::from_node(
                    crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Body(99),
                    ]),
                );
                let rows = ledger.normal_return_dispositions.as_mut().unwrap();
                let proof = rows
                    .values()
                    .find_map(|row| match row {
                        NormalReturnDispositionV1::Verified { proof } => Some(Rc::clone(proof)),
                        _ => None,
                    })
                    .unwrap();
                rows.insert(
                    (owner, extra),
                    NormalReturnDispositionV1::Verified { proof },
                );
                if corruption == 5 {
                    ledger.completion_index.remove(&owner);
                }
            }
            _ => unreachable!(),
        }
        let result = ledger.checked_root_object_result_v1();
        if corruption < 2 {
            assert!(result.unwrap().is_none());
        } else {
            assert!(result.err().unwrap().contains(if corruption == 2 {
                "terminal-table-identity"
            } else if corruption == 3 {
                "completion-identity"
            } else {
                "disposition-exit"
            }));
        }
    }

    for corruption in 0..4 {
        let mut package = package(false, false, false, false);
        let (owner, exit) = identity(&package);
        let seeds = original_child_seeds(&package);
        assert!(!seeds.completion_index().is_empty());
        let child = seeds.completion_index().into_values().next().unwrap();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger.completion_index.clear();
        ledger.terminal_relation_index.clear();
        let reason = match corruption {
            0 => {
                ledger.root_completion = Some(Ok(child));
                "root-owner-collision"
            }
            1 => {
                ledger.terminal_relation =
                    Rc::clone(&foreign.ordinary_new_claim_ledger.terminal_relation);
                "root-terminal-owner"
            }
            2 | 3 => {
                let TerminalRelationV1::Value(value) = ledger.terminal_relation[&exit].clone()
                else {
                    panic!("value")
                };
                let extra = SourceStmtSiteV1::from_node(
                    crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Body(99),
                    ]),
                );
                let row = if corruption == 3 {
                    TerminalRelationV1::Value(value.with_return_site_for_test(extra.clone()))
                } else {
                    TerminalRelationV1::Value(value)
                };
                Rc::make_mut(&mut ledger.terminal_relation).insert(extra, row);
                if corruption == 3 {
                    "root-terminal-exit"
                } else {
                    "root-terminal-site"
                }
            }
            _ => unreachable!(),
        };
        assert!(
            matches!(ledger.retain_completion_index(&seeds), Err(OrdinaryNewCoSealIssueV1::CompletionIndexRetention { reason: actual }) if actual == reason)
        );
        assert!(ledger.completion_index.is_empty());
        assert!(ledger.terminal_relation_index.is_empty());
        assert!(ledger.root_completion.is_some());
        if corruption != 0 {
            assert_eq!(ledger.root_owner(), Some(owner));
        }
    }
}

#[test]
fn root_completion_index_repeat_preserves_both_original_indices() {
    for retained in 0..3 {
        let mut package = package(true, true, false, false);
        let seeds = original_child_seeds(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if retained == 1 {
            ledger.completion_index.clear();
        } else if retained == 2 {
            ledger.terminal_relation_index.clear();
        }
        let completions: Vec<_> = ledger
            .completion_index
            .iter()
            .map(|(owner, row)| (*owner, Rc::clone(row.as_ref().unwrap())))
            .collect();
        let terminals = ledger.terminal_relation_index.clone();
        assert!(matches!(
            ledger.retain_completion_index(&seeds),
            Err(OrdinaryNewCoSealIssueV1::CompletionIndexRetention {
                reason: "duplicate-install"
            })
        ));
        assert_eq!(ledger.completion_index.len(), completions.len());
        for (owner, original) in completions {
            assert!(Rc::ptr_eq(
                ledger.completion_index[&owner].as_ref().unwrap(),
                &original
            ));
        }
        assert_eq!(ledger.terminal_relation_index.len(), terminals.len());
        for (owner, original) in terminals {
            assert!(Rc::ptr_eq(
                &ledger.terminal_relation_index[&owner],
                &original
            ));
        }
    }
}

#[test]
fn root_completion_index_missing_exit_is_unavailable_and_no_root_retry() {
    let mut package = package(false, false, false, false);
    let (owner, exit) = identity(&package);
    let seeds = original_child_seeds(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.clear();
    ledger.terminal_relation_index.clear();
    Rc::make_mut(&mut ledger.terminal_relation).remove(&exit);
    ledger.retain_completion_index(&seeds).unwrap();
    assert!(Rc::ptr_eq(
        &ledger.terminal_relation,
        &ledger.terminal_relation_index[&owner]
    ));
    assert!(!ledger.object_return_construction_ready_v1(owner).unwrap());
    assert!(ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap()
        .is_none());
}

#[test]
fn root_completion_index_unavailable_completion_keeps_original_fields() {
    for missing in [false, true] {
        let mut package = package(false, false, false, false);
        let (owner, _) = identity(&package);
        let seeds = original_child_seeds(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger.completion_index.clear();
        ledger.terminal_relation_index.clear();
        let original = Rc::clone(&ledger.terminal_relation);
        ledger.root_completion = if missing {
            None
        } else {
            Some(Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::OwnerClosureMismatch))
        };
        ledger.retain_completion_index(&seeds).unwrap();
        assert!(Rc::ptr_eq(&original, &ledger.terminal_relation));
        assert!(!ledger.completion_index.contains_key(&owner));
        assert!(!ledger.terminal_relation_index.contains_key(&owner));
        if missing {
            assert!(ledger.root_completion.is_none());
        } else {
            assert!(matches!(ledger.root_completion, Some(Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::OwnerClosureMismatch))));
        }
        assert!(!ledger.completion_index.is_empty());
    }
}
