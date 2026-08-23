use crate::ast::ASTNode;
use crate::parser::{
    NormalCallableParserCompatibilityV1, NyashParser, ParsedNormalCallableProgramV1,
    ParserBuildConfig,
};

use super::normal_callable_transform::require_unchanged_source_macro_output_v1;
use super::{
    transform_normal_callable_program_v1, NormalCallableTransformCompatibilityV1,
    NormalCallableTransformOutcomeV1, NormalCallableTransformRejectV1,
};

fn parse(source: &str) -> ParsedNormalCallableProgramV1 {
    NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source")
}

#[test]
fn disabled_macro_keeps_static_source_backed() {
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = transform_normal_callable_program_v1(parse(
            "static box ParserScanLoopBox { skip_while(a, b, c, d) { return b } }",
        ))
        .expect("exact source transform");
        assert!(matches!(
            transformed,
            NormalCallableTransformOutcomeV1::SourceBacked(_)
        ));
    });
}

#[test]
fn instance_default_derive_selects_compatibility_before_transform() {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_ENABLE", Some("1")),
        ],
        || {
            let transformed = transform_normal_callable_program_v1(parse(
                "box Node { value: i64 run() { return me.value } }",
            ))
            .expect("typed compatibility transform");
            assert!(matches!(
                transformed,
                NormalCallableTransformOutcomeV1::Compatibility { .. }
            ));
        },
    );
}

#[test]
fn parser_compatibility_remains_typed_before_macro_execution() {
    let parsed = parse("record Pair { left: i64, right: i64 }");
    assert!(matches!(
        parsed,
        ParsedNormalCallableProgramV1::Compatibility {
            cohort: NormalCallableParserCompatibilityV1::RecordBox,
            ..
        }
    ));
}

#[test]
fn enabled_macro_with_no_actual_test_tail_stays_source_backed() {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_TEST_RUN", Some("0")),
            ("NYASH_TEST_ARGS_JSON", None),
            ("NYASH_MACRO_PATHS", None),
        ],
        || {
            let transformed =
                transform_normal_callable_program_v1(parse("function helper() { return 0 }"))
                    .expect("actual no-op remains exact");
            assert!(matches!(
                transformed,
                NormalCallableTransformOutcomeV1::SourceBacked(_)
            ));
        },
    );
}

#[test]
fn actual_test_harness_tail_enters_typed_compatibility() {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_TEST_RUN", Some("0")),
            ("NYASH_TEST_ARGS_JSON", None),
            ("NYASH_MACRO_PATHS", None),
        ],
        || {
            let transformed =
                transform_normal_callable_program_v1(parse("function test_zero() { return 0 }"))
                    .expect("generated test tail has an explicit compatibility owner");
            let NormalCallableTransformOutcomeV1::Compatibility { ast, reason } = transformed
            else {
                panic!("generated test tail must not receive a parser root token")
            };
            assert_eq!(
                reason,
                NormalCallableTransformCompatibilityV1::TestHarnessGeneratedTail
            );
            let ASTNode::Program { statements, .. } = ast else {
                panic!("compatibility output must remain a Program")
            };
            assert_eq!(statements.len(), 2);
            assert!(matches!(
                statements.last(),
                Some(ASTNode::FunctionCall { name, arguments, .. })
                    if name == "test_zero" && arguments.is_empty()
            ));
        },
    );
}

#[test]
fn composite_ready_test_tail_rejects_compatibility_loss() {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_TEST_RUN", Some("0")),
            ("NYASH_TEST_ARGS_JSON", None),
            ("NYASH_MACRO_PATHS", None),
        ],
        || {
            let result = transform_normal_callable_program_v1(parse(
                "static box Helpers { test_run() { return 1 } }\nHelpers.test_run()",
            ));
            assert!(matches!(
                result,
                Err(NormalCallableTransformRejectV1::ExactSourceChanged(
                    crate::parser::FinalCallableProgramSourceRejectV1::Composite(
                        crate::parser::callable_parameter_source::
                            ParserCompositeTransformRejectV1::CompatibilityLoss
                    )
                ))
            ));
        },
    );
}

#[test]
fn unclassified_macro_mutation_is_not_exact_or_compatibility() {
    let source = NyashParser::parse_from_string("print(1)").expect("source fixture");
    let changed = NyashParser::parse_from_string("print(2)").expect("changed fixture");
    assert_eq!(
        require_unchanged_source_macro_output_v1(&source, &changed),
        Err(NormalCallableTransformRejectV1::UnclassifiedSourceMutation)
    );
}
