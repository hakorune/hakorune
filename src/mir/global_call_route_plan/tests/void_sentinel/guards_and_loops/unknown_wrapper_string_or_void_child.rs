use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_unknown_wrapper_returning_string_or_void_child() {
    let mut module =
        MirModule::new("global_call_unknown_wrapper_string_or_void_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(20)), vec![]);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/0".to_string(),
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
    let mut child_text = BasicBlock::new(BasicBlockId::new(1));
    child_text.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_text.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void = BasicBlock::new(BasicBlockId::new(2));
    child_void.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text);
    child.blocks.insert(BasicBlockId::new(2), child_void);

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.child/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let child_route = &module.functions["Helper.wrapper/0"]
        .metadata
        .global_call_routes[0];
    assert_eq!(child_route.target_shape(), None);

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
