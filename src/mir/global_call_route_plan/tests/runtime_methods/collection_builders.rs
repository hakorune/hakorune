use super::*;

#[test]
fn refresh_module_semantic_metadata_accepts_read_char_unknown_receiver_from_string_corridor() {
    let mut module = MirModule::new("global_call_string_read_char_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "StringScanBox.read_char/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut read_char = MirFunction::new(
        FunctionSignature {
            name: "StringScanBox.read_char/2".to_string(),
            params: vec![MirType::Unknown, MirType::Integer],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read_char.params = vec![ValueId::new(0), ValueId::new(1)];
    let block = read_char.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(4),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(3),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(4)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("StringScanBox.read_char/2".to_string(), read_char);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");

    let read_char = &module.functions["StringScanBox.read_char/2"];
    assert!(read_char
        .metadata
        .generic_method_routes
        .iter()
        .any(|route| route.route_id() == "generic_method.len"
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_len"));
    assert!(read_char
        .metadata
        .generic_method_routes
        .iter()
        .any(|route| route.route_id() == "generic_method.substring"
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_substring"));
}

#[test]
fn refresh_module_semantic_metadata_accepts_read_char_null_guard_string_body() {
    let mut module = MirModule::new("global_call_program_json_read_char_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "ProgramJsonV0ScannerBox._read_char/2",
        Some(ValueId::new(30)),
        vec![ValueId::new(10), ValueId::new(11)],
    );
    let mut read_char = MirFunction::new(
        FunctionSignature {
            name: "ProgramJsonV0ScannerBox._read_char/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read_char.params = vec![ValueId::new(0), ValueId::new(1)];
    let entry = read_char.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(0),
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

    let mut null_receiver_block = BasicBlock::new(BasicBlockId::new(1));
    null_receiver_block
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        });
    null_receiver_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    let mut index_guard_block = BasicBlock::new(BasicBlockId::new(2));
    index_guard_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(5),
        },
    ]);
    index_guard_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(6),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_index_block = BasicBlock::new(BasicBlockId::new(3));
    null_index_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(7),
        value: ConstValue::String(String::new()),
    });
    null_index_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });

    let mut negative_guard_block = BasicBlock::new(BasicBlockId::new(4));
    negative_guard_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Compare {
            dst: ValueId::new(9),
            op: CompareOp::Lt,
            lhs: ValueId::new(1),
            rhs: ValueId::new(8),
        },
    ]);
    negative_guard_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(9),
        then_bb: BasicBlockId::new(5),
        else_bb: BasicBlockId::new(6),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut negative_index_block = BasicBlock::new(BasicBlockId::new(5));
    negative_index_block
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String(String::new()),
        });
    negative_index_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    let mut bounds_guard_block = BasicBlock::new(BasicBlockId::new(6));
    bounds_guard_block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Compare {
            dst: ValueId::new(12),
            op: CompareOp::Ge,
            lhs: ValueId::new(1),
            rhs: ValueId::new(11),
        },
    ]);
    bounds_guard_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(12),
        then_bb: BasicBlockId::new(7),
        else_bb: BasicBlockId::new(8),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut out_of_bounds_block = BasicBlock::new(BasicBlockId::new(7));
    out_of_bounds_block
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::String(String::new()),
        });
    out_of_bounds_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(13)),
    });

    let mut slice_block = BasicBlock::new(BasicBlockId::new(8));
    slice_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(14),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(15),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(14),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(16)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(15)],
            effects: EffectMask::PURE,
        },
    ]);
    slice_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(16)),
    });

    for block in [
        null_receiver_block,
        index_guard_block,
        null_index_block,
        negative_guard_block,
        negative_index_block,
        bounds_guard_block,
        out_of_bounds_block,
        slice_block,
    ] {
        read_char.blocks.insert(block.id, block);
    }
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "ProgramJsonV0ScannerBox._read_char/2".to_string(),
        read_char,
    );

    refresh_module_semantic_metadata(&mut module);

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
}

