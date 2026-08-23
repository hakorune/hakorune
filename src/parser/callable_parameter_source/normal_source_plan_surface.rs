//! Parser-owned normal source-plan surface for the bounded I0-A handoff.
//!
//! This module co-seals final placement, parser source relation, and the
//! static-parent/member payload once.  It emits no policy, Recipe, or MIR
//! meaning.  The compiler must consume this product instead of rebuilding a
//! surface from the AST.

use crate::ast::ASTNode;

use super::super::callable_source_anchor::{
    CallableDeclarationIdentityV1, DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use super::catalog::ParserCallableParameterSourceDispositionV1;
use super::normal_source_plan_seed::ParserNormalSourcePlanSeedDispositionV1;
use super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::static_box_source::PreparedParserStaticBoxParentSourceV1;
use crate::parser::build_cfg::program_item_slots::ProjectedProgramItemSlotV1;
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::source_path::SourceProgramCallablePathV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanUnsupportedKindV1 {
    NestedProgram,
    BuildGate,
    Using,
    Import,
    Box,
    Enum,
    Brand,
    TypeAlias,
    Global,
    StaticConstTable,
}

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalSourcePlanTopLevelRowV1 {
    Executable {
        slot: ProjectedProgramItemSlotV1,
    },
    TopLevelCallable {
        slot: ProjectedProgramItemSlotV1,
        callable_identity: CallableDeclarationIdentityV1,
        callable_kind: DirectCallableDeclarationKindV1,
    },
    StaticBox {
        slot: ProjectedProgramItemSlotV1,
        source: PreparedParserStaticBoxParentSourceV1,
    },
    Unsupported {
        slot: ProjectedProgramItemSlotV1,
        kind: ParserNormalSourcePlanUnsupportedKindV1,
    },
}

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceV1 {
    CompleteEmpty,
    CompleteRows(Box<[ParserNormalSourcePlanTopLevelRowV1]>),
}

#[derive(Debug)]
pub(in crate::parser) struct ParserBackedNormalSourcePlanBoundV1 {
    invocation: ParserInvocationWitnessV1,
    surface: ParserNormalSourcePlanSurfaceV1,
    _seal: ParserBackedNormalSourcePlanBoundSealV1,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserBackedNormalSourcePlanBoundSealV1;

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceDispositionV1 {
    Ready(ParserBackedNormalSourcePlanBoundV1),
    CompatibilityOutside,
    SourceAuthorityUnavailable(ParserNormalSourcePlanSurfaceUnavailableV1),
    Incomplete(ParserNormalSourcePlanSurfaceIncompleteV1),
    IntegrityInvalid(ParserNormalSourcePlanSurfaceIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceUnavailableV1 {
    PostpassNotSourceBacked,
    ParameterSourceUnavailable,
    SeedAlreadyConsumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceIncompleteV1 {
    ProgramMissing,
    SlotCoverageMismatch,
    SlotOutOfRange,
    CallableSourceMissing,
    StaticParentSourceMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceIntegrityIssueV1 {
    ForeignParserRelation,
    DuplicateCallableSource,
    OrphanStaticParentSource,
}

impl ParserBackedNormalSourcePlanBoundV1 {
    pub(in crate::parser) fn invocation(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    #[cfg(test)]
    pub(in crate::parser) fn surface(&self) -> &ParserNormalSourcePlanSurfaceV1 {
        &self.surface
    }
}

pub(in crate::parser) struct ParserNormalSourcePlanSurfaceIssuerV1;

impl ParserNormalSourcePlanSurfaceIssuerV1 {
    pub(in crate::parser) fn issue_once(
        completed: &CompletedParserPostpassV1,
        parameter_source: &ParserCallableParameterSourceDispositionV1,
        seed: ParserNormalSourcePlanSeedDispositionV1,
    ) -> ParserNormalSourcePlanSurfaceDispositionV1 {
        if !completed.is_source_backed() {
            return ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
            );
        }
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
            return ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalSourcePlanSurfaceUnavailableV1::ParameterSourceUnavailable,
            );
        };
        let ParserNormalSourcePlanSeedDispositionV1::Ready(seed) = seed else {
            return match seed {
                ParserNormalSourcePlanSeedDispositionV1::CompatibilityOutside => {
                    ParserNormalSourcePlanSurfaceDispositionV1::CompatibilityOutside
                }
                ParserNormalSourcePlanSeedDispositionV1::Consumed => {
                    ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                        ParserNormalSourcePlanSurfaceUnavailableV1::SeedAlreadyConsumed,
                    )
                }
                ParserNormalSourcePlanSeedDispositionV1::Ready(_) => unreachable!(),
            };
        };
        let (slot_set, static_parent_sources) = seed.into_parts();
        let invocation = ParserInvocationWitnessV1::from_brand(slot_set.brand());
        if !catalog.same_parser_brand(slot_set.brand()) {
            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::ForeignParserRelation,
            );
        }
        let slots = slot_set.into_rows();
        let ASTNode::Program { statements, .. } = completed.ast() else {
            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                ParserNormalSourcePlanSurfaceIncompleteV1::ProgramMissing,
            );
        };
        if slots.len() != statements.len() {
            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                ParserNormalSourcePlanSurfaceIncompleteV1::SlotCoverageMismatch,
            );
        }

        let mut static_parent_sources = static_parent_sources.into_vec();
        let mut rows = Vec::with_capacity(slots.len());
        for slot in slots {
            let Some(statement) = statements.get(slot.final_statement_slot() as usize) else {
                return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                    ParserNormalSourcePlanSurfaceIncompleteV1::SlotOutOfRange,
                );
            };
            let row = match statement {
                ASTNode::FunctionDeclaration { .. } => {
                    let callable_matches = completed
                        .callable_rows()
                        .iter()
                        .filter_map(PreparedCallableSourceV1::direct)
                        .filter(|source| {
                            matches!(
                                source.path(),
                                SourceProgramCallablePathV1::TopLevel { declaration }
                                    if *declaration == *slot.source_path()
                            )
                        })
                        .collect::<Vec<_>>();
                    let Some(source) = callable_matches.first() else {
                        return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                            ParserNormalSourcePlanSurfaceIncompleteV1::CallableSourceMissing,
                        );
                    };
                    if callable_matches.len() != 1 {
                        return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                            ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSource,
                        );
                    }
                    if !source.parser_brand().same_as(slot.source_path().brand()) {
                        return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                            ParserNormalSourcePlanSurfaceIntegrityIssueV1::ForeignParserRelation,
                        );
                    }
                    ParserNormalSourcePlanTopLevelRowV1::TopLevelCallable {
                        slot,
                        callable_identity: source.anchor().identity(),
                        callable_kind: source.kind(),
                    }
                }
                ASTNode::BoxDeclaration { is_static, .. } if *is_static => {
                    let Some(index) = static_parent_sources.iter().position(|source| {
                        source.box_site().path() == slot.source_path().compatibility_box_path()
                    }) else {
                        return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                            ParserNormalSourcePlanSurfaceIncompleteV1::StaticParentSourceMissing,
                        );
                    };
                    ParserNormalSourcePlanTopLevelRowV1::StaticBox {
                        slot,
                        source: static_parent_sources.swap_remove(index),
                    }
                }
                ASTNode::BoxDeclaration { .. } => {
                    ParserNormalSourcePlanTopLevelRowV1::Unsupported {
                        slot,
                        kind: ParserNormalSourcePlanUnsupportedKindV1::Box,
                    }
                }
                ASTNode::Program { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::NestedProgram)
                }
                ASTNode::BuildGate { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::BuildGate)
                }
                ASTNode::UsingStatement { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Using)
                }
                ASTNode::ImportStatement { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Import)
                }
                ASTNode::EnumDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Enum)
                }
                ASTNode::BrandDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Brand)
                }
                ASTNode::TypeAliasDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::TypeAlias)
                }
                ASTNode::GlobalVar { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Global)
                }
                ASTNode::StaticConstTable { .. } => unsupported(
                    slot,
                    ParserNormalSourcePlanUnsupportedKindV1::StaticConstTable,
                ),
                _ => ParserNormalSourcePlanTopLevelRowV1::Executable { slot },
            };
            rows.push(row);
        }
        if !static_parent_sources.is_empty() {
            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::OrphanStaticParentSource,
            );
        }
        let surface = match rows.len() {
            0 => ParserNormalSourcePlanSurfaceV1::CompleteEmpty,
            _ => ParserNormalSourcePlanSurfaceV1::CompleteRows(rows.into_boxed_slice()),
        };
        ParserNormalSourcePlanSurfaceDispositionV1::Ready(ParserBackedNormalSourcePlanBoundV1 {
            invocation,
            surface,
            _seal: ParserBackedNormalSourcePlanBoundSealV1,
        })
    }
}

fn unsupported(
    slot: ProjectedProgramItemSlotV1,
    kind: ParserNormalSourcePlanUnsupportedKindV1,
) -> ParserNormalSourcePlanTopLevelRowV1 {
    ParserNormalSourcePlanTopLevelRowV1::Unsupported { slot, kind }
}
