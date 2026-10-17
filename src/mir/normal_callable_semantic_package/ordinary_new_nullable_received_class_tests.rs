use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;

/// A sealed `Nullable` receiver-call observation also mints the caller's
/// `Nullable` local-call flow row — the emission-side membership check
/// (`nullable_call_source`) reads exactly this row, so the scan and the
/// ledger lookup must agree on site, owner, and class.
#[test]
fn nullable_receiver_call_mints_the_caller_local_call_flow_row() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) { return new Page(flag) }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Page(7)
    }
    observe(flag) {
        local made = me.make(flag)
        local fetched = me.fetch(flag)
        return 0
    }
}
box OtherPage {
    fetch(flag) { if flag == 0 { return null } return new OtherPage() }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("nullable receiver-call flow source package");
    let catalog = crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(
        package.batch(),
    )
    .unwrap();
    let entry_loans = crate::mir::resolved_semantics::CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(package.batch(), &catalog).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    for (site, row) in ledger.receiver_call_observations_for_test() {
        let expected_nullable = matches!(
            row.class(),
            crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(
                _
            )
        );
        let declaration = package
            .batch()
            .declarations()
            .find(|declaration| declaration.owner() == site.owner())
            .unwrap();
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            &package.selected,
            package.batch(),
            entry_loans.for_batch_slot(declaration.batch_slot()),
            declaration.batch_slot(),
        )
        .unwrap();
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                let consult = |proof| {
                    super::nullable_received_result_class(
                        &package.selected,
                        package.batch(),
                        &ledger.callable_result_classes,
                        &[],
                        input,
                        proof,
                        row.destination(),
                    )
                };
                assert_eq!(
                    consult(Some(receiver)),
                    expected_nullable.then(|| "Page".into())
                );
                assert_eq!(
                    consult(None),
                    None,
                    "self initializer requires original receiver proof"
                );
                assert_eq!(
                    consult(Some((row.destination(), receiver.1))),
                    None,
                    "a local destination cannot replace the receiver proof"
                );
                let other_box = package
                    .batch()
                    .ordinary_box_coverage()
                    .row_for("OtherPage")
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    consult(Some((receiver.0, other_box))),
                    None,
                    "same receiver binding plus another box cannot lend another callee class"
                );
            })
            .unwrap();
        let flow = ledger
            .completion_for_owner(site.owner())
            .and_then(|completion| completion.cleanup().root_flow())
            .unwrap_or_else(|| panic!("{site:?} caller has a root flow"));
        let local = flow.local_calls().iter().find(|call| call.site() == site);
        if expected_nullable {
            assert_eq!(
                ledger
                    .borrowed_nullable_actual_class_v1(row.destination())
                    .unwrap()
                    .as_deref(),
                Some("Page")
            );
            let local = local.unwrap_or_else(|| panic!("{site:?} mints a local-call flow row"));
            assert_eq!(
                local.result(),
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable,
                "{site:?} flow row class mirrors the sealed observation class"
            );
            assert_eq!(
                local.local_binding().expect("local destination").1,
                row.destination(),
                "the flow row keeps the exact declared destination"
            );
        } else {
            // Original Object result membership records a Handle source row.
            // Argument borrowing cannot classify it as an I64 result; this
            // source observation alone grants no Normal or physical permission.
            let local = local.expect("definite receiver records its original Handle source");
            assert_eq!(
                local.result(),
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Handle
            );
            assert_eq!(
                local.local_binding().expect("local destination").1,
                row.destination()
            );
        }
    }
    // The root source is the SAME Completion, even when it has no indexed row.
    let mut root = issue_with_brand_catalog(
        "box Token {} box Maker { fetch(flag) { if flag == 0 { return null } return new Token() } } static box Main { main() { local maker = new Maker() local result = maker.fetch(1) return 0 } }"
    ).unwrap();
    let binding = root
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .local_calls()
        .iter()
        .find(|call| {
            call.result()
                == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable
        })
        .unwrap()
        .local_binding()
        .unwrap()
        .1;
    let ledger = std::rc::Rc::get_mut(&mut root.ordinary_new_claim_ledger).unwrap();
    let indexed = ledger.completion_index.remove(&binding.owner());
    assert_eq!(
        ledger
            .borrowed_nullable_actual_class_v1(binding)
            .unwrap()
            .as_deref(),
        Some("Token")
    );
    ledger.completion_index.insert(binding.owner(), Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation("retained-index-refusal".into())));
    assert!(ledger
        .borrowed_nullable_actual_class_v1(binding)
        .unwrap_err()
        .contains("retained-index-refusal"));
    ledger.completion_index.remove(&binding.owner());
    let original = ledger.root_completion.take();
    assert_eq!(
        ledger.borrowed_nullable_actual_class_v1(binding).unwrap(),
        None
    );
    ledger.root_completion = Some(Err(
        crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation(
            "retained-root-refusal".into(),
        ),
    ));
    assert!(ledger
        .borrowed_nullable_actual_class_v1(binding)
        .unwrap_err()
        .contains("retained-root-refusal"));
    ledger.root_completion = original;
    if let Some(indexed) = indexed {
        ledger.completion_index.insert(binding.owner(), indexed);
    }
}
