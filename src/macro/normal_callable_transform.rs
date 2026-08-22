//! Total macro transform boundary for callable-aware normal source.

use crate::ast::ASTNode;
use crate::parser::{
    issue_final_callable_program_source_v1, FinalCallableProgramSourceRejectV1,
    NormalCallableParserCompatibilityV1, ParsedNormalCallableProgramV1,
    VerifiedFinalCallableProgramSourceV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalCallableTransformCompatibilityV1 {
    Parser(NormalCallableParserCompatibilityV1),
    DefaultDeriveWouldGenerateCallable,
    RegisteredMacroBox,
}

#[derive(Debug)]
pub(crate) enum NormalCallableTransformOutcomeV1 {
    SourceBacked(VerifiedFinalCallableProgramSourceV1),
    Compatibility {
        ast: ASTNode,
        reason: NormalCallableTransformCompatibilityV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalCallableTransformRejectV1 {
    ExactSourceChanged(FinalCallableProgramSourceRejectV1),
}

pub(crate) fn transform_normal_callable_program_v1(
    parsed: ParsedNormalCallableProgramV1,
) -> Result<NormalCallableTransformOutcomeV1, NormalCallableTransformRejectV1> {
    match parsed {
        ParsedNormalCallableProgramV1::Compatibility { ast, cohort } => {
            Ok(NormalCallableTransformOutcomeV1::Compatibility {
                ast: super::maybe_expand_and_dump(&ast, false),
                reason: NormalCallableTransformCompatibilityV1::Parser(cohort),
            })
        }
        ParsedNormalCallableProgramV1::SourceBacked(initial) => {
            super::macro_box::init_builtin();
            super::macro_box_ny::init_from_env();
            let compatibility = if !super::enabled() {
                None
            } else if super::macro_box::has_registered_transform() {
                Some(NormalCallableTransformCompatibilityV1::RegisteredMacroBox)
            } else if super::engine::MacroEngine::would_generate_default_callable(initial.ast()) {
                Some(NormalCallableTransformCompatibilityV1::DefaultDeriveWouldGenerateCallable)
            } else {
                None
            };
            if let Some(reason) = compatibility {
                if initial.composite_source_is_ready() {
                    return Err(NormalCallableTransformRejectV1::ExactSourceChanged(
                        FinalCallableProgramSourceRejectV1::Composite(
                            crate::parser::callable_parameter_source::
                                ParserCompositeTransformRejectV1::CompatibilityLoss,
                        ),
                    ));
                }
                let ast = initial.into_ast();
                return Ok(NormalCallableTransformOutcomeV1::Compatibility {
                    ast: super::maybe_expand_and_dump(&ast, false),
                    reason,
                });
            }
            let output = super::maybe_expand_and_dump(initial.ast(), false);
            issue_final_callable_program_source_v1(initial, output)
                .map(NormalCallableTransformOutcomeV1::SourceBacked)
                .map_err(NormalCallableTransformRejectV1::ExactSourceChanged)
        }
    }
}
