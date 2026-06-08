use super::*;
use crate::ast::{BinaryOperator, LiteralValue};

mod branch;
mod memops;
mod region;

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn int_lit(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn bool_lit(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: span(),
    }
}

fn mem_addr(arg: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: "mem.addr".to_string(),
        arguments: vec![arg],
        span: span(),
    }
}

fn bin(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

fn index(target: ASTNode, idx: ASTNode) -> ASTNode {
    ASTNode::Index {
        target: Box::new(target),
        index: Box::new(idx),
        span: span(),
    }
}

fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.to_string(),
        span: span(),
    }
}

fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: span(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: Vec::new(),
        span: span(),
    }
}

fn local_no_init(name: &str) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![None],
        declared_type_names: Vec::new(),
        span: span(),
    }
}
