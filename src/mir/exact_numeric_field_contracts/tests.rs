use super::*;
use crate::mir::{EffectMask, FunctionSignature, MirFunction, MirModule, UserBoxFieldDecl};

fn module_with_numeric_field(declared_type_name: &str, function: MirFunction) -> MirModule {
    let mut module = MirModule::new("test".to_string());
    module.metadata.user_box_field_decls.insert(
        "Page".to_string(),
        vec![UserBoxFieldDecl {
            name: "capacity".to_string(),
            declared_type_name: Some(declared_type_name.to_string()),
            is_weak: false,
        }],
    );
    module.add_function(function);
    module
}

fn field_set_param_value_function() -> MirFunction {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(signature, entry);
    let value_param = function.params[0];
    let object = function.next_value_id();

    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::NewBox {
        dst: object,
        box_type: "Page".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: object,
        field: "capacity".to_string(),
        value: value_param,
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Return { value: None });
    function
}

fn field_set_const_value_function(value: i64) -> MirFunction {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(signature, entry);
    let object = function.next_value_id();
    let stored_value = function.next_value_id();
    let block = function.get_block_mut(entry).unwrap();
    block.add_instruction(MirInstruction::NewBox {
        dst: object,
        box_type: "Page".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: stored_value,
        value: ConstValue::Integer(value),
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: object,
        field: "capacity".to_string(),
        value: stored_value,
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Return { value: None });
    function
}

#[test]
fn attaches_dynamic_integer_range_contract_for_usize_field_param_write() {
    let mut module = module_with_numeric_field("usize", field_set_param_value_function());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );

    let contract = &module
        .get_function("main")
        .unwrap()
        .metadata
        .exact_numeric_runtime_check_contracts[0];
    assert_eq!(contract.block, BasicBlockId::new(0));
    assert_eq!(contract.instruction_index, 1);
    assert_eq!(contract.field, "capacity");
    assert_eq!(contract.value, ValueId::new(0));
    assert_eq!(contract.declared_type_name, "usize");
    assert_eq!(
        contract.kind,
        ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
    );
}

#[test]
fn attaches_type_check_contract_for_dynamic_i64_without_parameter_proof() {
    let mut module = module_with_numeric_field("i64", field_set_param_value_function());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );
    assert_eq!(
        module
            .get_function("main")
            .unwrap()
            .metadata
            .exact_numeric_runtime_check_contracts[0]
            .declared_type_name,
        "i64"
    );
}

#[test]
fn rebuilds_dynamic_integer_range_contract_without_duplication() {
    let mut module = module_with_numeric_field("usize", field_set_param_value_function());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );
    assert_eq!(
        module
            .get_function("main")
            .unwrap()
            .metadata
            .exact_numeric_runtime_check_contracts
            .len(),
        1
    );
}

#[test]
fn semantic_refresh_rebuilds_proof_after_declared_type_drift() {
    let mut module = module_with_numeric_field("usize", field_set_const_value_function(42));
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        0
    );
    assert_eq!(
        module
            .get_function("main")
            .unwrap()
            .metadata
            .exact_numeric_field_contract_proofs[0]
            .expected_type,
        "usize"
    );

    module
        .metadata
        .user_box_field_decls
        .get_mut("Page")
        .unwrap()[0]
        .declared_type_name = Some("u8".to_string());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        0
    );
    let proofs = &module
        .get_function("main")
        .unwrap()
        .metadata
        .exact_numeric_field_contract_proofs;
    assert_eq!(proofs.len(), 1);
    assert_eq!(proofs[0].expected_type, "u8");
}

#[test]
fn unsupported_backend_guard_allows_module_without_runtime_check_contracts() {
    let module = module_with_numeric_field("i64", field_set_param_value_function());
    assert!(enforce_exact_numeric_runtime_checks_supported(&module, "wasm").is_ok());
}

#[test]
fn unsupported_backend_guard_rejects_dynamic_range_contracts() {
    let mut module = module_with_numeric_field("usize", field_set_param_value_function());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );

    let err = enforce_exact_numeric_runtime_checks_supported(&module, "wasm").unwrap_err();
    assert!(err.contains(EXACT_NUMERIC_RUNTIME_CHECK_UNSUPPORTED_BACKEND_TAG));
    assert!(err.contains("backend=wasm"));
    assert!(err.contains("contracts=1"));
}

#[test]
fn backend_guard_accepts_ny_llvmc_exe_runtime_check_lowering() {
    let mut module = module_with_numeric_field("usize", field_set_param_value_function());
    assert_eq!(
        refresh_module_exact_numeric_runtime_check_contracts(&mut module),
        1
    );
    assert!(enforce_exact_numeric_runtime_checks_supported(&module, "ny-llvmc-exe").is_ok());
}
