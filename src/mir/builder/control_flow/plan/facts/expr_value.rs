//! SSOT: supported value expression checks for Facts.

use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};

pub(in crate::mir::builder) fn is_supported_value_expr(
    ast: &ASTNode,
    allow_extended: bool,
) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall { .. } => true,
        ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => allow_extended,
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus | UnaryOperator::BitNot,
            operand,
            ..
        } => is_supported_value_expr(operand, allow_extended),
        ASTNode::BinaryOp { operator, .. } => matches!(
            operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
        ),
        _ => false,
    }
}

pub(in crate::mir::builder) fn value_expr_requires_canon(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Modulo,
            ..
        } => true,
        ASTNode::BinaryOp { left, right, .. } => {
            value_expr_requires_canon(left) || value_expr_requires_canon(right)
        }
        ASTNode::UnaryOp { operand, .. } => value_expr_requires_canon(operand),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{is_supported_value_expr, value_expr_requires_canon};
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

    #[test]
    fn unary_minus_is_supported() {
        let expr = ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(v("x")),
            span: Span::unknown(),
        };
        assert!(is_supported_value_expr(&expr, false));
    }

    #[test]
    fn unary_not_is_not_value_expr() {
        let expr = ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(v("x")),
            span: Span::unknown(),
        };
        assert!(!is_supported_value_expr(&expr, false));
    }

    #[test]
    fn modulo_requires_canon() {
        let expr = ASTNode::BinaryOp {
            operator: BinaryOperator::Modulo,
            left: Box::new(v("i")),
            right: Box::new(lit_int(2)),
            span: Span::unknown(),
        };
        assert!(value_expr_requires_canon(&expr));
    }
}
