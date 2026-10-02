use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, TerminalCallArgumentV1,
};

fn package(
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() {main} }} }}");
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
