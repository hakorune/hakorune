//! SSOT: supported boolean expression checks for Facts.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::control_flow::plan::canon::cond::canon_bool_or_condition;
use crate::mir::builder::control_flow::plan::policies::cond_prelude_vocab::classify_cond_prelude_stmt;
use super::expr_value::{is_supported_value_expr, value_expr_requires_canon};

pub(in crate::mir::builder) fn is_supported_bool_expr_with_canon(
    ast: &ASTNode,
    allow_extended: bool,
) -> bool {
    if let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = ast
    {
        if prelude_stmts
            .iter()
            .any(|stmt| stmt.contains_non_local_exit())
        {
            return false;
        }
        if prelude_stmts
            .iter()
            .any(|stmt| classify_cond_prelude_stmt(stmt).is_none())
        {
            return false;
        }
        return is_supported_bool_expr_with_canon(tail_expr, allow_extended);
    }

    if !allow_extended {
        return is_supported_bool_expr(ast, allow_extended);
    }
    if bool_expr_requires_canon(ast) {
        return canon_bool_or_condition(ast).is_some();
    }
    is_supported_bool_expr(ast, allow_extended)
}

fn bool_expr_requires_canon(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::BlockExpr { tail_expr, .. } => bool_expr_requires_canon(tail_expr),
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => bool_expr_requires_canon(operand),
        ASTNode::BinaryOp {
            operator: BinaryOperator::And | BinaryOperator::Or,
            left,
            right,
            ..
        } => bool_expr_requires_canon(left) || bool_expr_requires_canon(right),
        ASTNode::BinaryOp {
            operator:
                BinaryOperator::Less
                | BinaryOperator::LessEqual
                | BinaryOperator::Greater
                | BinaryOperator::GreaterEqual
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual,
            left,
            right,
            ..
        } => value_expr_requires_canon(left) || value_expr_requires_canon(right),
        _ => false,
    }
}

fn is_supported_bool_expr(ast: &ASTNode, allow_extended: bool) -> bool {
    match ast {
        ASTNode::BlockExpr { tail_expr, .. } => is_supported_bool_expr(tail_expr, allow_extended),
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => allow_extended,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => is_supported_bool_expr(operand, allow_extended),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr(left, allow_extended)
                    && is_supported_bool_expr(right, allow_extended)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr(left, allow_extended)
                    && is_supported_value_expr(right, allow_extended)
            }
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_supported_bool_expr_with_canon;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn lit_bool(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn modulo_requires_canon() {
        let expr = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Modulo,
                left: Box::new(v("i")),
                right: Box::new(lit_int(2)),
                span: Span::unknown(),
            }),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        assert!(!is_supported_bool_expr_with_canon(&expr, true));
    }

    #[test]
    fn and_or_nesting_is_supported() {
        let expr = ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(v("a")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Or,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::NotEqual,
                    left: Box::new(v("b")),
                    right: Box::new(lit_int(0)),
                    span: Span::unknown(),
                }),
                right: Box::new(lit_bool(true)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        assert!(is_supported_bool_expr_with_canon(&expr, false));
    }

    #[test]
    fn comparison_allows_unary_value_expr() {
        let expr = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(v("x")),
                span: Span::unknown(),
            }),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        assert!(is_supported_bool_expr_with_canon(&expr, false));
    }
}
