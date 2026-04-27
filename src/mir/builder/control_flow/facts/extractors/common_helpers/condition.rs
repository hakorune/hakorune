use crate::ast::{ASTNode, LiteralValue};

/// ============================================================
/// Group 3: Condition Validation (比較演算検証)
/// ============================================================

/// Check if condition is true literal
pub(crate) fn is_true_literal(condition: &ASTNode) -> bool {
    matches!(
        condition,
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    )
}
