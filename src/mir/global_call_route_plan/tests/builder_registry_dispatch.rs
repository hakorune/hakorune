use super::*;

#[test]
fn refresh_module_global_call_routes_marks_builder_registry_dispatch_shape() {
    let mut module = MirModule::new("global_call_builder_registry_dispatch_test".to_string());
    let caller = make_function_with_global_call_args(
        "Registry.try_lower/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Registry.try_lower/1".to_string(),
        make_registry_dispatch("LowerGood.try_lower/1"),
    );
    module.functions.insert(
        "LowerGood.try_lower/1".to_string(),
        make_string_or_void_child("LowerGood.try_lower/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
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

#[test]
fn refresh_module_global_call_routes_propagates_builder_registry_dispatch_child_blocker() {
    let mut module =
        MirModule::new("global_call_builder_registry_dispatch_child_blocker_test".to_string());
    let caller = make_function_with_global_call_args(
        "Registry.try_lower/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Registry.try_lower/1".to_string(),
        make_registry_dispatch("LowerBad.try_lower/1"),
    );
    module.functions.insert(
        "LowerBad.try_lower/1".to_string(),
        make_unsupported_child("LowerBad.try_lower/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("LowerBad.try_lower/1")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_return_abi_not_handle_compatible")
    );
}

fn make_registry_dispatch(child_name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Registry.try_lower/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Compare {
            dst: ValueId::new(5),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut match_block = BasicBlock::new(BasicBlockId::new(1));
    match_block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(6)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(4)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::String("return.int".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(6),
            rhs: ValueId::new(7),
        },
    ]);
    match_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut child_block = BasicBlock::new(BasicBlockId::new(2));
    child_block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(9)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(child_name.to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(11),
            op: CompareOp::Ne,
            lhs: ValueId::new(9),
            rhs: ValueId::new(10),
        },
    ]);
    child_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut return_child = BasicBlock::new(BasicBlockId::new(3));
    return_child.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    let mut return_void = BasicBlock::new(BasicBlockId::new(5));
    return_void.instructions.push(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Void,
    });
    return_void.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    function.blocks.insert(BasicBlockId::new(1), match_block);
    function.blocks.insert(BasicBlockId::new(2), child_block);
    function.blocks.insert(BasicBlockId::new(3), return_child);
    function.blocks.insert(BasicBlockId::new(5), return_void);
    function
}

fn make_string_or_void_child(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
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
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::String("ok".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    function.blocks.insert(BasicBlockId::new(1), text_block);
    function.blocks.insert(BasicBlockId::new(2), void_block);
    function
}

fn make_unsupported_child(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "UnknownBox".to_string(),
            method: "unknown".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    function
}
