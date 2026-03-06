use super::test_helpers::{bin, lit_i, span, var};
use super::{ExprContext, ExprLowerer, ExprLoweringError};
use crate::ast::{BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::scope_manager::LoopBreakScopeManager;
use crate::mir::join_ir::{BinOpKind, JoinInst, MirLikeInst, UnaryOp as JoinUnaryOp};
use crate::mir::{builder::MirBuilder, ValueId};

// Helper to create a test MirBuilder (Phase 231: minimal stub)
fn create_test_builder() -> MirBuilder {
    MirBuilder::new()
}

fn not(expr: crate::ast::ASTNode) -> crate::ast::ASTNode {
    crate::ast::ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand: Box::new(expr),
        span: span(),
    }
}

#[test]
fn test_expr_lowerer_simple_comparison() {
    let mut condition_env = ConditionEnv::new();
    condition_env.insert("i".to_string(), ValueId(100));

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };

    let scope = LoopBreakScopeManager {
        condition_env: &condition_env,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: &carrier_info,
    };

    let mut builder = create_test_builder();

    // AST: i < 10
    let ast = crate::ast::ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(crate::ast::ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(crate::ast::ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);

    assert!(
        result.is_ok(),
        "Should lower simple comparison successfully"
    );
}

#[test]
fn test_expr_lowerer_variable_not_found() {
    let condition_env = ConditionEnv::new();

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };

    let scope = LoopBreakScopeManager {
        condition_env: &condition_env,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: &carrier_info,
    };

    let mut builder = create_test_builder();

    // AST: unknown_var < 10
    let ast = crate::ast::ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(crate::ast::ASTNode::Variable {
            name: "unknown_var".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(crate::ast::ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);

    assert!(matches!(
        result,
        Err(ExprLoweringError::VariableNotFound(_))
    ));
}

#[test]
fn test_expr_lowerer_unsupported_node() {
    let condition_env = ConditionEnv::new();

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };

    let scope = LoopBreakScopeManager {
        condition_env: &condition_env,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: &carrier_info,
    };

    let mut builder = create_test_builder();

    // AST: Break (unsupported in condition context)
    let ast = crate::ast::ASTNode::Break {
        span: Span::unknown(),
    };

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);

    assert!(matches!(result, Err(ExprLoweringError::UnsupportedNode(_))));
}

