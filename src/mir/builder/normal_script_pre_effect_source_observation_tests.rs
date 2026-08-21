use super::*;

use crate::mir::builder::normal_script_direct_static_lookup::ScriptDirectStaticCallLookupIssuerV1;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn package(source: &str, session_id: u32) -> VerifiedNormalCallableSemanticPackageV1 {
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

fn inputs(
    package: &VerifiedNormalCallableSemanticPackageV1,
    window: &PreparedCanonicalScriptNeutralProgramWindowV1,
) -> (
    VerifiedScriptDirectStaticCallLookupV1,
    PreparedNormalProgramDeclarationFactsV1,
) {
    let (lookup, _) = ScriptDirectStaticCallLookupIssuerV1::issue(package, Some(window), &[])
        .expect("owned lookup");
    let lookup = lookup.expect("Script lookup");
    let facts = package
        .with_normal_program_source_loan(|loan| {
            PreparedNormalProgramDeclarationFactsV1::collect(loan.program())
        })
        .expect("source loan")
        .expect("declaration facts");
    (lookup, facts)
}

#[test]
fn zero_call_source_is_complete_before_effects() {
    let package = package("42", 2601);
    let window =
        PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package).expect("neutral window");
    let (lookup, facts) = inputs(&package, &window);
    let mut resolver = FunctionSemanticResolverSessionV1::new(2602).expect("resolver");

    assert!(matches!(
        NormalScriptPreEffectSourceObservationIssuerV1::issue(
            &package,
            &window,
            lookup,
            &facts,
            &mut resolver,
        ),
        Ok(_)
    ));
}

#[test]
fn foreign_neutral_window_is_rejected_before_resolver_observation() {
    let owner_package = package("42", 2603);
    let foreign = package("42", 2604);
    let window = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&owner_package)
        .expect("neutral window");
    let foreign_window = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&foreign)
        .expect("foreign neutral window");
    let (lookup, facts) = inputs(&owner_package, &window);
    let mut resolver = FunctionSemanticResolverSessionV1::new(2605).expect("resolver");

    assert!(matches!(
        NormalScriptPreEffectSourceObservationIssuerV1::issue(
            &owner_package,
            &foreign_window,
            lookup,
            &facts,
            &mut resolver,
        ),
        Err(NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid(_))
    ));
}
