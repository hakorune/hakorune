use crate::backend::mir_interpreter::MirInterpreter;
use crate::backend::vm_types::VMValue;
use crate::mir::exact_numeric_value_facts::refresh_module_exact_numeric_value_facts;
use crate::mir::function::MirParamDecl;
use crate::mir::{
    BasicBlockId, BinaryOp, CompareOp, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType,
};

fn module_with_exact_numeric_arithmetic_route(declared_type_name: &str, op: BinaryOp) -> MirModule {
    let entry = BasicBlockId::new(0);
    let signature = FunctionSignature {
        name: "Main.arithmetic/2".to_string(),
        params: vec![MirType::Integer, MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, entry);
    let lhs = function.params[0];
    let rhs = function.params[1];
    let sum = function.next_value_id();
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "lhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
        MirParamDecl {
            name: "rhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
    ];

    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::BinOp {
        dst: sum,
        op,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return { value: Some(sum) });

    let mut module = MirModule::new("vm_exact_numeric_add_test".to_string());
    module.add_function(function);
    refresh_module_exact_numeric_value_facts(&mut module);
    let route_count = module
        .functions
        .get("Main.arithmetic/2")
        .expect("test function must exist")
        .metadata
        .exact_numeric_binary_op_route_facts
        .len();
    assert_eq!(route_count, 1);
    module
}

fn module_with_chained_exact_numeric_add_route(declared_type_name: &str) -> MirModule {
    let entry = BasicBlockId::new(0);
    let signature = FunctionSignature {
        name: "Main.chained_add/2".to_string(),
        params: vec![MirType::Integer, MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, entry);
    let lhs = function.params[0];
    let rhs = function.params[1];
    let first = function.next_value_id();
    let second = function.next_value_id();
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "lhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
        MirParamDecl {
            name: "rhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
    ];

    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::BinOp {
        dst: first,
        op: BinaryOp::Add,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: second,
        op: BinaryOp::Add,
        lhs: first,
        rhs,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(second),
    });

    let mut module = MirModule::new("vm_exact_numeric_chained_add_test".to_string());
    module.add_function(function);
    refresh_module_exact_numeric_value_facts(&mut module);
    let route_count = module
        .functions
        .get("Main.chained_add/2")
        .expect("test function must exist")
        .metadata
        .exact_numeric_binary_op_route_facts
        .len();
    assert_eq!(route_count, 2);
    module
}

fn assert_exact_numeric(value: VMValue, source_name: &str, expected: i128) {
    match value {
        VMValue::ExactNumeric(exact) => {
            assert_eq!(exact.source_name, source_name);
            assert_eq!(exact.value, expected);
        }
        other => panic!("expected exact numeric value, got {:?}", other),
    }
}

fn module_with_exact_numeric_compare_route(declared_type_name: &str, op: CompareOp) -> MirModule {
    let entry = BasicBlockId::new(0);
    let signature = FunctionSignature {
        name: "Main.compare/2".to_string(),
        params: vec![MirType::Integer, MirType::Integer],
        return_type: MirType::Bool,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, entry);
    let lhs = function.params[0];
    let rhs = function.params[1];
    let result = function.next_value_id();
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "lhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
        MirParamDecl {
            name: "rhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
    ];

    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::Compare {
        dst: result,
        op,
        lhs,
        rhs,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(result),
    });
    let mut module = MirModule::new("exact_numeric_vm_reference_test".to_string());
    module.add_function(function);

    refresh_module_exact_numeric_value_facts(&mut module);
    let route_count = module
        .functions
        .get("Main.compare/2")
        .expect("test function must exist")
        .metadata
        .exact_numeric_compare_route_facts
        .len();
    assert_eq!(
        route_count, 1,
        "test module must publish one exact compare route"
    );
    module
}

fn module_with_exact_numeric_shift_route(declared_type_name: &str) -> MirModule {
    let entry = BasicBlockId::new(0);
    let signature = FunctionSignature {
        name: "Main.shift/2".to_string(),
        params: vec![MirType::Integer, MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, entry);
    let lhs = function.params[0];
    let rhs = function.params[1];
    let result = function.next_value_id();
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "lhs".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
        },
        MirParamDecl {
            name: "rhs".to_string(),
            declared_type_name: None,
        },
    ];

    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::BinOp {
        dst: result,
        op: BinaryOp::Shr,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });

    let mut module = MirModule::new("exact_numeric_vm_shift_reference_test".to_string());
    module.add_function(function);
    refresh_module_exact_numeric_value_facts(&mut module);
    let route_count = module
        .functions
        .get("Main.shift/2")
        .expect("test function must exist")
        .metadata
        .exact_numeric_shift_route_facts
        .len();
    assert_eq!(
        route_count, 1,
        "test module must publish one exact shift route"
    );
    module
}

