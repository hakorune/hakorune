//! Original scalar initializer evidence retained by the same source preflight.
//! No physical identity, runtime admission or second semantic issuer.
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedBinaryExpressionSourceV1, ResolvedInitializerRelationV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScalarInitializerOperationSourceV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    initializer: ResolvedInitializerRelationV1,
    binary: ResolvedBinaryExpressionSourceV1,
    positive_divisor: Option<i64>,
}

impl ScalarInitializerOperationSourceV1 {
    // Only the original scalar preflight may pass its successful Integer children.
    pub(super) fn retain(
        owner: FunctionOwnerIdV1,
        statement: SourceStmtSiteV1,
        initializer: ResolvedInitializerRelationV1,
        binary: ResolvedBinaryExpressionSourceV1,
        positive_divisor: Option<i64>,
    ) -> Self {
        Self {
            owner,
            statement,
            initializer,
            binary,
            positive_divisor,
        }
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }
    pub(crate) fn initializer(&self) -> &ResolvedInitializerRelationV1 {
        &self.initializer
    }
    pub(crate) fn binary(&self) -> &ResolvedBinaryExpressionSourceV1 {
        &self.binary
    }
    pub(crate) fn positive_divisor(&self) -> Option<i64> {
        self.positive_divisor
    }
}
