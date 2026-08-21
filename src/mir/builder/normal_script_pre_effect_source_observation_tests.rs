use super::*;

use crate::mir::builder::normal_script_direct_static_lookup::ScriptDirectStaticCallLookupIssuerV1;
use crate::mir::builder::normal_script_neutral_window::PreparedCanonicalScriptNeutralProgramWindowV1;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ScriptRootResolvedDemandV1,
    ScriptRootSemanticDispositionV1,
};
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
    let (source_window, _post_install) = window.split_for_pre_effect();
    let mut resolver = FunctionSemanticResolverSessionV1::new(2602).expect("resolver");

    let observation = NormalScriptPreEffectSourceObservationIssuerV1::issue(
        &package,
        source_window,
        lookup,
        &facts,
        &mut resolver,
    )
    .expect("complete pre-effect observation");
    let (source_window, _observation) = observation.split_for_work_plan();
    assert_eq!(source_window.window().entries().len(), 1);
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
    let (foreign_source_window, _post_install) = foreign_window.split_for_pre_effect();
    let mut resolver = FunctionSemanticResolverSessionV1::new(2605).expect("resolver");

    assert!(matches!(
        NormalScriptPreEffectSourceObservationIssuerV1::issue(
            &owner_package,
            foreign_source_window,
            lookup,
            &facts,
            &mut resolver,
        ),
        Err(NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid(_))
    ));
}

#[test]
fn empty_script_window_moves_without_default_repair() {
    let package = package("", 2606);
    let window =
        PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package).expect("empty window");
    let (lookup, facts) = inputs(&package, &window);
    let (source_window, _post_install) = window.split_for_pre_effect();
    let mut resolver = FunctionSemanticResolverSessionV1::new(2607).expect("resolver");

    let observation = NormalScriptPreEffectSourceObservationIssuerV1::issue(
        &package,
        source_window,
        lookup,
        &facts,
        &mut resolver,
    )
    .expect("empty pre-effect observation");
    let (source_window, _observation) = observation.split_for_work_plan();
    assert!(source_window.window().entries().is_empty());
}

#[test]
fn final_return_admission_moves_with_the_source_window() {
    let package = package("return 42", 2608);
    let window =
        PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package).expect("return window");
    let (lookup, facts) = inputs(&package, &window);
    let (source_window, _post_install) = window.split_for_pre_effect();
    let mut resolver = FunctionSemanticResolverSessionV1::new(2609).expect("resolver");

    let observation = NormalScriptPreEffectSourceObservationIssuerV1::issue(
        &package,
        source_window,
        lookup,
        &facts,
        &mut resolver,
    )
    .expect("return pre-effect observation");
    let (source_window, _observation) = observation.split_for_work_plan();
    assert!(matches!(
        source_window
            .window()
            .entry_at(0)
            .expect("return admission")
            .semantic(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(_))
    ));
}
