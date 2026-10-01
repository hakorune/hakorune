use super::*;

fn package(
    main: &str,
    extra: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!(
        "box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} {extra} }} \
         static box Main {{ main() {{ {main} }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("source-backed ordinary package")
}

fn only_actual(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> &PreparedBorrowedFormalActualV1 {
    let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
    assert_eq!(rows.len(), 1);
    let (site, rows) = rows.iter().next().unwrap();
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_i64_call_source(site)
            .is_none(),
        "pending borrowed actual must not install the old lifecycle local-call row"
    );
    let rows = rows.as_ref().expect("complete pending actual source");
    assert_eq!(rows.len(), 1);
    &rows[0]
}

#[test]
fn production_prefix_keeps_signed_integer_and_bool_payload_domains_separate() {
    for (argument, expected) in [
        ("0", BorrowedFormalActualSourceV1::Integer(0)),
        ("-7", BorrowedFormalActualSourceV1::Integer(-7)),
        ("false", BorrowedFormalActualSourceV1::Bool(false)),
        ("true", BorrowedFormalActualSourceV1::Bool(true)),
    ] {
        let package = package(
            &format!("local recv = new Transport() local out = recv.probe({argument}) return 0"),
            "",
        );
        assert_eq!(only_actual(&package).source, expected);
    }
}

#[test]
fn exact_scalar_bindings_are_prepared_without_reclassifying_the_formal() {
    for (initializer, kind) in [
        ("12", SourceScalarKind::Integer),
        ("true", SourceScalarKind::Bool),
    ] {
        let package = package(&format!("local arg = {initializer} local recv = new Transport() local out = recv.probe(arg) return 0"), "");
        assert!(
            matches!(only_actual(&package).source, BorrowedFormalActualSourceV1::Scalar { kind: actual, .. } if actual == kind)
        );
    }
}

#[test]
fn typed_object_home_is_borrowed_and_not_transferred_into_the_pending_call() {
    let package = package("local obj = new Transport() local recv = new Transport() local out = recv.probe(obj) return 0", "");
    let row = only_actual(&package);
    let BorrowedFormalActualSourceV1::TypedHome {
        binding,
        root,
        class,
        ..
    } = &row.source
    else {
        panic!("typed object provenance")
    };
    assert_eq!(binding, root);
    assert_eq!(class.as_ref(), "Transport");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    assert!(flow.local_calls().is_empty());
}

#[test]
fn formal_copy_forward_preserves_the_original_formal_and_actual_binding() {
    let package = package("local recv = new Transport() local out = recv.bridge(0) return 0",
        "bridge(p): i64 { local alias = p local recv = new Transport() local out = recv.probe(alias) return 0 }");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .borrowed_formal_actuals
            .len(),
        2
    );
    let row = package
        .ordinary_new_claim_ledger
        .borrowed_formal_actuals
        .values()
        .filter_map(|rows| rows.as_ref().ok())
        .flat_map(|rows| rows.iter())
        .find(|row| matches!(row.source, BorrowedFormalActualSourceV1::Forwarded { .. }))
        .unwrap();
    let BorrowedFormalActualSourceV1::Forwarded { binding, formal } = row.source else {
        panic!("selected opaque formal forwarding")
    };
    assert_ne!(binding, formal);
    assert_eq!(binding.owner(), formal.owner());
    let completion = package
        .ordinary_new_claim_ledger
        .completion_for_owner(binding.owner())
        .expect("existing homes-aware completion is preserved");
    assert!(
        completion
            .cleanup()
            .root_flow()
            .unwrap()
            .local_calls()
            .is_empty(),
        "pending borrowed actuals must not arm lifecycle calls"
    );
}

#[test]
fn unsupported_actuals_are_pending_named_errors_and_never_integer_payloads() {
    for argument in ["1.5", "\"text\"", "null", "%{\"key\"=>1}"] {
        let package = package(
            &format!("local recv = new Transport() local out = recv.probe({argument}) return 0"),
            "",
        );
        let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
        assert_eq!(rows.len(), 1, "argument: {argument}");
        let error = rows
            .values()
            .next()
            .unwrap()
            .as_ref()
            .expect_err("unsupported domain");
        assert!(
            error.contains("borrowed-actual/unsupported-or-unavailable"),
            "{error}"
        );
    }
}

#[test]
fn a_home_consumed_by_a_map_cannot_be_borrowed_afterwards() {
    let package = package("local obj = new Transport() local m = %{\"key\"=>obj} local recv = new Transport() local out = recv.probe(obj) return 0", "");
    let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
    assert_eq!(rows.len(), 1);
    assert!(
        rows.values().next().unwrap().is_err(),
        "consumed root never obtains tag 3"
    );
}

