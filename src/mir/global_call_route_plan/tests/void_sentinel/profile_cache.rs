use super::*;

use crate::mir::global_call_route_plan::string_return_profile::generic_string_return_profile_test_cache_key;

fn integer_passthrough_with_global_child(child_name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Helper.parent/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(child_name.to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    function
}

fn integer_leaf(name: &str, value: i64) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(value),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    function
}

fn integer_with_hostbridge_surface() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Helper.hostbridge/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut hostbridge_block = BasicBlock::new(BasicBlockId::new(1));
    hostbridge_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("env.mirbuilder".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("emit".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("hostbridge.extern_invoke".to_string())),
            args: vec![ValueId::new(2), ValueId::new(3), ValueId::new(4)],
            effects: EffectMask::IO,
        },
    ]);
    hostbridge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    function
        .blocks
        .insert(BasicBlockId::new(1), hostbridge_block);
    function.blocks.insert(BasicBlockId::new(2), void_block);
    function
}

#[test]
fn string_return_profile_cache_key_changes_when_callee_contract_changes() {
    let function = integer_passthrough_with_global_child("Helper.child/0");
    let mut integer_targets = BTreeMap::new();
    integer_targets.insert(
        "Helper.child/0".to_string(),
        GlobalCallTargetFacts::present_with_shape(0, GlobalCallTargetShape::GenericI64Body),
    );
    let mut string_targets = BTreeMap::new();
    string_targets.insert(
        "Helper.child/0".to_string(),
        GlobalCallTargetFacts::present_with_shape(0, GlobalCallTargetShape::GenericPureStringBody),
    );

    let integer_key = generic_string_return_profile_test_cache_key(&function, &integer_targets);
    let string_key = generic_string_return_profile_test_cache_key(&function, &string_targets);

    assert_ne!(integer_key, string_key);
}

#[test]
fn refresh_module_global_call_routes_keeps_integer_leaf_on_i64_path() {
    let mut module = MirModule::new("integer_leaf_precheck_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.count/0", Some(ValueId::new(7)), vec![]);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Helper.count/0".to_string(),
        integer_leaf("Helper.count/0", 42),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
    assert_eq!(route.proof(), "typed_global_call_leaf_numeric_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_does_not_skip_integer_hostbridge_surface() {
    let mut module = MirModule::new("integer_hostbridge_surface_precheck_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.hostbridge/0", Some(ValueId::new(7)), vec![]);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Helper.hostbridge/0".to_string(),
        integer_with_hostbridge_surface(),
    );
    crate::mir::extern_call_route_plan::refresh_module_extern_call_routes(&mut module);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
