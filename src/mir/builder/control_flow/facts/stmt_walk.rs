use crate::ast::ASTNode;

/// Walk a statement list and flatten Program/ScopeBox wrappers.
///
/// This is analysis-only: it does not rewrite or mutate AST nodes.
/// The visitor returns `true` to stop traversal early.
pub(crate) fn walk_stmt_list<'a, F>(body: &'a [ASTNode], mut visit: F) -> bool
where
    F: FnMut(&'a ASTNode) -> bool,
{
    fn walk_node<'a, F>(node: &'a ASTNode, visit: &mut F) -> bool
    where
        F: FnMut(&'a ASTNode) -> bool,
    {
        match node {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    if walk_node(stmt, visit) {
                        return true;
                    }
                }
                false
            }
            ASTNode::ScopeBox { body, .. } => {
                for stmt in body {
                    if walk_node(stmt, visit) {
                        return true;
                    }
                }
                false
            }
            _ => visit(node),
        }
    }

    for stmt in body {
        if walk_node(stmt, &mut visit) {
            return true;
        }
    }
    false
}

pub(crate) fn flatten_stmt_list<'a>(body: &'a [ASTNode]) -> Vec<&'a ASTNode> {
    let mut out = Vec::new();
    walk_stmt_list(body, |stmt| {
        out.push(stmt);
        false
    });
    out
}

/// View-only helper: drop a trailing top-level `continue` (no AST rewrite).
pub(crate) fn strip_trailing_continue_view<'a>(body: &'a [ASTNode]) -> (&'a [ASTNode], bool) {
    match body.last() {
        Some(ASTNode::Continue { .. }) => (&body[..body.len().saturating_sub(1)], true),
        _ => (body, false),
    }
}