#[test]
fn test_is_supported_condition() {
    // Supported: i < 10
    let ast = crate::ast::ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(crate::ast::ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(crate::ast::ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    assert!(ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(
        &ast
    ));

    // Supported: MethodCall
    let ast = crate::ast::ASTNode::MethodCall {
        object: Box::new(crate::ast::ASTNode::Variable {
            name: "s".to_string(),
            span: Span::unknown(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: Span::unknown(),
    };
    assert!(ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(
        &ast
    ));

    // Unsupported: Break node
    let ast = crate::ast::ASTNode::Break {
        span: Span::unknown(),
    };
    assert!(!ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast));
}

// Phase 235: Additional patterns for condition lowering

fn make_basic_scope() -> LoopBreakScopeManager<'static> {
    // NOTE: we leak these small envs for the duration of the test to satisfy lifetimes simply.
    // テスト専用なので許容する。
    let mut condition_env = ConditionEnv::new();
    condition_env.insert("i".to_string(), ValueId(1));
    condition_env.insert("j".to_string(), ValueId(2));

    let boxed_env: Box<ConditionEnv> = Box::new(condition_env);
    let condition_env_ref: &'static ConditionEnv = Box::leak(boxed_env);

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };
    let boxed_carrier: Box<CarrierInfo> = Box::new(carrier_info);
    let carrier_ref: &'static CarrierInfo = Box::leak(boxed_carrier);

    LoopBreakScopeManager {
        condition_env: condition_env_ref,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: carrier_ref,
    }
}

fn make_scope_with_p_and_s() -> LoopBreakScopeManager<'static> {
    // Leak these tiny envs for test lifetime convenience only.
    let mut condition_env = ConditionEnv::new();
    condition_env.insert("p".to_string(), ValueId(1));
    condition_env.insert("s".to_string(), ValueId(2));

    let boxed_env: Box<ConditionEnv> = Box::new(condition_env);
    let condition_env_ref: &'static ConditionEnv = Box::leak(boxed_env);

    let carrier_info = CarrierInfo {
        loop_var_name: "p".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };
    let boxed_carrier: Box<CarrierInfo> = Box::new(carrier_info);
    let carrier_ref: &'static CarrierInfo = Box::leak(boxed_carrier);

    LoopBreakScopeManager {
        condition_env: condition_env_ref,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: carrier_ref,
    }
}

fn assert_has_compare(instructions: &[JoinInst]) {
    assert!(
        instructions
            .iter()
            .any(|inst| matches!(inst, JoinInst::Compute(MirLikeInst::Compare { .. }))),
        "Expected at least one Compare instruction, got {:?}",
        instructions
    );
}

fn assert_has_binop(instructions: &[JoinInst], op: BinOpKind) {
    assert!(
        instructions.iter().any(|inst| matches!(
            inst,
            JoinInst::Compute(MirLikeInst::BinOp { op: o, .. } ) if *o == op
        )),
        "Expected at least one BinOp {:?}, got {:?}",
        op,
        instructions
    );
}

fn assert_has_not(instructions: &[JoinInst]) {
    assert!(
        instructions.iter().any(|inst| matches!(
            inst,
            JoinInst::Compute(MirLikeInst::UnaryOp {
                op: JoinUnaryOp::Not,
                ..
            })
        )),
        "Expected at least one UnaryOp::Not, got {:?}",
        instructions
    );
}

#[test]
fn test_expr_lowerer_var_less_literal_generates_compare() {
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    // i < 10
    let ast = bin(BinaryOperator::Less, var("i"), lit_i(10));

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "i < 10 should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "instructions should not be empty");
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_literal_less_var_generates_compare() {
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    // 0 < i
    let ast = bin(BinaryOperator::Less, lit_i(0), var("i"));

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "0 < i should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "instructions should not be empty");
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_greater_than_between_vars() {
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    // i > j
    let ast = bin(BinaryOperator::Greater, var("i"), var("j"));

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "i > j should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "instructions should not be empty");
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_and_combination() {
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    // i > 0 && j < 5
    let left = bin(BinaryOperator::Greater, var("i"), lit_i(0));
    let right = bin(BinaryOperator::Less, var("j"), lit_i(5));
    let ast = bin(BinaryOperator::And, left, right);

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "i > 0 && j < 5 should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "instructions should not be empty");
    assert_has_compare(&instructions);
    assert_has_binop(&instructions, BinOpKind::And);
}

#[test]
fn test_expr_lowerer_not_of_comparison() {
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    // !(i < 10)
    let inner = bin(BinaryOperator::Less, var("i"), lit_i(10));
    let ast = not(inner);

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "!(i < 10) should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "instructions should not be empty");
    assert_has_compare(&instructions);
    assert_has_not(&instructions);
}

#[test]
fn test_expr_lowerer_loop_break_digit_pos_less_zero_generates_compare() {
    let mut condition_env = ConditionEnv::new();
    condition_env.insert("digit_pos".to_string(), ValueId(10));

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };

    let scope = LoopBreakScopeManager {
        condition_env: &condition_env,
        loop_body_local_env: None,
        captured_env: None,
        carrier_info: &carrier_info,
    };

    let mut builder = create_test_builder();

    // digit_pos < 0
    let ast = bin(BinaryOperator::Less, var("digit_pos"), lit_i(0));
    assert!(
        ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast),
        "digit_pos < 0 should be supported in loop_break condition"
    );

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);
    assert!(result.is_ok(), "digit_pos < 0 should lower successfully");

    let instructions = expr_lowerer.take_last_instructions();
    assert!(
        !instructions.is_empty(),
        "instructions for digit_pos < 0 should not be empty"
    );
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_methodcall_unknown_method_is_rejected() {
    let scope = make_scope_with_p_and_s();
    let mut builder = create_test_builder();

    // Unknown method name should fail through MethodCallLowerer
    let ast = crate::ast::ASTNode::MethodCall {
        object: Box::new(var("s")),
        method: "unknown_method".to_string(),
        arguments: vec![],
        span: span(),
    };

    assert!(
        ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast),
        "MethodCall nodes should be routed to MethodCallLowerer for validation"
    );

    let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);
    let result = expr_lowerer.lower(&ast);

    assert!(
        matches!(result, Err(ExprLoweringError::LoweringError(msg)) if msg.contains("MethodCall")),
        "Unknown method should fail-fast via MethodCallLowerer"
    );
}

