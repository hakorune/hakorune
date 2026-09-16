//! Natural-source correspondence and the selected scalar-edge Stop.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::direct_call_loan::DirectCallLoanErrorV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::MirBuilder;

#[path = "direct_call_physical_tests.rs"]
mod physical;

#[test]
fn terminal_map_call_borrows_real_caller_cleanup_and_stops_scalar_emission() {
    for prefix in ["", "local page = new Page()"] {
        let mut package = issue(&format!(
            "box Page {{}} static box Main {{
                main() {{ {prefix} return helper(30, 5) }}
                helper(value: i64, other: i64): i64 {{
                    local m = %{{\"v\" => value, \"other\" => other}} return 30
                }}
            }}"
        ))
        .unwrap();
        let (completion, call) = package
            .ordinary_new_claim_ledger
            .call_source_completion()
            .unwrap();
        assert_eq!(completion.owner(), call.owner());
        assert_eq!(completion.explicit_site(), Some(call.return_site()));
        assert_eq!(call.arguments(), [30, 5]);
        assert_eq!(
            completion
                .cleanup()
                .terminal_homes()
                .unwrap()
                .unwrap()
                .len(),
            usize::from(!prefix.is_empty())
        );
        let owner = call.owner();
        let site = call.call_site().clone();
        let row = package
            .direct_call_loans
            .as_mut()
            .unwrap()
            .get_mut(owner)
            .unwrap()
            .take_once(owner, site)
            .unwrap();
        assert_eq!(row.argument_sites().len(), 2);
        assert_eq!(
            row.into_scalar_emission().err(),
            Some(DirectCallLoanErrorV1::LifecycleConsumerMissing)
        );
        let mut context = crate::mir::builder::CompilationContext::new();
        assert!(matches!(
            package.prepare_install(&mut context),
            Err((
                _,
                super::NormalCallableSemanticPackageInstallIssueV1::MapLifecycleConsumerMissing
            ))
        ));
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn local_map_call_and_terminal_map_call_share_the_root_source_owner() {
    let mut package = issue(
        r#"static box Main {
            main() { local first = helper(10) return helper(20) }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
        }"#,
    )
    .expect("bounded local plus terminal Map package");
    let (completion, terminal) = package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .expect("terminal Call relation");
    let flow = completion.cleanup().root_flow().expect("root home flow");
    assert_eq!(flow.local_calls().len(), 1);
    let local = &flow.local_calls()[0];
    assert_eq!(local.owner(), terminal.owner());
    assert_eq!(local.arguments(), [10]);
    assert_eq!(terminal.arguments(), [20]);

    let owner = terminal.owner();
    let local_site = local.site().site().clone();
    let terminal_site = terminal.call_site().clone();
    let mut loans = package
        .direct_call_loans
        .take()
        .expect("AppMain direct-call loans");
    assert!(loans
        .get_mut(owner)
        .unwrap()
        .take_once(owner, local_site)
        .expect("local lifecycle row")
        .lifecycle_emission()
        .is_ok());
    assert!(loans
        .get_mut(owner)
        .unwrap()
        .take_once(owner, terminal_site)
        .expect("terminal lifecycle row")
        .lifecycle_emission()
        .is_ok());
    loans.finish_empty().expect("all lifecycle rows consumed");
}

#[test]
fn distinct_map_call_owners_share_the_existing_install_preflight() {
    let package = issue(
        r#"static box Main {
            main() {
                local first = helper(10)
                local root_map = %{"root" => 1}
                return other(20)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#,
    )
    .expect("two distinct ordinary Map owners");
    let targets: std::collections::BTreeSet<_> = package
        .direct_call_loans
        .as_ref()
        .expect("direct-call loans")
        .iter()
        .flat_map(|loan| {
            loan.map_target_owners(&package.batch)
                .unwrap_or_default()
                .into_vec()
        })
        .collect();
    assert_eq!(targets.len(), 2);
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn three_distinct_map_call_owners_share_source_ordered_local_bindings() {
    let package = issue(
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#,
    )
    .expect("three distinct ordinary Map owners");
    let targets: std::collections::BTreeSet<_> = package
        .direct_call_loans
        .as_ref()
        .expect("direct-call loans")
        .iter()
        .flat_map(|loan| {
            loan.map_target_owners(&package.batch)
                .unwrap_or_default()
                .into_vec()
        })
        .collect();
    assert_eq!(targets.len(), 3);
    let completion = package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .expect("terminal Call relation")
        .0;
    assert_eq!(
        completion
            .cleanup()
            .root_flow()
            .unwrap()
            .local_calls()
            .len(),
        2
    );
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn repeated_map_target_fourth_call_installs_under_the_undertaking() {
    // A repeated call to an already-covered callee is one more edge, not
    // a new obligation — the fixed call-count shape bound is gone.
    let package = issue(
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local repeated = helper(11)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#,
    )
    .expect("source facts remain issuable for the bounded rejection");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn four_distinct_map_call_owners_share_three_source_ordered_local_bindings() {
    let package = issue(
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local third = later(1)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            later(value: i64): i64 { local m = %{"later" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
        }"#,
    )
    .expect("four distinct ordinary Map owners");
    let targets: std::collections::BTreeSet<_> = package
        .direct_call_loans
        .as_ref()
        .expect("direct-call loans")
        .iter()
        .flat_map(|loan| {
            loan.map_target_owners(&package.batch)
                .unwrap_or_default()
                .into_vec()
        })
        .collect();
    assert_eq!(targets.len(), 4);
    let completion = package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .expect("terminal Call relation")
        .0;
    assert_eq!(
        completion
            .cleanup()
            .root_flow()
            .unwrap()
            .local_calls()
            .len(),
        3
    );
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn five_distinct_map_call_owners_share_four_source_ordered_local_bindings() {
    let package = issue(
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local third = later(1)
                local fourth = extra(2)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            later(value: i64): i64 { local m = %{"later" => value} return 30 }
            extra(value: i64): i64 { local m = %{"extra" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
        }"#,
    )
    .expect("five distinct ordinary Map owners");
    let targets: std::collections::BTreeSet<_> = package
        .direct_call_loans
        .as_ref()
        .expect("direct-call loans")
        .iter()
        .flat_map(|loan| {
            loan.map_target_owners(&package.batch)
                .unwrap_or_default()
                .into_vec()
        })
        .collect();
    assert_eq!(targets.len(), 5);
    let completion = package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .expect("terminal Call relation")
        .0;
    assert_eq!(
        completion
            .cleanup()
            .root_flow()
            .unwrap()
            .local_calls()
            .len(),
        4
    );
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn sixth_repeated_or_distinct_map_call_installs_under_the_undertaking() {
    for source in [
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local third = later(1)
                local fourth = extra(2)
                local repeated = helper(11)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            later(value: i64): i64 { local m = %{"later" => value} return 30 }
            extra(value: i64): i64 { local m = %{"extra" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
        }"#,
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local third = later(1)
                local fourth = extra(2)
                local fifth = fifth_owner(3)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            later(value: i64): i64 { local m = %{"later" => value} return 30 }
            extra(value: i64): i64 { local m = %{"extra" => value} return 30 }
            fifth_owner(value: i64): i64 { local m = %{"fifth" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
        }"#,
    ] {
        let package = issue(source).expect("source facts remain issuable for bounded rejection");
        let mut context = crate::mir::builder::CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_ok());
    }
}

#[test]
fn root_map_before_first_call_keeps_prior_homes_rejection() {
    let mut package = issue(
        r#"static box Main {
            main() {
                local root_map = %{"root" => 1}
                local first = helper(10)
                return other(20)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#,
    )
    .expect("source relation remains available for physical rejection");
    let mut loans = package
        .direct_call_loans
        .take()
        .expect("AppMain direct-call loans");
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .expect("source-backed Main");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .expect("Main declaration");
    let mut builder = MirBuilder::new();
    let result = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                Some(&mut loans),
            )
        })
        .expect("lowering input");
    assert!(matches!(
        result,
        Err(error) if error.contains("local-call-prior-homes-unsupported")
    ));
}

#[test]
fn map_target_without_exact_terminal_arguments_or_cleanup_cannot_remain_scalar() {
    for (main, helper) in [
        ("return helper(-5)", "local m = %{\"v\" => value} return 30"),
        (
            "return helper(true)",
            "local m = %{\"v\" => value} return 30",
        ),
        (
            "local value = 30 return helper(value)",
            "local m = %{\"v\" => value} return 30",
        ),
        (
            "local x = helper(30) return 30",
            "local m = %{\"v\" => value} return 30",
        ),
        (
            "return helper(30) + 1",
            "local m = %{\"v\" => value} return 30",
        ),
        (
            "return helper(30)",
            "local m = %{\"v\" => value} return value",
        ),
        (
            "return helper(30)",
            "print(value) local m = %{\"v\" => value} return 30",
        ),
    ] {
        let result = issue(&format!(
            "static box Main {{ main() {{ {main} }} helper(value: i64): i64 {{ {helper} }} }}"
        ));
        assert!(
            matches!(
                result,
                Err(
                    super::NormalCallableSemanticPackageIssueV1::DirectCall {
                        _error: super::issuer::DirectCallDispositionIssueV1::Loan(
                            DirectCallLoanErrorV1::LifecycleSourceMismatch
                        ),
                    }
                )
            ),
            "{main} / {helper}: {result:?}"
        );
    }
}

#[test]
fn non_map_terminal_call_retains_source_completion_and_scalar_row() {
    let mut package = issue(
        "static box Main { main() { return helper(30) } helper(value: i64): i64 { return value } }",
    )
    .unwrap();
    assert!(package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .is_some());
    let (owner, site) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .direct_call_observations()
                        .next()
                        .map(|(site, _)| (input.owner(), site.clone()))
                })
                .unwrap()
        })
        .unwrap();
    let row = package
        .direct_call_loans
        .as_mut()
        .unwrap()
        .get_mut(owner)
        .unwrap()
        .take_once(owner, site)
        .unwrap();
    assert!(row.into_scalar_emission().is_ok());
}

#[test]
fn non_map_local_call_selects_lifecycle_without_reclassifying_terminal() {
    let mut package = issue(
        "static box Main { main() { local first = helper(10) return helper(20) }
         helper(value: i64): i64 { return value } }",
    )
    .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let (completion, terminal) = ledger.call_source_completion().unwrap();
    let owner = completion.owner();
    let locals = completion.cleanup().root_flow().unwrap().local_calls();
    assert_eq!(locals.len(), 1);
    assert_eq!(locals[0].arguments(), &[10]);
    let loans = package.direct_call_loans.as_mut().unwrap();
    let loan = loans.get_mut(owner).unwrap();
    let local = loan
        .take_once(owner, locals[0].site().site().clone())
        .unwrap();
    assert!(local.lifecycle_emission().is_ok());
    assert_eq!(
        local.into_scalar_emission().err(),
        Some(DirectCallLoanErrorV1::LifecycleConsumerMissing)
    );
    let terminal = loan.take_once(owner, terminal.call_site().clone()).unwrap();
    assert!(terminal.into_scalar_emission().is_ok());
}

#[test]
fn non_map_local_call_with_prior_home_rejects_instead_of_scalar_fallback() {
    let result = issue(
        "box Page {} static box Main {
         main() { local page = new Page() local first = helper(10) return helper(20) }
         helper(value: i64): i64 { return value } }",
    );
    assert!(
        matches!(
            result,
            Err(
                super::NormalCallableSemanticPackageIssueV1::DirectCall {
                    _error: super::issuer::DirectCallDispositionIssueV1::Loan(
                        DirectCallLoanErrorV1::LifecycleSourceMismatch
                    ),
                }
            )
        ),
        "{result:?}"
    );
}

#[test]
fn non_map_local_call_with_plain_return_preserves_scalar() {
    let mut package = issue(
        "static box Main { main() { local first = helper(10) return 0 }
         helper(value: i64): i64 { return value } }",
    )
    .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger.call_source_completion().is_none());
    let completion = ledger.root_completion_for_test();
    let locals = completion.cleanup().root_flow().unwrap().local_calls();
    assert_eq!(locals.len(), 1, "the source observation remains present");
    let row = package
        .direct_call_loans
        .as_mut()
        .unwrap()
        .get_mut(completion.owner())
        .unwrap()
        .take_once(completion.owner(), locals[0].site().site().clone())
        .unwrap();
    assert!(row.into_scalar_emission().is_ok());
}

