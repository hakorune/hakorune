//! Condition prelude vocabulary SSOT.
//!
//! Purpose: keep the accepted statement vocabulary for `CondBlockView::prelude_stmts`
//! consistent across:
//! - Facts (accept-shape checks)
//! - Normalizer/lowering (actual effect lowering)
//!
//! Contract (v1):
//! - Prelude statements must be "stmt-only effects" (no control-flow exits).
//! - Exit checks are done separately via `ASTNode::contains_non_local_exit()`.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CondPreludeStmtKind {
    Local,
    Assignment,
    MethodCall,
    FunctionCall,
    Print,
}

pub(in crate::mir::builder) fn classify_cond_prelude_stmt(
    stmt: &ASTNode,
) -> Option<CondPreludeStmtKind> {
    match stmt {
        ASTNode::Local { .. } => Some(CondPreludeStmtKind::Local),
        ASTNode::Assignment { .. } => Some(CondPreludeStmtKind::Assignment),
        ASTNode::MethodCall { .. } => Some(CondPreludeStmtKind::MethodCall),
        ASTNode::FunctionCall { .. } => Some(CondPreludeStmtKind::FunctionCall),
        ASTNode::Print { .. } => Some(CondPreludeStmtKind::Print),
        _ => None,
    }
}

