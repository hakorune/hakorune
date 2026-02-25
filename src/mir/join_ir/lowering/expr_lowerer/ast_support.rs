use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};

pub(crate) fn is_supported_condition(ast: &ASTNode) -> bool {
    use ASTNode::*;
    match ast {
        Literal { .. } => true,
        Variable { .. } => true,
        BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            is_supported_binary_op(operator)
                && is_supported_condition(left)
                && is_supported_condition(right)
        }
        UnaryOp {
            operator, operand, ..
        } => matches!(operator, UnaryOperator::Not) && is_supported_condition(operand),
        MethodCall { .. } => is_supported_method_call(ast),
        _ => false,
    }
}

fn is_supported_binary_op(op: &BinaryOperator) -> bool {
    use BinaryOperator::*;
    matches!(
        op,
        Less | LessEqual | Greater | GreaterEqual | Equal | NotEqual | And | Or
    )
}

fn is_supported_method_call(ast: &ASTNode) -> bool {
    // Phase 224-C: Accept all MethodCall nodes for syntax support.
    // Validation of method names and signatures happens during lowering in MethodCallLowerer.
    // This allows is_supported_condition() to return true for any MethodCall,
    // and fail-fast validation occurs in ExprLowerer::lower() -> MethodCallLowerer.
    matches!(ast, ASTNode::MethodCall { .. })
}
