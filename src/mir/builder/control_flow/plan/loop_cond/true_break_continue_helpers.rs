use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::is_true_literal;

pub(super) fn is_supported_nested_loop_condition(condition: &ASTNode) -> bool {
    if is_true_literal(condition) {
        return true;
    }
    is_supported_bool_expr_for_true_loop(condition)
}

fn is_supported_bool_expr_for_true_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::UnaryOp { operand, .. } => is_supported_bool_expr_for_true_loop(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr_for_true_loop(left)
                    && is_supported_bool_expr_for_true_loop(right)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr_for_true_loop(left)
                    && is_supported_value_expr_for_true_loop(right)
            }
            _ => false,
        },
        _ => false,
    }
}

fn is_supported_value_expr_for_true_loop(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_supported_value_expr_for_true_loop(operand),
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
            ) && is_supported_value_expr_for_true_loop(left)
                && is_supported_value_expr_for_true_loop(right)
        }
        _ => false,
    }
}
