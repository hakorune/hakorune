use super::*;

#[test]
fn test_skip_whitespace_pattern_recognition() {
    // Build skip_whitespace pattern: loop(p < len) { if is_ws == 1 { p = p + 1 } else { break } }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "p".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "len".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::Variable {
                    name: "is_ws".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok());

    let (skeleton, decision) = result.unwrap();

    // Verify success
    assert!(decision.is_success());
    // Phase 137-5: Pattern choice reflects ExitContract (has_break=true → LoopBreak)
    assert_eq!(decision.chosen, Some(LoopPatternKind::LoopBreak));
    assert_eq!(decision.missing_caps.len(), 0);

    // Verify skeleton structure
    assert_eq!(skeleton.steps.len(), 2); // HeaderCond + Update
    assert!(matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }));
    assert!(matches!(skeleton.steps[1], SkeletonStep::Update { .. }));

    // Verify carrier
    assert_eq!(skeleton.carriers.len(), 1);
    assert_eq!(skeleton.carriers[0].name, "p");
    assert_eq!(skeleton.carriers[0].role, CarrierRole::Counter);
    match &skeleton.carriers[0].update_kind {
        UpdateKind::ConstStep { delta } => assert_eq!(*delta, 1),
        _ => panic!("Expected ConstStep update"),
    }

    // Verify exit contract
    assert!(skeleton.exits.has_break);
    assert!(!skeleton.exits.has_continue);
    assert!(!skeleton.exits.has_return);
    assert!(!skeleton.exits.break_has_value);
}

#[test]
fn test_skip_whitespace_with_body_statements() {
    // Build pattern with body statements before the if:
    // loop(p < len) {
    //   local ch = get_char(p)
    //   if is_ws { p = p + 1 } else { break }
    // }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "p".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "len".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::FunctionCall {
                    name: "get_char".to_string(),
                    arguments: vec![ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Equal,
                    left: Box::new(ASTNode::Variable {
                        name: "is_ws".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "p".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                else_body: Some(vec![ASTNode::Break {
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok());

    let (skeleton, decision) = result.unwrap();

    // Verify success
    assert!(decision.is_success());
    assert_eq!(decision.chosen, Some(LoopPatternKind::LoopBreak));
    assert!(decision.missing_caps.is_empty());

    // Verify skeleton structure
    assert_eq!(skeleton.steps.len(), 3); // HeaderCond + Body + Update
    assert!(matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }));
    assert!(matches!(skeleton.steps[1], SkeletonStep::Body { .. }));
    assert!(matches!(skeleton.steps[2], SkeletonStep::Update { .. }));

    // Verify carrier
    assert_eq!(skeleton.carriers.len(), 1);
    assert_eq!(skeleton.carriers[0].name, "p");
    assert_eq!(skeleton.carriers[0].role, CarrierRole::Counter);
    match &skeleton.carriers[0].update_kind {
        UpdateKind::ConstStep { delta } => assert_eq!(*delta, 1),
        _ => panic!("Expected ConstStep update"),
    }

    // Verify exit contract
    assert!(skeleton.exits.has_break);
    assert!(!skeleton.exits.has_continue);
    assert!(!skeleton.exits.has_return);
    assert!(!skeleton.exits.break_has_value);
}

#[test]
fn test_skip_whitespace_fails_without_else() {
    // Build pattern with missing else: loop(p < len) { if is_ws == 1 { p = p + 1 } }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "p".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "len".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::Variable {
                    name: "is_ws".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: None, // No else branch
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok());

    let (_, decision) = result.unwrap();
    assert!(decision.is_fail_fast());
    assert!(decision.notes[0].contains("Loop does not match"));
    assert!(decision.notes[0].contains("skip_whitespace"));
}

#[test]
fn test_skip_whitespace_fails_with_wrong_delta() {
    // Build pattern with wrong update (p = p * 2, not +/-)
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply, // Wrong operator
                    left: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(2),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok());

    let (_, decision) = result.unwrap();
    assert!(decision.is_fail_fast());
}
