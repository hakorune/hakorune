//! Analysis-only condition entry view (no rewrite).

use crate::ast::{ASTNode, Span};

/// Internal condition view: prelude + tail expression (evaluated once).
#[derive(Debug, Clone)]
pub(crate) struct CondBlockView {
    pub(crate) prelude_stmts: Vec<ASTNode>,
    pub(crate) tail_expr: ASTNode,
    pub(crate) span: Span,
}

impl CondBlockView {
    pub(crate) fn from_expr(expr: &ASTNode) -> Self {
        match expr {
            // B2-3: BlockExpr - extract prelude_stmts and tail_expr directly
            // Exit stmts in prelude are forbidden (checked at lower with [freeze:contract][blockexpr])
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                span,
            } => Self {
                prelude_stmts: prelude_stmts.clone(),
                tail_expr: (**tail_expr).clone(),
                span: *span,
            },
            // Other expressions: no prelude, expression is the tail
            _ => Self {
                prelude_stmts: Vec::new(),
                tail_expr: expr.clone(),
                span: expr.span(),
            },
        }
    }
}
