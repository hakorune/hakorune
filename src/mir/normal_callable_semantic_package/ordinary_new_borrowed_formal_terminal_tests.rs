use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, TerminalCallArgumentV1,
};

fn package(
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} wrap(v: i64): i64 {{ return v }} pair(a: i64, b: i64): i64 {{ return a }} }} static box Main {{ main() {{ local recv = new Transport() {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("borrowed terminal source package")
}

fn exit(ledger: &OrdinaryNewClaimLedgerV1) -> crate::mir::resolved_semantics::SourceStmtSiteV1 {
    let completion = ledger.root_completion_for_test();
    assert_eq!(completion.explicit_sites().len(), 1);
    completion.explicit_sites()[0].clone()
}

#[test]
fn direct_borrowed_return_keeps_terminal_args_and_one_original_lexical_row() {
    for argument in ["true", "-7", "recv"] {
        let package = package(&format!("return recv.probe({argument})"));
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        let args = ledger
            .borrowed_terminal_arguments_v1(owner, &exit)
            .unwrap()
            .unwrap();
        assert!(matches!(
            args.as_ref(),
            [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
        ));
        let (_, terminal) = ledger
            .call_source_completion_for_owner_at(owner, &exit)
            .unwrap();
        assert!(matches!(
            terminal.arguments(),
            [TerminalCallArgumentV1::Lexical(_)]
        ));
        assert!(!ledger.root_instance_call_expected(owner));
        let flow = ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        assert!(
            flow.local_calls().is_empty(),
            "return is never a local group"
        );
        assert_eq!(flow.exit_row(&exit).unwrap().unwrap().homes().len(), 1);
        let taken = ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap()
            .unwrap();
        assert_eq!(taken.call_site().site(), terminal.call_site());
        assert!(ledger.borrowed_call_actuals_v1(&taken).unwrap().is_some());
        assert!(ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap_err()
            .contains("already-taken"));
    }
}

#[test]
fn return_selection_precedes_corroboration_but_taken_lender_requires_it() {
    let mut package = package("return recv.probe(true)");
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = ledger.root_owner().unwrap();
    let exit = exit(ledger);
    let proof = ledger
        .borrowed_i64_results
        .values_mut()
        .next()
        .unwrap()
        .as_mut()
        .unwrap();
    proof.contract_corroborated = false;
    assert!(ledger
        .borrowed_terminal_arguments_v1(owner, &exit)
        .unwrap()
        .is_some());
    let error = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit)
        .unwrap_err();
    assert!(
        error.contains("borrowed-call/result-not-corroborated"),
        "{error}"
    );
}

#[test]
fn return_missing_source_incoming_and_ordered_proof_reject_before_take() {
    for mutation in 0..3 {
        let mut package = package("return recv.probe(true)");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        let (_, terminal) = ledger
            .call_source_completion_for_owner_at(owner, &exit)
            .unwrap();
        let site = terminal.call_site().clone();
        let expected = match mutation {
            0 => {
                ledger.borrowed_formal_source = None;
                "borrowed-terminal/source-missing"
            }
            1 => {
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .incoming = Box::new([]);
                "borrowed-terminal/incoming-missing"
            }
            _ => {
                let rows = ledger
                    .borrowed_formal_actuals
                    .values_mut()
                    .next()
                    .unwrap()
                    .as_mut()
                    .unwrap();
                rows.ordered_arguments[0] = LocalCallArgumentV1::Integer(0);
                "borrowed-entry/ordered-arguments-identity"
            }
        };
        let error = ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap_err();
        assert!(error.contains(expected), "{error}");
        assert!(
            ledger.lexical_instance_call_covered(owner, &site),
            "rejection did not consume the correct call"
        );
    }
}

#[test]
fn sibling_return_claims_keep_each_exit_and_call_site_separate() {
    let package = package("if true { return recv.probe(true) } else { return recv.probe(false) }");
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    let owner = completion.owner();
    assert_eq!(completion.explicit_sites().len(), 2);
    let mut sites = std::collections::BTreeSet::new();
    for exit in completion.explicit_sites() {
        let row = ledger
            .take_borrowed_lexical_call_for_return_v1(owner, exit)
            .unwrap()
            .unwrap();
        assert!(sites.insert(row.call_site().clone()));
    }
    assert_eq!(sites.len(), 2);
}

#[test]
fn sibling_relation_misseated_at_another_exit_rejects_without_consuming_either_call() {
    let mut package =
        package("if true { return recv.probe(true) } else { return recv.probe(false) }");
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = ledger.root_owner().unwrap();
    let exits = ledger.root_completion_for_test().explicit_sites().to_vec();
    let sites: Vec<_> = exits
        .iter()
        .map(|exit| {
            ledger
                .call_source_completion_for_owner_at(owner, exit)
                .unwrap()
                .1
                .call_site()
                .clone()
        })
        .collect();
    let wrong = ledger
        .terminal_relation_for_owner_at(owner, &exits[1])
        .unwrap()
        .clone();
    if let Some(index) = ledger.terminal_relation_index.get(&owner) {
        let mut changed = index.as_ref().clone();
        changed.insert(exits[0].clone(), wrong);
        ledger
            .terminal_relation_index
            .insert(owner, std::rc::Rc::new(changed));
    } else {
        ledger.terminal_relation.insert(exits[0].clone(), wrong);
    }
    let error = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exits[0])
        .unwrap_err();
    assert!(error.contains("borrowed-terminal/return-site"), "{error}");
    assert!(sites
        .iter()
        .all(|site| ledger.lexical_instance_call_covered(owner, site)));
}

#[test]
fn selected_return_source_error_stays_terminal_before_lexical_take() {
    let mut package = package("return recv.probe(true)");
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = ledger.root_owner().unwrap();
    let exit = exit(ledger);
    let site = ledger
        .call_source_completion_for_owner_at(owner, &exit)
        .unwrap()
        .1
        .call_site()
        .clone();
    ledger.borrowed_formal_source = Some(Err(
        "[freeze:contract][borrowed-terminal/source-test-rejection]".into(),
    ));
    let error = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit)
        .unwrap_err();
    assert!(
        error.contains("borrowed-terminal/source-test-rejection"),
        "{error}"
    );
    assert!(ledger.lexical_instance_call_covered(owner, &site));
}

#[test]
fn strict_outer_borrowed_return_keeps_original_inner_and_outer_rows() {
    for expression in [
        "recv.wrap(recv.probe(true))",
        "recv.wrap(recv.wrap(recv.probe(-7)))",
    ] {
        let package = package(&format!("return {expression}"));
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        let arguments = ledger
            .borrowed_terminal_arguments_v1(owner, &exit)
            .unwrap()
            .unwrap();
        assert!(!ledger.root_instance_call_expected(owner));
        assert!(ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap()
            .local_calls()
            .is_empty());
        let outer = ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap()
            .unwrap();
        assert_eq!(
            outer.result(),
            Some(crate::mir::instruction::InvokeCallResultKind::I64)
        );
        assert!(ledger.borrowed_call_actuals_v1(&outer).unwrap().is_none());
        let mut arguments = arguments.as_ref();
        loop {
            let [LocalCallArgumentV1::CallResult(inner)] = arguments else {
                panic!("nested call result")
            };
            let taken = ledger
                .take_lexical_instance_call(owner, inner.site().site())
                .unwrap()
                .unwrap();
            assert_eq!(taken.call_site(), inner.site());
            assert_eq!(
                taken.result(),
                Some(crate::mir::instruction::InvokeCallResultKind::I64)
            );
            let actuals = ledger.borrowed_call_actuals_v1(&taken).unwrap();
            if matches!(
                inner.arguments(),
                [LocalCallArgumentV1::BorrowedActual { .. }]
            ) {
                assert!(actuals.is_some());
                assert!(ledger
                    .take_lexical_instance_call(owner, inner.site().site())
                    .unwrap_err()
                    .contains("already-taken"));
                break;
            }
            assert!(actuals.is_none());
            arguments = inner.arguments();
        }
        assert!(ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap_err()
            .contains("already-taken"));
    }
}

#[test]
fn borrowed_return_tree_preserves_a_strict_sibling_with_no_borrowed_descendant() {
    let package = package("return recv.pair(recv.probe(true), recv.wrap(8))");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().unwrap();
    let exit = exit(ledger);
    let arguments = ledger
        .borrowed_terminal_arguments_v1(owner, &exit)
        .unwrap()
        .unwrap();
    let [LocalCallArgumentV1::CallResult(borrowed), LocalCallArgumentV1::CallResult(strict)] =
        arguments.as_ref()
    else {
        panic!("two exact nested arguments")
    };
    assert!(matches!(
        borrowed.arguments(),
        [LocalCallArgumentV1::BorrowedActual { .. }]
    ));
    assert!(matches!(
        strict.arguments(),
        [LocalCallArgumentV1::Integer(8)]
    ));
    ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit)
        .unwrap()
        .unwrap();
    let strict_row = ledger
        .take_lexical_instance_call(owner, strict.site().site())
        .unwrap()
        .unwrap();
    assert!(ledger
        .borrowed_call_actuals_v1(&strict_row)
        .unwrap()
        .is_none());
    let borrowed_row = ledger
        .take_lexical_instance_call(owner, borrowed.site().site())
        .unwrap()
        .unwrap();
    assert!(ledger
        .borrowed_call_actuals_v1(&borrowed_row)
        .unwrap()
        .is_some());
}

#[test]
fn pure_strict_return_does_not_select_the_borrowed_terminal_seam() {
    let package = package("return recv.wrap(8)");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().unwrap();
    assert!(ledger
        .borrowed_terminal_arguments_v1(owner, &exit(ledger))
        .unwrap()
        .is_none());
    if let Some(arguments) = ledger.terminal_call_arguments_for_owner_at(owner, &exit(ledger)) {
        assert!(arguments
            .iter()
            .all(|argument| !matches!(argument, TerminalCallArgumentV1::Lexical(_))));
    }
}

#[test]
fn nested_borrowed_missing_incoming_and_actual_drift_refuse_before_outer_take() {
    for missing in [true, false] {
        let mut package = package("return recv.wrap(recv.probe(true))");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        let outer = ledger
            .call_source_completion_for_owner_at(owner, &exit)
            .unwrap()
            .1
            .call_site()
            .clone();
        if missing {
            ledger
                .borrowed_formal_source
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap()
                .incoming = Box::new([]);
        } else {
            ledger
                .borrowed_formal_actuals
                .values_mut()
                .next()
                .unwrap()
                .as_mut()
                .unwrap()
                .ordered_arguments[0] = LocalCallArgumentV1::Integer(0);
        }
        let error = ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap_err();
        assert!(
            error.contains(if missing {
                "borrowed-terminal/incoming-missing"
            } else {
                "borrowed-entry/ordered-arguments-identity"
            }),
            "{error}"
        );
        assert!(ledger.lexical_instance_call_covered(owner, &outer));
    }
}

#[test]
fn nested_argument_site_swap_and_foreign_home_set_refuse_without_consumption() {
    let package = package("return recv.pair(recv.probe(true), recv.wrap(8))");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().unwrap();
    let exit = exit(ledger);
    let site = OwnedExprSiteV1::new(
        owner,
        ledger
            .call_source_completion_for_owner_at(owner, &exit)
            .unwrap()
            .1
            .call_site()
            .clone(),
    );
    let original = ledger
        .borrowed_terminal_arguments_v1(owner, &exit)
        .unwrap()
        .unwrap();
    let homes = ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .exit_row(&exit)
        .unwrap()
        .unwrap();
    let mut moved = original.clone();
    moved.swap(0, 1);
    let error = ledger
        .validate_borrowed_terminal_arguments_v1(&site, &moved, homes.homes())
        .unwrap_err();
    assert!(
        error.contains("borrowed-terminal/nested-site-or-homes"),
        "{error}"
    );
    let error = ledger
        .validate_borrowed_terminal_arguments_v1(&site, &original, &[])
        .unwrap_err();
    assert!(
        error.contains("borrowed-terminal/nested-site-or-homes"),
        "{error}"
    );
    assert!(ledger.lexical_instance_call_covered(owner, site.site()));
    for argument in original.iter() {
        let LocalCallArgumentV1::CallResult(inner) = argument else {
            panic!("nested")
        };
        assert!(ledger.lexical_instance_call_covered(owner, inner.site().site()));
    }
}

#[test]
fn final_terminal_coverage_refuses_missing_outer_inner_and_foreign_argument_sites() {
    for corruption in 0..3 {
        let package = package("return recv.wrap(recv.probe(true))");
        let ledger = &package.ordinary_new_claim_ledger;
        ledger.validate_terminal_lexical_ready_v1().unwrap();
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        let outer = OwnedExprSiteV1::new(
            owner,
            ledger
                .call_source_completion_for_owner_at(owner, &exit)
                .unwrap()
                .1
                .call_site()
                .clone(),
        );
        let arguments = ledger
            .borrowed_terminal_arguments_v1(owner, &exit)
            .unwrap()
            .unwrap();
        let [LocalCallArgumentV1::CallResult(inner)] = arguments.as_ref() else {
            panic!("inner")
        };
        {
            let mut slots = ledger.lexical_instance_calls.borrow_mut();
            match corruption {
                0 => {
                    slots.remove(&outer);
                }
                1 => {
                    slots.remove(inner.site());
                }
                _ => {
                    let LexicalInstanceCallDispositionSlotV1::Ready(row) =
                        slots.get_mut(&outer).unwrap()
                    else {
                        panic!("ready")
                    };
                    row.source.argument_sites[0] = outer.site().clone();
                }
            }
        }
        let error = ledger.validate_terminal_lexical_ready_v1().unwrap_err();
        assert!(
            error.contains(if corruption < 2 {
                "terminal-disposition-missing"
            } else {
                "terminal-nested-source-mismatch"
            }),
            "{error}"
        );
        assert!(ledger.lexical_instance_call_covered(
            owner,
            if corruption == 1 {
                outer.site()
            } else {
                inner.site().site()
            }
        ));
    }
}

#[test]
fn final_terminal_result_gate_refuses_real_unit_float_missing_and_foreign_contracts() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        TerminalRelationV1, TerminalReturnedSourceV1,
    };
    let text = r#"box Probe {
        birth() {}
        unit(q: i64) { local m = %{"k" => 1} return void }
        floating(q: i64) { local m = %{"k" => 1} return 1.5 }
    }
    static box Main { main() { local recv = new Probe() recv.unit(1) recv.floating(1) return 0 } }"#;
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    for name in ["unit", "floating"] {
        let source = slots
            .values()
            .find_map(|slot| match slot {
                LexicalInstanceCallDispositionSlotV1::Ready(row) if row.target().name() == name => {
                    Some(row.source_target())
                }
                _ => None,
            })
            .unwrap();
        let contract = package
            .result_contracts
            .row(source.target_batch_slot())
            .unwrap()
            .borrow();
        assert_eq!(contract.owner(), source.callee_owner());
        assert!(contract.result().is_none());
        if name == "unit" {
            assert!(matches!(
                contract.sole_terminal_relation(),
                Some(TerminalRelationV1::Unit(_))
            ));
            assert!(!contract.completion().returns_value());
        } else {
            assert!(
                matches!(contract.sole_terminal_relation(), Some(TerminalRelationV1::Value(value)) if value.returned() == &TerminalReturnedSourceV1::FloatLiteral)
            );
            assert!(contract.completion().returns_value());
        }
        assert!(ledger
            .corroborate_terminal_lexical_result_v1(source, &package.result_contracts)
            .unwrap_err()
            .contains("terminal-result-mismatch"));
    }
    drop(slots);
    let valid = super::terminal_tests::package("return recv.wrap(recv.probe(true))");
    let ledger = &valid.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().unwrap();
    let outer = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit(ledger))
        .unwrap()
        .unwrap();
    ledger
        .corroborate_terminal_lexical_result_v1(outer.source_target(), &valid.result_contracts)
        .unwrap();
    for missing in [true, false] {
        let mut source = outer.source_target().clone();
        if missing {
            source.target_batch_slot = u32::MAX;
        } else {
            source.callee_owner = owner;
        }
        assert!(ledger
            .corroborate_terminal_lexical_result_v1(&source, &valid.result_contracts)
            .unwrap_err()
            .contains("terminal-result-mismatch"));
    }
}

