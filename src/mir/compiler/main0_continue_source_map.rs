//! Neutral source site/target rows retained for the derived-predicate map.
use super::callable_single_loop_source_shapes::{SourceLiteralShapeV1, SyntaxBinaryOperatorV1};
use crate::mir::resolved_semantics::{BindingRefV1, RegionId, SourceExprSiteV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Main0ContinueMapSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

impl Main0ContinueMapSiteV1 {
    pub(crate) fn expression(&self) -> Option<&SourceExprSiteV1> {
        match self {
            Self::Expression(site) => Some(site),
            Self::Statement(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Main0ContinueMapTargetV1 {
    LocalDeclaration {
        binding: BindingRefV1,
        literal: SourceLiteralShapeV1,
    },
    Binding(BindingRefV1),
    Literal(SourceLiteralShapeV1),
    Operator(SyntaxBinaryOperatorV1),
    Continue {
        target_loop: RegionId,
    },
    Tail {
        statement: SourceStmtSiteV1,
        binding: BindingRefV1,
    },
}

impl Main0ContinueMapTargetV1 {
    pub(crate) fn local_declaration(&self) -> Option<(BindingRefV1, &SourceLiteralShapeV1)> {
        match self {
            Self::LocalDeclaration { binding, literal } => Some((*binding, literal)),
            _ => None,
        }
    }

    pub(crate) fn literal(&self) -> Option<&SourceLiteralShapeV1> {
        match self {
            Self::Literal(literal) => Some(literal),
            Self::LocalDeclaration { literal, .. } => Some(literal),
            _ => None,
        }
    }

    pub(crate) fn continue_target(&self) -> Option<RegionId> {
        match self {
            Self::Continue { target_loop } => Some(*target_loop),
            _ => None,
        }
    }

    pub(crate) fn tail(&self) -> Option<(&SourceStmtSiteV1, BindingRefV1)> {
        match self {
            Self::Tail { statement, binding } => Some((statement, *binding)),
            _ => None,
        }
    }
}
