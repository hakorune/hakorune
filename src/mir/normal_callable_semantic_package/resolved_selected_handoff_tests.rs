use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageInstallIssueV1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("handoff source");
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

fn admission(key: &CanonicalSameModuleCallableKeyV1) -> NormalCatalogedBoxMethodDraftAdmissionV1 {
    NormalCatalogedBoxMethodDraftAdmissionV1::seal(key.clone()).expect("catalog admission")
}

#[test]
fn selected_static_and_instance_rows_lend_one_signature_sibling() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(951).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source(
            r#"
static box StaticApi {
  run(source: StringBox, needle: StringBox) { return { 0 } }
}
box InstanceApi {
  run(source: StringBox) { return 0 }
}
"#,
        ),
    )
    .expect("selected ordinary package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let static_key =
        CanonicalSameModuleCallableKeyV1::test_static_box_method("StaticApi", "run", 2);
    let instance_key =
        CanonicalSameModuleCallableKeyV1::test_instance_box_method("InstanceApi", "run", 1);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_selected_cataloged_lowering_input_and_signature(
        admission(&static_key),
        |input, signature| {
            input.with_selected_and_admission(|selected, admitted| {
                assert_eq!(selected.source().owner(), signature.owner());
                assert_eq!(selected.block_expr_expectation().pair_count(), 1);
                assert_eq!(
                    admitted.source_key().arity(),
                    signature.source_logical_arity()
                );
                assert_eq!(signature.receiver_lane_count(), 0);
                assert_eq!(signature.physical_formal_lane_count(), 4);
                assert_eq!(signature.physical_callable_lane_count(), 4);
            });
        },
    )
    .expect("static sibling loan");

    port.with_selected_cataloged_lowering_input_and_signature(
        admission(&instance_key),
        |input, signature| {
            input.with_selected_and_admission(|selected, admitted| {
                assert_eq!(selected.source().owner(), signature.owner());
                assert_eq!(
                    admitted.source_key().arity(),
                    signature.source_logical_arity()
                );
                assert_eq!(signature.receiver_lane_count(), 1);
                assert_eq!(signature.physical_formal_lane_count(), 2);
                assert_eq!(signature.physical_callable_lane_count(), 3);
            });
        },
    )
    .expect("instance sibling loan");

    assert_eq!(
        port.with_selected_cataloged_lowering_input_and_signature(
            admission(&static_key),
            |_, _| ()
        ),
        Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey)
    );
    port.complete().expect("all selected rows consumed");
}
