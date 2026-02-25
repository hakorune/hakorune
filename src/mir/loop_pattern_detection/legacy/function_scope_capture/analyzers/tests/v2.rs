use super::super::analyze_captured_vars_v2;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::BasicBlockId;
use std::collections::{BTreeMap, BTreeSet};

// Phase 245C: Function parameter capture tests

#[test]
fn test_capture_function_param_used_in_condition() {
    // Simulate: fn parse_number(s, p, len) { loop(p < len) { ... } }
    // Expected: 'len' should be captured (used in condition, not reassigned)

    let condition = Box::new(ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "p".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Variable {
            name: "len".to_string(), // function parameter
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    });

    let body = vec![ASTNode::Assignment {
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
    }];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["p".to_string()]), // p is loop param
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    // Use analyze_captured_vars_v2 with structural matching
    let env = analyze_captured_vars_v2(&[], condition.as_ref(), &body, &scope);

    // Should capture 'len' (function parameter used in condition)
    assert_eq!(env.vars.len(), 1);
    assert!(env.get("len").is_some());
    let var = env.get("len").unwrap();
    assert_eq!(var.name, "len");
    assert!(var.is_immutable);
}

#[test]
fn test_capture_function_param_used_in_method_call() {
    // Simulate: fn parse_number(s, p) { loop(p < s.length()) { ch = s.charAt(p) } }
    // Expected: 's' should be captured (used in condition and body, not reassigned)

    let condition = Box::new(ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "p".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "s".to_string(), // function parameter
                span: Span::unknown(),
            }),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    });

    let body = vec![
        ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(), // function parameter
                    span: Span::unknown(),
                }),
                method: "charAt".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
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
    ];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["p".to_string()]), // p is loop param
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::from(["ch".to_string()]),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    // Use analyze_captured_vars_v2 with structural matching
    let env = analyze_captured_vars_v2(&[], condition.as_ref(), &body, &scope);

    // Should capture 's' (function parameter used in condition and body)
    assert_eq!(env.vars.len(), 1);
    assert!(env.get("s").is_some());
    let var = env.get("s").unwrap();
    assert_eq!(var.name, "s");
    assert!(var.is_immutable);
}

#[test]
fn test_capture_function_param_reassigned_rejected() {
    // Simulate: fn bad_func(x) { x = 5; loop(x < 10) { x = x + 1 } }
    // Expected: 'x' should NOT be captured (reassigned in function)

    let condition = Box::new(ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    });

    let body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
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

    // fn_body includes reassignment before loop
    let fn_body = vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(5),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["x".to_string()]), // x is loop param
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    // Use analyze_captured_vars_v2 with structural matching
    let env = analyze_captured_vars_v2(&fn_body, condition.as_ref(), &body, &scope);

    // Should NOT capture 'x' (reassigned in fn_body)
    assert_eq!(env.vars.len(), 0);
}

#[test]
fn test_capture_mixed_locals_and_params() {
    // Simulate: fn parse(s, len) { local digits = "0123"; loop(p < len) { ch = digits.indexOf(...); s.charAt(...) } }
    // Expected: 'len', 's', and 'digits' should all be captured

    let condition = Box::new(ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "p".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Variable {
            name: "len".to_string(), // function parameter
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    });

    let body = vec![
        ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(), // function parameter
                    span: Span::unknown(),
                }),
                method: "charAt".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["digit".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "digits".to_string(), // pre-loop local
                    span: Span::unknown(),
                }),
                method: "indexOf".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
    ];

    // fn_body includes local declaration before loop
    let fn_body = vec![ASTNode::Local {
        variables: vec!["digits".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::String("0123".to_string()),
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    }];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["p".to_string()]), // p is loop param
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::from(["ch".to_string(), "digit".to_string()]),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    // Use analyze_captured_vars_v2 with structural matching
    let env = analyze_captured_vars_v2(&fn_body, condition.as_ref(), &body, &scope);

    // Should capture all three: 'len' (param), 's' (param), 'digits' (pre-loop local)
    assert_eq!(env.vars.len(), 3);
    assert!(env.get("len").is_some());
    assert!(env.get("s").is_some());
    assert!(env.get("digits").is_some());
}
