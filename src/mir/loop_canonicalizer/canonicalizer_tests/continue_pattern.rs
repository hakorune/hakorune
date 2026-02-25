use super::*;

#[test]
fn test_simple_continue_pattern_recognized() {
    // Phase 142 P1: Test simple continue pattern
    // Build: loop(i < n) { if is_even { i = i + 1; continue } sum = sum + i; i = i + 1 }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "n".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![
            // if is_even { i = i + 1; continue }
            ASTNode::If {
                condition: Box::new(ASTNode::Variable {
                    name: "is_even".to_string(),
                    span: Span::unknown(),
                }),
                then_body: vec![
                    ASTNode::Assignment {
                        target: Box::new(ASTNode::Variable {
                            name: "i".to_string(),
                            span: Span::unknown(),
                        }),
                        value: Box::new(ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(ASTNode::Variable {
                                name: "i".to_string(),
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
            // sum = sum + i
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "sum".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // i = i + 1
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
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
    // chosen == Pattern4Continue
    assert_eq!(decision.chosen, Some(LoopPatternKind::Pattern4Continue));
    // missing_caps == []
    assert!(decision.missing_caps.is_empty());

    // Verify skeleton structure
    // HeaderCond + Body (sum = sum + i) + Update
    assert!(skeleton.steps.len() >= 2);
    assert!(matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }));

    // Verify carrier
    assert_eq!(skeleton.carriers.len(), 1);
    assert_eq!(skeleton.carriers[0].name, "i");
    assert_eq!(skeleton.carriers[0].role, CarrierRole::Counter);
    match &skeleton.carriers[0].update_kind {
        UpdateKind::ConstStep { delta } => assert_eq!(*delta, 1),
        _ => panic!("Expected ConstStep update"),
    }

    // Verify exit contract
    assert!(!skeleton.exits.has_break);
    assert!(skeleton.exits.has_continue);
    assert!(!skeleton.exits.has_return);
}
