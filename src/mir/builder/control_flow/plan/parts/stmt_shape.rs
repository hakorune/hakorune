//! Statement shape predicates for return-prelude lowering.
//!
//! Keep these helpers analysis-only. They classify AST shape for `stmt.rs`;
//! they must not lower, mutate builder state, or choose PHI bindings.

use crate::ast::ASTNode;

pub(super) fn tail_is_exit(body: &[ASTNode]) -> bool {
    matches!(
        body.last(),
        Some(ASTNode::Return { .. } | ASTNode::Break { .. } | ASTNode::Continue { .. })
    )
}

pub(super) fn value_has_blockexpr_prelude_loop(value: &ASTNode) -> bool {
    let ASTNode::BlockExpr { prelude_stmts, .. } = value else {
        return false;
    };
    prelude_stmts.iter().any(stmt_has_loop_stmt_recursive)
}

pub(super) fn stmt_has_loop_stmt_recursive(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_loop_stmt_recursive)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(stmt_has_loop_stmt_recursive))
        }
        ASTNode::Program { statements, .. } => statements.iter().any(stmt_has_loop_stmt_recursive),
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_loop_stmt_recursive),
        _ => false,
    }
}
