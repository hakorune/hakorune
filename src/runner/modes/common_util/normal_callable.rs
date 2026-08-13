//! Shared normal-callable source materialization for MIR and LLVM ingress.
//!
//! The parser/transform pair is the only source-backed issuer.  Compatibility
//! remains an explicit AST lane; its legacy normalization is applied exactly
//! once here and never touches a `VerifiedFinalCallableProgramSourceV1`.

use crate::parser::{NyashParser, ParseError, ParserBuildConfig};
use crate::r#macro::{
    transform_normal_callable_program_v1, NormalCallableTransformOutcomeV1,
    NormalCallableTransformRejectV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableMaterializationErrorV1 {
    Parse(ParseError),
    Transform(NormalCallableTransformRejectV1),
}

/// Parse and classify one normal-callable source exactly once.
///
/// `SourceBacked` preserves the parser-issued callable product.  Only the
/// explicit Compatibility AST lane receives the existing runner normalization
/// pass, so the exact source product cannot be silently rewritten afterward.
pub(crate) fn materialize_normal_callable_program_v1(
    input: impl Into<String>,
    build_config: ParserBuildConfig,
) -> Result<NormalCallableTransformOutcomeV1, NormalCallableMaterializationErrorV1> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(input, build_config)
        .map_err(NormalCallableMaterializationErrorV1::Parse)?;
    let transformed = transform_normal_callable_program_v1(parsed)
        .map_err(NormalCallableMaterializationErrorV1::Transform)?;
    Ok(match transformed {
        NormalCallableTransformOutcomeV1::SourceBacked(source) => {
            NormalCallableTransformOutcomeV1::SourceBacked(source)
        }
        NormalCallableTransformOutcomeV1::Compatibility { ast, reason } => {
            NormalCallableTransformOutcomeV1::Compatibility {
                ast: super::super::macro_child::normalize_core_pass(&ast),
                reason,
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{materialize_normal_callable_program_v1, NormalCallableMaterializationErrorV1};
    use crate::r#macro::NormalCallableTransformOutcomeV1;

    #[test]
    fn exact_callable_source_stays_source_backed_without_compat_normalization() {
        let outcome = materialize_normal_callable_program_v1(
            "static box Scan { run(x) { return x } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("exact callable source");
        assert!(matches!(
            outcome,
            NormalCallableTransformOutcomeV1::SourceBacked(_)
        ));
    }

    #[test]
    fn compatibility_source_stays_on_the_explicit_ast_lane() {
        let outcome = materialize_normal_callable_program_v1(
            "box Node { value: i64 run() { return me.value } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("compatibility source");
        assert!(matches!(
            outcome,
            NormalCallableTransformOutcomeV1::Compatibility { .. }
        ));
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
