use super::*;
use crate::backend::vm_types::ExactNumericRuntimeValue;
use crate::mir::function::MirParamDecl;
use crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts;
use crate::mir::{
    BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};
use std::collections::HashMap;

fn parameter_function(declared_type_name: Option<&str>) -> MirFunction {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.identity/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "value".to_string(),
        declared_type_name: declared_type_name.map(str::to_string),
        implicit_receiver: false,
    }];
    let parameter = function.params[0];
    function
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Return {
            value: Some(parameter),
        });
    refresh_function_parameter_entry_contracts(&mut function);
    function
}

fn module_with(function: MirFunction) -> MirModule {
    let mut module = MirModule::new("parameter-contract-test".to_string());
    module.add_function(function);
    module
}

fn caller_of(callee: &str, argument: ConstValue) -> MirFunction {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.caller/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let argument_id = function.next_value_id();
    let result_id = function.next_value_id();
    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: argument_id,
        value: argument,
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(result_id),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(callee.to_string())),
        args: vec![argument_id],
        effects: EffectMask::PURE,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result_id),
    });
    function
}

fn parameter_method(class_name: &str, declared_type_name: &str) -> MirFunction {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: format!("{class_name}.take/1"),
            params: vec![MirType::Box(class_name.to_string()), MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "me".to_string(),
            declared_type_name: None,
            implicit_receiver: true,
        },
        MirParamDecl {
            name: "value".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
            implicit_receiver: false,
        },
    ];
    let value = function.params[1];
    function
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(value) });
    refresh_function_parameter_entry_contracts(&mut function);
    function
}

#[test]
fn exact_numeric_entry_accepts_integer_and_matching_exact_value() {
    let module = module_with(parameter_function(Some("u8")));
    let mut vm = MirInterpreter::new();
    assert!(vm
        .execute_function_with_args(&module, "Main.identity/1", &[VMValue::Integer(42)],)
        .is_ok());
    assert!(vm
        .execute_function_with_args(
            &module,
            "Main.identity/1",
            &[VMValue::ExactNumeric(ExactNumericRuntimeValue::new(
                "u8", 42
            ))],
        )
        .is_ok());
}

#[test]
fn exact_numeric_entry_rejects_wrong_type_and_range_before_binding() {
    let module = module_with(parameter_function(Some("u8")));
    let mut vm = MirInterpreter::new();
    for argument in [VMValue::String("no".to_string()), VMValue::Integer(256)] {
        let error = vm
            .execute_function_with_args(&module, "Main.identity/1", &[argument])
            .unwrap_err()
            .to_string();
        assert!(error.contains(PARAMETER_CONTRACT_VIOLATION_TAG), "{error}");
    }
}

#[test]
fn contracted_function_requires_complete_exact_arity() {
    let module = module_with(parameter_function(Some("i64")));
    let mut vm = MirInterpreter::new();
    for arguments in [vec![], vec![VMValue::Integer(1), VMValue::Integer(2)]] {
        let error = vm
            .execute_function_with_args(&module, "Main.identity/1", &arguments)
            .unwrap_err()
            .to_string();
        assert!(error.contains(PARAMETER_ARITY_MISMATCH_TAG), "{error}");
    }
}

#[test]
fn one_contract_requires_arity_for_the_complete_formal_parameter_list() {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.mixed/2".to_string(),
            params: vec![MirType::Integer, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "count".to_string(),
            declared_type_name: Some("u8".to_string()),
            implicit_receiver: false,
        },
        MirParamDecl {
            name: "payload".to_string(),
            declared_type_name: None,
            implicit_receiver: false,
        },
    ];
    function
        .get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: None });
    refresh_function_parameter_entry_contracts(&mut function);
    let module = module_with(function);

    let error = MirInterpreter::new()
        .execute_function_with_args(&module, "Main.mixed/2", &[VMValue::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(error.contains(PARAMETER_ARITY_MISMATCH_TAG), "{error}");
}

#[test]
fn unannotated_function_retains_legacy_missing_argument_behavior() {
    let module = module_with(parameter_function(None));
    let mut vm = MirInterpreter::new();
    let value = vm
        .execute_function_with_args(&module, "Main.identity/1", &[])
        .unwrap();
    assert!(matches!(value, VMValue::Void));
}

#[test]
fn nested_mir_call_uses_the_same_final_callee_entry_owner() {
    let callee = parameter_function(Some("u8"));
    let caller = caller_of("Main.identity/1", ConstValue::String("bad".to_string()));
    let mut module = module_with(callee);
    module.add_function(caller);

    let error = MirInterpreter::new()
        .execute_function_with_args(&module, "Main.caller/0", &[])
        .unwrap_err()
        .to_string();
    assert!(error.contains(PARAMETER_CONTRACT_VIOLATION_TAG), "{error}");
}

#[test]
fn recursive_mir_call_rechecks_the_final_callee_contract() {
    let mut recursive = parameter_function(Some("u8"));
    let entry = recursive.entry_block;
    let bad_argument = recursive.next_value_id();
    let result = recursive.next_value_id();
    let block = recursive.get_block_mut(entry).unwrap();
    block.instructions.clear();
    block.instruction_spans.clear();
    block.terminator = None;
    block.terminator_span = None;
    block.add_instruction(MirInstruction::Const {
        dst: bad_argument,
        value: ConstValue::String("bad".to_string()),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(result),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Main.identity/1".to_string())),
        args: vec![bad_argument],
        effects: EffectMask::PURE,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    let module = module_with(recursive);

    let error = MirInterpreter::new()
        .execute_function_with_args(&module, "Main.identity/1", &[VMValue::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(error.contains(PARAMETER_CONTRACT_VIOLATION_TAG), "{error}");
}

#[test]
fn method_reroute_checks_only_the_final_callee_contract() {
    let mut module = module_with(parameter_method("Base", "u8"));
    module.add_function(parameter_method("Child", "i64"));
    let receiver = crate::instance_v2::InstanceBox::from_declaration(
        "Child".to_string(),
        vec![],
        HashMap::new(),
    );
    let arguments = [
        VMValue::from_nyash_box(Box::new(receiver)),
        VMValue::Integer(300),
    ];

    let value = MirInterpreter::new()
        .execute_function_with_args(&module, "Base.take/1", &arguments)
        .expect("final Child i64 contract must accept 300");
    assert!(matches!(value, VMValue::Integer(300)));
}
