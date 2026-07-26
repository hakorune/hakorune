//! Input vocabulary for the one Raw expression matcher.
//!
//! These carriers own syntax transport only. They do not own source sites,
//! Builder state, routes, result contracts, or type publication.

use crate::ast::ASTNode;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawLegacyExpressionInputV1(ASTNode);

pub(in crate::mir::builder) trait RawExpressionInputViewV1 {
    fn into_legacy_expression(self) -> ASTNode;
}

impl RawLegacyExpressionInputV1 {
    pub(in crate::mir::builder) fn new(node: ASTNode) -> Self {
        Self(node)
    }
}

impl RawExpressionInputViewV1 for RawLegacyExpressionInputV1 {
    fn into_legacy_expression(self) -> ASTNode {
        self.0
    }
}
