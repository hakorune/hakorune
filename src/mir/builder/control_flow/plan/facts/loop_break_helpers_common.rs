//! Shared constructor and control-flow helpers for loop_break extraction.
//!
//! Kept as a tiny shared surface so the larger pattern-matching helpers can
//! stay focused on the actual detection logic.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

pub(in crate::mir::builder::control_flow::plan::facts) fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn add(
    left: ASTNode,
    right: ASTNode,
) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn lit_str(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn length_call(obj: &str) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(obj)),
        method: "length".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn index_of_call(
    haystack: &str,
    sep: &str,
    loop_var: &str,
) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "indexOf".to_string(),
        arguments: vec![lit_str(sep), var(loop_var)],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn index_of_call_expr(
    haystack: &str,
    needle: ASTNode,
) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "indexOf".to_string(),
        arguments: vec![needle],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn substring_call(
    haystack: &str,
    start: ASTNode,
    end: ASTNode,
) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(haystack)),
        method: "substring".to_string(),
        arguments: vec![start, end],
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn has_continue_statement(
    body: &[ASTNode],
) -> bool {
    use crate::mir::builder::control_flow::plan::extractors::common_helpers::has_continue_statement as common_has_continue;
    common_has_continue(body)
}

pub(in crate::mir::builder::control_flow::plan::facts) fn has_return_statement(
    body: &[ASTNode],
) -> bool {
    use crate::mir::builder::control_flow::plan::extractors::common_helpers::has_return_statement as common_has_return;
    common_has_return(body)
}
