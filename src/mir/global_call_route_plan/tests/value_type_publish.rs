use super::*;

#[test]
fn refresh_module_global_call_routes_publishes_string_handle_args_to_target_params() {
    let mut module = MirModule::new("global_call_param_value_type_publish_test".to_string());

    let mut caller = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let caller_entry = caller.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    caller_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("{\"user\":\"ana\"}".to_string()),
    });
    caller_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::String("user".to_string()),
    });
    caller_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("JsonLine.stringField/2".to_string())),
        args: vec![ValueId::new(2), ValueId::new(3)],
        effects: EffectMask::PURE,
    });
    caller_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    caller
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::String);
    caller
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::String);

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "JsonLine.stringField/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0), ValueId::new(1)];
    let callee_entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    callee_entry.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(1),
    });
    callee_entry.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(9),
        src: ValueId::new(4),
    });
    callee_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(8),
        value: ConstValue::String("\"".to_string()),
    });
    callee_entry.instructions.push(MirInstruction::BinOp {
        dst: ValueId::new(7),
        op: BinaryOp::Add,
        lhs: ValueId::new(8),
        rhs: ValueId::new(9),
    });
    callee_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });
    callee
        .metadata
        .value_types
        .insert(ValueId::new(8), MirType::String);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("JsonLine.stringField/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let callee = &module.functions["JsonLine.stringField/2"];
    let string_box = MirType::Box("StringBox".to_string());
    assert_eq!(
        callee.metadata.value_types.get(&ValueId::new(0)),
        Some(&string_box)
    );
    assert_eq!(
        callee.metadata.value_types.get(&ValueId::new(1)),
        Some(&string_box)
    );
    assert_eq!(
        callee.metadata.value_types.get(&ValueId::new(4)),
        Some(&string_box)
    );
    assert_eq!(
        callee.metadata.value_types.get(&ValueId::new(9)),
        Some(&string_box)
    );
}