#[test]
fn map_result_local_call_installs_map_class_and_map_row() {
    let mut package = issue(
        "static box Main {
            main() { local m = make_map() return 0 }
            make_map() { return %{\"a\" => 1} }
        }",
    )
    .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    let owner = completion.owner();
    let locals = completion.cleanup().root_flow().unwrap().local_calls();
    assert_eq!(locals.len(), 1);
    assert_eq!(
        locals[0].result(),
        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Map
    );
    let row = package
        .direct_call_loans
        .as_mut()
        .unwrap()
        .get_mut(owner)
        .unwrap()
        .take_once(owner, locals[0].site().site().clone())
        .unwrap();
    assert_eq!(
        row.result(),
        crate::mir::instruction::InvokeCallResultKind::Map
    );
    assert!(row.lifecycle_emission().is_ok());
    assert_eq!(
        row.into_scalar_emission().err(),
        Some(DirectCallLoanErrorV1::LifecycleConsumerMissing)
    );
}

#[test]
fn map_result_lane_rejects_unannotated_scalar_callee() {
    let result = issue(
        "static box Main {
            main() { local m = make_map() return 0 }
            make_map() { return 0 }
        }",
    );
    assert!(
        matches!(
            result,
            Err(
                super::NormalCallableSemanticPackageIssueV1::DirectCall {
                    _error: super::issuer::DirectCallDispositionIssueV1::Loan(
                        DirectCallLoanErrorV1::LifecycleSourceMismatch
                    ),
                }
            )
        ),
        "{result:?}"
    );
}

