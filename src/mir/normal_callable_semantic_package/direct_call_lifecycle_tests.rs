//! Natural-source correspondence and the selected scalar-edge Stop.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::direct_call_loan::AppMainDirectCallLoanErrorV1;

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
