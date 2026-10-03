use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;

fn package(
    main: &str,
    methods: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} {methods} }} static box Main {{ main() {{ local recv = new Transport() {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("source-backed borrowed discard package")
}

#[test]
fn borrowed_statement_discard_retains_original_actuals_without_local_destination() {
    for argument in ["true", "-7", "recv"] {
        let package = package(&format!("recv.probe({argument}) return 0"), "");
        let ledger = &package.ordinary_new_claim_ledger;
        let completion = ledger.root_completion_for_test();
        let flow = completion
            .cleanup()
            .root_flow()
            .expect("complete caller flow");
        assert_eq!(flow.local_calls().len(), 1);
        let call = &flow.local_calls()[0];
        assert!(call.local_binding().is_none(), "discard creates no local");
        assert_eq!(call.result(), LocalCallResultClassV1::I64);
        assert_eq!(call.prior_homes().len(), 1, "only the receiver Home exists");
        let pending = ledger.borrowed_formal_actuals[call.site()]
            .as_ref()
            .unwrap();
        assert_eq!(call.arguments(), pending.ordered_arguments.as_ref());
        assert!(matches!(
            call.arguments(),
            [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
        ));
        let taken = ledger
            .take_lexical_instance_call(call.owner(), call.site().site())
            .unwrap()
            .expect("same affine disposition");
        assert!(std::ptr::eq(
            ledger.borrowed_call_actuals_v1(&taken).unwrap().unwrap(),
            pending.opaque_actuals.as_ref()
        ));
        assert!(
            ledger
                .take_lexical_instance_call(call.owner(), call.site().site())
                .is_err(),
            "one-shot"
        );
    }
}

#[test]
fn strict_statement_call_does_not_acquire_borrowed_discard_membership() {
    let package = package(
        "recv.strict(7) return 0",
        "strict(p: i64): i64 { return p }",
    );
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    assert!(ledger.borrowed_formal_actuals.is_empty());
    assert!(completion
        .cleanup()
        .root_flow()
        .map_or(true, |flow| flow.local_calls().is_empty()));
}

#[test]
fn selected_discard_failure_is_terminal_without_strict_retry() {
    for argument in ["\"unsupported\"", "1.5"] {
        let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() recv.probe({argument}) return 0 }} }}");
        let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source)
            .err().expect("selected unsupported discard must reject");
        assert!(
            format!("{error:?}").contains("borrowed-actual/unsupported-or-unavailable"),
            "{error:?}"
        );
    }
}

#[test]
fn branch_discards_keep_distinct_sites_and_only_their_own_exit_groups() {
    let package = package(
        "if true { recv.probe(true) return 0 } else { recv.probe(false) return 0 }",
        "",
    );
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion
        .cleanup()
        .root_flow()
        .expect("complete branch flow");
    assert!(flow.all_exits_ready());
    assert_eq!(completion.explicit_sites().len(), 2);
    assert_eq!(flow.local_calls().len(), 2);
    assert_ne!(flow.local_calls()[0].site(), flow.local_calls()[1].site());
    assert!(flow
        .local_calls()
        .iter()
        .all(|call| call.local_binding().is_none()));
    let mut covered = std::collections::BTreeSet::new();
    for exit in completion.explicit_sites() {
        let row = flow.exit_row(exit).unwrap().unwrap();
        assert_eq!(row.homes().len(), 1, "only the original receiver Home");
        assert_eq!(row.covered_calls().len(), 1, "branch-local call group");
        assert!(covered.insert(row.covered_calls()[0].clone()));
        let call = flow
            .local_calls()
            .iter()
            .find(|call| call.site() == &row.covered_calls()[0])
            .unwrap();
        assert_eq!(call.prior_homes(), row.homes());
    }
    assert_eq!(covered.len(), 2);
}
