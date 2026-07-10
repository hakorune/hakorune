use super::*;
use crate::backend::mir_interpreter::MirInterpreter;
use crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract;
use crate::mir::{BasicBlock, CompareOp};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};
use crate::mir::{BinaryOp, Callee, ConstValue, MirInstruction, MirModule, ValueId};
use std::collections::HashMap;

fn function_with_contract(declared: Option<&str>) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.value/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_return_type_name = declared.map(str::to_string);
    refresh_function_return_exit_contract(&mut function);
    function
}

#[test]
fn exact_numeric_return_owner_accepts_values_and_rejects_void_or_mismatch() {
    let interpreter = MirInterpreter::new();
    let function = function_with_contract(Some("u8"));
    assert!(interpreter
        .validate_function_return_contract(&function, &VMValue::Integer(255))
        .is_ok());

    let range_error = interpreter
        .validate_function_return_contract(&function, &VMValue::Integer(256))
        .unwrap_err();
    assert!(format!("{range_error:?}").contains(RETURN_CONTRACT_VIOLATION_TAG));

    let void_error = interpreter
        .validate_function_return_contract(&function, &VMValue::Void)
        .unwrap_err();
    assert!(format!("{void_error:?}").contains(RETURN_CONTRACT_VOID_TAG));
}

#[test]
fn unannotated_return_keeps_existing_dynamic_behavior() {
    let interpreter = MirInterpreter::new();
    let function = function_with_contract(None);
    assert!(interpreter
        .validate_function_return_contract(&function, &VMValue::Void)
        .is_ok());
}

fn returning_function(value: Option<ConstValue>, declared: Option<&str>) -> MirFunction {
    let mut function = function_with_contract(declared);
    let return_value = value.map(|value| {
        let id = function.next_value_id();
        function
            .get_block_mut(function.entry_block)
            .unwrap()
            .add_instruction(MirInstruction::Const { dst: id, value });
        id
    });
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::Return {
            value: return_value,
        });
    function
}

fn module_with(functions: Vec<MirFunction>) -> MirModule {
    let mut module = MirModule::new("return-contract-runtime".to_string());
    for function in functions {
        module.add_function(function);
    }
    module
}

#[test]
fn final_block_outcome_checks_runtime_value_before_publication() {
    let valid = returning_function(Some(ConstValue::Integer(255)), Some("u8"));
    let value = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![valid]), "Main.value/0", &[])
        .unwrap();
    assert!(matches!(value, VMValue::Integer(255)));

    let invalid = returning_function(Some(ConstValue::Integer(256)), Some("u8"));
    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![invalid]), "Main.value/0", &[])
        .unwrap_err()
        .to_string();
    assert!(error.contains(RETURN_CONTRACT_VIOLATION_TAG), "{error}");

    let void_return = returning_function(None, Some("u8"));
    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![void_return]), "Main.value/0", &[])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("[type/return_contract_fallthrough_forbidden]"),
        "{error}"
    );
}

#[test]
fn contract_failure_restores_call_depth_and_runtime_error_remains_primary() {
    let invalid = returning_function(Some(ConstValue::Integer(256)), Some("u8"));
    let mut vm = MirInterpreter::new();
    let _ = vm.execute_function_with_args(&module_with(vec![invalid]), "Main.value/0", &[]);
    assert_eq!(vm.call_depth, 0);

    let mut faulting = function_with_contract(Some("i64"));
    let lhs = faulting.next_value_id();
    let rhs = faulting.next_value_id();
    let result = faulting.next_value_id();
    let block = faulting.get_block_mut(faulting.entry_block).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: lhs,
        value: ConstValue::Integer(1),
    });
    block.add_instruction(MirInstruction::Const {
        dst: rhs,
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: result,
        op: BinaryOp::Div,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![faulting]), "Main.value/0", &[])
        .unwrap_err()
        .to_string();
    assert!(!error.contains(RETURN_CONTRACT_VIOLATION_TAG), "{error}");
    assert!(!error.contains(RETURN_CONTRACT_VOID_TAG), "{error}");
}

#[test]
fn nested_ignored_result_still_uses_final_callee_return_owner() {
    let callee = returning_function(Some(ConstValue::String("bad".to_string())), Some("u8"));
    let entry = BasicBlockId::new(0);
    let mut caller = MirFunction::new(
        FunctionSignature {
            name: "Main.caller/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        entry,
    );
    caller
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Main.value/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    caller
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: None });

    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![callee, caller]), "Main.caller/0", &[])
        .unwrap_err()
        .to_string();
    assert!(error.contains(RETURN_CONTRACT_VIOLATION_TAG), "{error}");
}

