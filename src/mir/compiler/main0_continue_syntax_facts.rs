//! Neutral Main0 facts still borrowed by the derived-predicate source owner.
use super::callable_single_loop_source_shapes::{
    SourceExprShapeV1, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0InitialLocalFactV1 {
    statement_site: SourceStmtSiteV1,
    initializer_site: SourceExprSiteV1,
    shape: SourceLiteralShapeV1,
}

impl Main0InitialLocalFactV1 {
    /// Construct one row for a sibling Main0 profile observer. The shape
    /// fields are the same neutral source sites the continue observer emits.
    pub(crate) const fn from_parts(
        statement_site: SourceStmtSiteV1,
        initializer_site: SourceExprSiteV1,
        shape: SourceLiteralShapeV1,
    ) -> Self {
        Self {
            statement_site,
            initializer_site,
            shape,
        }
    }

    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn initializer_site(&self) -> &SourceExprSiteV1 {
        &self.initializer_site
    }

    pub(crate) fn shape(&self) -> &SourceLiteralShapeV1 {
        &self.shape
    }
}

/// One `target = lhs + rhs` assignment whose target and left operand are
/// variable references and whose right operand is an integer literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0StepFactsV1 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    target_shape: SourceExprShapeV1,
    value_site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    lhs_shape: SourceExprShapeV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceLiteralShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl Main0StepFactsV1 {
    /// Construct one row for a sibling Main0 profile observer.
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn from_parts(
        statement_site: SourceStmtSiteV1,
        target_site: SourceExprSiteV1,
        target_shape: SourceExprShapeV1,
        value_site: SourceExprSiteV1,
        lhs_site: SourceExprSiteV1,
        lhs_shape: SourceExprShapeV1,
        rhs_site: SourceExprSiteV1,
        rhs_shape: SourceLiteralShapeV1,
        operator: SyntaxBinaryOperatorV1,
    ) -> Self {
        Self {
            statement_site,
            target_site,
            target_shape,
            value_site,
            lhs_site,
            lhs_shape,
            rhs_site,
            rhs_shape,
            operator,
        }
    }

    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn lhs_site(&self) -> &SourceExprSiteV1 {
        &self.lhs_site
    }

    pub(crate) fn rhs_site(&self) -> &SourceExprSiteV1 {
        &self.rhs_site
    }

    pub(crate) fn rhs_shape(&self) -> &SourceLiteralShapeV1 {
        &self.rhs_shape
    }

    pub(crate) const fn operator(&self) -> SyntaxBinaryOperatorV1 {
        self.operator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0TailReturnFactV1 {
    statement_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value_shape: SourceExprShapeV1,
}

impl Main0TailReturnFactV1 {
    /// Construct one row for a sibling Main0 profile observer.
    pub(crate) const fn from_parts(
        statement_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        value_shape: SourceExprShapeV1,
    ) -> Self {
        Self {
            statement_site,
            value_site,
            value_shape,
        }
    }

    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
}
