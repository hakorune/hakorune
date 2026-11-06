//! generic_loop 専用 expr 判定 helpers (SSOT)
//!
//! generic_loop の Facts 判定で使う bool/value expr 検証ロジック。

use crate::ast::{ASTNode, BinaryOperator};

/// generic_loop 専用: pure value expr 検証
pub(in crate::mir::builder) fn is_pure_value_expr_for_generic_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let Some(else_body) = else_body else {
                return false;
            };
            if then_body.len() != 1 || else_body.len() != 1 {
                return false;
            }
            is_pure_value_expr_for_generic_loop(condition)
                && is_pure_value_expr_for_generic_loop(&then_body[0])
                && is_pure_value_expr_for_generic_loop(&else_body[0])
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => prelude_stmts.is_empty() && is_pure_value_expr_for_generic_loop(tail_expr),
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr_for_generic_loop(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr_for_generic_loop(left)
                && is_pure_value_expr_for_generic_loop(right)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_pure_value_expr_for_generic_loop;
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn pure_value_expr_allows_empty_blockexpr_wrapper() {
        let wrapped = ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(int_lit(42)),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr_for_generic_loop(&wrapped));
    }

    #[test]
    fn pure_value_expr_rejects_nonempty_blockexpr_wrapper() {
        let wrapped = ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Local {
                variables: vec!["x".to_string()],
                initial_values: vec![Some(Box::new(int_lit(1)))],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            }],
            tail_expr: Box::new(int_lit(42)),
            span: Span::unknown(),
        };
        assert!(!is_pure_value_expr_for_generic_loop(&wrapped));
    }
}
