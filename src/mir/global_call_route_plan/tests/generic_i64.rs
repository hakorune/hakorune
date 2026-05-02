use super::*;

#[test]
fn refresh_module_global_call_routes_marks_generic_i64_body_direct_target() {
    let mut module = MirModule::new("global_call_generic_i64_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug/0", Some(ValueId::new(7)), vec![]);
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("DEBUG".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    flag.params = vec![ValueId::new(1)];
    let entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.get/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Ne,
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
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    flag.blocks.insert(BasicBlockId::new(1), yes_block);
    flag.blocks.insert(BasicBlockId::new(2), no_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug/0".to_string(), wrapper);
    module.functions.insert("Helper.flag/1".to_string(), flag);

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
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");

    let wrapper_route = &module.functions["Helper.debug/0"]
        .metadata
        .global_call_routes[0];
    assert_eq!(wrapper_route.target_shape(), Some("generic_i64_body"));
    assert_eq!(wrapper_route.proof(), "typed_global_call_generic_i64");
}

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

#[test]
fn refresh_module_global_call_routes_marks_string_scan_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_scan_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find/3",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.find/3".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
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
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
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
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3), ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(7),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut found_block = BasicBlock::new(BasicBlockId::new(1));
    found_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut missing_block = BasicBlock::new(BasicBlockId::new(2));
    missing_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::Integer(-1),
    });
    missing_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    callee.blocks.insert(BasicBlockId::new(1), found_block);
    callee.blocks.insert(BasicBlockId::new(2), missing_block);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.find/3".to_string(), callee);

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

#[test]
fn refresh_module_global_call_routes_accepts_string_indexof_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_indexof_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.index_of_version/1",
        Some(ValueId::new(10)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.index_of_version/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("\"version\":0".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "indexOf".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.index_of_version/1".to_string(), callee);

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

#[test]
fn refresh_module_global_call_routes_accepts_string_ordered_compare_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_ordered_compare_i64_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.digit_floor/1",
        Some(ValueId::new(10)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.digit_floor/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
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
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("0".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(5),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(6),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut below = BasicBlock::new(BasicBlockId::new(1));
    below.instructions.push(MirInstruction::Const {
        dst: ValueId::new(7),
        value: ConstValue::Integer(0),
    });
    below.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });
    let mut ok = BasicBlock::new(BasicBlockId::new(2));
    ok.instructions.push(MirInstruction::Const {
        dst: ValueId::new(8),
        value: ConstValue::Integer(1),
    });
    ok.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });
    callee.blocks.insert(BasicBlockId::new(1), below);
    callee.blocks.insert(BasicBlockId::new(2), ok);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.digit_floor/1".to_string(), callee);

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

#[test]
fn refresh_module_global_call_routes_accepts_debug_string_concat_in_generic_i64_body() {
    let mut module = MirModule::new("global_call_generic_i64_debug_concat_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug_concat/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_concat/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("[debug] ".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(42),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(1),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_concat/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}
