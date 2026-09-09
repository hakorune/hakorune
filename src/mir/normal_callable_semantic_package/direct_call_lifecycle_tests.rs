//! Natural-source correspondence and the selected scalar-edge Stop.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::direct_call_loan::AppMainDirectCallLoanErrorV1;
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
            .app_main_direct_call_loan
            .as_mut()
            .unwrap()
            .take_once(owner, site)
            .unwrap();
        assert_eq!(row.argument_sites().len(), 2);
        assert_eq!(
            row.into_scalar_emission().err(),
            Some(AppMainDirectCallLoanErrorV1::LifecycleConsumerMissing)
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
    let mut loan = package
        .app_main_direct_call_loan
        .take()
        .expect("AppMain direct-call loan");
    assert!(loan
        .take_once(owner, local_site)
        .expect("local lifecycle row")
        .lifecycle_emission()
        .is_ok());
    assert!(loan
        .take_once(owner, terminal_site)
        .expect("terminal lifecycle row")
        .lifecycle_emission()
        .is_ok());
    loan.finish_empty().expect("all lifecycle rows consumed");
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
    let targets = package
        .app_main_direct_call_loan
        .as_ref()
        .expect("AppMain direct-call loan")
        .map_target_owners(&package.batch)
        .expect("bounded target owner set");
    assert_eq!(targets.len(), 2);
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
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
    let mut loan = package
        .app_main_direct_call_loan
        .take()
        .expect("AppMain direct-call loan");
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
                Some(&mut loan),
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
                    super::NormalCallableSemanticPackageIssueV1::AppMainDirectCall {
                        _error: super::issuer::AppMainDirectCallDispositionIssueV1::Loan(
                            AppMainDirectCallLoanErrorV1::LifecycleSourceMismatch
                        ),
                    }
                )
            ),
            "{main} / {helper}: {result:?}"
        );
    }
}

#[test]
fn non_map_scalar_call_keeps_its_existing_owner() {
    let mut package = issue(
        "static box Main { main() { return helper(30) } helper(value: i64): i64 { return value } }",
    )
    .unwrap();
    assert!(package
        .ordinary_new_claim_ledger
        .call_source_completion()
        .is_none());
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
        .app_main_direct_call_loan
        .as_mut()
        .unwrap()
        .take_once(owner, site)
        .unwrap();
    assert!(row.into_scalar_emission().is_ok());
}