#[test]
fn checked_actual_join_rejects_changed_argument_ordinal() {
    let package = package(
        "local recv = new Transport() local out = recv.probe(0) return 0",
        "",
    );
    let (call, pending) = package
        .ordinary_new_claim_ledger
        .borrowed_formal_actuals
        .iter()
        .next()
        .unwrap();
    let actual = &pending.as_ref().unwrap()[0];
    let candidates = [BorrowedCallActualCandidateV1 {
        ordinal: actual.ordinal + 1,
        site: actual.site.clone(),
        value: BorrowedCallActualValueV1::Integer(0),
    }];
    let error = prepare_borrowed_call_actuals_v1(
        package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap(),
        &package.parameter_contracts,
        call,
        &candidates,
        &[],
        None,
    )
    .unwrap_err();
    assert!(error.contains("borrowed-actual/source-identity"), "{error}");
}

#[test]
fn expression_statement_and_return_calls_have_pending_actuals() {
    for call in ["recv.probe(0) return 0", "return recv.probe(0)"] {
        let package = package(&format!("local recv = new Transport() {call}"), "");
        assert_eq!(
            only_actual(&package).source,
            BorrowedFormalActualSourceV1::Integer(0)
        );
    }
}

#[test]
fn verified_entry_receiver_proves_its_typed_object_domain() {
    let package = package(
        "return 0",
        "bridge(): i64 { local recv = new Transport() local out = recv.probe(me) return 0 }",
    );
    assert!(matches!(only_actual(&package).source,
        BorrowedFormalActualSourceV1::EntryReceiver { ref class, .. } if class.as_ref() == "Transport"));
}

#[test]
fn unwalked_nested_incoming_is_named_instead_of_silently_disappearing() {
    let package = package(
        "local recv = new Transport() local out = recv.probe(recv.probe(0)) return 0",
        "",
    );
    let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
    assert_eq!(rows.len(), 2);
    assert!(rows.values().all(Result::is_err));
    assert!(rows.values().any(|row| row
        .as_ref()
        .unwrap_err()
        .contains("borrowed-actual/selected-incoming-unobserved")));
}

#[test]
fn existing_control_rejects_incoming_after_explicit_return() {
    assert_unreachable_incoming_rejected("return 0 local out = recv.probe(0)");
}

#[test]
fn existing_control_rejects_incoming_after_both_branches_return() {
    assert_unreachable_incoming_rejected(
        "if true { return 0 } else { return 0 } local out = recv.probe(0)",
    );
}

fn assert_unreachable_incoming_rejected(body: &str) {
    let source = format!(
        "box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} \
        bridge(): i64 {{ local recv = new Transport() {body} }} }} \
        static box Main {{ main() {{ return 0 }} }}"
    );
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::
        issue_with_brand_catalog(&source).expect_err("unreachable incoming cannot supply an ABI proof");
    assert!(
        format!("{error:?}").contains("NonTerminalReturn"),
        "{error:?}"
    );
}

#[test]
fn failed_source_only_walk_invalidates_all_partial_owner_actuals() {
    let package = package("local recv = new Transport() local first = recv.probe(0) local second = recv.probe(true) return 0", "");
    let ledger = &package.ordinary_new_claim_ledger;
    let mut pending = ledger.borrowed_formal_actuals.clone();
    assert_eq!(pending.len(), 2);
    assert!(pending.values().all(Result::is_ok));
    let owner = pending.keys().next().unwrap().owner();
    reject_borrowed_actuals_for_owner_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        owner,
        &mut pending,
        "borrowed-actual/source-walk:test-failure".into(),
    );
    assert!(pending.values().all(|row| row
        .as_ref()
        .unwrap_err()
        .contains("source-walk:test-failure")));
}

#[test]
fn source_only_actual_probe_keeps_the_existing_plain_completion() {
    let package = package("return 0",
        "bridge(): i64 { local text = \"uncovered\" local recv = new Transport() local out = recv.probe(0) return 0 }");
    let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
    assert_eq!(rows.len(), 1);
    let (call, rows) = rows.iter().next().unwrap();
    assert_eq!(
        rows.as_ref().unwrap()[0].source,
        BorrowedFormalActualSourceV1::Integer(0)
    );
    let completion = package
        .ordinary_new_claim_ledger
        .completion_for_owner(call.owner())
        .expect("original plain completion");
    assert!(
        completion.cleanup().root_flow().is_none(),
        "source-only observation must not publish a homes-aware completion"
    );
}
