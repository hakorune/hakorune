use super::*;

#[test]
fn test_parse_number_pattern_recognized() {
    // Phase 143-P0: Test parse_number pattern (break in THEN clause)
    // Build: loop(i < len) { digit_pos = digits.indexOf(ch); if digit_pos < 0 { break } result = result + ch; i = i + 1 }
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
            // Body statement: digit_pos = digits.indexOf(ch)
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "digit_pos".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::FunctionCall {
                    name: "indexOf".to_string(),
                    arguments: vec![ASTNode::Variable {
                        name: "ch".to_string(),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // Break check: if digit_pos < 0 { break }
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(ASTNode::Variable {
                        name: "digit_pos".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None, // No else branch
                span: Span::unknown(),
            },
            // Rest: result = result + ch
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "result".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "result".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Variable {
                        name: "ch".to_string(),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // Carrier update: i = i + 1
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
    // chosen == LoopBreak (has_break=true)
    assert_eq!(decision.chosen, Some(LoopPatternKind::LoopBreak));
    // missing_caps == []
    assert!(decision.missing_caps.is_empty());

    // Verify skeleton structure
    // HeaderCond + Body (digit_pos assignment) + Body (result assignment) + Update
    assert!(skeleton.steps.len() >= 3);
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
    assert!(skeleton.exits.has_break);
    assert!(!skeleton.exits.has_continue);
    assert!(!skeleton.exits.has_return);
    assert!(!skeleton.exits.break_has_value);
}
