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
    if let Some(relation) = package
        .ordinary_new_claim_ledger
        .lexical_i64_call_source(site)
    {
        assert_eq!(
            relation.arguments(),
            rows.as_ref().unwrap().ordered_arguments.as_ref()
        );
    }
    let rows = &rows
        .as_ref()
        .expect("complete pending actual source")
        .opaque_actuals;
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
        assert_local_projection(&package);
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
    assert_eq!(flow.local_calls().len(), 1);
    assert!(flow.local_calls()[0].prior_homes().contains(root));
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
        .flat_map(|rows| rows.opaque_actuals.iter())
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
    let calls = completion.cleanup().root_flow().unwrap().local_calls();
    assert_eq!(calls.len(), 1);
    assert!(matches!(
        calls[0].arguments(),
        [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
    ));
}

#[test]
fn unsupported_actuals_are_pending_named_errors_and_never_integer_payloads() {
    for argument in ["1.5", "\"text\"", "null", "%{\"key\"=>1}"] {
        let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe({argument}) return 0 }} }}");
        let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).err().expect("selected unsupported domain stops package issue");
        let error = format!("{error:?}");
        assert!(
            error.contains("borrowed-actual/unsupported-or-unavailable"),
            "{error}"
        );
    }
}

#[test]
fn a_home_consumed_by_a_map_cannot_be_borrowed_afterwards() {
    let source = "box Transport { birth() { } probe(p): i64 { return 0 } } static box Main { main() { local obj = new Transport() local m = %{\"key\"=>obj} local recv = new Transport() local out = recv.probe(obj) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).err().expect("consumed root never obtains tag 3");
    assert!(format!("{error:?}").contains("borrowed-actual/unsupported-or-unavailable"));
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
    let actual = &pending.as_ref().unwrap().opaque_actuals[0];
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
    // The direct inner call is now observed. Its enclosing opaque argument
    // remains an unsupported CallResult, so the whole incoming cohort rejects.
    let source = "box Transport { birth() { } probe(p): i64 { return 0 } } static box Main { main() { local recv = new Transport() local out = recv.probe(recv.probe(0)) return 0 } }";
    assert_package_rejected(source, "borrowed-actual/unsupported-or-unavailable");
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
        rows.as_ref().unwrap().opaque_actuals[0].source,
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

fn mixed_package(
    scalar_type: &str,
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p, q: {scalar_type}): i64 {{ return 0 }} }} static box Main {{ main() {{ {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .unwrap()
}

fn mixed_candidates(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> (OwnedExprSiteV1, Vec<BorrowedCallActualCandidateV1>) {
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let call = &source.incoming[0];
    let rows = call
        .source
        .argument_sites()
        .iter()
        .enumerate()
        .map(|(index, site)| BorrowedCallActualCandidateV1 {
            ordinal: index as u32,
            site: site.clone(),
            value: if index == 0 {
                BorrowedCallActualValueV1::Bool(true)
            } else {
                BorrowedCallActualValueV1::Integer(7)
            },
        })
        .collect();
    (call.call.clone(), rows)
}

#[test]
fn mixed_actuals_preserve_opaque_domain_and_exact_integer_parameter_lanes() {
    for scalar_type in ["i64", "usize"] {
        for prefix_and_argument in [("", "7"), ("local n = 7", "n")] {
            let (prefix, argument) = prefix_and_argument;
            let package = mixed_package(scalar_type, &format!("{prefix} local recv = new Transport() local out = recv.probe(true, {argument}) return 0"));
            assert_local_projection(&package);
            let row = only_actual(&package);
            assert_eq!(row.ordinal, 0);
            assert_eq!(row.source, BorrowedFormalActualSourceV1::Bool(true));
            let pending = package
                .ordinary_new_claim_ledger
                .borrowed_formal_actuals
                .values()
                .next()
                .unwrap()
                .as_ref()
                .unwrap();
            assert_eq!(pending.ordered_arguments.len(), 2);
            assert_eq!(
                pending.ordered_arguments[0],
                LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: row.site.clone()
                }
            );
            if argument == "7" {
                assert_eq!(
                    pending.ordered_arguments[1],
                    LocalCallArgumentV1::Integer(7)
                );
            } else {
                let caller = package
                    .ordinary_new_claim_ledger
                    .borrowed_formal_actuals
                    .keys()
                    .next()
                    .unwrap()
                    .owner();
                assert!(
                    matches!(pending.ordered_arguments[1], LocalCallArgumentV1::Scalar(binding) if binding.owner() == caller)
                );
            }
        }
    }
    let package = mixed_package(
        "i64",
        "local recv = new Transport() local out = recv.probe(true, -7) return 0",
    );
    assert_eq!(
        only_actual(&package).source,
        BorrowedFormalActualSourceV1::Bool(true)
    );
    let pending = package
        .ordinary_new_claim_ledger
        .borrowed_formal_actuals
        .values()
        .next()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(
        pending.ordered_arguments[1],
        LocalCallArgumentV1::Integer(-7)
    );
}

#[test]
fn mixed_actuals_reject_noninteger_domains_in_the_nonopaque_scalar_slot() {
    for argument in ["false", "recv", "\"text\"", "null", "1.5", "%{\"key\"=>1}"] {
        let source = format!("box Transport {{ birth() {{ }} probe(p, q: i64): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe(true, {argument}) return 0 }} }}");
        let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).err().expect("selected nonopaque slot is required");
        let error = format!("{error:?}");
        assert!(
            error.contains("borrowed-actual/nonopaque-scalar-unproved"),
            "{argument}: {error}"
        );
    }
}

#[test]
fn mixed_actuals_reject_bool_bindings_in_the_nonopaque_scalar_slot() {
    let source = "box Transport { birth() { } probe(p, q: i64): i64 { return 0 } } static box Main { main() { local n = true local recv = new Transport() local out = recv.probe(0, n) return 0 } }";
    assert_package_rejected(source, "borrowed-actual/nonopaque-scalar-unproved");
}

#[test]
fn mixed_actuals_check_nonopaque_ordinal_and_source_site() {
    let package = mixed_package(
        "i64",
        "local recv = new Transport() local out = recv.probe(true, 7) return 0",
    );
    for wrong_site in [false, true] {
        let (call, mut actuals) = mixed_candidates(&package);
        if wrong_site {
            actuals[1].site = actuals[0].site.clone();
        } else {
            actuals[1].ordinal = 0;
        }
        let error = prepare_borrowed_call_actuals_v1(
            package
                .ordinary_new_claim_ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap(),
            &package.parameter_contracts,
            &call,
            &actuals,
            &[],
            None,
        )
        .unwrap_err();
        assert!(error.contains("borrowed-actual/source-identity"), "{error}");
    }
}

#[test]
fn mixed_actuals_reject_foreign_nonopaque_scalar_bindings() {
    let package = mixed_package(
        "i64",
        "local recv = new Transport() local out = recv.probe(true, 7) return 0",
    );
    let (call, mut actuals) = mixed_candidates(&package);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let callee = source.incoming[0].callee;
    let foreign = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == callee)
        .unwrap()
        .parameters[1]
        .binding;
    actuals[1].value = BorrowedCallActualValueV1::Scalar(foreign, SourceScalarKind::Integer);
    let error = prepare_borrowed_call_actuals_v1(
        package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap(),
        &package.parameter_contracts,
        &call,
        &actuals,
        &[],
        None,
    )
    .unwrap_err();
    assert!(
        error.contains("borrowed-actual/nonopaque-scalar-unproved"),
        "{error}"
    );
}

#[test]
fn mixed_actuals_check_the_nonopaque_formal_ordinal() {
    let mut package = mixed_package(
        "i64",
        "local recv = new Transport() local out = recv.probe(true, 7) return 0",
    );
    let (call, actuals) = mixed_candidates(&package);
    let callee = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming[0]
        .callee;
    package
        .parameter_contracts
        .iter_mut()
        .find(|row| row.owner == callee)
        .unwrap()
        .parameters[1]
        .ordinal = 0;
    let error = prepare_borrowed_call_actuals_v1(
        package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap(),
        &package.parameter_contracts,
        &call,
        &actuals,
        &[],
        None,
    )
    .unwrap_err();
    assert!(error.contains("borrowed-actual/source-identity"), "{error}");
}

#[test]
fn installed_entry_demands_nonopaque_actual_proof_before_borrowed_values() {
    for bad in [false, true] {
        let mut package = mixed_package(
            "i64",
            "local recv = new Transport() local out = recv.probe(true, 7) return 0",
        );
        if bad {
            *std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger)
                .unwrap()
                .borrowed_formal_actuals
                .values_mut()
                .next()
                .unwrap() = Err("borrowed-actual/nonopaque-scalar-unproved".into());
        }
        let source = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let callee = source.incoming[0].callee;
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == callee)
            .unwrap();
        let key = package
            .selected
            .key_for_batch_slot(contract.batch_slot)
            .unwrap()
            .clone();
        let ledger = std::rc::Rc::clone(&package.ordinary_new_claim_ledger);
        let mut context = crate::mir::builder::CompilationContext::new();
        let installed = package.prepare_install(&mut context).unwrap().commit();
        let mut port = installed.begin_lowering(&context).unwrap();
        port.with_selected_lowering_input(&key, |input| {
            let projection = ledger.borrowed_ordinary_entry_source_v1(&input);
            if bad {
                let error = projection.unwrap_err();
                assert!(
                    error.contains("borrowed-actual/nonopaque-scalar-unproved"),
                    "{error}"
                );
            } else {
                let projection = projection.unwrap().unwrap();
                assert_eq!(projection.formals().len(), 1);
                assert_eq!(projection.incoming().len(), 1);
                assert_eq!(
                    projection.incoming()[0].1[0].source,
                    BorrowedFormalActualSourceV1::Bool(true)
                );
            }
        })
        .unwrap();
    }
}

