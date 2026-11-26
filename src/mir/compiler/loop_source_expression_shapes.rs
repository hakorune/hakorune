//! As-written operator and literal shapes shared by typed loop producers.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::located::LocatedExprV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SyntaxBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLiteralShapeV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    Other,
}

pub(crate) fn binary_operator_shape(operator: &BinaryOperator) -> SyntaxBinaryOperatorV1 {
    match operator {
        BinaryOperator::Add => SyntaxBinaryOperatorV1::Add,
        BinaryOperator::Subtract => SyntaxBinaryOperatorV1::Subtract,
        BinaryOperator::Multiply => SyntaxBinaryOperatorV1::Multiply,
        BinaryOperator::Divide => SyntaxBinaryOperatorV1::Divide,
        BinaryOperator::Modulo => SyntaxBinaryOperatorV1::Modulo,
        BinaryOperator::BitAnd => SyntaxBinaryOperatorV1::BitAnd,
        BinaryOperator::BitOr => SyntaxBinaryOperatorV1::BitOr,
        BinaryOperator::BitXor => SyntaxBinaryOperatorV1::BitXor,
        BinaryOperator::Shl => SyntaxBinaryOperatorV1::Shl,
        BinaryOperator::Shr => SyntaxBinaryOperatorV1::Shr,
        BinaryOperator::Equal => SyntaxBinaryOperatorV1::Equal,
        BinaryOperator::NotEqual => SyntaxBinaryOperatorV1::NotEqual,
        BinaryOperator::Less => SyntaxBinaryOperatorV1::Less,
        BinaryOperator::Greater => SyntaxBinaryOperatorV1::Greater,
        BinaryOperator::LessEqual => SyntaxBinaryOperatorV1::LessEqual,
        BinaryOperator::GreaterEqual => SyntaxBinaryOperatorV1::GreaterEqual,
        BinaryOperator::And => SyntaxBinaryOperatorV1::And,
        BinaryOperator::Or => SyntaxBinaryOperatorV1::Or,
    }
}

pub(crate) fn literal_shape(value: &LiteralValue) -> SourceLiteralShapeV1 {
    match value {
        LiteralValue::Integer(value) => SourceLiteralShapeV1::Integer(*value),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => SourceLiteralShapeV1::TypedInteger {
            value: *value,
            declared_type_name: declared_type_name.clone().into_boxed_str(),
        },
        _ => SourceLiteralShapeV1::Other,
    }
}

pub(crate) fn literal_shape_from_expr(expr: &LocatedExprV1<'_>) -> Option<SourceLiteralShapeV1> {
    match expr.node() {
        ASTNode::Literal { value, .. } => Some(literal_shape(value)),
        _ => None,
    }
}
