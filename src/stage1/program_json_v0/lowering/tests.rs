use super::program_json_v0_from_body;
use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};
use serde_json::json;

fn float_lit(value: f64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Float(value),
        span: Span::unknown(),
    }
}

#[test]
fn program_json_v0_from_body_preserves_float_return_literal() {
    let body = vec![ASTNode::Return {
        value: Some(Box::new(float_lit(2.5))),
        span: Span::unknown(),
    }];

    let program = program_json_v0_from_body(&body).expect("float return literal should lower");

    assert_eq!(
        program,
        json!({
            "version": 0,
            "kind": "Program",
            "body": [{
                "type": "Return",
                "expr": {
                    "type": "Float",
                    "value": 2.5
                }
            }],
        })
    );
}

#[test]
fn program_json_v0_from_body_preserves_negative_float_return_literal() {
    let body = vec![ASTNode::Return {
        value: Some(Box::new(ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(float_lit(1.25)),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }];

    let program =
        program_json_v0_from_body(&body).expect("negative float return literal should lower");

    assert_eq!(
        program,
        json!({
            "version": 0,
            "kind": "Program",
            "body": [{
                "type": "Return",
                "expr": {
                    "type": "Float",
                    "value": -1.25
                }
            }],
        })
    );
}
