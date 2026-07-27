use crate::mir::resolved_semantics::SourcePathSegmentV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetV1;

use super::super::call_substitution::substitute_required_arguments;
use super::super::expression_proof::I64ExpressionFactV1;
use super::super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1, VerifiedCallableResultEvidenceV1,
    VerifiedSameModuleCallableResultCatalogV1,
};
use super::support::{
    declarations, extend_current_owner_targets, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

fn return_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

#[test]
fn substitutes_only_callee_required_arguments_and_retains_target_evidence() {
    let source = r#"
        static box ProviderV1 {
            second(left, right) { return right }
        }
        static box ConsumerV1 {
            forward(value) { return ProviderV1.second("ignored", value) }
            fixed(value) { return ProviderV1.second("ignored", 41) }
            invalid(value) { return ProviderV1.second(0, "not-i64") }
        }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "forward",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "fixed",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "invalid",
                caller_arity: 1,
                site: return_site(),
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);

    assert_eq!(
        results.disposition(&key(&declarations, "ConsumerV1", "forward", 1)),
        Some(&VerifiedCallableResultDispositionV1::ExactI64 {
            required_i64_arguments: Box::new([0]),
        })
    );
    assert_eq!(
        results.disposition(&key(&declarations, "ConsumerV1", "fixed", 1)),
        Some(&VerifiedCallableResultDispositionV1::ExactI64 {
            required_i64_arguments: Box::new([]),
        })
    );
    assert_eq!(
        results.disposition(&key(&declarations, "ConsumerV1", "invalid", 1)),
        Some(&VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::RequiredArgumentRepresentationUnavailable,
        ))
    );

    let caller = key(&declarations, "ConsumerV1", "forward", 1);
    let row = results.call_result(&caller, &return_site()).unwrap();
    assert_eq!(row.required_i64_arguments(), &[0]);
    match row.evidence() {
        VerifiedCallableResultEvidenceV1::SameModuleStatic {
            source_target,
            callee_required_i64_arguments,
        } => {
            let target = match source_target {
                VerifiedSourceStaticCallTargetV1::QualifiedStatic(target) => target.target(),
                VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(target) => target.target(),
            };
            assert_eq!(target.owner(), "ProviderV1");
            assert_eq!(callee_required_i64_arguments.as_ref(), &[1]);
        }
        VerifiedCallableResultEvidenceV1::CoreStringMethod { .. } => {
            panic!("expected same-module evidence")
        }
    }
}

#[test]
fn nested_calls_are_site_exact_and_child_requirements_reach_the_caller() {
    let source = r#"
        static box ProviderV1 { step(value) { return value + 1 } }
        static box ConsumerV1 {
            nested(value) { return ProviderV1.step(ProviderV1.step(value)) }
        }
    "#;
    let declarations = declarations(source);
    let caller = key(&declarations, "ConsumerV1", "nested", 1);
    let outer = return_site();
    let inner = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Argument(0),
    ]);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "nested",
                caller_arity: 1,
                site: outer.clone(),
            },
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "nested",
                caller_arity: 1,
                site: inner.clone(),
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);

    assert_eq!(
        results.disposition(&caller),
        Some(&VerifiedCallableResultDispositionV1::ExactI64 {
            required_i64_arguments: Box::new([0]),
        })
    );
    assert_eq!(
        results
            .call_result(&caller, &outer)
            .unwrap()
            .required_i64_arguments(),
        &[0]
    );
    assert_eq!(
        results
            .call_result(&caller, &inner)
            .unwrap()
            .required_i64_arguments(),
        &[0]
    );
}

#[test]
fn declaration_reorder_preserves_result_requirements() {
    let provider = "static box ProviderV1 { step(value) { return value + 1 } }";
    let consumer = "static box ConsumerV1 { wrap(value) { return ProviderV1.step(value) } }";

    fn solve(source: &str) -> Vec<(String, String, u32, VerifiedCallableResultDispositionV1)> {
        let declarations = declarations(source);
        let targets = qualified_targets(
            &declarations,
            &[],
            &[CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "wrap",
                caller_arity: 1,
                site: return_site(),
            }],
        );
        let results = seal_with_targets(&declarations, &targets);
        results
            .rows()
            .map(|(key, row)| {
                (
                    key.owner().to_string(),
                    key.name().to_string(),
                    key.arity(),
                    row.clone(),
                )
            })
            .collect()
    }

    let forward = format!("{consumer}\n{provider}");
    let backward = format!("{provider}\n{consumer}");
    assert_eq!(solve(&forward), solve(&backward));
}

