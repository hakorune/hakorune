use super::super::try_build_loop_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn len_call(var: &str) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(v(var)),
        method: "length".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    }
}

#[test]
fn loopfacts_ok_some_for_canonical_scan_with_init_minimal() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("ch")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(v("i"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt, step]).expect("Ok");
    assert!(facts.is_some());
}

#[test]
fn loopfacts_ok_some_for_reverse_scan_with_init_minimal() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::GreaterEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("ch")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(v("i"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt, step])
        .expect("Ok")
        .expect("Some");
    let scan = facts.scan_with_init.expect("scan facts");
    assert_eq!(scan.loop_var, "i");
    assert_eq!(scan.haystack, "s");
    assert_eq!(scan.step_lit, -1);
}

#[test]
fn loopfacts_ok_some_for_reverse_scan_with_init_starts_with() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::GreaterEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::MethodCall {
            object: Box::new(v("me")),
            method: "starts_with".to_string(),
            arguments: vec![v("s"), v("i"), v("pat")],
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(v("i"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt, step])
        .expect("Ok")
        .expect("Some");
    let scan = facts.scan_with_init.expect("scan facts");
    assert_eq!(scan.loop_var, "i");
    assert_eq!(scan.haystack, "s");
    assert_eq!(scan.needle, "pat");
    assert_eq!(scan.step_lit, -1);
    assert!(scan.dynamic_needle);
}

#[test]
fn loopfacts_ok_some_for_dynamic_needle_scan_with_init() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(len_call("s")),
            right: Box::new(len_call("needle")),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(len_call("needle")),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("needle")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(v("i"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt, step])
        .expect("Ok")
        .expect("Some");
    let scan = facts.scan_with_init.expect("scan facts");
    assert_eq!(scan.loop_var, "i");
    assert_eq!(scan.haystack, "s");
    assert_eq!(scan.needle, "needle");
    assert_eq!(scan.step_lit, 1);
    assert!(scan.dynamic_needle);
}

#[test]
fn loopfacts_ok_some_for_index_of_add_bound_return_step() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("j")),
            right: Box::new(v("m")),
            span: Span::unknown(),
        }),
        right: Box::new(v("n")),
        span: Span::unknown(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::MethodCall {
            object: Box::new(v("me")),
            method: "starts_with".to_string(),
            arguments: vec![v("src"), v("j"), v("pat")],
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(v("j"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("j")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("j")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt, step])
        .expect("Ok")
        .expect("Some");
    assert!(
        facts.scan_with_init.is_some() || facts.loop_cond_return_in_body.is_some(),
        "expected scan_with_init or loop_cond_return_in_body facts"
    );
}
