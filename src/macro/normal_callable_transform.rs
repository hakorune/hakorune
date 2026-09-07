//! Total macro transform boundary for callable-aware normal source.

use crate::ast::ASTNode;
use crate::parser::{
    FinalCallableProgramSourceRejectV1, NormalCallableParserCompatibilityV1,
    ParsedNormalCallableProgramV1, ParserNormalCallableTransformSessionV1,
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
    UnclassifiedSourceMutation,
}

pub(crate) fn transform_normal_callable_program_v1(
    parsed: ParsedNormalCallableProgramV1,
) -> Result<NormalCallableTransformOutcomeV1, NormalCallableTransformRejectV1> {
    transform_normal_callable_program_with_policy_v1(parsed, super::NormalMacroPolicyV1::capture())
}

pub(crate) fn transform_normal_callable_program_with_policy_v1(
    parsed: ParsedNormalCallableProgramV1,
    policy: super::NormalMacroPolicyV1,
) -> Result<NormalCallableTransformOutcomeV1, NormalCallableTransformRejectV1> {
    match parsed {
        ParsedNormalCallableProgramV1::Compatibility { ast, cohort } => {
            Ok(NormalCallableTransformOutcomeV1::Compatibility {
                ast: expand_compatibility_with_policy(ast, policy),
                reason: NormalCallableTransformCompatibilityV1::Parser(cohort),
            })
        }
        ParsedNormalCallableProgramV1::SourceBacked(initial) => {
            super::macro_box::init_builtin();
            super::macro_box_ny::init_from_env();
            let macro_enabled = policy.enabled();
            let compatibility = if !macro_enabled {
                None
            } else if super::macro_box::has_registered_transform() {
                Some(NormalCallableTransformCompatibilityV1::RegisteredMacroBox)
            } else if policy.would_generate(initial.ast()) {
                Some(NormalCallableTransformCompatibilityV1::DefaultDeriveWouldGenerateCallable)
            } else {
                None
            };
            if let Some(reason) = compatibility {
                let error = if initial.composite_source_is_ready() {
                    NormalCallableTransformRejectV1::ExactSourceChanged(
                        FinalCallableProgramSourceRejectV1::Composite(
                            crate::parser::callable_parameter_source::
                                ParserCompositeTransformRejectV1::CompatibilityLoss,
                        ),
                    )
                } else {
                    NormalCallableTransformRejectV1::ExactSourceChanged(
                        FinalCallableProgramSourceRejectV1::RootPreservation(
                            crate::parser::ParserNormalRootExecutionPreservationRejectV1::CompatibilityLoss,
                        ),
                    )
                };
                drop(reason);
                initial.discard_at_named_transform_reject_terminal();
                return Err(error);
            }
            if macro_enabled {
                let trace = crate::config::env::macro_trace();
                if trace {
                    crate::macro_log!("[macro] input AST: {:?}", initial.ast());
                }
                // Default generation has already been handled before co-seal.
                // This source-backed stage owns no AST mutation or macro retry.
                let expanded = initial.ast();
                match super::test_harness::issue_test_harness_transform_v1(&expanded) {
                    super::test_harness::TestHarnessTransformDispositionV1::Unchanged => {
                        if trace {
                            crate::macro_log!("[macro] output AST: {:?}", expanded);
                        }
                    }
                    super::test_harness::TestHarnessTransformDispositionV1::GeneratedTail(
                        transformed,
                    ) => {
                        if trace {
                            crate::macro_log!("[macro] output AST: {:?}", transformed);
                        }
                        let error = if initial.composite_source_is_ready() {
                            NormalCallableTransformRejectV1::ExactSourceChanged(
                                FinalCallableProgramSourceRejectV1::Composite(
                                    crate::parser::callable_parameter_source::
                                        ParserCompositeTransformRejectV1::CompatibilityLoss,
                                ),
                            )
                        } else {
                            NormalCallableTransformRejectV1::ExactSourceChanged(
                                FinalCallableProgramSourceRejectV1::RootPreservation(
                                    crate::parser::ParserNormalRootExecutionPreservationRejectV1::CompatibilityLoss,
                                ),
                            )
                        };
                        drop(transformed);
                        initial.discard_at_named_transform_reject_terminal();
                        return Err(error);
                    }
                }
            }

            let session: ParserNormalCallableTransformSessionV1 = initial.begin_transform();
            session
                .finish_exact()
                .map(NormalCallableTransformOutcomeV1::SourceBacked)
                .map_err(NormalCallableTransformRejectV1::ExactSourceChanged)
        }
    }
}

pub(super) fn require_unchanged_source_macro_output_v1(
    source: &ASTNode,
    expanded: &ASTNode,
) -> Result<(), NormalCallableTransformRejectV1> {
    if source != expanded {
        return Err(NormalCallableTransformRejectV1::UnclassifiedSourceMutation);
    }
    Ok(())
}

fn expand_compatibility_with_policy(ast: ASTNode, policy: super::NormalMacroPolicyV1) -> ASTNode {
    if !policy.enabled() {
        return ast;
    }
    super::macro_box::init_builtin();
    super::macro_box_ny::init_from_env();
    let (expanded, _) = super::engine::MacroEngine::with_default_policy(policy).expand(&ast);
    match super::test_harness::issue_test_harness_transform_v1(&expanded) {
        super::test_harness::TestHarnessTransformDispositionV1::Unchanged => expanded,
        super::test_harness::TestHarnessTransformDispositionV1::GeneratedTail(ast) => ast,
    }
}
