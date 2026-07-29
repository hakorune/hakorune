//! Input vocabulary for Raw body and statement ingress.
//!
//! These carriers own syntax transport only. They do not own source sites,
//! Builder state, routes, result contracts, or type publication.

use crate::ast::ASTNode;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawLegacyBodyInputV1(Vec<ASTNode>);

#[derive(Debug)]
pub(in crate::mir::builder) struct RawLegacyStatementInputV1(ASTNode);

pub(in crate::mir::builder) trait RawBodyInputViewV1 {
    fn into_legacy_body(self) -> Vec<ASTNode>;
}

pub(in crate::mir::builder) trait RawStatementInputViewV1 {
    fn into_legacy_statement(self) -> ASTNode;
}

impl RawLegacyBodyInputV1 {
    pub(in crate::mir::builder) fn new(nodes: Vec<ASTNode>) -> Self {
        Self(nodes)
    }
}

impl RawLegacyStatementInputV1 {
    pub(in crate::mir::builder) fn new(node: ASTNode) -> Self {
        Self(node)
    }
}

impl RawBodyInputViewV1 for RawLegacyBodyInputV1 {
    fn into_legacy_body(self) -> Vec<ASTNode> {
        self.0
    }
}

impl RawStatementInputViewV1 for RawLegacyStatementInputV1 {
    fn into_legacy_statement(self) -> ASTNode {
        self.0
    }
}
