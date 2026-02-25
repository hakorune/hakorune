use super::{lower_condition_to_joinir, lower_condition_to_joinir_no_body_locals};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::{JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Helper to create a test ConditionEnv with variables
fn create_test_env() -> ConditionEnv {
    let mut env = ConditionEnv::new();
    // Register test variables (using JoinIR-local ValueIds)
    env.insert("i".to_string(), ValueId(0));
    env.insert("end".to_string(), ValueId(1));
    env
}

#[test]
fn test_simple_comparison() {
    let env = create_test_env();
    let mut value_counter = 2u32; // Start after i=0, end=1
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: i < end
    let ast = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Variable {
            name: "end".to_string(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
    assert!(result.is_ok(), "Simple comparison should succeed");

    let (_cond_value, instructions) = result.unwrap();
    assert_eq!(instructions.len(), 1, "Should generate 1 Compare instruction");
}

#[test]
fn test_comparison_with_literal() {
    let env = create_test_env();
    let mut value_counter = 2u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: i < 10
    let ast = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
    assert!(result.is_ok(), "Comparison with literal should succeed");

    let (_cond_value, instructions) = result.unwrap();
    // Should have: Const(10), Compare
    assert_eq!(instructions.len(), 2, "Should generate Const + Compare");
}

#[test]
fn test_logical_or() {
    let mut env = ConditionEnv::new();
    env.insert("a".to_string(), ValueId(2));
    env.insert("b".to_string(), ValueId(3));

    let mut value_counter = 4u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: a < 5 || b < 5
    let ast = ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "a".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(5),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "b".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(5),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
    assert!(result.is_ok(), "OR expression should succeed");

    let (_cond_value, instructions) = result.unwrap();
    // Should have: Const(5), Compare(a<5), Const(5), Compare(b<5), BinOp(Or)
    assert_eq!(instructions.len(), 5, "Should generate proper OR chain");
}

#[test]
fn test_not_operator() {
    let env = create_test_env();
    let mut value_counter = 2u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: !(i < end)
    let ast = ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
    assert!(result.is_ok(), "NOT operator should succeed");

    let (_cond_value, instructions) = result.unwrap();
    // Should have: Compare, UnaryOp(Not)
    assert_eq!(instructions.len(), 2, "Should generate Compare + Not");
}

/// Phase 92 P4 Level 2: Test body-local variable resolution
///
/// This test verifies that conditions can reference body-local variables
/// (e.g., `ch == '\\'` in escape sequence patterns).
///
/// Variable resolution priority:
/// 1. ConditionEnv (loop parameters, captured variables)
/// 2. LoopBodyLocalEnv (body-local variables like `ch`)
#[test]
fn test_body_local_variable_resolution() {
    // Setup ConditionEnv with loop variable
    let mut env = ConditionEnv::new();
    env.insert("i".to_string(), ValueId(100));

    // Setup LoopBodyLocalEnv with body-local variable
    let mut body_local_env = LoopBodyLocalEnv::new();
    body_local_env.insert("ch".to_string(), ValueId(200));

    let mut value_counter = 300u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: ch == "\\"
    let ast = ASTNode::BinaryOp {
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
    };

    // Phase 92 P2-2: Use lower_condition_to_joinir with body_local_env
    let result = lower_condition_to_joinir(
        &ast,
        &mut alloc_value,
        &env,
        Some(&body_local_env),
        None,
    );
    assert!(
        result.is_ok(),
        "Body-local variable resolution should succeed"
    );

    let (cond_value, instructions) = result.unwrap();
    // Should have: Const("\\"), Compare(ch == "\\")
    assert_eq!(
        instructions.len(),
        2,
        "Should generate Const + Compare for body-local variable"
    );

    // Verify the comparison uses the body-local variable's ValueId(200)
    if let Some(JoinInst::Compute(MirLikeInst::Compare { lhs, .. })) = instructions.get(1) {
        assert_eq!(
            *lhs,
            ValueId(200),
            "Compare should use body-local variable ValueId(200)"
        );
    } else {
        panic!("Expected Compare instruction at position 1");
    }

    assert!(cond_value.0 >= 300, "Result should use newly allocated ValueId");
}

/// Phase 92 P4 Level 2: Test variable resolution priority (ConditionEnv takes precedence)
///
/// When a variable exists in both ConditionEnv and LoopBodyLocalEnv,
/// ConditionEnv should take priority.
#[test]
fn test_variable_resolution_priority() {
    // Setup both environments with overlapping variable "x"
    let mut env = ConditionEnv::new();
    env.insert("x".to_string(), ValueId(100)); // ConditionEnv priority

    let mut body_local_env = LoopBodyLocalEnv::new();
    body_local_env.insert("x".to_string(), ValueId(200)); // Should be shadowed

    let mut value_counter = 300u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: x == 42
    let ast = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir(&ast, &mut alloc_value, &env, Some(&body_local_env), None);
    assert!(result.is_ok(), "Variable resolution should succeed");

    let (_cond_value, instructions) = result.unwrap();

    // Verify the comparison uses ConditionEnv's ValueId(100), not LoopBodyLocalEnv's ValueId(200)
    if let Some(JoinInst::Compute(MirLikeInst::Compare { lhs, .. })) = instructions.get(1) {
        assert_eq!(
            *lhs,
            ValueId(100),
            "ConditionEnv should take priority over LoopBodyLocalEnv"
        );
    } else {
        panic!("Expected Compare instruction at position 1");
    }
}

/// Phase 92 P4 Level 2: Test error handling for undefined variables
///
/// Variables not found in either environment should produce clear error messages.
#[test]
fn test_undefined_variable_error() {
    let env = ConditionEnv::new();
    let body_local_env = LoopBodyLocalEnv::new();

    let mut value_counter = 300u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: undefined_var == 42
    let ast = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(ASTNode::Variable {
            name: "undefined_var".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir(&ast, &mut alloc_value, &env, Some(&body_local_env), None);
    assert!(result.is_err(), "Undefined variable should fail");

    let err = result.unwrap_err();
    assert!(
        err.contains("undefined_var"),
        "Error message should mention the undefined variable name"
    );
    assert!(
        err.contains("not found"),
        "Error message should indicate variable was not found"
    );
}

/// Phase 252 P1: Test this.methodcall(...) in conditions
///
/// Verifies that user-defined static box method calls work in conditions
#[test]
fn test_this_methodcall_in_condition() {
    let env = create_test_env();
    let mut value_counter = 2u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: not this.is_whitespace(ch)
    // Simulates StringUtils.trim_end break condition
    let method_call = ASTNode::MethodCall {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        method: "is_whitespace".to_string(),
        arguments: vec![ASTNode::Variable {
            name: "ch".to_string(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let ast = ASTNode::UnaryOp {
        operator: crate::ast::UnaryOperator::Not,
        operand: Box::new(method_call),
        span: Span::unknown(),
    };

    // Register 'ch' variable for the test
    let mut env = env;
    env.insert("ch".to_string(), ValueId(100));

    let result = lower_condition_to_joinir(
        &ast,
        &mut alloc_value,
        &env,
        None,
        Some("StringUtils"), // Phase 252: static box context
    );

    assert!(result.is_ok(), "this.methodcall should succeed: {:?}", result);

    let (_cond_value, instructions) = result.unwrap();

    // Should have: BoxCall for is_whitespace, UnaryOp(Not)
    assert!(
        instructions.len() >= 2,
        "Should generate BoxCall + Not instructions"
    );

    // Verify BoxCall instruction exists
    let has_box_call = instructions.iter().any(|inst| matches!(
        inst,
        JoinInst::Compute(MirLikeInst::BoxCall { method, .. }) if method == "is_whitespace"
    ));
    assert!(has_box_call, "Should generate BoxCall for is_whitespace");
}

/// Phase 252 P1: Test this.methodcall fails without static box context
#[test]
fn test_this_methodcall_requires_context() {
    let env = create_test_env();
    let mut value_counter = 2u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: this.is_whitespace(ch)
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        method: "is_whitespace".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir(&ast, &mut alloc_value, &env, None, None);

    assert!(result.is_err(), "this.methodcall should fail without context");
    let err = result.unwrap_err();
    assert!(
        err.contains("current_static_box_name"),
        "Error should mention missing static box context"
    );
}

/// Phase 252 P1: Test disallowed method fails
#[test]
fn test_this_methodcall_disallowed_method() {
    let env = create_test_env();
    let mut value_counter = 2u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    // AST: this.trim("test") - trim is NOT allowed in conditions
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        method: "trim".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    };

    let result = lower_condition_to_joinir(
        &ast,
        &mut alloc_value,
        &env,
        None,
        Some("StringUtils"),
    );

    assert!(result.is_err(), "Disallowed method should fail");
    let err = result.unwrap_err();
    assert!(
        err.contains("not allowed") || err.contains("not whitelisted"),
        "Error should indicate method is not allowed: {}",
        err
    );
}
