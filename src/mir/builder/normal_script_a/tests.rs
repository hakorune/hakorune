use super::super::super::normal_script_direct_static_lookup::ScriptDirectStaticCallLookupIssuerV1;
use super::super::super::normal_script_neutral_window::PreparedCanonicalScriptNeutralProgramWindowV1;
use super::super::NormalScriptPreEffectSourceObservationIssuerV1;
use super::super::super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::super::issue_into_c_transport;
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_v1, VerifiedNormalCallableSemanticPackageV1,
};
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

fn pre_effect_parts(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (
    PreparedNormalProgramDeclarationFactsV1,
    crate::mir::builder::normal_script_root_demand_window::PreparedScriptRootAdmissionV1,
    crate::mir::source_call_target::VerifiedScriptDirectStaticCallLookupV1,
    FunctionSemanticResolverSessionV1,
) {
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(package)
        .expect("neutral source window");
    let (lookup, _) = ScriptDirectStaticCallLookupIssuerV1::issue(package, Some(&neutral), &[])
        .expect("owned lookup relation");
    let lookup = lookup.expect("non-App Script lookup");
    let facts = package
        .with_normal_program_source_loan(|loan| {
            PreparedNormalProgramDeclarationFactsV1::collect(loan.program())
        })
        .expect("source loan")
        .expect("declaration facts");
    let (source_window, _) = neutral.split_for_pre_effect();
    let resolver = FunctionSemanticResolverSessionV1::new(9121).expect("resolver session");
    (facts, source_window, lookup, resolver)
}

#[test]
fn no_direct_arm_retains_each_explicit_non_direct_source_row() {
    let package = package("return 1.run()", 9120);
    let (facts, source_window, lookup, mut resolver) = pre_effect_parts(&package);
    let observation = NormalScriptPreEffectSourceObservationIssuerV1::issue(
        &package,
        source_window,
        lookup,
        &facts,
        &mut resolver,
    )
    .expect("complete source observation");
    let transport = issue_into_c_transport(observation).expect("A/C disposition");
    assert_eq!(transport.disposition_counts(), (0, 1));
}

#[test]
fn direct_arm_retains_candidate_rows_without_a_second_lookup() {
    let package = package(
        "static box Helpers { run(value) { return 7 } }\nreturn Helpers.run(1)",
        9122,
    );
    let (facts, source_window, lookup, mut resolver) = pre_effect_parts(&package);
    let observation = NormalScriptPreEffectSourceObservationIssuerV1::issue(
        &package,
        source_window,
        lookup,
        &facts,
        &mut resolver,
    )
    .expect("complete source observation");
    let transport = issue_into_c_transport(observation).expect("A/C disposition");
    assert_eq!(transport.disposition_counts(), (1, 0));
}
