use crate::ast::ASTNode;

/// ============================================================
/// Group 5: loop_true_early_exit-specific helpers (NOT generalized)
/// ============================================================
///
/// **IMPORTANT**: These helpers are loop_true_early_exit-specific and intentionally NOT generalized.

/// Validate continue is at body end (loop_true_early_exit specific)
pub(crate) fn validate_continue_at_end(body: &[ASTNode]) -> bool {
    matches!(body.last(), Some(ASTNode::Continue { .. }))
}

/// Validate break is in simple if pattern (loop_true_early_exit specific)
pub(crate) fn validate_break_in_simple_if(body: &[ASTNode]) -> bool {
    for stmt in body {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            if then_body.len() == 1
                && matches!(then_body[0], ASTNode::Break { .. })
                && else_body.is_none()
            {
                return true;
            }
        }
    }
    false
}
