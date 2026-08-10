use crate::ast::ASTNode;

use super::super::callable_source_anchor::PreparedCallableSourceV1;
use super::super::initial_callable_program_source::{
    InitialCallableFinalSlotV1, VerifiedInitialCallableProgramSourceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableParserCompatibilityV1 {
    InterfaceBox,
    RecordBox,
    MixedProgram,
    TopLevelBuildGate,
    NoBoxDeclarations,
    NonProgram,
    UnsupportedCallableSource,
}

#[derive(Debug)]
pub(crate) enum ParsedNormalCallableProgramV1 {
    SourceBacked(VerifiedInitialCallableProgramSourceV1),
    Compatibility {
        ast: ASTNode,
        cohort: NormalCallableParserCompatibilityV1,
    },
}

impl ParsedNormalCallableProgramV1 {
    pub(crate) fn ast(&self) -> &ASTNode {
        match self {
            Self::SourceBacked(source) => source.ast(),
            Self::Compatibility { ast, .. } => ast,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedFinalCallableProgramSourceV1 {
    ast: ASTNode,
    sources: Box<[PreparedCallableSourceV1]>,
    slots: Box<[InitialCallableFinalSlotV1]>,
    _lineage: ExactCallablePreservingTransformReceiptV1,
}

#[derive(Debug)]
struct ExactCallablePreservingTransformReceiptV1;

impl VerifiedFinalCallableProgramSourceV1 {
    pub(super) fn issue(
        ast: ASTNode,
        sources: Box<[PreparedCallableSourceV1]>,
        slots: Box<[InitialCallableFinalSlotV1]>,
    ) -> Self {
        Self {
            ast,
            sources,
            slots,
            _lineage: ExactCallablePreservingTransformReceiptV1,
        }
    }

    pub(crate) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(in crate::parser) fn callable_count(&self) -> usize {
        debug_assert_eq!(self.sources.len(), self.slots.len());
        self.sources.len()
    }
}
