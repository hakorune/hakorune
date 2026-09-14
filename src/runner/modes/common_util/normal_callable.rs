//! Shared normal-callable source materialization for MIR and LLVM ingress.
//!
//! The parser/transform pair is the only source-backed issuer.  Compatibility
//! remains an explicit AST lane; its legacy normalization is applied exactly
//! once here and never touches a `VerifiedFinalCallableProgramSourceV1`.

#[cfg(test)]
use crate::parser::NyashParser;

use crate::mir::normal_source_plan::{
    NormalCallableCompatibilityOriginErrorV1, NormalCallableCompatibilityOriginV1,
};
use crate::mir::CanonicalSourceBytesDigestV1;
use crate::parser::{
    NormalParserSourceLineageErrorV1, NormalParserSourceLineageV1, ParseError,
    ParserSourceAdmissionErrorV1,
    ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};
use crate::runner::modes::common_util::resolve::MergedSourceLineageV1;
use crate::r#macro::{
    transform_normal_callable_program_with_policy_v1, NormalCallableTransformOutcomeV1,
    NormalCallableTransformRejectV1, NormalMacroPolicyV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableMaterializationErrorV1 {
    Parse(ParseError),
    SourceLineage(NormalParserSourceLineageErrorV1),
    SourceAdmission(ParserSourceAdmissionErrorV1),
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
    materialize_normal_callable_program_with_identity_and_optional_lineage_v1(
        input,
        build_config,
        source_identity,
        None,
    )
}

pub(crate) fn materialize_normal_callable_program_with_identity_and_lineage_v1(
    input: impl Into<String>,
    build_config: ParserBuildConfig,
    source_identity: impl Into<Box<str>>,
    merged_source_lineage: MergedSourceLineageV1,
) -> Result<NormalCallableMaterializationOutcomeV1, NormalCallableMaterializationErrorV1> {
    materialize_normal_callable_program_with_identity_and_optional_lineage_v1(
        input,
        build_config,
        source_identity,
        Some(merged_source_lineage),
    )
}

fn materialize_normal_callable_program_with_identity_and_optional_lineage_v1(
    input: impl Into<String>,
    build_config: ParserBuildConfig,
    source_identity: impl Into<Box<str>>,
    merged_source_lineage: Option<MergedSourceLineageV1>,
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
    let source_lineage = if let Some(lineage) = merged_source_lineage {
        source_lineage.with_merged_source_lineage(lineage)
    } else {
        source_lineage
    };
    let policy = NormalMacroPolicyV1::capture();
    let product =
        crate::parser::string_postpass_entry::parse_with_callable_parameter_source_policy(
            input,
            Some(100_000),
            build_config,
            Some(&policy),
        )
        .map_err(NormalCallableMaterializationErrorV1::Parse)?;
    let source_admission_witness = match source_lineage.merged_source_lineage() {
        Some(lineage) => product
            .issue_source_admission_witness(lineage)
            .map_err(NormalCallableMaterializationErrorV1::SourceAdmission)?,
        None => None,
    };
    let parsed = product
        .into_normal_callable_program()
        .map_err(NormalCallableMaterializationErrorV1::Parse)?;
    let transformed = transform_normal_callable_program_with_policy_v1(parsed, policy)
        .map_err(NormalCallableMaterializationErrorV1::Transform)?;
    Ok(match transformed {
        NormalCallableTransformOutcomeV1::SourceBacked(source) => {
            let invocation = source
                .parser_invocation_witness()
                .cloned()
                .ok_or(NormalCallableMaterializationErrorV1::SourceLineage(
                    NormalParserSourceLineageErrorV1::ParserInvocationMissing,
                ))?;
            let source_lineage = source_lineage.co_seal_parser_invocation(invocation);
            let source_lineage = match source_admission_witness {
                Some(witness) => source_lineage.with_source_admission_witness(witness),
                None => source_lineage,
            };
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
        materialize_normal_callable_program_v1,
        materialize_normal_callable_program_with_identity_and_lineage_v1,
        NormalCallableMaterializationErrorV1, NormalCallableMaterializationOutcomeV1,
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
        source.discard_at_named_root_execution_terminal();
    }

    #[test]
    fn merged_lineage_is_co_sealed_to_the_parser_invocation() {
        let lineage = crate::runner::modes::common_util::resolve::MergedSourceLineageV1::root_only(
            "static box Scan { run() { return 1 } }",
            "scan.hako",
        )
        .expect("root lineage");
        let outcome = materialize_normal_callable_program_with_identity_and_lineage_v1(
            "static box Scan { run() { return 1 } }",
            crate::parser::ParserBuildConfig::default(),
            "scan.hako",
            lineage,
        )
        .expect("lineage-aware source");
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = outcome else {
            panic!("lineage-aware source must stay source-backed")
        };
        let source_lineage = source.source_lineage().expect("source lineage");
        assert_eq!(source_lineage.merged_source_lineage().unwrap().segments().len(), 1);
        assert!(source_lineage.parser_invocation_witness().is_some());
        assert!(source_lineage.source_admission_witness().is_some());
        source.discard_at_named_root_execution_terminal();
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
                    "record Pair { left: i64, right: i64 }",
                    crate::parser::ParserBuildConfig::default(),
                )
                .expect("compatibility source");
                assert!(matches!(
                    outcome,
                    NormalCallableMaterializationOutcomeV1::Compatibility(origin)
                        if matches!(
                            origin.reason(),
                            NormalCallableTransformCompatibilityV1::Parser(
                                crate::parser::NormalCallableParserCompatibilityV1::RecordBox
                            )
                        )
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

#[cfg(test)]
#[path = "normal_callable_default_derive_tests.rs"]
mod default_derive_tests;
