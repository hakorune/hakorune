//! Test-only normalized P0 proof.
//!
//! The snapshot deliberately excludes catalog pointers, borrowed AST identity,
//! source declaration order, and route-product addresses. It is not a
//! production representation or a new callable/result authority.

use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use crate::mir::core_method_result_kind::CoreMethodResultKindV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};
use crate::mir::source_core_receiver::SourceCoreReceiverFactV1;

use super::super::{
    CallableResultUnavailableReasonV1, VerifiedCallableResultDispositionV1,
    VerifiedCallableResultEvidenceV1, VerifiedSameModuleCallableResultCatalogV1,
};
use super::support::{
    declarations, extend_current_owner_targets, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedCallableKeyV1 {
    namespace: SameModuleCallableNamespaceV1,
    owner: String,
    name: String,
    arity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalizedResultDispositionV1 {
    ExactI64 { required_arguments: Vec<u32> },
    Unavailable(CallableResultUnavailableReasonV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalizedCallEvidenceV1 {
    SameModuleStatic {
        target: NormalizedCallableKeyV1,
        callee_required_arguments: Vec<u32>,
    },
    CoreStringMethod {
        receiver_fact: SourceCoreReceiverFactV1,
        receiver_box: String,
        canonical: String,
        admitted_arities: Vec<u32>,
        result_kind: CoreMethodResultKindV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedCallSiteV1 {
    caller: NormalizedCallableKeyV1,
    site: Vec<SourcePathSegmentV1>,
    required_arguments: Vec<u32>,
    evidence: NormalizedCallEvidenceV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedResultCatalogV1 {
    rows: Vec<(NormalizedCallableKeyV1, NormalizedResultDispositionV1)>,
    call_sites: Vec<NormalizedCallSiteV1>,
}

fn normalize(
    catalog: &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
) -> NormalizedResultCatalogV1 {
    let rows = catalog
        .rows()
        .map(|(key, row)| (normalize_key(key), normalize_disposition(row)))
        .collect();
    let call_sites = catalog
        .call_rows()
        .map(|((caller, site), row)| NormalizedCallSiteV1 {
            caller: normalize_key(caller),
            site: normalize_site(site),
            required_arguments: row.required_i64_arguments().to_vec(),
            evidence: match row.evidence() {
                VerifiedCallableResultEvidenceV1::SameModuleStatic {
                    source_target,
                    callee_required_i64_arguments,
                } => NormalizedCallEvidenceV1::SameModuleStatic {
                    target: normalize_key(source_target.target()),
                    callee_required_arguments: callee_required_i64_arguments.to_vec(),
                },
                VerifiedCallableResultEvidenceV1::CoreStringMethod {
                    receiver_fact,
                    contract,
                } => NormalizedCallEvidenceV1::CoreStringMethod {
                    receiver_fact: *receiver_fact,
                    receiver_box: contract.receiver_box.to_string(),
                    canonical: contract.canonical.to_string(),
                    admitted_arities: contract.arities.to_vec(),
                    result_kind: contract.result_kind,
                },
            },
        })
        .collect();
    NormalizedResultCatalogV1 { rows, call_sites }
}

fn normalize_key(key: &CanonicalSameModuleCallableKeyV1) -> NormalizedCallableKeyV1 {
    NormalizedCallableKeyV1 {
        namespace: key.namespace(),
        owner: key.owner().to_string(),
        name: key.name().to_string(),
        arity: key.arity(),
    }
}

fn normalize_disposition(
    disposition: &VerifiedCallableResultDispositionV1,
) -> NormalizedResultDispositionV1 {
    match disposition {
        VerifiedCallableResultDispositionV1::ExactI64 {
            required_i64_arguments,
        } => NormalizedResultDispositionV1::ExactI64 {
            required_arguments: required_i64_arguments.to_vec(),
        },
        VerifiedCallableResultDispositionV1::Unavailable(reason) => {
            NormalizedResultDispositionV1::Unavailable(reason.clone())
        }
    }
}

fn normalize_site(site: &SourceExprSiteV1) -> Vec<SourcePathSegmentV1> {
    site.node().segments().to_vec()
}

fn return_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

#[test]
fn declaration_reorder_and_fresh_catalog_identity_normalize_equally() {
    let provider = "static box ProviderV1 { step(value) { return value + 1 } }";
    let middle = "static box MiddleV1 { wrap(value) { return ProviderV1.step(value) } }";
    let consumer = "static box ConsumerV1 { wrap(value) { return MiddleV1.wrap(value) } }";

    fn solve(source: &str) -> NormalizedResultCatalogV1 {
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
        normalize(&seal_with_targets(&declarations, &targets))
    }

    let forward = format!("{consumer}\n{middle}\n{provider}");
    let backward = format!("{provider}\n{middle}\n{consumer}");
    assert_eq!(solve(&forward), solve(&backward));
    assert_eq!(solve(&forward), solve(&forward));
}

#[test]
fn repeated_sites_retain_multiplicity_without_duplicate_target_authority() {
    let source = r#"
        static box ProviderV1 { step(value) { return value + 1 } }
        static box ConsumerV1 {
            twice(value) {
                return ProviderV1.step(value) + ProviderV1.step(value)
            }
        }
    "#;
    let declarations = declarations(source);
    let left = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Lhs,
    ]);
    let right = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Rhs,
    ]);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "twice",
                caller_arity: 1,
                site: left,
            },
            CallSiteSpecV1 {
                caller_owner: "ConsumerV1",
                caller_name: "twice",
                caller_arity: 1,
                site: right,
            },
        ],
    );
    let snapshot = normalize(&seal_with_targets(&declarations, &targets));

    assert_eq!(snapshot.call_sites.len(), 2);
    assert_ne!(snapshot.call_sites[0].site, snapshot.call_sites[1].site);
    assert_eq!(
        snapshot.call_sites[0].evidence,
        snapshot.call_sites[1].evidence,
    );
    assert_eq!(snapshot.call_sites[0].required_arguments, vec![0]);
    assert_eq!(snapshot.call_sites[1].required_arguments, vec![0]);
}

