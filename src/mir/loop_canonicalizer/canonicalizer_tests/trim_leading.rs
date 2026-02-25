use super::*;

#[test]
fn test_trim_leading_pattern_recognized() {
    // Phase 142 P0: Test trim_leading pattern (start = start + 1)
    // Build: loop(start < end) { if is_ws { start = start + 1 } else { break } }
    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "start".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
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
                    name: "start".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "start".to_string(),
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
    // chosen == Pattern2Break (ExitContract priority)
    assert_eq!(decision.chosen, Some(LoopPatternKind::Pattern2Break));
    // missing_caps == []
    assert!(decision.missing_caps.is_empty());

    // Verify skeleton structure
    assert_eq!(skeleton.steps.len(), 2); // HeaderCond + Update
    assert!(matches!(skeleton.steps[0], SkeletonStep::HeaderCond { .. }));
    assert!(matches!(skeleton.steps[1], SkeletonStep::Update { .. }));

    // Verify carrier
    assert_eq!(skeleton.carriers.len(), 1);
    assert_eq!(skeleton.carriers[0].name, "start");
    assert_eq!(skeleton.carriers[0].role, CarrierRole::Counter);
    match &skeleton.carriers[0].update_kind {
        UpdateKind::ConstStep { delta } => assert_eq!(*delta, 1),
        _ => panic!("Expected ConstStep update"),
    }

    // Verify exit contract
    assert!(skeleton.exits.has_break);
    assert!(!skeleton.exits.has_continue);
    assert!(!skeleton.exits.has_return);
}
