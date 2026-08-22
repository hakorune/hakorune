//! Callable-aware whole-Program source transport for normal compilation.

mod model;
mod semantic_syntax_loan;
mod transform;

pub(crate) use super::callable_parameter_source::ParserCallableSourceDispositionV1;
pub(in crate::parser) use model::NormalCallableParameterSourceRejectV1;
pub(crate) use model::{
    NormalCallableParserCompatibilityV1, NormalParserSourceLineageErrorV1,
    NormalParserSourceLineageV1, ParsedNormalCallableProgramV1,
    PreparedNormalCallableProgramSourceV1, VerifiedFinalCallableProgramSourceV1,
};
pub(crate) use semantic_syntax_loan::{
    CallableMethodSourceObservationV1, FinalCallableDeclarationModeV1,
    FinalCallableSemanticSyntaxLoanErrorV1,
};
pub(crate) use transform::{
    issue_final_callable_program_source_v1, FinalCallableProgramSourceRejectV1,
};

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
mod tests;
