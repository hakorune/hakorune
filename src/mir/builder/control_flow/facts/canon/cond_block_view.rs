//! Analysis-only condition entry view (no rewrite).

use crate::ast::{ASTNode, Span};

/// Internal condition view: prelude + tail expression (evaluated once).
#[derive(Debug, Clone)]
pub(crate) struct CondBlockView {
    pub(crate) prelude_stmts: Vec<ASTNode>,
    pub(crate) tail_expr: ASTNode,
    #[allow(dead_code)]
    pub(crate) span: Span,
}

impl CondBlockView {
    pub(crate) fn from_expr(expr: &ASTNode) -> Self {
        match expr {
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                span,
            } => Self {
                prelude_stmts: prelude_stmts.clone(),
                tail_expr: (**tail_expr).clone(),
                span: *span,
            },
            _ => Self {
                prelude_stmts: Vec::new(),
                tail_expr: expr.clone(),
                span: expr.span(),
            },
        }
    }
}
