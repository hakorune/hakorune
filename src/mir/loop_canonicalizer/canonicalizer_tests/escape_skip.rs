use super::*;

#[test]
fn test_escape_skip_pattern_recognition() {
    // Phase 91 P5b: Escape sequence handling pattern
    // Build: loop(i < len) {
    //   ch = get_char(i)
    //   if ch == "\"" { break }
    //   if ch == "\\" { i = i + 2 } else { i = i + 1 }
    // }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "len".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![
            // Body: ch = get_char(i)
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::FunctionCall {
                    name: "get_char".to_string(),
                    arguments: vec![ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // Break check: if ch == "\"" { break }
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
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            // Escape check: if ch == "\\" { i = i + 2 } else { i = i + 1 }
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
                then_body: vec![ASTNode::Assignment {
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
                            value: LiteralValue::Integer(2),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                else_body: Some(vec![ASTNode::Assignment {
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
                }]),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&loop_node);
    assert!(result.is_ok(), "Escape pattern canonicalization should succeed");

    let (skeleton, decision) = result.unwrap();

    // Verify decision success
    assert!(decision.is_success(), "Decision should indicate success");
    assert_eq!(
        decision.chosen,
        Some(LoopPatternKind::LoopBreak),
        "P5b should route to LoopBreak (has_break=true)"
    );
    assert!(decision.missing_caps.is_empty(), "No missing capabilities");

    // Verify skeleton structure
    // Expected: HeaderCond + Body + Update
    assert!(
        skeleton.steps.len() >= 3,
        "Expected at least 3 steps: HeaderCond, Body, Update"
    );
    assert!(
        matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }),
        "First step should be HeaderCond"
    );
    assert!(
        matches!(skeleton.steps[skeleton.steps.len() - 1], SkeletonStep::Update { .. }),
        "Last step should be Update"
    );

    // Verify carrier (counter variable "i")
    assert_eq!(skeleton.carriers.len(), 1, "Should have 1 carrier");
    let carrier = &skeleton.carriers[0];
    assert_eq!(carrier.name, "i", "Carrier should be named 'i'");
    assert_eq!(carrier.role, CarrierRole::Counter, "Carrier should be a Counter");

    // Verify ConditionalStep with escape_delta=2, normal_delta=1
    // Phase 92 P0-3: ConditionalStep now includes cond
    match &carrier.update_kind {
        UpdateKind::ConditionalStep {
            cond: _,  // Phase 92 P0-3: Condition for Select (don't check exact AST)
            then_delta,
            else_delta,
        } => {
            assert_eq!(*then_delta, 2, "Escape delta (then) should be 2");
            assert_eq!(*else_delta, 1, "Normal delta (else) should be 1");
        }
        other => panic!(
            "Expected ConditionalStep, got {:?}",
            other
        ),
    }

    // Verify exit contract (P5b has break for string boundary)
    assert!(skeleton.exits.has_break, "P5b should have break");
    assert!(!skeleton.exits.has_continue, "P5b should not have continue");
    assert!(!skeleton.exits.has_return, "P5b should not have return");
    assert!(!skeleton.exits.break_has_value, "Break should not have value");
}
