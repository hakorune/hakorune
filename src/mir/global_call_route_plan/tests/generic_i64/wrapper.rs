use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_unknown_return_generic_i64_wrapper() {
    let mut module = MirModule::new("global_call_generic_i64_unknown_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find_quote/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.find_unescaped/3".to_string(),
            params: vec![MirType::Unknown, MirType::String, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    child.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.find_quote/2".to_string(),
            params: vec![MirType::Unknown, MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(0), ValueId::new(1)];
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("\"".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.find_unescaped/3".to_string())),
            args: vec![ValueId::new(0), ValueId::new(2), ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.find_quote/2".to_string(), wrapper);
    module
        .functions
        .insert("Helper.find_unescaped/3".to_string(), child);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    let wrapper_route = &module.functions["Helper.find_quote/2"]
        .metadata
        .global_call_routes[0];
    assert_eq!(wrapper_route.target_shape(), Some("generic_i64_body"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_or_void_child_null_guard_in_generic_i64_body() {
    let mut module =
        MirModule::new("global_call_generic_i64_string_or_void_guard_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.exit_code/0", Some(ValueId::new(20)), vec![]);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    child_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
    child_text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("body".to_string()),
    });
    child_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_null_block = BasicBlock::new(BasicBlockId::new(2));
    child_null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_null_block);

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.exit_code/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.maybe_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    wrapper_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(96),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut ok_block = BasicBlock::new(BasicBlockId::new(2));
    ok_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(0),
    });
    ok_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    wrapper.blocks.insert(BasicBlockId::new(1), null_block);
    wrapper.blocks.insert(BasicBlockId::new(2), ok_block);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(1), MirType::Void);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), child);
    module
        .functions
        .insert("Helper.exit_code/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}
