use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_i64_or_null_return_as_zero_sentinel() {
    let mut module = MirModule::new("global_call_generic_i64_or_null_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.get_or_null/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.get_or_null/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut int_block = BasicBlock::new(BasicBlockId::new(1));
    int_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(42),
    });
    int_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), int_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.get_or_null/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_void_typed_i64_or_null_return_as_zero_sentinel() {
    let mut module = MirModule::new("global_call_generic_i64_or_null_void_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.get_or_null/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.get_or_null/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut int_block = BasicBlock::new(BasicBlockId::new(1));
    int_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(42),
    });
    int_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), int_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.get_or_null/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}
