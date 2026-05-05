use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_hostbridge_extern_invoke_string_or_void_route_fact() {
    let mut module = MirModule::new("global_call_hostbridge_extern_invoke_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.delegate/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.delegate/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(1));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut hostbridge_block = BasicBlock::new(BasicBlockId::new(2));
    hostbridge_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("env.mirbuilder".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("emit".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(6)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
            args: vec![ValueId::new(4), ValueId::new(5), ValueId::new(1)],
            effects: EffectMask::IO,
        },
    ]);
    hostbridge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), void_block);
    callee.blocks.insert(BasicBlockId::new(2), hostbridge_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.delegate/1".to_string(), callee);

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
