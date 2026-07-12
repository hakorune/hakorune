use super::test_support::{with_joinir_env, with_strict_joinir_env};
use super::v1::try_extract_generic_loop_v1_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_i(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bin(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn method_call(receiver: &str, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(var(receiver)),
        method: method.to_string(),
        arguments,
        span: Span::unknown(),
    }
}

fn close_or_advance(cursor: &str) -> ASTNode {
    ASTNode::If {
        condition: Box::new(method_call("source", "is_close", vec![var(cursor)])),
        then_body: vec![
            assign(cursor, bin(BinaryOperator::Add, var(cursor), lit_i(1))),
            ASTNode::Break {
                span: Span::unknown(),
            },
        ],
        else_body: None,
        span: Span::unknown(),
    }
}

#[test]
fn delegate_style_cursor_records_no_accepted_progression_role() {
    with_strict_joinir_env(|| {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body = vec![
            close_or_advance("cursor"),
            assign("cursor", method_call("source", "scan", vec![var("cursor")])),
            method_call("source", "observe", vec![var("cursor")]),
        ];

        let facts = try_extract_generic_loop_v1_facts(&condition, &body)
            .expect("A0 structural fixture has no candidate-local compiler bug freeze");

        assert!(
            facts.is_none(),
            "rebased/post-update cursor has no accepted progression role before A1"
        );
    });
}

#[test]
fn scanner_style_cursor_records_no_condition_anchored_candidate() {
    with_joinir_env(None, None, || {
        let condition = bin(BinaryOperator::Less, var("bound"), lit_i(8));
        let body = vec![
            assign("cursor", method_call("source", "scan", vec![var("cursor")])),
            method_call("source", "observe", vec![var("cursor")]),
        ];

        let facts = try_extract_generic_loop_v1_facts(&condition, &body)
            .expect("A0 baseline rejection is not a compiler bug freeze");

        assert!(
            facts.is_none(),
            "body-only state cursor must not become an unrestricted candidate"
        );
    });
}