#[test]
fn refresh_module_global_call_routes_accepts_print_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_print_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_print/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_print/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("[debug] ".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_print/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_collection_births_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_collection_birth_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.with_collections/0",
        Some(ValueId::new(7)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.with_collections/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("ok".to_string()),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.with_collections/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_semantic_metadata_accepts_collection_builder_surface_in_generic_pure_string_body()
{
    let mut module = MirModule::new("global_call_string_collection_builder_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.with_collection_builder/0",
        Some(ValueId::new(7)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.with_collection_builder/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        method_call(None, "ArrayBox", "birth", ValueId::new(1), vec![]),
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        method_call(None, "MapBox", "birth", ValueId::new(2), vec![]),
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("items".to_string()),
        },
        method_call(
            None,
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(3), ValueId::new(1)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("ok".to_string()),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.with_collection_builder/0".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.with_collection_builder/0"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.set"
            && route.method() == "set"
            && route.receiver_origin_box() == Some("MapBox")
            && route.route_kind_tag() == "map_store_any"
            && route.helper_symbol() == "nyash.map.slot_store_hhh"
            && route.value_demand().as_metadata_name() == "write_any"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_array_size_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_array_size_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.array_size/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.array_size/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Bool(true),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(5),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(3)),
                (BasicBlockId::new(2), ValueId::new(4)),
            ],
            type_hint: Some(MirType::Box("ArrayBox".to_string())),
        },
        MirInstruction::Copy {
            dst: ValueId::new(6),
            src: ValueId::new(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(8)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "size".to_string(),
                receiver: Some(ValueId::new(6)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::String("ok".to_string()),
        },
    ]);
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });

    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.array_size/0".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.array_size/0"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.len"
            && route.method() == "size"
            && route.receiver_origin_box() == Some("ArrayBox")
            && route.route_kind_tag() == "array_slot_len"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_array_string_push_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_array_push_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.collect/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.collect/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Bool(true),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(5),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(3)),
                (BasicBlockId::new(2), ValueId::new(4)),
            ],
            type_hint: Some(MirType::Box("ArrayBox".to_string())),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("obj".to_string()),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(5)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(6)],
            effects: EffectMask::PURE,
        },
    ]);
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.collect/0".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.collect/0"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.push"
            && route.method() == "push"
            && route.receiver_origin_box() == Some("ArrayBox")
            && route.route_kind_tag() == "array_append_any"
            && route.helper_symbol() == "nyash.array.slot_append_hh"
            && route.value_demand().as_metadata_name() == "write_any"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_array_push_write_any_payload_in_string_or_void_body() {
    let mut module = MirModule::new("global_call_string_array_push_write_any_test".to_string());
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
    callee.params = vec![ValueId::new(0)];
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

    let mut miss_block = BasicBlock::new(BasicBlockId::new(1));
    miss_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    miss_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut invoke_block = BasicBlock::new(BasicBlockId::new(2));
    invoke_block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(4),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(5),
            src: ValueId::new(4),
        },
        MirInstruction::Copy {
            dst: ValueId::new(6),
            src: ValueId::new(0),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(5)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(5), ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("env.mirbuilder".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::String("emit".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
            args: vec![ValueId::new(8), ValueId::new(9), ValueId::new(5)],
            effects: EffectMask::IO,
        },
    ]);
    invoke_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    callee.blocks.insert(BasicBlockId::new(1), miss_block);
    callee.blocks.insert(BasicBlockId::new(2), invoke_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.delegate/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

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
    let callee = &module.functions["Helper.delegate/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.push"
            && route.method() == "push"
            && route.receiver_origin_box() == Some("ArrayBox")
            && route.route_kind_tag() == "array_append_any"
            && route.helper_symbol() == "nyash.array.slot_append_hh"
            && route.value_demand().as_metadata_name() == "write_any"
    }));
}