#[test]
fn vm_reference_executes_exact_usize_add_route() {
    let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(40), VMValue::Integer(2)],
        )
        .expect("exact usize add route should execute");

    assert_exact_numeric(result, "usize", 42);
}

#[test]
fn vm_reference_executes_exact_usize_sub_route() {
    let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Sub);
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(40), VMValue::Integer(2)],
        )
        .expect("exact usize sub route should execute");

    assert_exact_numeric(result, "usize", 38);
}

#[test]
fn vm_reference_rejects_negative_usize_add_operand() {
    let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(-1), VMValue::Integer(2)],
        )
        .expect_err("negative usize operand must fail before generic i64 add");

    assert!(error.to_string().contains("[vm/exact_numeric_op_range]"));
    assert!(error.to_string().contains("negative-to-unsigned"));
}

#[test]
fn vm_reference_rejects_exact_u8_add_overflow() {
    let module = module_with_exact_numeric_arithmetic_route("u8", BinaryOp::Add);
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(250), VMValue::Integer(10)],
        )
        .expect_err("u8 exact add overflow must fail before generic i64 add");

    assert!(error.to_string().contains("[vm/exact_numeric_op_overflow]"));
}

#[test]
fn vm_reference_rejects_exact_u8_mul_overflow() {
    let module = module_with_exact_numeric_arithmetic_route("u8", BinaryOp::Mul);
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(16), VMValue::Integer(16)],
        )
        .expect_err("u8 exact mul overflow must fail before generic i64 mul");

    assert!(error.to_string().contains("[vm/exact_numeric_op_overflow]"));
}

#[test]
fn vm_reference_keeps_exact_usize_result_outside_current_i64_lane() {
    let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(i64::MAX), VMValue::Integer(1)],
        )
        .expect("usize result above i64 should stay in exact numeric runtime value");

    assert_exact_numeric(result, "usize", i128::from(i64::MAX) + 1);
}

#[test]
fn vm_reference_consumes_chained_exact_usize_result() {
    let module = module_with_chained_exact_numeric_add_route("usize");
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.chained_add/2",
            &[VMValue::Integer(i64::MAX), VMValue::Integer(1)],
        )
        .expect("second exact usize add should consume the first tagged exact result");

    assert_exact_numeric(result, "usize", i128::from(i64::MAX) + 2);
}

#[test]
fn vm_reference_executes_exact_usize_compare_route() {
    let module = module_with_exact_numeric_compare_route("usize", CompareOp::Lt);
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.compare/2",
            &[VMValue::Integer(2), VMValue::Integer(40)],
        )
        .expect("exact usize compare route should execute");

    assert_eq!(result, VMValue::Bool(true));
}

#[test]
fn vm_reference_rejects_negative_usize_compare_operand() {
    let module = module_with_exact_numeric_compare_route("usize", CompareOp::Lt);
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.compare/2",
            &[VMValue::Integer(-1), VMValue::Integer(2)],
        )
        .expect_err("negative usize compare operand must fail before generic i64 compare");

    assert!(error.to_string().contains("[vm/exact_numeric_op_range]"));
}

#[test]
fn vm_reference_executes_exact_usize_logical_shift_route() {
    let module = module_with_exact_numeric_shift_route("usize");
    let mut vm = MirInterpreter::new();

    let result = vm
        .execute_function_with_args(
            &module,
            "Main.shift/2",
            &[VMValue::Integer(40), VMValue::Integer(3)],
        )
        .expect("exact usize logical shift route should execute");

    assert_exact_numeric(result, "usize", 5);
}

#[test]
fn vm_reference_rejects_negative_exact_usize_shift_count() {
    let module = module_with_exact_numeric_shift_route("usize");
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.shift/2",
            &[VMValue::Integer(40), VMValue::Integer(-1)],
        )
        .expect_err("negative exact usize shift count must fail before generic i64 shr");

    assert!(error.to_string().contains("[vm/exact_numeric_shift_count]"));
}
