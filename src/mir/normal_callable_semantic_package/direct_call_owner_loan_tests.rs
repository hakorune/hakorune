//! Per-owner direct-call loans: scoped issuance, install preflight, and drain.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::direct_call_loan::DirectCallLoanErrorV1;

#[test]
fn non_app_main_map_receive_issues_owner_scoped_loans() {
    // `use_map` — not AppMain — is the owner that receives and releases the
    // Map: the sealed loan and the lifecycle classification follow the exact
    // owner, not the package root.
    let mut package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("non-AppMain map-receive package");
    let ledger = &package.ordinary_new_claim_ledger;
    let mut receive = None;
    let mut lifecycle_call = None;
    for declaration in package.batch().declarations() {
        let owner = declaration.owner();
        let Some(flow) = ledger
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            continue;
        };
        for call in flow.local_calls() {
            if call.result()
                == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Map
            {
                receive = Some((owner, call.site().site().clone()));
            } else {
                lifecycle_call = Some((owner, call.site().site().clone()));
            }
        }
    }
    let (receive_owner, receive_site) = receive.expect("use_map Map-result call");
    let (callee_owner, callee_site) = lifecycle_call.expect("main local call");
    assert_ne!(receive_owner, callee_owner);
    let loans = package.direct_call_loans.as_mut().expect("owner loans");
    assert_eq!(loans.iter().count(), 2, "main and use_map loans");
    let row = loans
        .get_mut(receive_owner)
        .expect("use_map loan")
        .take_once(receive_owner, receive_site)
        .expect("receive row");
    assert_eq!(
        row.result(),
        crate::mir::instruction::InvokeCallResultKind::Map
    );
    assert!(row.lifecycle_emission().is_ok());
    assert_eq!(
        row.into_scalar_emission().err(),
        Some(DirectCallLoanErrorV1::LifecycleConsumerMissing)
    );
    // `use_map` returns i64, but its sealed body already carries lifecycle
    // local calls — the edge must stay on the Invoke route so the artifact
    // can reach the lifecycle-bearing callee. Scalar `Call` is rejected.
    let callee = loans
        .get_mut(callee_owner)
        .expect("main loan")
        .take_once(callee_owner, callee_site)
        .expect("lifecycle row");
    assert_eq!(
        callee.result(),
        crate::mir::instruction::InvokeCallResultKind::I64
    );
    assert!(callee.lifecycle_emission().is_ok());
    assert_eq!(
        callee.into_scalar_emission().err(),
        Some(DirectCallLoanErrorV1::LifecycleConsumerMissing)
    );
    package
        .direct_call_loans
        .take()
        .unwrap()
        .finish_empty()
        .expect("all owner rows consumed");
}

#[test]
fn non_app_main_map_receive_installs_through_preflight() {
    let package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("non-AppMain map-receive package");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn non_app_main_map_receive_with_prior_home_rejects_before_catalog_mutation() {
    // The second `make_map()` call still has a live prior Map home: the
    // bounded consumer owns no prior-home cleanup path, so the undertaking
    // rejects it before install instead of at physical emission.
    let package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local a = make_map() local b = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("source facts remain issuable for the bounded rejection");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(matches!(
        package.prepare_install(&mut context),
        Err((
            _,
            super::NormalCallableSemanticPackageInstallIssueV1::MapObligationDescribe(
                super::map_lifecycle_undertaking::MapObligationDescribeIssueV1::CallPriorHomesUnsupported { .. }
            )
        ))
    ));
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn partially_consumed_owner_loan_rejects_at_finish() {
    let mut package = issue(
        r#"static box Main {
            main() { local r = use_map(10) local s = echo(1) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
            echo(value: i64): i64 { return value }
        }"#,
    )
    .expect("two-row main loan package");
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .unwrap();
    let owner = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .unwrap()
        .owner();
    let flow = package
        .ordinary_new_claim_ledger
        .completion_for_owner(owner)
        .unwrap()
        .cleanup()
        .root_flow()
        .unwrap();
    let site = flow.local_calls()[0].site().site().clone();
    let mut loans = package.direct_call_loans.take().expect("owner loans");
    loans
        .get_mut(owner)
        .unwrap()
        .take_once(owner, site)
        .expect("first row");
    assert_eq!(
        loans.finish_empty(),
        Err(DirectCallLoanErrorV1::ResidualRows)
    );
}

#[test]
fn untouched_owner_loans_drain_only_with_bypass_evidence() {
    // An owner whose selected lane never enters direct-call scope leaves its
    // loan fully untouched — but only the canonical route's own mark may
    // close it. Untouched and bypassed are different states.
    let mut package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("non-AppMain map-receive package");
    let mut loans = package.direct_call_loans.take().expect("owner loans");
    let owners: Vec<_> = loans.iter().map(|loan| loan.owner()).collect();
    assert!(!owners.is_empty());
    for owner in owners {
        loans.mark_canonical_route_bypass(owner);
    }
    loans
        .finish_empty()
        .expect("marked bypass loans drain without residual");
}

#[test]
fn untouched_owner_loans_reject_without_bypass_evidence() {
    // The same untouched loans, with no route claiming them, are residual:
    // finish_empty must not confuse "never entered scope" with "consumed".
    let mut package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("non-AppMain map-receive package");
    let loans = package.direct_call_loans.take().expect("owner loans");
    assert_eq!(
        loans.finish_empty(),
        Err(DirectCallLoanErrorV1::ResidualRows)
    );
}

#[test]
fn unrelated_map_owner_with_plain_return_stays_admitted() {
    // `bystander` makes a Map and returns a non-map `Value` terminal: the
    // removed all-owner terminal whitelist rejected `Value` rows that did
    // not return a map, but the undertaking only asks for describable
    // obligations and exit evidence.
    let package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
            bystander() { local m = %{"b" => 2} return "x" }
        }"#,
    )
    .expect("unrelated map owner stays admitted");
    let mut context = crate::mir::builder::CompilationContext::new();
    package
        .prepare_install(&mut context)
        .expect("unrelated map owner install");
}