#[test]
fn vm_preflight_rejects_reachable_fallthrough_for_active_contract() {
    let function = function_with_contract(Some("i64"));
    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![function]), "Main.value/0", &[])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("[type/return_contract_fallthrough_forbidden]"),
        "{error}"
    );
}

#[test]
fn recursive_call_checks_the_innermost_final_return() {
    let entry = BasicBlockId::new(0);
    let base = BasicBlockId::new(1);
    let recurse = BasicBlockId::new(2);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.recur/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.metadata.declared_return_type_name = Some("u8".to_string());
    refresh_function_return_exit_contract(&mut function);
    let n = function.params[0];
    let zero = function.next_value_id();
    let is_zero = function.next_value_id();
    let one = function.next_value_id();
    let next = function.next_value_id();
    let nested = function.next_value_id();
    let invalid = function.next_value_id();
    let entry_block = function.get_block_mut(entry).unwrap();
    entry_block.add_instruction(MirInstruction::Const {
        dst: zero,
        value: ConstValue::Integer(0),
    });
    entry_block.add_instruction(MirInstruction::Compare {
        dst: is_zero,
        op: CompareOp::Eq,
        lhs: n,
        rhs: zero,
    });
    entry_block.add_instruction(MirInstruction::Branch {
        condition: is_zero,
        then_bb: base,
        else_bb: recurse,
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut base_block = BasicBlock::new(base);
    base_block.add_instruction(MirInstruction::Const {
        dst: invalid,
        value: ConstValue::Integer(256),
    });
    base_block.add_instruction(MirInstruction::Return {
        value: Some(invalid),
    });
    function.add_block(base_block);
    let mut recurse_block = BasicBlock::new(recurse);
    recurse_block.add_instruction(MirInstruction::Const {
        dst: one,
        value: ConstValue::Integer(1),
    });
    recurse_block.add_instruction(MirInstruction::BinOp {
        dst: next,
        op: BinaryOp::Sub,
        lhs: n,
        rhs: one,
    });
    recurse_block.add_instruction(MirInstruction::Call {
        dst: Some(nested),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Main.recur/1".to_string())),
        args: vec![next],
        effects: EffectMask::PURE,
    });
    recurse_block.add_instruction(MirInstruction::Return {
        value: Some(nested),
    });
    function.add_block(recurse_block);

    let error = MirInterpreter::new()
        .execute_function_with_args(
            &module_with(vec![function]),
            "Main.recur/1",
            &[VMValue::Integer(1)],
        )
        .unwrap_err()
        .to_string();
    assert!(error.contains(RETURN_CONTRACT_VIOLATION_TAG), "{error}");
}

fn return_method(class_name: &str, value: i64, declared_type: &str) -> MirFunction {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: format!("{class_name}.value/0"),
            params: vec![MirType::Box(class_name.to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.metadata.declared_return_type_name = Some(declared_type.to_string());
    refresh_function_return_exit_contract(&mut function);
    let result = function.next_value_id();
    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: result,
        value: ConstValue::Integer(value),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function
}

#[test]
fn method_reroute_uses_only_the_final_callee_return_contract() {
    let mut module = module_with(vec![return_method("Base", 256, "u8")]);
    module.add_function(return_method("Child", 256, "i64"));
    let receiver = crate::instance_v2::InstanceBox::from_declaration(
        "Child".to_string(),
        vec![],
        HashMap::new(),
    );
    let value = MirInterpreter::new()
        .execute_function_with_args(
            &module,
            "Base.value/0",
            &[VMValue::from_nyash_box(Box::new(receiver))],
        )
        .expect("final Child i64 return contract must accept 256");
    assert!(matches!(value, VMValue::Integer(256)));
}

#[test]
fn cleanup_cfg_final_return_is_the_only_checked_value() {
    let entry = BasicBlockId::new(0);
    let cleanup_exit = BasicBlockId::new(1);
    let mut function = function_with_contract(Some("u8"));
    function
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Jump {
            target: cleanup_exit,
            edge_args: None,
        });
    let result = function.next_value_id();
    let mut cleanup_block = BasicBlock::new(cleanup_exit);
    cleanup_block.add_instruction(MirInstruction::Const {
        dst: result,
        value: ConstValue::String("cleanup-result".to_string()),
    });
    cleanup_block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function.add_block(cleanup_block);

    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with(vec![function]), "Main.value/0", &[])
        .unwrap_err()
        .to_string();
    assert!(error.contains(RETURN_CONTRACT_VIOLATION_TAG), "{error}");
}
