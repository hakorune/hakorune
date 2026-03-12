//! Phase 29ai P11: Tests for loop_break facts extraction.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

use super::loop_break_core::try_extract_loop_break_facts;

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

fn lit_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn lit_str(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(value))],
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(v(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn method_call(obj: &str, method: &str, args: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(v(obj)),
        method: method.to_string(),
        arguments: args,
        span: Span::unknown(),
    }
}

fn this_method_call(method: &str, args: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(ASTNode::This {
            span: Span::unknown(),
        }),
        method: method.to_string(),
        arguments: args,
        span: Span::unknown(),
    }
}

fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

#[test]
fn extract_loop_break_parse_integer_subset() {
    let condition = binop(
        BinaryOperator::Less,
        v("i"),
        method_call("s", "length", vec![]),
    );
    let body = vec![
        local(
            "ch",
            method_call(
                "s",
                "substring",
                vec![v("i"), binop(BinaryOperator::Add, v("i"), lit_int(1))],
            ),
        ),
        ASTNode::Local {
            variables: vec!["d".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::This {
                    span: Span::unknown(),
                }),
                method: "index_of".to_string(),
                arguments: vec![v("digits"), v("ch")],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Less, v("d"), lit_int(0))),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign(
            "acc",
            binop(
                BinaryOperator::Add,
                binop(BinaryOperator::Multiply, v("acc"), lit_int(10)),
                v("d"),
            ),
        ),
        assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "i");
    assert_eq!(facts.carrier_var, "acc");
    assert!(matches!(
        facts.break_condition,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            ..
        }
    ));
}

#[test]
fn extract_loop_break_realworld_subset() {
    let condition = lit_bool(true);
    let body = vec![
        local(
            "j",
            method_call("table", "indexOf", vec![lit_str("|||"), v("i")]),
        ),
        local("seg", lit_str("")),
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::GreaterEqual, v("j"), lit_int(0))),
            then_body: vec![assign(
                "seg",
                method_call("table", "substring", vec![v("i"), v("j")]),
            )],
            else_body: Some(vec![assign(
                "seg",
                method_call(
                    "table",
                    "substring",
                    vec![v("i"), method_call("table", "length", vec![])],
                ),
            )]),
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, v("seg"), lit_str(""))),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("i", binop(BinaryOperator::Add, v("j"), lit_int(3))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "i");
    assert_eq!(facts.carrier_var, "i");

    match facts.loop_condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => {
            assert!(matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "i"));
            assert!(matches!(
                right.as_ref(),
                ASTNode::MethodCall { method, .. } if method == "length"
            ));
        }
        other => panic!("unexpected loop_condition: {:?}", other),
    }

    match facts.break_condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            assert!(matches!(
                right.as_ref(),
                ASTNode::Literal { value: LiteralValue::String(value), .. } if value.is_empty()
            ));
            assert!(matches!(
                left.as_ref(),
                ASTNode::MethodCall { method, .. } if method == "substring"
            ));
        }
        other => panic!("unexpected break_condition: {:?}", other),
    }

    match facts.loop_increment {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            assert!(matches!(
                left.as_ref(),
                ASTNode::MethodCall { method, .. } if method == "indexOf"
            ));
            assert!(matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(3),
                    ..
                }
            ));
        }
        other => panic!("unexpected loop_increment: {:?}", other),
    }
}

#[test]
fn extract_loop_break_trim_whitespace_subset_start() {
    use crate::ast::UnaryOperator;

    let condition = binop(
        BinaryOperator::Less,
        v("i"),
        method_call("s", "length", vec![]),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(this_method_call(
                    "is_whitespace",
                    vec![method_call(
                        "s",
                        "substring",
                        vec![v("i"), binop(BinaryOperator::Add, v("i"), lit_int(1))],
                    )],
                )),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "i");
    assert_eq!(facts.carrier_var, "i");
    assert!(matches!(
        facts.break_condition,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            ..
        }
    ));
}

#[test]
fn extract_loop_break_trim_whitespace_subset_end() {
    use crate::ast::UnaryOperator;

    let condition = binop(BinaryOperator::GreaterEqual, v("i"), lit_int(0));
    let body = vec![
        ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(this_method_call(
                    "is_whitespace",
                    vec![method_call(
                        "s",
                        "substring",
                        vec![v("i"), binop(BinaryOperator::Add, v("i"), lit_int(1))],
                    )],
                )),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("i", binop(BinaryOperator::Subtract, v("i"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "i");
    assert_eq!(facts.carrier_var, "i");
    assert!(matches!(
        facts.loop_increment,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            ..
        }
    ));
}

#[test]
fn extract_loop_break_trim_whitespace_subset_rejects_missing_not() {
    let condition = binop(
        BinaryOperator::Less,
        v("i"),
        method_call("s", "length", vec![]),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(this_method_call(
                "is_whitespace",
                vec![method_call(
                    "s",
                    "substring",
                    vec![v("i"), binop(BinaryOperator::Add, v("i"), lit_int(1))],
                )],
            )),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body).expect("Ok");
    assert!(facts.is_none());
}

#[test]
fn extract_loop_break_body_local_subset_trim_seg_subset() {
    let condition = binop(
        BinaryOperator::Less,
        v("i"),
        method_call("s", "length", vec![]),
    );
    let body = vec![
        local(
            "seg",
            method_call(
                "s",
                "substring",
                vec![v("i"), binop(BinaryOperator::Add, v("i"), lit_int(1))],
            ),
        ),
        ASTNode::If {
            condition: Box::new(binop(
                BinaryOperator::Or,
                binop(BinaryOperator::Equal, v("seg"), lit_str(" ")),
                binop(BinaryOperator::Equal, v("seg"), lit_str("\t")),
            )),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "i");
    assert_eq!(facts.carrier_var, "i");
}

#[test]
fn extract_loop_break_body_local_subset_digit_pos_subset() {
    let condition = binop(
        BinaryOperator::Less,
        v("p"),
        method_call("s", "length", vec![]),
    );
    let body = vec![
        local(
            "ch",
            method_call(
                "s",
                "substring",
                vec![v("p"), binop(BinaryOperator::Add, v("p"), lit_int(1))],
            ),
        ),
        local("digit_pos", method_call("digits", "indexOf", vec![v("ch")])),
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Less, v("digit_pos"), lit_int(0))),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign("p", binop(BinaryOperator::Add, v("p"), lit_int(1))),
    ];

    let facts = try_extract_loop_break_facts(&condition, &body)
        .expect("Ok")
        .expect("Some facts");
    assert_eq!(facts.loop_var, "p");
    assert_eq!(facts.carrier_var, "p");
}
