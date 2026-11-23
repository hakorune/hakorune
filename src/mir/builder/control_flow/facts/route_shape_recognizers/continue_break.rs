//! Continue and break detection within the current loop body.

use super::super::stmt_walk::walk_stmt_list;
use crate::ast::ASTNode;

/// Detect if a loop body contains continue statements
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `true` if at least one continue statement is found in the body or nested structures
///
/// # Notes
///
/// Nested loops are intentionally ignored for break/continue detection so the
/// outer loop isn't misclassified by inner-loop control flow.
pub(crate) fn detect_continue_in_body(body: &[ASTNode]) -> bool {
    walk_stmt_list(body, |stmt| has_continue_node(stmt))
}

/// Detect if a loop body contains break statements
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `true` if at least one break statement is found in the body or nested structures
pub(crate) fn detect_break_in_body(body: &[ASTNode]) -> bool {
    walk_stmt_list(body, |stmt| has_break_node(stmt))
}

/// Recursive helper to check if AST node contains continue
pub(super) fn has_continue_node(node: &ASTNode) -> bool {
    match node {
        ASTNode::Continue { .. } => true,
        ASTNode::Program { statements, .. } => {
            walk_stmt_list(statements, |stmt| has_continue_node(stmt))
        }
        ASTNode::ScopeBox { body, .. } => walk_stmt_list(body, |stmt| has_continue_node(stmt)),
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            walk_stmt_list(then_body, |stmt| has_continue_node(stmt))
                || else_body
                    .as_ref()
                    .map_or(false, |e| walk_stmt_list(e, |stmt| has_continue_node(stmt)))
        }
        ASTNode::Loop { .. } => false,
        _ => false,
    }
}

/// Recursive helper to check if AST node contains break
fn has_break_node(node: &ASTNode) -> bool {
    match node {
        ASTNode::Break { .. } => true,
        ASTNode::Program { statements, .. } => {
            walk_stmt_list(statements, |stmt| has_break_node(stmt))
        }
        ASTNode::ScopeBox { body, .. } => walk_stmt_list(body, |stmt| has_break_node(stmt)),
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            walk_stmt_list(then_body, |stmt| has_break_node(stmt))
                || else_body
                    .as_ref()
                    .map_or(false, |e| walk_stmt_list(e, |stmt| has_break_node(stmt)))
        }
        ASTNode::Loop { .. } => false,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_continue_simple() {
        let continue_node = ASTNode::Continue {
            span: crate::ast::Span::unknown(),
        };
        assert!(has_continue_node(&continue_node));
    }

    #[test]
    fn test_detect_break_simple() {
        let break_node = ASTNode::Break {
            span: crate::ast::Span::unknown(),
        };
        assert!(has_break_node(&break_node));
    }

    #[test]
    fn test_empty_body() {
        let empty: Vec<ASTNode> = vec![];
        assert!(!detect_continue_in_body(&empty));
        assert!(!detect_break_in_body(&empty));
    }
}
