use super::super::try_build_loop_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

#[test]
fn loopfacts_ok_some_for_canonical_split_scan_minimal() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("separator")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
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
                        right: Box::new(ASTNode::MethodCall {
                            object: Box::new(v("separator")),
                            method: "length".to_string(),
                            arguments: vec![],
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("separator")),
            span: Span::unknown(),
        }),
        then_body: vec![
            ASTNode::MethodCall {
                object: Box::new(v("result")),
                method: "push".to_string(),
                arguments: vec![ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![v("start"), v("i")],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("start")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("i")),
                    right: Box::new(ASTNode::MethodCall {
                        object: Box::new(v("separator")),
                        method: "length".to_string(),
                        arguments: vec![],
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("i")),
                value: Box::new(v("start")),
                span: Span::unknown(),
            },
        ],
        else_body: Some(vec![ASTNode::Assignment {
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
        }]),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt]).expect("Ok");
    let split_scan = facts.and_then(|facts| facts.split_scan);
    let split_scan = split_scan.expect("SplitScan facts");
    assert_eq!(split_scan.s_var, "s");
    assert_eq!(split_scan.sep_var, "separator");
    assert_eq!(split_scan.result_var, "result");
    assert_eq!(split_scan.i_var, "i");
    assert_eq!(split_scan.start_var, "start");
    assert!(matches!(
        split_scan.shape,
        crate::mir::builder::control_flow::plan::facts::scan_shapes::SplitScanShape::Minimal
    ));
}

#[test]
fn loopfacts_ok_some_for_loop_cond_break_continue_when_split_scan_missing_else() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("separator")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
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
                        right: Box::new(ASTNode::MethodCall {
                            object: Box::new(v("separator")),
                            method: "length".to_string(),
                            arguments: vec![],
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("separator")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::MethodCall {
            object: Box::new(v("result")),
            method: "push".to_string(),
            arguments: vec![ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![v("start"), v("i")],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt]).expect("Ok");
    let facts = facts.expect("Some");
    assert!(facts.loop_cond_break_continue.is_some());
    assert!(facts.scan_with_init.is_none());
    assert!(facts.split_scan.is_none());
}
