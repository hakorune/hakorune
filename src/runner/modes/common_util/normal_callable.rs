//! Shared normal-callable source materialization for MIR and LLVM ingress.
//!
//! The parser/transform pair is the only source-backed issuer.  Compatibility
//! remains an explicit AST lane; its legacy normalization is applied exactly
//! once here and never touches a `VerifiedFinalCallableProgramSourceV1`.

use crate::mir::normal_source_plan::{
    NormalCallableCompatibilityOriginErrorV1, NormalCallableCompatibilityOriginV1,
    NormalParserCallableSourceHandoffV1,
};
use crate::mir::CanonicalSourceBytesDigestV1;
use crate::parser::{
    NormalParserSourceLineageErrorV1, NormalParserSourceLineageV1, NyashParser, ParseError,
    ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};
use crate::r#macro::{
    transform_normal_callable_program_v1, NormalCallableTransformOutcomeV1,
    NormalCallableTransformRejectV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableMaterializationErrorV1 {
    Parse(ParseError),
    SourceLineage(NormalParserSourceLineageErrorV1),
    Transform(NormalCallableTransformRejectV1),
    CompatibilityOrigin(NormalCallableCompatibilityOriginErrorV1),
}

#[derive(Debug)]
pub(crate) enum NormalCallableMaterializationOutcomeV1 {
    SourceBacked(VerifiedFinalCallableProgramSourceV1),
    Compatibility(NormalCallableCompatibilityOriginV1),
}

/// Parse and classify one normal-callable source exactly once.
///
/// `SourceBacked` preserves the parser-issued callable product.  Only the
/// explicit Compatibility AST lane receives the existing runner normalization
/// pass, so the exact source product cannot be silently rewritten afterward.
pub(crate) fn materialize_normal_callable_program_v1(
    input: impl Into<String>,
    build_config: ParserBuildConfig,
) -> Result<NormalCallableMaterializationOutcomeV1, NormalCallableMaterializationErrorV1> {
    materialize_normal_callable_program_with_identity_v1(input, build_config, "<selected-normal>")
}

pub(crate) fn materialize_normal_callable_program_with_identity_v1(
    input: impl Into<String>,
    build_config: ParserBuildConfig,
    source_identity: impl Into<Box<str>>,
) -> Result<NormalCallableMaterializationOutcomeV1, NormalCallableMaterializationErrorV1> {
    let input = input.into();
    let source_digest = CanonicalSourceBytesDigestV1::from_utf8_bytes(input.as_bytes());
    let source_lineage = NormalParserSourceLineageV1::issue(
        source_identity,
        source_digest,
        build_config.grammar_profile,
        input.len(),
        1,
        1,
    )
    .map_err(NormalCallableMaterializationErrorV1::SourceLineage)?;
    let product =
        NyashParser::parse_from_string_with_callable_parameter_source(input, build_config)
            .map_err(NormalCallableMaterializationErrorV1::Parse)?;
    let handoff =
        NormalParserCallableSourceHandoffV1::new(product.into_source_disposition(), source_lineage);
    let (parsed, source_lineage) = handoff
        .into_normal_callable_program()
        .map_err(NormalCallableMaterializationErrorV1::Parse)?;
    let transformed = transform_normal_callable_program_v1(parsed)
        .map_err(NormalCallableMaterializationErrorV1::Transform)?;
    Ok(match transformed {
        NormalCallableTransformOutcomeV1::SourceBacked(source) => {
            NormalCallableMaterializationOutcomeV1::SourceBacked(
                source.with_source_lineage(source_lineage),
            )
        }
        NormalCallableTransformOutcomeV1::Compatibility { ast, reason } => {
            let ast = super::super::macro_child::normalize_core_pass(&ast);
            let origin = NormalCallableCompatibilityOriginV1::issue(ast, reason, source_lineage)
                .map_err(NormalCallableMaterializationErrorV1::CompatibilityOrigin)?;
            NormalCallableMaterializationOutcomeV1::Compatibility(origin)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationErrorV1,
        NormalCallableMaterializationOutcomeV1,
    };
    use crate::r#macro::NormalCallableTransformCompatibilityV1;

    #[test]
    fn exact_callable_source_stays_source_backed_without_compat_normalization() {
        let outcome = materialize_normal_callable_program_v1(
            "static box Scan { run(x) { return x } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("exact callable source");
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = outcome else {
            panic!("exact callable source must stay source-backed")
        };
        let lineage = source.source_lineage().expect("parser lineage");
        assert_eq!(lineage.source_identity(), "<selected-normal>");
        assert_eq!(lineage.receipt_counts(), (1, 1));
    }

    #[test]
    fn compatibility_source_stays_on_the_explicit_ast_lane() {
        crate::test_support::with_env_vars(
            &[
                ("NYASH_MACRO_DISABLE", Some("0")),
                ("NYASH_MACRO_ENABLE", Some("1")),
            ],
            || {
                let outcome = materialize_normal_callable_program_v1(
                    "box Node { value: i64 run() { return me.value } }",
                    crate::parser::ParserBuildConfig::default(),
                )
                .expect("compatibility source");
                assert!(matches!(
                    outcome,
                    NormalCallableMaterializationOutcomeV1::Compatibility(origin)
                        if matches!(origin.reason(), NormalCallableTransformCompatibilityV1::DefaultDeriveWouldGenerateCallable)
                ));
            },
        );
    }

    #[test]
    fn parser_failure_stays_before_compatibility_normalization() {
        let error = materialize_normal_callable_program_v1(
            "static box Scan {",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect_err("malformed source must reject");
        assert!(matches!(
            error,
            NormalCallableMaterializationErrorV1::Parse(_)
        ));
    }
}
