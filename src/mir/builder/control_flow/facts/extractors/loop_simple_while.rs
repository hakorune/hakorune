//! Loop simple-while condition helper.
//!
//! The original extractor shelf was retired during compiler-cleanliness cleanup.
//! This file now keeps only the shared condition-shape helper reused by
//! `if_phi_join`.

use crate::ast::{ASTNode, BinaryOperator};

/// Validate condition: 比較演算 (左辺が変数)
///
/// Exported for reuse by if_phi_join extractor.
pub(crate) fn validate_loop_condition_structure(condition: &ASTNode) -> Option<String> {
    match condition {
        ASTNode::BinaryOp { operator, left, .. } => {
            // 比較演算子チェック
            if !matches!(
                operator,
                BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) {
                return None; // 比較でない（算術演算など）
            }

            // 左辺が変数であることを確認
            if let ASTNode::Variable { name, .. } = left.as_ref() {
                return Some(name.clone());
            }

            None // 左辺が変数でない
        }
        _ => None, // 比較演算でない
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    #[test]
    fn validate_loop_condition_structure_accepts_comparison_with_variable_lhs() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        assert_eq!(
            validate_loop_condition_structure(&condition),
            Some("i".to_string())
        );
    }

    #[test]
    fn validate_loop_condition_structure_rejects_non_comparison() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        assert_eq!(validate_loop_condition_structure(&condition), None);
    }

    #[test]
    fn validate_loop_condition_structure_rejects_non_variable_lhs() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        assert_eq!(validate_loop_condition_structure(&condition), None);
    }
}
