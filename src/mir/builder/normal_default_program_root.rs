//! Closed source-route owner for the normal/default lifecycle.
//!
//! A parser-backed callable source can only enter the one-shot root consumer.
//! Only the explicit compatibility branch retains AST access.

use crate::ast::ASTNode;
use crate::mir::normal_source_plan::NormalCallableCompatibilityOriginV1;
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::normal_root_execution::{
    PreparedNormalRootExecutionConsumptionV1, RejectedNormalRootExecutionConsumptionV1,
};
use super::NormalRootExecutionConsumerV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalDefaultProgramRootV1 {
    source: PreparedNormalDefaultProgramSourceV1,
    _seal: PreparedNormalDefaultProgramRootSealV1,
}

#[derive(Debug)]
struct PreparedNormalDefaultProgramRootSealV1;

#[derive(Debug)]
enum PreparedNormalDefaultProgramSourceV1 {
    Callable(VerifiedFinalCallableProgramSourceV1),
    TypedCompatibility(NormalCallableCompatibilityOriginV1),
    Compatibility(ASTNode),
}

#[derive(Debug)]
enum PreparedNormalDefaultCompatibilitySourceV1 {
    Typed(NormalCallableCompatibilityOriginV1),
    Ast(ASTNode),
}

#[derive(Debug)]
pub(super) struct PreparedNormalDefaultCompatibilityRootV1 {
    source: PreparedNormalDefaultCompatibilitySourceV1,
    _seal: PreparedNormalDefaultCompatibilityRootSealV1,
}

#[derive(Debug)]
struct PreparedNormalDefaultCompatibilityRootSealV1;

#[derive(Debug)]
pub(super) enum NormalDefaultProgramRootConsumptionV1 {
    SourceBacked(
        Result<PreparedNormalRootExecutionConsumptionV1, RejectedNormalRootExecutionConsumptionV1>,
    ),
    Compatibility(PreparedNormalDefaultCompatibilityRootV1),
}

#[derive(Debug)]
pub(super) enum RejectedNormalDefaultRootOwnerV1 {
    Compatibility(PreparedNormalDefaultCompatibilityRootV1),
    RootExecution(RejectedNormalRootExecutionConsumptionV1),
}

impl PreparedNormalDefaultProgramRootV1 {
    pub(in crate::mir) fn seal(ast: ASTNode) -> Result<Self, ASTNode> {
        if !matches!(ast, ASTNode::Program { .. }) {
            return Err(ast);
        }
        Ok(Self {
            source: PreparedNormalDefaultProgramSourceV1::Compatibility(ast),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        })
    }

    pub(in crate::mir) fn from_callable_source(
        source: VerifiedFinalCallableProgramSourceV1,
    ) -> Self {
        Self {
            source: PreparedNormalDefaultProgramSourceV1::Callable(source),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        }
    }

    pub(in crate::mir) fn from_compatibility_origin(
        origin: NormalCallableCompatibilityOriginV1,
    ) -> Self {
        Self {
            source: PreparedNormalDefaultProgramSourceV1::TypedCompatibility(origin),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        }
    }

    pub(super) fn consume_source_backed_root_once(self) -> NormalDefaultProgramRootConsumptionV1 {
        let Self { source, _seal } = self;
        match source {
            PreparedNormalDefaultProgramSourceV1::Callable(source) => {
                NormalDefaultProgramRootConsumptionV1::SourceBacked(
                    NormalRootExecutionConsumerV1::consume_once(source),
                )
            }
            PreparedNormalDefaultProgramSourceV1::TypedCompatibility(origin) => {
                NormalDefaultProgramRootConsumptionV1::Compatibility(
                    PreparedNormalDefaultCompatibilityRootV1 {
                        source: PreparedNormalDefaultCompatibilitySourceV1::Typed(origin),
                        _seal: PreparedNormalDefaultCompatibilityRootSealV1,
                    },
                )
            }
            PreparedNormalDefaultProgramSourceV1::Compatibility(ast) => {
                NormalDefaultProgramRootConsumptionV1::Compatibility(
                    PreparedNormalDefaultCompatibilityRootV1 {
                        source: PreparedNormalDefaultCompatibilitySourceV1::Ast(ast),
                        _seal: PreparedNormalDefaultCompatibilityRootSealV1,
                    },
                )
            }
        }
    }

    #[cfg(test)]
    pub(super) fn source_ast(&self) -> &ASTNode {
        match &self.source {
            PreparedNormalDefaultProgramSourceV1::Callable(_) => {
                panic!("source-backed root must enter the one-shot lifecycle facade")
            }
            PreparedNormalDefaultProgramSourceV1::TypedCompatibility(origin) => origin.ast(),
            PreparedNormalDefaultProgramSourceV1::Compatibility(ast) => ast,
        }
    }

    #[cfg(test)]
    pub(in crate::mir) fn is_callable_source_backed(&self) -> bool {
        matches!(
            &self.source,
            PreparedNormalDefaultProgramSourceV1::Callable(_)
        )
    }

    #[cfg(test)]
    pub(in crate::mir) fn is_typed_compatibility(&self) -> bool {
        matches!(
            &self.source,
            PreparedNormalDefaultProgramSourceV1::TypedCompatibility(_)
        )
    }
}

impl PreparedNormalDefaultCompatibilityRootV1 {
    fn source_ast(&self) -> &ASTNode {
        match &self.source {
            PreparedNormalDefaultCompatibilitySourceV1::Typed(origin) => origin.ast(),
            PreparedNormalDefaultCompatibilitySourceV1::Ast(ast) => ast,
        }
    }

    fn discard_at_named_compatibility_terminal(self) {
        let Self { source, _seal } = self;
        drop(source);
    }
}

impl RejectedNormalDefaultRootOwnerV1 {
    pub(super) fn source_ast(&self) -> &ASTNode {
        match self {
            Self::Compatibility(source) => source.source_ast(),
            Self::RootExecution(_) => {
                panic!("root-execution rejection does not expose source AST")
            }
        }
    }

    pub(super) fn discard_at_named_lifecycle_terminal(self) {
        match self {
            Self::Compatibility(source) => source.discard_at_named_compatibility_terminal(),
            Self::RootExecution(source) => source.discard_at_named_root_execution_terminal(),
        }
    }
}
