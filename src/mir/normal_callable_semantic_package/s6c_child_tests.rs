use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageInstallIssueV1,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("S6C source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

#[test]
fn s6c_child_cannot_be_taken_through_generic_key_loan() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(603).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source(FIXTURE))
        .expect("same-cohort S6C child");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install_with_consumer(&mut context, None)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Main", "find_ok", 2),
    );
    assert_eq!(
        port.with_selected_lowering_input(&key, |_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired)
    );
}