#[test]
fn unavailable_results_are_normalized_without_publication_or_recovery() {
    let source = r#"
        static box ProviderV1 { step(value) { return value } }
        static box NegativeV1 {
            unknown(value) { return ProviderV1.step(value.field) }
            text() { return ProviderV1.step("text") }
            heterogeneous(flag) {
                if flag == 0 { return 1 }
                return "text"
            }
        }
        static box DirectV1 { again(value) { return me.again(value) } }
        static box LeftV1 { call(value) { return RightV1.call(value) } }
        static box RightV1 { call(value) { return LeftV1.call(value) } }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "NegativeV1",
                caller_name: "unknown",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "NegativeV1",
                caller_name: "text",
                caller_arity: 0,
                site: return_site(),
            },
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
        ],
    );
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "DirectV1",
            caller_name: "again",
            caller_arity: 1,
            site: return_site(),
        }],
    );
    let snapshot = normalize(&seal_with_targets(&declarations, &targets));

    let disposition = |owner: &str, name: &str, arity: u32| {
        snapshot
            .rows
            .iter()
            .find(|(key, _)| key.owner == owner && key.name == name && key.arity == arity)
            .map(|(_, row)| row)
            .unwrap()
    };
    let required_unavailable = NormalizedResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::RequiredArgumentRepresentationUnavailable,
    );
    assert_eq!(
        disposition("NegativeV1", "unknown", 1),
        &required_unavailable
    );
    assert_eq!(disposition("NegativeV1", "text", 0), &required_unavailable);
    assert_eq!(
        disposition("NegativeV1", "heterogeneous", 1),
        &NormalizedResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
        ),
    );
    for owner in ["DirectV1", "LeftV1", "RightV1"] {
        assert_eq!(
            disposition(owner, if owner == "DirectV1" { "again" } else { "call" }, 1),
            &NormalizedResultDispositionV1::Unavailable(
                CallableResultUnavailableReasonV1::RecursiveDependency,
            ),
        );
    }
}

#[test]
fn actual_string_helper_chain_normalizes_static_and_core_evidence() {
    let source = format!(
        "{}\n{}",
        include_str!(concat!(
            "../../../../lang/src/shared/common/",
            "string_helpers.hako"
        )),
        include_str!(concat!(
            "../../../../lang/src/compiler/parser/scan/",
            "parser_string_utils_box.hako"
        )),
    );
    let declarations = declarations(&source);
    let wrapper_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    let digit_site = site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::LoopBody(2),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let targets = qualified_targets(
        &declarations,
        &[("StringHelpers", "StringHelpers")],
        &[CallSiteSpecV1 {
            caller_owner: "ParserStringUtilsBox",
            caller_name: "skip_ws",
            caller_arity: 2,
            site: wrapper_site,
        }],
    );
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "to_i64",
            caller_arity: 1,
            site: digit_site,
        }],
    );
    let snapshot = normalize(&seal_with_targets(&declarations, &targets));

    for (owner, name, arity, required) in [
        ("StringHelpers", "skip_ws", 2, vec![1]),
        ("ParserStringUtilsBox", "skip_ws", 2, vec![1]),
        ("StringHelpers", "to_i64", 1, vec![]),
        ("StringHelpers", "_digit_value", 1, vec![]),
    ] {
        let row = snapshot
            .rows
            .iter()
            .find(|(key, _)| key.owner == owner && key.name == name && key.arity == arity)
            .map(|(_, row)| row)
            .unwrap();
        assert_eq!(
            row,
            &NormalizedResultDispositionV1::ExactI64 {
                required_arguments: required,
            },
        );
    }

    assert!(snapshot.call_sites.iter().any(|row| matches!(
        &row.evidence,
        NormalizedCallEvidenceV1::SameModuleStatic { target, .. }
            if target.owner == "StringHelpers" && target.name == "_digit_value"
    )));
    assert!(snapshot.call_sites.iter().any(|row| matches!(
        &row.evidence,
        NormalizedCallEvidenceV1::CoreStringMethod {
            receiver_box,
            canonical,
            admitted_arities,
            result_kind: CoreMethodResultKindV1::I64Value,
            ..
        } if receiver_box == "StringBox" && canonical == "length" && admitted_arities == &[0]
    )));

    let wrapper = key(&declarations, "ParserStringUtilsBox", "skip_ws", 2);
    assert!(snapshot
        .call_sites
        .iter()
        .any(|row| row.caller == normalize_key(&wrapper)));
}
