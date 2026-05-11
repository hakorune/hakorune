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

    assert_eq!(result, VMValue::Integer(42));
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

    assert_eq!(result, VMValue::Integer(38));
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
fn vm_reference_rejects_exact_usize_result_outside_current_i64_lane() {
    let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
    let mut vm = MirInterpreter::new();

    let error = vm
        .execute_function_with_args(
            &module,
            "Main.arithmetic/2",
            &[VMValue::Integer(i64::MAX), VMValue::Integer(1)],
        )
        .expect_err("usize result above i64 must fail until exact VMValue storage exists");

    assert!(error
        .to_string()
        .contains("[vm/exact_numeric_op_result_unrepresentable]"));
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
