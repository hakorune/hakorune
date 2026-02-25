//! Policy SSOT for return-prelude statement vocabulary.

use crate::ast::ASTNode;

/// Return-prelude statement vocabulary (container lowering).
///
/// Notes:
/// - `Print` is gated by `allow_extended` to match the existing "extended" vocab split.
/// - Today, callers pass `allow_extended=true` (no behavior change); false is for future strict paths.
/// - `If` is accepted as a plain stmt; branch lowering happens in Parts.
pub fn return_prelude_stmt_is_allowed(stmt: &ASTNode, allow_extended: bool) -> bool {
    match stmt {
        ASTNode::Assignment { .. }
        | ASTNode::Local { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Call { .. }
        | ASTNode::If { .. } => true,
        ASTNode::Print { .. } => allow_extended,
        _ => false,
    }
}

/// Else-only return prelude vocabulary (loop_cond_break_continue lowering).
pub fn else_only_return_prelude_stmt_is_allowed(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::Local { .. } | ASTNode::Print { .. })
}

/// Then-only return prelude vocabulary (loop_cond_break_continue lowering).
pub fn then_only_return_prelude_stmt_is_allowed(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::Local { .. })
}
