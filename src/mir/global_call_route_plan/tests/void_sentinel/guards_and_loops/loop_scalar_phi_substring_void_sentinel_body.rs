use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_loop_scalar_phi_substring_void_sentinel_body() {
    let mut module =
        MirModule::new("global_call_loop_scalar_phi_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.scan_or_null/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.scan_or_null/2".to_string(),
            params: vec![MirType::String, MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut loop_block = BasicBlock::new(BasicBlockId::new(1));
    loop_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(4),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(3)),
                (BasicBlockId::new(2), ValueId::new(8)),
            ],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Compare {
            dst: ValueId::new(5),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(2),
        },
    ]);
    loop_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut back_block = BasicBlock::new(BasicBlockId::new(2));
    back_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(8),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(6),
        },
    ]);
    back_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut exit_block = BasicBlock::new(BasicBlockId::new(3));
    exit_block.instructions.push(MirInstruction::Compare {
        dst: ValueId::new(9),
        op: CompareOp::Eq,
        lhs: ValueId::new(2),
        rhs: ValueId::new(3),
    });
    exit_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(9),
        then_bb: BasicBlockId::new(5),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut text_block = BasicBlock::new(BasicBlockId::new(4));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(10)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "substring".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3), ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    let mut void_block = BasicBlock::new(BasicBlockId::new(5));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    callee.blocks.insert(BasicBlockId::new(1), loop_block);
    callee.blocks.insert(BasicBlockId::new(2), back_block);
    callee.blocks.insert(BasicBlockId::new(3), exit_block);
    callee.blocks.insert(BasicBlockId::new(4), text_block);
    callee.blocks.insert(BasicBlockId::new(5), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.scan_or_null/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("?".to_string()));
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
