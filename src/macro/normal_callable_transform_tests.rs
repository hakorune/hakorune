use crate::parser::{
    NormalCallableParserCompatibilityV1, NyashParser, ParsedNormalCallableProgramV1,
    ParserBuildConfig,
};

use super::{transform_normal_callable_program_v1, NormalCallableTransformOutcomeV1};

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