#[test]
fn two_forwarding_wrappers_are_order_independent() {
    let provider = "static box ProviderV1 { step(value) { return value + 1 } }";
    let middle = "static box MiddleV1 { wrap(value) { return ProviderV1.step(value) } }";
    let consumer = "static box ConsumerV1 { wrap(value) { return MiddleV1.wrap(value) } }";

    fn solve(source: &str) -> Vec<(String, String, u32, VerifiedCallableResultDispositionV1)> {
        let declarations = declarations(source);
        let targets = qualified_targets(
            &declarations,
            &[],
            &[
                CallSiteSpecV1 {
                    caller_owner: "MiddleV1",
                    caller_name: "wrap",
                    caller_arity: 1,
                    site: return_site(),
                },
                CallSiteSpecV1 {
                    caller_owner: "ConsumerV1",
                    caller_name: "wrap",
                    caller_arity: 1,
                    site: return_site(),
                },
            ],
        );
        seal_with_targets(&declarations, &targets)
            .rows()
            .map(|(key, row)| {
                (
                    key.owner().to_string(),
                    key.name().to_string(),
                    key.arity(),
                    row.clone(),
                )
            })
            .collect()
    }

    let forward = format!("{consumer}\n{middle}\n{provider}");
    let backward = format!("{provider}\n{middle}\n{consumer}");
    let expected = VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments: Box::new([0]),
    };
    let forward_rows = solve(&forward);
    assert_eq!(forward_rows, solve(&backward));
    assert!(
        forward_rows
            .iter()
            .filter(|(_, name, _, row)| name == "wrap" && row == &expected)
            .count()
            == 2
    );
}

#[test]
fn exact_targets_close_direct_and_mutual_recursion_without_scc_inference() {
    let source = r#"
        static box DirectV1 { again(value) { return me.again(value) } }
        static box WrappedV1 { again(value) { return "-" + me.again(value) } }
        static box LeftV1 { call(value) { return RightV1.call(value) } }
        static box RightV1 { call(value) { return LeftV1.call(value) } }
        static box LeftWrappedV1 {
            call(value) { return "-" + RightWrappedV1.call(value) }
        }
        static box RightWrappedV1 {
            call(value) { return LeftWrappedV1.call(value) }
        }
    "#;
    let declarations = declarations(source);
    let binary_right_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Rhs,
    ]);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "LeftV1",
                caller_name: "call",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "RightV1",
                caller_name: "call",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "LeftWrappedV1",
                caller_name: "call",
                caller_arity: 1,
                site: binary_right_site.clone(),
            },
            CallSiteSpecV1 {
                caller_owner: "RightWrappedV1",
                caller_name: "call",
                caller_arity: 1,
                site: return_site(),
            },
        ],
    );
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[
            CallSiteSpecV1 {
                caller_owner: "DirectV1",
                caller_name: "again",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "WrappedV1",
                caller_name: "again",
                caller_arity: 1,
                site: binary_right_site,
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);
    let recursive = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::RecursiveDependency,
    );

    for (owner, name) in [
        ("DirectV1", "again"),
        ("WrappedV1", "again"),
        ("LeftV1", "call"),
        ("RightV1", "call"),
        ("LeftWrappedV1", "call"),
        ("RightWrappedV1", "call"),
    ] {
        assert_eq!(
            results.disposition(&key(&declarations, owner, name, 1)),
            Some(&recursive),
        );
    }
}

#[test]
fn equal_keys_from_a_foreign_catalog_do_not_brand_the_result_catalog() {
    let source = "static box ProviderV1 { step(value) { return value } }";
    let primary = declarations(source);
    let foreign = declarations(source);
    let targets = qualified_targets(&primary, &[], &[]);

    assert_eq!(
        VerifiedSameModuleCallableResultCatalogV1::verify(&foreign, &targets).unwrap_err(),
        CallableResultCatalogErrorV1::SourceTargetCatalogBrandMismatch,
    );
}

#[test]
fn missing_and_unknown_required_arguments_are_non_publishing() {
    let unavailable = I64ExpressionFactV1::Unknown(
        CallableResultUnavailableReasonV1::RequiredArgumentRepresentationUnavailable,
    );

    assert_eq!(substitute_required_arguments(&[0], &[]), unavailable);
    assert_eq!(
        substitute_required_arguments(
            &[0],
            &[I64ExpressionFactV1::Unknown(
                CallableResultUnavailableReasonV1::UnknownExpression,
            )],
        ),
        unavailable,
    );
}
