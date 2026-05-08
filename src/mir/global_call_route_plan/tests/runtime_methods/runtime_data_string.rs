use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_length_method() {
    let mut module = MirModule::new("global_call_string_len_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_len/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut coerce = MirFunction::new(
        FunctionSignature {
            name: "Helper.coerce/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    coerce.params = vec![ValueId::new(1)];
    let coerce_block = coerce.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    coerce_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
    ]);
    coerce_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut debug_len = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_len/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    debug_len.params = vec![ValueId::new(1)];
    let block = debug_len.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.coerce/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
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
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.coerce/1".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.coerce/1".to_string(), coerce);
    module
        .functions
        .insert("Helper.debug_len/1".to_string(), debug_len);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_semantic_metadata_accepts_stringbox_length_self_arg_in_generic_pure_string_body()
{
    let mut module = MirModule::new("global_call_stringbox_len_self_arg_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_len/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_len/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        },
        MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(5),
            rhs: ValueId::new(1),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_len/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.debug_len/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.len"
            && route.method() == "length"
            && route.arity() == 1
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_len"
    }));
}

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_substring_method() {
    let mut module = MirModule::new("global_call_string_substring_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_preview/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_preview/1".to_string(),
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
            value: ConstValue::Integer(64),
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
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_preview/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_substring_suffix_method() {
    let mut module = MirModule::new("global_call_string_substring_suffix_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_suffix/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_suffix/1".to_string(),
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
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_suffix/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    let callee = &module.functions["Helper.debug_suffix/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.substring"
            && route.method() == "substring"
            && route.arity() == 1
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_substring"
    }));
}

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_substring_typed_phi_bound() {
    let mut module =
        MirModule::new("global_call_string_substring_typed_phi_bound_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.loop_bound_preview/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.loop_bound_preview/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut loop_block = BasicBlock::new(BasicBlockId::new(1));
    loop_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(3),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(2)),
                (BasicBlockId::new(2), ValueId::new(6)),
            ],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Bool(false),
        },
    ]);
    loop_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut back_block = BasicBlock::new(BasicBlockId::new(2));
    back_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
    ]);
    back_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut exit_block = BasicBlock::new(BasicBlockId::new(3));
    exit_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(8)),
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
    });
    exit_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });

    callee.blocks.insert(BasicBlockId::new(1), loop_block);
    callee.blocks.insert(BasicBlockId::new(2), back_block);
    callee.blocks.insert(BasicBlockId::new(3), exit_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.loop_bound_preview/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_pure_string_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_string_or_void_null_guarded_length_receiver() {
    let mut module =
        MirModule::new("global_call_string_or_void_null_guarded_length_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.preview/0", Some(ValueId::new(30)), vec![]);

    let mut maybe_text = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let maybe_entry = maybe_text.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    maybe_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    maybe_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("abcd".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    maybe_text.blocks.insert(BasicBlockId::new(1), text_block);
    maybe_text.blocks.insert(BasicBlockId::new(2), void_block);

    let mut preview = MirFunction::new(
        FunctionSignature {
            name: "Helper.preview/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = preview.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.maybe_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
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

    let mut text_preview = BasicBlock::new(BasicBlockId::new(1));
    text_preview.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(5),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
            type_hint: None,
        },
        MirInstruction::Copy {
            dst: ValueId::new(6),
            src: ValueId::new(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(6)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("len=".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(9),
            op: BinaryOp::Add,
            lhs: ValueId::new(8),
            rhs: ValueId::new(7),
        },
    ]);
    text_preview.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    let mut fallback = BasicBlock::new(BasicBlockId::new(2));
    fallback.instructions.push(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::String("none".to_string()),
    });
    fallback.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    preview.blocks.insert(BasicBlockId::new(1), text_preview);
    preview.blocks.insert(BasicBlockId::new(2), fallback);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), maybe_text);
    module
        .functions
        .insert("Helper.preview/0".to_string(), preview);

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
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}
