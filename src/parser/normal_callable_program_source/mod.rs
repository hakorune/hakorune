//! Callable-aware whole-Program source transport for normal compilation.

mod model;
mod normal_root_execution_preservation;
mod ordinary_new_source;
mod semantic_syntax_loan;
mod transform;

pub(in crate::parser) use model::NormalCallableParameterSourceRejectV1;
pub(crate) use model::{
    NormalCallableParserCompatibilityV1, NormalParserSourceLineageErrorV1,
    NormalParserSourceLineageV1, ParsedNormalCallableProgramV1,
    ParserNormalCallableTransformSessionV1, PreparedNormalCallableProgramSourceV1,
    VerifiedFinalCallableProgramSourceV1,
};
pub(crate) use ordinary_new_source::{
    ParserOrdinaryBoxSourceCoverageV1, ParserOrdinaryBoxSourceRowV1,
};
pub(crate) use normal_root_execution_preservation::{
    ParserNormalRootExecutionPreservationIssuerV1, ParserNormalRootExecutionPreservationRejectV1,
    ParserNormalRootExecutionPreservationV1,
};
pub(crate) use semantic_syntax_loan::{
    CallableMethodSourceObservationV1, FinalCallableDeclarationModeV1,
    FinalCallableSemanticSyntaxLoanErrorV1, FinalCallableSemanticSyntaxRowRefV1,
};
pub(crate) use transform::FinalCallableProgramSourceRejectV1;

impl super::NyashParser {
    pub(crate) fn parse_normal_callable_program_with_build_config(
        input: impl Into<String>,
        build_config: super::ParserBuildConfig,
    ) -> Result<ParsedNormalCallableProgramV1, super::ParseError> {
        super::string_postpass_entry::parse_normal_callable_program(
            input.into(),
            Some(100_000),
            build_config,
        )
    }
}

#[cfg(test)]
mod normal_root_execution_preservation_tests;
#[cfg(test)]
mod tests;
