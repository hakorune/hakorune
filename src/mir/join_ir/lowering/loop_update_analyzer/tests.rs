use super::*;

#[test]
fn test_analyze_simple_increment() {
    // Test case: count = count + 1
    use crate::ast::Span;

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "count".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "count".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let carriers = vec![CarrierVar {
        name: "count".to_string(),
        host_id: crate::mir::ValueId(0),
        join_id: None, // Phase 177-STRUCT-1
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
    }];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    assert_eq!(updates.len(), 1);
    assert!(updates.contains_key("count"));

    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("count") {
        assert_eq!(lhs, "count");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::Const(n) = rhs {
            assert_eq!(*n, 1);
        } else {
            panic!("Expected Const(1), got {:?}", rhs);
        }
    } else {
        panic!("Expected BinOp, got {:?}", updates.get("count"));
    }
}

#[test]
fn test_analyze_number_accumulation_base10() {
    // Test case: result = result * 10 + digit
    use crate::ast::Span;

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "result".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Multiply,
                left: Box::new(ASTNode::Variable {
                    name: "result".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "digit".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let carriers = vec![CarrierVar {
        name: "result".to_string(),
        host_id: crate::mir::ValueId(0),
        join_id: None,
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
    }];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    assert_eq!(updates.len(), 1);
    assert!(updates.contains_key("result"));

    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
        assert_eq!(lhs, "result");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
            assert_eq!(*base, 10);
            assert_eq!(digit_var, "digit");
        } else {
            panic!("Expected NumberAccumulation, got {:?}", rhs);
        }
    } else {
        panic!("Expected BinOp, got {:?}", updates.get("result"));
    }
}

#[test]
fn test_analyze_number_accumulation_base2() {
    // Test case: result = result * 2 + bit
    use crate::ast::Span;

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "result".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Multiply,
                left: Box::new(ASTNode::Variable {
                    name: "result".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "bit".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let carriers = vec![CarrierVar {
        name: "result".to_string(),
        host_id: crate::mir::ValueId(0),
        join_id: None,
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
    }];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    assert_eq!(updates.len(), 1);
    assert!(updates.contains_key("result"));

    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
        assert_eq!(lhs, "result");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
            assert_eq!(*base, 2);
            assert_eq!(digit_var, "bit");
        } else {
            panic!("Expected NumberAccumulation, got {:?}", rhs);
        }
    } else {
        panic!("Expected BinOp, got {:?}", updates.get("result"));
    }
}

#[test]
fn test_analyze_number_accumulation_wrong_lhs() {
    // Test case: result = other * 10 + digit (should be Complex/Other)
    use crate::ast::Span;

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "result".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Multiply,
                left: Box::new(ASTNode::Variable {
                    name: "other".to_string(), // Wrong variable!
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "digit".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let carriers = vec![CarrierVar {
        name: "result".to_string(),
        host_id: crate::mir::ValueId(0),
        join_id: None,
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
    }];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    // Should detect assignment but with Other (complex) RHS
    assert_eq!(updates.len(), 0); // Won't match because lhs != carrier
}

#[test]
fn test_analyze_num_str_string_append() {
    // Phase 245B: Test case: num_str = num_str + ch (string append pattern)
    use crate::ast::Span;

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "num_str".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "num_str".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let carriers = vec![CarrierVar {
        name: "num_str".to_string(),
        host_id: crate::mir::ValueId(0),
        join_id: None,
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
    }];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    assert_eq!(updates.len(), 1);
    assert!(updates.contains_key("num_str"));

    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("num_str") {
        assert_eq!(lhs, "num_str");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::Variable(var_name) = rhs {
            assert_eq!(var_name, "ch");
        } else {
            panic!("Expected Variable('ch'), got {:?}", rhs);
        }
    } else {
        panic!("Expected BinOp, got {:?}", updates.get("num_str"));
    }
}

#[test]
fn test_atoi_update_expr_detection() {
    // Phase 246-EX Step 3: _atoi loop multi-carrier update detection
    // Tests two carriers with different update patterns:
    // - i = i + 1 (Const increment)
    // - result = result * 10 + digit_pos (NumberAccumulation)
    use crate::ast::Span;

    let body = vec![
        // result = result * 10 + digit_pos
        ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "result".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(ASTNode::Variable {
                        name: "result".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(10),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "digit_pos".to_string(),
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
    ];

    let carriers = vec![
        CarrierVar {
            name: "result".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
        },
        CarrierVar {
            name: "i".to_string(),
            host_id: crate::mir::ValueId(1),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
        },
    ];

    let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

    // Verify both carriers have updates
    assert_eq!(
        updates.len(),
        2,
        "Should detect updates for both i and result"
    );

    // Verify i = i + 1 (Const increment)
    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("i") {
        assert_eq!(lhs, "i");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::Const(n) = rhs {
            assert_eq!(*n, 1, "i should increment by 1");
        } else {
            panic!("Expected Const(1) for i update, got {:?}", rhs);
        }
    } else {
        panic!("Expected BinOp for i update, got {:?}", updates.get("i"));
    }

    // Verify result = result * 10 + digit_pos (NumberAccumulation)
    if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
        assert_eq!(lhs, "result");
        assert_eq!(*op, BinOpKind::Add);
        if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
            assert_eq!(*base, 10, "NumberAccumulation should use base 10");
            assert_eq!(digit_var, "digit_pos", "Should use digit_pos variable");
        } else {
            panic!(
                "Expected NumberAccumulation for result update, got {:?}",
                rhs
            );
        }
    } else {
        panic!(
            "Expected BinOp for result update, got {:?}",
            updates.get("result")
        );
    }
}
