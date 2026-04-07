use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

pub(super) fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

pub(super) fn eq_str(left: ASTNode, s: &str) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(left),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

pub(super) fn eq_int(left: ASTNode, n: i64) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(left),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}
