use super::*;
use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateRhs;
use crate::mir::join_ir::lowering::update_env::UpdateEnv;
use crate::mir::join_ir::BinOpKind;
use crate::mir::join_ir::{ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

// Helper: Create a test ConditionEnv
fn test_env() -> ConditionEnv {
    let mut env = ConditionEnv::new();
    env.insert("count".to_string(), ValueId(10));
    env.insert("sum".to_string(), ValueId(20));
    env.insert("i".to_string(), ValueId(30));
    env
}

// Helper: Create a test LoopBodyLocalEnv
fn test_body_local_env() -> LoopBodyLocalEnv {
    let mut env = LoopBodyLocalEnv::new();
    env.insert("temp".to_string(), ValueId(50));
    env.insert("digit".to_string(), ValueId(60));
    env
}

// Helper: Create a test UpdateEnv
fn test_update_env() -> (ConditionEnv, LoopBodyLocalEnv) {
    (test_env(), test_body_local_env())
}

// Helper: Create a test CarrierVar
fn test_carrier(name: &str, host_id: u32) -> CarrierVar {
    CarrierVar {
        name: name.to_string(),
        host_id: ValueId(host_id),
        join_id: None, // Phase 177-STRUCT-1
        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
    }
}

#[test]
fn test_emit_const_update() {
    // Test: count = count + 1 (UpdateExpr::Const)
    let carrier = test_carrier("count", 100);
    let update = UpdateExpr::Const(1);
    let env = test_env();

    let mut value_counter = 50u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_ok());
    let result_id = result.unwrap();

    // Should generate 2 instructions: Const(1) + BinOp(Add)
    assert_eq!(instructions.len(), 2);

    // Instruction 1: Const(1)
    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
            assert_eq!(*dst, ValueId(50)); // First allocated
            assert!(matches!(value, ConstValue::Integer(1)));
        }
        _ => panic!("Expected Const instruction"),
    }

    // Instruction 2: BinOp(Add, count, const_1)
    match &instructions[1] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(51)); // Second allocated
            assert_eq!(*op, BinOpKind::Add);
            assert_eq!(*lhs, ValueId(10)); // count from env
            assert_eq!(*rhs, ValueId(50)); // const_1
        }
        _ => panic!("Expected BinOp instruction"),
    }

    assert_eq!(result_id, ValueId(51));
}

#[test]
fn test_emit_binop_update_with_const() {
    // Test: sum = sum + 5 (UpdateExpr::BinOp with Const RHS)
    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Const(5),
    };
    let env = test_env();

    let mut value_counter = 60u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_ok());
    let result_id = result.unwrap();

    // Should generate 2 instructions: Const(5) + BinOp(Add)
    assert_eq!(instructions.len(), 2);

    // Instruction 1: Const(5)
    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
            assert_eq!(*dst, ValueId(60));
            assert!(matches!(value, ConstValue::Integer(5)));
        }
        _ => panic!("Expected Const instruction"),
    }

    // Instruction 2: BinOp(Add, sum, const_5)
    match &instructions[1] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(61));
            assert_eq!(*op, BinOpKind::Add);
            assert_eq!(*lhs, ValueId(20)); // sum from env
            assert_eq!(*rhs, ValueId(60)); // const_5
        }
        _ => panic!("Expected BinOp instruction"),
    }

    assert_eq!(result_id, ValueId(61));
}

#[test]
fn test_emit_binop_update_with_variable() {
    // Test: sum = sum + i (UpdateExpr::BinOp with Variable RHS)
    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Variable("i".to_string()),
    };
    let env = test_env();

    let mut value_counter = 70u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_ok());
    let result_id = result.unwrap();

    // Should generate 1 instruction: BinOp(Add, sum, i)
    assert_eq!(instructions.len(), 1);

    // Instruction: BinOp(Add, sum, i)
    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(70));
            assert_eq!(*op, BinOpKind::Add);
            assert_eq!(*lhs, ValueId(20)); // sum from env
            assert_eq!(*rhs, ValueId(30)); // i from env
        }
        _ => panic!("Expected BinOp instruction"),
    }

    assert_eq!(result_id, ValueId(70));
}

#[test]
fn test_emit_update_carrier_not_in_env() {
    // Test error case: carrier not found in env
    let carrier = test_carrier("unknown", 300);
    let update = UpdateExpr::Const(1);
    let env = test_env(); // doesn't have "unknown"

    let mut value_counter = 80u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Carrier 'unknown' not found"));
}

#[test]
fn test_emit_update_lhs_mismatch() {
    // Test error case: LHS doesn't match carrier name
    let carrier = test_carrier("count", 100);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(), // Wrong! Should be "count"
        op: BinOpKind::Add,
        rhs: UpdateRhs::Const(1),
    };
    let env = test_env();

    let mut value_counter = 90u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("doesn't match carrier"));
}

#[test]
fn test_emit_update_rhs_variable_not_found() {
    // Test error case: RHS variable not in env
    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Variable("unknown_var".to_string()),
    };
    let env = test_env();

    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result =
        emit_carrier_update(&carrier, &update, &mut alloc_value, &env, &mut instructions);

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Update RHS variable 'unknown_var' not found"));
}

// ============================================================================
// Phase 184: UpdateEnv version tests
// ============================================================================

