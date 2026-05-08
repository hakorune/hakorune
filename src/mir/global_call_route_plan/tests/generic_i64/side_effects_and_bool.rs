use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_print_in_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_print_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug_flag/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_flag/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_print_dead_dst_in_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_print_dead_dst_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug_flag/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_flag/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_rejects_print_used_dst_in_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_print_used_dst_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug_flag/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_flag/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_ne!(route.proof(), "typed_global_call_generic_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_bool_return_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_bool_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.is_blank/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.is_blank/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(" ".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.is_blank/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_preserves_bool_dst_from_generic_i64_global_call() {
    let mut module = MirModule::new("global_call_generic_i64_bool_call_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.classify_blank/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.classify_blank/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Bool);
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.is_blank/1".to_string())),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), yes_block);
    callee.blocks.insert(BasicBlockId::new(2), no_block);

    let mut predicate = MirFunction::new(
        FunctionSignature {
            name: "Helper.is_blank/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    predicate.params = vec![ValueId::new(1)];
    let predicate_block = predicate.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    predicate_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(" ".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    predicate_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.classify_blank/1".to_string(), callee);
    module
        .functions
        .insert("Helper.is_blank/1".to_string(), predicate);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_typed_i64_phi_from_unknown_param() {
    let mut module = MirModule::new("global_call_generic_i64_typed_phi_param_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.skip_like/2",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.skip_like/2".to_string(),
            params: vec![MirType::String, MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "length".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
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
                (BasicBlockId::new(0), ValueId::new(2)),
                (BasicBlockId::new(2), ValueId::new(8)),
            ],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Compare {
            dst: ValueId::new(5),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
    ]);
    loop_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut next_block = BasicBlock::new(BasicBlockId::new(2));
    next_block.instructions.extend([
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
    next_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    let mut exit_block = BasicBlock::new(BasicBlockId::new(3));
    exit_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), loop_block);
    callee.blocks.insert(BasicBlockId::new(2), next_block);
    callee.blocks.insert(BasicBlockId::new(3), exit_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.skip_like/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_i64_bool_phi_in_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_bool_phi_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.bool_phi/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.bool_phi/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee
        .metadata
        .value_types
        .insert(ValueId::new(4), MirType::Bool);
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
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(4),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(2)),
            (BasicBlockId::new(2), ValueId::new(3)),
        ],
        type_hint: Some(MirType::Bool),
    });
    merge_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(4),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut ret_yes = BasicBlock::new(BasicBlockId::new(4));
    ret_yes.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(1),
    });
    ret_yes.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut ret_no = BasicBlock::new(BasicBlockId::new(5));
    ret_no.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(0),
    });
    ret_no.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), yes_block);
    callee.blocks.insert(BasicBlockId::new(2), no_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    callee.blocks.insert(BasicBlockId::new(4), ret_yes);
    callee.blocks.insert(BasicBlockId::new(5), ret_no);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.bool_phi/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}
