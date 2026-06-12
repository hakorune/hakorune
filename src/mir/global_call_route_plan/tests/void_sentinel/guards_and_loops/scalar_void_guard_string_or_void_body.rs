use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_scalar_void_guard_in_string_or_void_body() {
    let mut module = MirModule::new("global_call_scalar_void_guard_string_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.lower_bool/1",
        Some(ValueId::new(40)),
        vec![ValueId::new(1)],
    );
    let mut parser = MirFunction::new(
        FunctionSignature {
            name: "Helper.read_bool/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(10),
    );
    parser.params = vec![ValueId::new(1)];
    let parser_entry = parser.blocks.get_mut(&BasicBlockId::new(10)).unwrap();
    parser_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    parser_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(11),
        else_bb: BasicBlockId::new(12),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut parser_scalar = BasicBlock::new(BasicBlockId::new(11));
    parser_scalar.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    parser_scalar.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut parser_null = BasicBlock::new(BasicBlockId::new(12));
    parser_null.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    parser_null.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    parser.blocks.insert(BasicBlockId::new(11), parser_scalar);
    parser.blocks.insert(BasicBlockId::new(12), parser_null);

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.lower_bool/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.read_bool/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Eq,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });

    let mut non_null = BasicBlock::new(BasicBlockId::new(2));
    non_null.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
            type_hint: Some(MirType::Unknown),
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(6),
            rhs: ValueId::new(7),
        },
    ]);
    non_null.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut true_block = BasicBlock::new(BasicBlockId::new(3));
    true_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::String("true-json".to_string()),
    });
    true_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });

    let mut false_block = BasicBlock::new(BasicBlockId::new(4));
    false_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::String("false-json".to_string()),
    });
    false_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), non_null);
    callee.blocks.insert(BasicBlockId::new(3), true_block);
    callee.blocks.insert(BasicBlockId::new(4), false_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.read_bool/1".to_string(), parser);
    module
        .functions
        .insert("Helper.lower_bool/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