#[test]
fn cataloged_map_result_call_keeps_unannotated_target_admissible() {
    issue(
        "static box Api {
            caller(seed: i64): i64 { local m = make_map() return seed }
            make_map() { return %{\"a\" => 1} }
        }",
    )
    .expect("cataloged map-result call has an admissible unannotated target");
}

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
    let mut scalar_call = None;
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
                scalar_call = Some((owner, call.site().site().clone()));
            }
        }
    }
    let (receive_owner, receive_site) = receive.expect("use_map Map-result call");
    let (scalar_owner, scalar_site) = scalar_call.expect("main scalar call");
    assert_ne!(receive_owner, scalar_owner);
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
    let scalar = loans
        .get_mut(scalar_owner)
        .expect("main loan")
        .take_once(scalar_owner, scalar_site)
        .expect("scalar row");
    assert_eq!(
        scalar.result(),
        crate::mir::instruction::InvokeCallResultKind::I64
    );
    assert!(scalar.into_scalar_emission().is_ok());
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
            super::NormalCallableSemanticPackageInstallIssueV1::MapLifecycleConsumerMissing
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
fn untouched_owner_loans_drain_for_a_bypassed_lane() {
    // An owner whose selected lane never enters direct-call scope leaves its
    // loan fully untouched; only a partially consumed loan is a violation.
    let mut package = issue(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("non-AppMain map-receive package");
    package
        .direct_call_loans
        .take()
        .expect("owner loans")
        .finish_empty()
        .expect("untouched loans drain without residual");
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