fn nested_package(
    parameters: &str,
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe({parameters}): i64 {{ return 0 }} wrap(x: i64): i64 {{ return x }} pair(x: i64, y: i64): i64 {{ return x }} }} static box Main {{ main() {{ local recv = new Transport() {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .unwrap()
}

fn assert_nested_consumers(
    package: crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    expected_error: Option<&str>,
) {
    let ledger = std::rc::Rc::clone(&package.ordinary_new_claim_ledger);
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let callee = source.incoming[0].callee;
    for call in &source.incoming {
        let row = ledger
            .take_lexical_instance_call(call.call.owner(), call.call.site())
            .unwrap()
            .unwrap();
        let actuals = ledger.borrowed_call_actuals_v1(&row);
        if let Some(expected) = expected_error {
            let error = actuals.unwrap_err();
            assert!(error.contains(expected), "{error}");
        } else {
            let actuals = actuals.unwrap().unwrap();
            assert!(std::ptr::eq(
                actuals,
                ledger.borrowed_formal_actuals[&call.call]
                    .as_ref()
                    .unwrap()
                    .opaque_actuals
                    .as_ref()
            ));
            assert_eq!(actuals[0].ordinal, 0);
            assert_eq!(actuals[0].site, row.argument_sites()[0]);
        }
    }
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == callee)
        .unwrap();
    let key = package
        .selected
        .key_for_batch_slot(contract.batch_slot)
        .unwrap()
        .clone();
    let expected_incoming = source.incoming.len();
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    let mut port = installed.begin_lowering(&context).unwrap();
    port.with_selected_lowering_input(&key, |input| {
        let entry = ledger.borrowed_ordinary_entry_source_v1(&input);
        if let Some(expected) = expected_error {
            let error = entry.unwrap_err();
            assert!(error.contains(expected), "{error}");
        } else {
            assert_eq!(entry.unwrap().unwrap().incoming().len(), expected_incoming);
        }
    })
    .unwrap();
}

#[test]
fn nested_actuals_reach_entry_and_taken_lender_from_all_call_roots() {
    for main in [
        "local out = recv.wrap(recv.probe(true)) return 0",
        "recv.wrap(recv.probe(true)) return 0",
        "return recv.wrap(recv.probe(true))",
        "local out = recv.wrap(recv.wrap(recv.probe(true))) return 0",
    ] {
        let package = nested_package("p", main);
        assert_eq!(
            only_actual(&package).source,
            BorrowedFormalActualSourceV1::Bool(true)
        );
        assert_nested_consumers(package, None);
    }
}

#[test]
fn nested_sibling_actuals_keep_separate_source_sites_and_full_incoming_coverage() {
    let package = nested_package(
        "p",
        "local out = recv.pair(recv.probe(true), recv.probe(false)) return 0",
    );
    let rows = &package.ordinary_new_claim_ledger.borrowed_formal_actuals;
    assert_eq!(rows.len(), 2);
    let actuals: Vec<_> = rows
        .values()
        .map(|row| &row.as_ref().unwrap().opaque_actuals[0])
        .collect();
    assert_ne!(actuals[0].site, actuals[1].site);
    assert_eq!(actuals[0].source, BorrowedFormalActualSourceV1::Bool(true));
    assert_eq!(actuals[1].source, BorrowedFormalActualSourceV1::Bool(false));
    assert_nested_consumers(package, None);
}

#[test]
fn nested_failed_actuals_remain_terminal_for_real_consumers() {
    for (parameters, call, expected) in [
        (
            "p",
            "recv.wrap(recv.probe(\"unsupported\"))",
            "borrowed-actual/unsupported-or-unavailable",
        ),
        (
            "p, q: i64",
            "recv.wrap(recv.probe(true, false))",
            "borrowed-actual/nonopaque-scalar-unproved",
        ),
        (
            "p",
            "recv.pair(recv.probe(true), recv.probe(\"unsupported\"))",
            "borrowed-actual/unsupported-or-unavailable",
        ),
    ] {
        let source = format!("box Transport {{ birth() {{ }} probe({parameters}): i64 {{ return 0 }} wrap(x: i64): i64 {{ return x }} pair(x: i64, y: i64): i64 {{ return x }} }} static box Main {{ main() {{ local recv = new Transport() local out = {call} return 0 }} }}");
        assert_package_rejected(&source, expected);
    }
}

#[test]
fn nested_observer_does_not_claim_calls_inside_binary_argument_subtrees() {
    let package = nested_package("p", "local out = recv.wrap(recv.probe(true) + 1) return 0");
    assert_nested_consumers(
        package,
        Some("borrowed-actual/selected-incoming-unobserved"),
    );
}

fn assert_package_rejected(source: &str, terminal: &str) {
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).err().expect("selected failure stops package issue");
    assert!(format!("{error:?}").contains(terminal), "{error:?}");
}

fn assert_local_projection(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) {
    let ledger = &package.ordinary_new_claim_ledger;
    for (site, actuals) in &ledger.borrowed_formal_actuals {
        let relation = ledger
            .lexical_i64_call_source(site)
            .expect("accepted local must retain its selected continuation");
        assert_eq!(
            relation.arguments(),
            actuals.as_ref().unwrap().ordered_arguments.as_ref()
        );
    }
}
