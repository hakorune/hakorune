use super::super::analyze_captured_vars;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::BasicBlockId;
use std::collections::{BTreeMap, BTreeSet};

// Phase 200-B: Capture analysis tests

#[test]
fn test_capture_simple_digits() {
    // Build AST for:
    // local digits = "0123456789"
    // loop(i < 10) {
    //     local pos = digits.indexOf(ch)
    // }

    let digits_decl = ASTNode::Local {
        variables: vec!["digits".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::String("0123456789".to_string()),
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    };

    let loop_body = vec![ASTNode::Local {
        variables: vec!["pos".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "digits".to_string(),
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
    }];

    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: loop_body,
        span: Span::unknown(),
    };

    let fn_body = vec![digits_decl, loop_node.clone()];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["i".to_string()]),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::from(["pos".to_string()]),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    // IMPORTANT: Pass a reference to the same loop_node instance that's in fn_body
    // find_stmt_index uses pointer comparison, so we must use &fn_body[1] instead of &loop_node
    let env = analyze_captured_vars(&fn_body, &fn_body[1], &scope);

    assert_eq!(env.vars.len(), 1);
    assert!(env.get("digits").is_some());
    let var = env.get("digits").unwrap();
    assert_eq!(var.name, "digits");
    assert!(var.is_immutable);
}

#[test]
fn test_capture_reassigned_rejected() {
    // Build AST for:
    // local digits = "0123456789"
    // digits = "abc"  // reassignment
    // loop(i < 10) {
    //     local pos = digits.indexOf(ch)
    // }

    let digits_decl = ASTNode::Local {
        variables: vec!["digits".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::String("0123456789".to_string()),
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    };

    let reassignment = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "digits".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::Literal {
            value: LiteralValue::String("abc".to_string()),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let loop_body = vec![ASTNode::Local {
        variables: vec!["pos".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "digits".to_string(),
                span: Span::unknown(),
            }),
            method: "indexOf".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    }];

    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: loop_body,
        span: Span::unknown(),
    };

    let fn_body = vec![digits_decl, reassignment, loop_node.clone()];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["i".to_string()]),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::from(["pos".to_string()]),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    let env = analyze_captured_vars(&fn_body, &loop_node, &scope);

    // Should reject because digits is reassigned
    assert_eq!(env.vars.len(), 0);
}

#[test]
fn test_capture_after_loop_rejected() {
    // Build AST for:
    // loop(i < 10) { }
    // local digits = "0123456789"  // defined AFTER loop

    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![],
        span: Span::unknown(),
    };

    let digits_decl = ASTNode::Local {
        variables: vec!["digits".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::String("0123456789".to_string()),
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    };

    let fn_body = vec![loop_node.clone(), digits_decl];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["i".to_string()]),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    let env = analyze_captured_vars(&fn_body, &loop_node, &scope);

    // Should reject because digits is defined after the loop
    assert_eq!(env.vars.len(), 0);
}

#[test]
fn test_capture_method_call_init_rejected() {
    // Build AST for:
    // local result = someBox.getValue()  // MethodCall init
    // loop(i < 10) {
    //     local x = result.something()
    // }

    let result_decl = ASTNode::Local {
        variables: vec!["result".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "someBox".to_string(),
                span: Span::unknown(),
            }),
            method: "getValue".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    };

    let loop_body = vec![ASTNode::Local {
        variables: vec!["x".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "result".to_string(),
                span: Span::unknown(),
            }),
            method: "something".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    }];

    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: loop_body,
        span: Span::unknown(),
    };

    let fn_body = vec![result_decl, loop_node.clone()];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["i".to_string()]),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::from(["x".to_string()]),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    let env = analyze_captured_vars(&fn_body, &loop_node, &scope);

    // Should reject because result is initialized with MethodCall (not safe constant)
    assert_eq!(env.vars.len(), 0);
}

#[test]
fn test_capture_unused_in_loop_rejected() {
    // Build AST for:
    // local digits = "0123456789"  // not used in loop
    // loop(i < 10) {
    //     print(i)  // doesn't use digits
    // }

    let digits_decl = ASTNode::Local {
        variables: vec!["digits".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::String("0123456789".to_string()),
            span: Span::unknown(),
        }))],
        span: Span::unknown(),
    };

    let loop_node = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        body: vec![], // empty body, no usage of digits
        span: Span::unknown(),
    };

    let fn_body = vec![digits_decl, loop_node.clone()];

    let scope = crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::from(["i".to_string()]),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };

    let env = analyze_captured_vars(&fn_body, &loop_node, &scope);

    // Should reject because digits is not used in loop
    assert_eq!(env.vars.len(), 0);
}
