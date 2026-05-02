use super::*;

#[test]
fn refresh_module_global_call_routes_marks_method_call_shape_reason() {
    let mut module = MirModule::new("global_call_method_reason_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "debugPreview".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_method_call")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_unknown_child_target_shape_reason() {
    let mut module = MirModule::new("global_call_child_reason_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(7)), vec![]);
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.pending/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let pending = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);
    module
        .functions
        .insert("Helper.pending/0".to_string(), pending);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("Helper.pending/0")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_numeric_i64_leaf_direct_target() {
    let mut module = MirModule::new("global_call_leaf_test".to_string());
    let caller = make_function_with_global_call("Helper.add/2", Some(ValueId::new(7)));
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.add/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId::new(3),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(2),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.add/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.add/2"));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_leaf_numeric_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_resolves_static_entry_alias_to_target_symbol() {
    let mut module = MirModule::new("global_call_static_entry_alias_test".to_string());
    let caller =
        make_function_with_global_call_args("main._helper/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Main._helper/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(42),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Main._helper/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.callee_name(), "main._helper/0");
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Main._helper/0"));
    assert_eq!(route.target_arity(), Some(0));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_generic_pure_string_body_direct_target() {
    let mut module = MirModule::new("global_call_generic_string_test".to_string());
    let caller = make_function_with_global_call("Helper.normalize/2", Some(ValueId::new(7)));
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.normalize/2".to_string(),
            params: vec![MirType::String, MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(9)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("dev".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("vm".to_string()),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(6),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(4)),
            (BasicBlockId::new(2), ValueId::new(5)),
        ],
        type_hint: Some(MirType::String),
    });
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.normalize/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.normalize/2"));
    assert_eq!(route.target_return_type(), Some("str".to_string()));
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_string_return_param_passthrough() {
    let mut module = MirModule::new("global_call_string_return_param_passthrough_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.normalize/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.normalize/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("legacy".to_string()),
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
    let mut canonical_block = BasicBlock::new(BasicBlockId::new(1));
    canonical_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::String("canonical".to_string()),
    });
    canonical_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut passthrough_block = BasicBlock::new(BasicBlockId::new(2));
    passthrough_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(6),
        src: ValueId::new(2),
    });
    passthrough_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), canonical_block);
    callee
        .blocks
        .insert(BasicBlockId::new(2), passthrough_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.normalize/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_pure_string_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    assert_eq!(route.return_shape(), Some("string_handle"));
}
