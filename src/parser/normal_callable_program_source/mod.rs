//! Callable-aware whole-Program source transport for normal compilation.

mod model;
mod transform;

pub(crate) use model::{
    NormalCallableParserCompatibilityV1, ParsedNormalCallableProgramV1,
    VerifiedFinalCallableProgramSourceV1,
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
