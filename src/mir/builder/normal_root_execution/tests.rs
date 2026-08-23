use super::{
    AdmittedNormalRootExecutionModeV1, NormalRootExecutionConsumerRejectV1,
    NormalRootExecutionConsumerV1,
};
use crate::mir::normal_source_plan::NormalSourcePlanErrorV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn consume(source: &str) -> AdmittedNormalRootExecutionModeV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact source-backed transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    NormalRootExecutionConsumerV1::consume_once(source)
        .expect("preserved root")
        .consume_at_named_test_terminal()
}

#[test]
fn consumes_program_runtime_once_without_ast_classification() {
    assert_eq!(
        consume("print(1)"),
        AdmittedNormalRootExecutionModeV1::ProgramRuntime
    );
}

#[test]
fn consumes_app_once_from_parser_relation() {
    assert_eq!(
        consume("static box Main { main() { 1 } helper() { 2 } }"),
        AdmittedNormalRootExecutionModeV1::App
    );
}

#[test]
fn same_named_top_level_and_main_child_are_paired_by_parser_identity() {
    assert_eq!(
        consume(
            "function helper() { return 1 }\nstatic box Main { main() { return 0 } helper() { return 2 } }",
        ),
        AdmittedNormalRootExecutionModeV1::App
    );
}

#[test]
fn non_static_main_rejects_with_source_policy_before_projection() {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        "box Main { main() { 1 } }",
        ParserBuildConfig::default(),
    )
    .expect("complete non-static Main source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact source-backed transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    assert_eq!(
        NormalRootExecutionConsumerV1::consume_once(source)
            .expect_err("non-static Main must reject before Builder projection")
            .into_error_after_discard(),
        NormalRootExecutionConsumerRejectV1::SourcePolicy(
            NormalSourcePlanErrorV1::MainMustBeStatic,
        )
    );
}
