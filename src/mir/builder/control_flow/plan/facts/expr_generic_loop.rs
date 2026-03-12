//! generic_loop 専用 expr 判定 helpers (SSOT)
//!
//! generic_loop の Facts 判定で使う bool/value expr 検証ロジック。

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// generic_loop 専用: value expr 検証
pub(in crate::mir::builder) fn is_supported_value_expr_for_generic_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. }
        | ASTNode::Literal { .. }
        | ASTNode::This { .. }
        | ASTNode::Me { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. } => true,
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => true,
        ASTNode::FromCall { .. } => true,
        ASTNode::New { .. } => true,
        ASTNode::FieldAccess { object, .. } => is_supported_value_expr_for_generic_loop(object),
        ASTNode::Index { target, index, .. } => {
            is_supported_value_expr_for_generic_loop(target)
                && is_supported_value_expr_for_generic_loop(index)
        }
        ASTNode::UnaryOp { operand, .. } => is_supported_value_expr_for_generic_loop(operand),
        ASTNode::GroupedAssignmentExpr { rhs, .. } => is_supported_value_expr_for_generic_loop(rhs),
        ASTNode::AwaitExpression { expression, .. } => {
            is_supported_value_expr_for_generic_loop(expression)
        }
        ASTNode::QMarkPropagate { expression, .. } => {
            is_supported_value_expr_for_generic_loop(expression)
        }
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
            ) && is_supported_value_expr_for_generic_loop(left)
                && is_supported_value_expr_for_generic_loop(right)
        }
        ASTNode::If { .. } => is_pure_value_expr_for_generic_loop(ast),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => {
            is_supported_value_expr_for_generic_loop(scrutinee)
                && arms
                    .iter()
                    .all(|(_, expr)| is_supported_value_expr_for_generic_loop(expr))
                && is_supported_value_expr_for_generic_loop(else_expr)
        }
        ASTNode::ArrayLiteral { elements, .. } => elements
            .iter()
            .all(is_supported_value_expr_for_generic_loop),
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .all(|(_, value)| is_supported_value_expr_for_generic_loop(value)),
        _ => false,
    }
}

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
                span: Span::unknown(),
            }],
            tail_expr: Box::new(int_lit(42)),
            span: Span::unknown(),
        };
        assert!(!is_pure_value_expr_for_generic_loop(&wrapped));
    }
}

/// generic_loop 専用: bool expr 検証
pub(in crate::mir::builder) fn is_supported_bool_expr_for_generic_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr_for_generic_loop(left)
                    && is_supported_bool_expr_for_generic_loop(right)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr_for_generic_loop(left)
                    && is_supported_value_expr_for_generic_loop(right)
            }
            _ => false,
        },
        _ => false,
    }
}