#[test]
fn test_emit_update_with_env_body_local_variable() {
    // Phase 184: Test using body-local variable in update expression
    // sum = sum + temp (temp is body-local)
    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Variable("temp".to_string()), // Body-local variable
    };

    let (cond_env, body_env) = test_update_env();
    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let mut value_counter = 110u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_ok());
    let result_id = result.unwrap();

    // Should generate 1 instruction: BinOp(Add, sum, temp)
    assert_eq!(instructions.len(), 1);

    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(110));
            assert_eq!(*op, BinOpKind::Add);
            assert_eq!(*lhs, ValueId(20)); // sum from condition env
            assert_eq!(*rhs, ValueId(50)); // temp from body-local env
        }
        _ => panic!("Expected BinOp instruction"),
    }

    assert_eq!(result_id, ValueId(110));
}

#[test]
fn test_emit_update_with_env_condition_priority() {
    // Phase 184: Test condition variable takes priority over body-local
    // If both envs have "x", condition env should win
    let mut cond_env = ConditionEnv::new();
    cond_env.insert("x".to_string(), ValueId(100)); // Condition: x=100
    cond_env.insert("sum".to_string(), ValueId(20));

    let mut body_env = LoopBodyLocalEnv::new();
    body_env.insert("x".to_string(), ValueId(200)); // Body-local: x=200 (should be ignored)

    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Variable("x".to_string()),
    };

    let mut value_counter = 120u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_ok());

    // Should use x=100 (condition env), not x=200 (body-local env)
    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BinOp {
            dst: _,
            op: _,
            lhs: _,
            rhs,
        }) => {
            assert_eq!(*rhs, ValueId(100)); // Condition env wins
        }
        _ => panic!("Expected BinOp instruction"),
    }
}

#[test]
fn test_emit_update_with_env_variable_not_found() {
    // Phase 184: Test error when variable not in either env
    let (cond_env, body_env) = test_update_env();
    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let carrier = test_carrier("sum", 200);
    let update = UpdateExpr::BinOp {
        lhs: "sum".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::Variable("nonexistent".to_string()),
    };

    let mut value_counter = 130u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Update RHS variable 'nonexistent' not found"));
    assert!(err.contains("neither condition nor body-local"));
}

#[test]
fn test_emit_update_with_env_const_update() {
    // Phase 184: Test UpdateEnv with simple const update (baseline)
    let (cond_env, body_env) = test_update_env();
    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let carrier = test_carrier("count", 100);
    let update = UpdateExpr::Const(1);

    let mut value_counter = 140u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_ok());
    assert_eq!(instructions.len(), 2); // Const + BinOp
}

#[test]
fn test_emit_number_accumulation_base10() {
    // Phase 190: Test number accumulation pattern: result = result * 10 + digit
    let mut cond_env = ConditionEnv::new();
    cond_env.insert("result".to_string(), ValueId(20)); // Carrier parameter
    cond_env.insert("digit".to_string(), ValueId(30)); // Digit variable

    let body_env = LoopBodyLocalEnv::new();
    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let carrier = test_carrier("result", 200);
    let update = UpdateExpr::BinOp {
        lhs: "result".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::NumberAccumulation {
            base: 10,
            digit_var: "digit".to_string(),
        },
    };

    let mut value_counter = 150u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_ok());
    let result_id = result.unwrap();

    // Should generate 3 instructions:
    // 1. Const(10) for base
    // 2. BinOp(Mul, result, base) for tmp
    // 3. BinOp(Add, tmp, digit) for final result
    assert_eq!(instructions.len(), 3);

    // Instruction 1: Const(10)
    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
            assert_eq!(*dst, ValueId(150)); // First allocated
            assert!(matches!(value, ConstValue::Integer(10)));
        }
        _ => panic!("Expected Const instruction"),
    }

    // Instruction 2: BinOp(Mul, result, base)
    match &instructions[1] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(151)); // Second allocated (tmp)
            assert_eq!(*op, BinOpKind::Mul);
            assert_eq!(*lhs, ValueId(20)); // result from env
            assert_eq!(*rhs, ValueId(150)); // base const
        }
        _ => panic!("Expected BinOp(Mul) instruction"),
    }

    // Instruction 3: BinOp(Add, tmp, digit)
    match &instructions[2] {
        JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
            assert_eq!(*dst, ValueId(152)); // Third allocated (final result)
            assert_eq!(*op, BinOpKind::Add);
            assert_eq!(*lhs, ValueId(151)); // tmp from previous mul
            assert_eq!(*rhs, ValueId(30)); // digit from env
        }
        _ => panic!("Expected BinOp(Add) instruction"),
    }

    assert_eq!(result_id, ValueId(152));
}

#[test]
fn test_emit_number_accumulation_digit_not_found() {
    // Phase 190: Test error when digit variable not in env
    let mut cond_env = ConditionEnv::new();
    cond_env.insert("result".to_string(), ValueId(20));
    // Note: digit NOT in env

    let body_env = LoopBodyLocalEnv::new();
    let promoted: Vec<String> = vec![];
    let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

    let carrier = test_carrier("result", 200);
    let update = UpdateExpr::BinOp {
        lhs: "result".to_string(),
        op: BinOpKind::Add,
        rhs: UpdateRhs::NumberAccumulation {
            base: 10,
            digit_var: "digit".to_string(),
        },
    };

    let mut value_counter = 160u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };

    let mut instructions = Vec::new();
    let result = emit_carrier_update_with_env(
        &carrier,
        &update,
        &mut alloc_value,
        &update_env,
        &mut instructions,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Number accumulation digit variable 'digit' not found"));
}
