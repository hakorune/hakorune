use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

pub(crate) fn span() -> Span {
    Span::unknown()
}

pub(crate) fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

pub(crate) fn lit_i(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

pub(crate) fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: op,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}
