use super::*;

#[test]
fn test_parse_string_route_shape_recognized() {
    // Phase 143-P1: Test parse_string route shape (both continue AND return)
    // Build: loop(p < len) {
    //   local ch = s.substring(p, p + 1)
    //   if ch == "\"" { return 0 }
    //   if ch == "\\" { p = p + 1; continue }
    //   p = p + 1
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
            // Body statement: local ch = s.substring(p, p + 1)
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::FunctionCall {
                    name: "substring".to_string(),
                    arguments: vec![
                        ASTNode::Variable {
                            name: "p".to_string(),
                            span: Span::unknown(),
                        },
                        ASTNode::BinaryOp {
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
                        },
                    ],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // Return check: if ch == "\"" { return 0 }
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Equal,
                    left: Box::new(ASTNode::Variable {
                        name: "ch".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("\"".to_string()),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    })),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            // Escape check: if ch == "\\" { p = p + 1; continue }
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Equal,
                    left: Box::new(ASTNode::Variable {
                        name: "ch".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("\\".to_string()),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![
                    ASTNode::Assignment {
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
                    },
                    ASTNode::Continue {
                        span: Span::unknown(),
                    },
                ],
                else_body: None,
                span: Span::unknown(),
            },
            // Carrier update: p = p + 1
            ASTNode::Assignment {
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
            },
        ],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok());

    let (skeleton, decision) = result.unwrap();

    // Verify success
    assert!(decision.is_success());
    // chosen == LoopContinueOnly (has both continue and return)
    assert_eq!(decision.chosen, Some(LoopRouteKind::LoopContinueOnly));
    // missing_caps == []
    assert!(decision.missing_caps.is_empty());

    // Verify skeleton structure
    // HeaderCond + Body (ch assignment) + Update
    assert!(skeleton.steps.len() >= 2);
    assert!(matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }));

    // Verify carrier
    assert_eq!(skeleton.carriers.len(), 1);
    assert_eq!(skeleton.carriers[0].name, "p");
    assert_eq!(skeleton.carriers[0].role, CarrierRole::Counter);
    match &skeleton.carriers[0].update_kind {
        UpdateKind::ConstStep { delta } => assert_eq!(*delta, 1),
        _ => panic!("Expected ConstStep update"),
    }

    // Verify exit contract
    assert!(!skeleton.exits.has_break);
    assert!(skeleton.exits.has_continue);
    assert!(skeleton.exits.has_return);
    assert!(!skeleton.exits.break_has_value);
}
