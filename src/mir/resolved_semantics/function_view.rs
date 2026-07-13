//! Borrowed canonical function syntax seam used before Lower decomposes it.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FunctionSyntaxViewV1<'a> {
    params: &'a [String],
    body: &'a [ASTNode],
    is_static: bool,
}

impl<'a> FunctionSyntaxViewV1<'a> {
    pub(crate) const fn new(params: &'a [String], body: &'a [ASTNode], is_static: bool) -> Self {
        Self {
            params,
            body,
            is_static,
        }
    }

    pub(crate) fn from_ast(function: &'a ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            params,
            body,
            is_static,
            ..
        } = function
        else {
            return None;
        };
        Some(Self::new(params, body, *is_static))
    }

    pub(crate) const fn params(self) -> &'a [String] {
        self.params
    }

    pub(crate) const fn body(self) -> &'a [ASTNode] {
        self.body
    }

    pub(crate) const fn is_static(self) -> bool {
        self.is_static
    }
}