#[test]
fn final_terminal_ready_closure_uses_the_same_index_priority_as_source_accessors() {
    let mut package = package("if true { return recv.wrap(recv.probe(true)) } else { return recv.wrap(recv.probe(false)) }");
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = ledger.root_owner().unwrap();
    let exits = ledger.root_completion_for_test().explicit_sites().to_vec();
    let canonical = ledger
        .terminal_relation_for_owner_at(owner, &exits[0])
        .unwrap()
        .clone();
    let stale = ledger
        .terminal_relation_for_owner_at(owner, &exits[1])
        .unwrap()
        .clone();
    let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Call(stale_call) =
        &stale
    else {
        panic!("call")
    };
    {
        let mut slots = ledger.lexical_instance_calls.borrow_mut();
        slots.remove(&OwnedExprSiteV1::new(owner, stale_call.call_site().clone()));
        for argument in stale_call.arguments() {
            if let TerminalCallArgumentV1::Lexical(LocalCallArgumentV1::CallResult(inner)) =
                argument
            {
                slots.remove(inner.site());
            }
        }
    }
    ledger.terminal_relation_index.insert(
        owner,
        std::rc::Rc::new(std::collections::BTreeMap::from([(
            exits[0].clone(),
            canonical,
        )])),
    );
    ledger.terminal_relation.insert(exits[1].clone(), stale);
    assert_eq!(ledger.call_relations_for_owner(owner).len(), 1);
    assert!(ledger
        .terminal_relation_for_owner_at(owner, &exits[1])
        .is_none());
    ledger.validate_terminal_lexical_ready_v1().unwrap();
}

#[test]
fn strict_outer_noninteger_return_does_not_claim_borrowed_terminal() {
    for body in [
        "return 1.5",
        "return true",
        "return \"s\"",
        "return null",
        "if x > 0 { return 1 } return 1.5",
    ] {
        let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} bad(x: i64): i64 {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() return recv.bad(recv.probe(true)) }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source)
            .unwrap_or_else(|error| panic!("{body}: {error:?}"));
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = ledger.root_owner().unwrap();
        let exit = exit(ledger);
        assert!(
            ledger
                .borrowed_terminal_arguments_v1(owner, &exit)
                .unwrap()
                .is_none(),
            "{body}"
        );
        if let Some((_, terminal)) = ledger.call_source_completion_for_owner_at(owner, &exit) {
            assert!(
                !terminal
                    .arguments()
                    .iter()
                    .any(|argument| matches!(argument, TerminalCallArgumentV1::Lexical(_))),
                "{body}"
            );
        }
    }
}