// Phase 240-EX: Header condition patterns

#[test]
fn test_expr_lowerer_supports_simple_header_condition_i_less_literal() {
    // header pattern: i < 10
    let ast = bin(BinaryOperator::Less, var("i"), lit_i(10));
    assert!(
        ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast),
        "i < 10 should be supported for loop_break header condition"
    );

    // lower and verify success
    let scope = make_basic_scope();
    let mut builder = create_test_builder();
    let mut lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);

    let result = lowerer.lower(&ast);
    assert!(result.is_ok(), "i < 10 should lower successfully");

    // Compare instruction should be present
    let instructions = lowerer.take_last_instructions();
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_supports_header_condition_var_less_var() {
    // header pattern: i < n (variable vs variable)
    let ast = bin(BinaryOperator::Less, var("i"), var("j"));
    assert!(
        ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast),
        "i < n should be supported for loop_break header condition"
    );

    // lower and verify success
    let scope = make_basic_scope();
    let mut builder = create_test_builder();
    let mut lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);

    let result = lowerer.lower(&ast);
    assert!(result.is_ok(), "i < n should lower successfully");

    // Compare instruction should be present
    let instructions = lowerer.take_last_instructions();
    assert_has_compare(&instructions);
}

#[test]
fn test_expr_lowerer_supports_header_condition_with_length_call() {
    // header pattern: p < s.length()
    let length_call = crate::ast::ASTNode::MethodCall {
        object: Box::new(var("s")),
        method: "length".to_string(),
        arguments: vec![],
        span: span(),
    };
    let ast = bin(BinaryOperator::Less, var("p"), length_call);

    assert!(
        ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(&ast),
        "p < s.length() should be supported for loop_break header condition"
    );

    let scope = make_scope_with_p_and_s();
    let mut builder = create_test_builder();
    let mut lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);

    let result = lowerer.lower(&ast);
    assert!(result.is_ok(), "p < s.length() should lower successfully");

    let instructions = lowerer.take_last_instructions();
    assert_has_compare(&instructions);
    assert!(
        instructions.iter().any(|inst| matches!(
            inst,
            JoinInst::Compute(MirLikeInst::BoxCall { method, .. }) if method == "length"
        )),
        "Expected BoxCall for length receiver in lowered instructions"
    );
}

#[test]
fn test_expr_lowerer_header_condition_generates_expected_instructions() {
    // Test that header condition i < 10 generates proper Compare instruction
    let scope = make_basic_scope();
    let mut builder = create_test_builder();

    let ast = bin(BinaryOperator::Less, var("i"), lit_i(10));
    let mut lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);

    let result = lowerer.lower(&ast);
    assert!(result.is_ok());

    let instructions = lowerer.take_last_instructions();
    assert!(!instructions.is_empty(), "Should generate instructions");

    // Should have Compare instruction
    assert_has_compare(&instructions);
}
