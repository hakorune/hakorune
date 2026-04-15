use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// ============================================================
/// Group 3: Condition Validation (比較演算検証)
/// ============================================================

/// Validate condition: 比較演算 (左辺が変数)
pub(crate) fn extract_loop_variable(condition: &ASTNode) -> Option<String> {
    match condition {
        ASTNode::BinaryOp { operator, left, .. } => {
            if !matches!(
                operator,
                BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) {
                return None;
            }

            if let ASTNode::Variable { name, .. } = left.as_ref() {
                return Some(name.clone());
            }

            None
        }
        _ => None,
    }
}

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
