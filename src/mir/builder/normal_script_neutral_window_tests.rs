use super::*;

use crate::mir::normal_callable_semantic_package::
    issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ScriptRootResolvedDemandV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1,
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
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
    else {
        panic!("fixture must remain source-backed")
    };
    let mut resolver = FunctionSemanticResolverSessionV1::new(session_id)
        .expect("resolver session");
    issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("semantic package")
}

#[test]
fn neutral_issuer_co_seals_composite_provider_and_root_return_window() {
    let package = package(
        "static box Helpers { run(value) { return value } }\nreturn Helpers.run(1)",
        152,
    );
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("bounded neutral Script window");
    let (admission, remainder) = neutral.split_for_pre_effect();
    let (transfers, _constructor_source_cohort) = remainder.into_parts();

    assert!(transfers.invocation_witness().same_as(
        &package
            .with_normal_program_source_loan(|loan| loan.invocation_witness().clone())
            .expect("source loan")
    ));
    assert!(matches!(
        admission.window().entry_at(0).expect("provider entry").semantic(),
        ScriptRootSemanticDispositionV1::Transferred(
            ScriptTransferredBoundaryV1::StaticCallableCatalogTransfer
        )
    ));
    assert!(matches!(
        admission.window().entry_at(1).expect("return entry").semantic(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(_))
    ));
}

#[test]
fn neutral_issuer_keeps_non_composite_source_explicitly_complete() {
    let package = package("42", 153);
    let neutral = PreparedCanonicalScriptNeutralProgramWindowV1::issue(&package)
        .expect("ordinary Script window remains complete");
    let (admission, _remainder) = neutral.split_for_pre_effect();

    assert!(matches!(
        admission.window().entry_at(0).expect("ordinary entry").semantic(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore)
    ));
    assert!(admission.deferred_residuals().entries().is_empty());
}
