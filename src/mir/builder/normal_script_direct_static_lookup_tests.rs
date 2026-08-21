use super::*;

use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::source_call_target::{
    ScriptDirectStaticCallCoverageIssueV1, ScriptDirectStaticCallLookupErrorV1,
    VerifiedScriptCallCoverageDispositionV1, VerifiedScriptNonDirectCallReasonV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn make_package(source: &str, session_id: u32) -> VerifiedNormalCallableSemanticPackageV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let mut resolver =
        FunctionSemanticResolverSessionV1::new(session_id).expect("resolver session");
    issue_normal_callable_semantic_package_v1(&mut resolver, source).expect("semantic package")
}

#[test]
fn issuer_moves_one_owned_target_result_relation_from_the_parser_loan() {
    let package = make_package(
        "static box Helpers { run(value) { return 7 } }\nreturn Helpers.run(1)",
        1801,
    );
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    let (lookup, _publication_owner) =
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[])
            .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    let rows = lookup.rows().collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "one root direct-static row expected");
    let (site, row) = rows[0];
    assert_eq!(site, row.site());
    assert_eq!(row.target().owner(), "Helpers");
    assert_eq!(row.target().name(), "run");
    assert_eq!(row.target().arity(), 1);
    assert_eq!(row.required_callee_i64_arguments(), &[] as &[u32]);
}

#[test]
fn issuer_retains_complete_source_coverage_with_the_selected_row() {
    let package = make_package(
        "static box Helpers { run(value) { return 7 } }\nreturn Helpers.run(1)",
        1805,
    );
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    let (lookup, _publication_owner) =
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[])
            .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    let coverage = lookup.source_coverage();
    assert_eq!(coverage.len(), 1);
    let row = coverage
        .rows()
        .expect("non-empty coverage")
        .values()
        .next()
        .expect("one coverage row");
    assert_eq!(
        row.disposition(),
        VerifiedScriptCallCoverageDispositionV1::QualifiedUnboundOrdinary
    );
    assert_eq!(row.result_site(), row.site());
    assert_eq!(row.argument_sites().len(), 1);
    let invocation = package
        .with_normal_program_source_loan(|loan| loan.invocation_witness().clone())
        .expect("parser invocation witness");
    assert!(coverage.is_from_invocation(&invocation));
}

#[test]
fn issuer_seals_true_zero_call_script_as_complete_empty_coverage() {
    let package = make_package("42", 1806);
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    let (lookup, _publication_owner) =
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[])
            .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    assert!(lookup.source_coverage().is_empty());
    assert_eq!(lookup.source_coverage().rows(), None);
}

#[test]
fn issuer_keeps_non_direct_source_routes_in_coverage() {
    let package = make_package("local Alias = 0\nreturn Alias.run(1)\nreturn 1.run()", 1807);
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    let (lookup, _publication_owner) =
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[])
            .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    let rows = lookup.source_coverage().rows().expect("non-empty coverage");
    assert_eq!(rows.len(), 2);
    assert!(rows.values().any(|row| {
        row.disposition()
            == VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::QualifiedReceiverBound,
            )
    }));
    assert!(rows.values().any(|row| {
        row.disposition()
            == VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::DynamicReceiver,
            )
    }));
    assert!(lookup.rows().next().is_none());
}

#[test]
fn issuer_keeps_typeop_and_reserved_routes_in_coverage() {
    let package = make_package(
        "return Helpers.is(\"Integer\")\nreturn __mir__.log(\"value\")",
        1808,
    );
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    let (lookup, _publication_owner) =
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[])
            .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    let rows = lookup.source_coverage().rows().expect("non-empty coverage");
    assert_eq!(rows.len(), 2);
    assert!(rows.values().any(|row| {
        row.disposition()
            == VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::TypeOperation,
            )
    }));
    assert!(rows.values().any(|row| {
        row.disposition()
            == VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::ReservedRoute,
            )
    }));
    assert!(lookup.rows().next().is_none());
}

#[test]
fn issuer_rejects_a_neutral_window_from_a_foreign_parser_invocation() {
    let package = make_package("42", 1802);
    let foreign_package = make_package("42", 1803);
    let foreign_window = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&foreign_package)
        .expect("foreign neutral source window");
    assert!(matches!(
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&foreign_window), &[]),
        Err(NormalScriptDirectStaticLookupIssueV1::Lookup(
            ScriptDirectStaticCallLookupErrorV1::Coverage(
                ScriptDirectStaticCallCoverageIssueV1::ForeignInvocation
            )
        ))
    ));
}

#[test]
fn issuer_rejects_a_target_outside_the_owned_catalog() {
    let package = make_package("return Missing.run(1)", 1804);
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("neutral source window");
    assert!(matches!(
        ScriptDirectStaticCallLookupIssuerV1::issue(&package, Some(&neutral), &[]),
        Err(NormalScriptDirectStaticLookupIssueV1::Lookup(
            ScriptDirectStaticCallLookupErrorV1::TargetOutsideCatalog { .. }
        ))
    ));
}
