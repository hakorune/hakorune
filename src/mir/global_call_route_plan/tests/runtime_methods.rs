use super::*;
use crate::mir::semantic_refresh::refresh_module_semantic_metadata;

fn method_call(
    dst: Option<ValueId>,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    args: Vec<ValueId>,
) -> MirInstruction {
    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_emit_box_value_field_reads() {
    let mut module = MirModule::new("global_call_mir_json_emit_box_value_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_box_value/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_box_value = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_box_value/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_box_value.params = vec![ValueId::new(1)];
    let block = emit_box_value
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("type".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("value".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(6)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("{\"type\":".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(9),
            op: BinaryOp::Add,
            lhs: ValueId::new(8),
            rhs: ValueId::new(5),
        },
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String(",\"value\":".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(11),
            op: BinaryOp::Add,
            lhs: ValueId::new(9),
            rhs: ValueId::new(10),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(12),
            op: BinaryOp::Add,
            lhs: ValueId::new(11),
            rhs: ValueId::new(7),
        },
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::String("}".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(14),
            op: BinaryOp::Add,
            lhs: ValueId::new(12),
            rhs: ValueId::new(13),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(14)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "MirJsonEmitBox._emit_box_value/1".to_string(),
        emit_box_value,
    );

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_box_value/1"];
    assert_eq!(helper.metadata.generic_method_routes.len(), 2);
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_function_field_reads() {
    let mut module = MirModule::new("global_call_mir_json_function_field_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_function/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_function = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_function/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_function.params = vec![ValueId::new(1)];
    let block = emit_function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("name".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_function/1".to_string(), emit_function);

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_function/1"];
    assert!(helper.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.get"
            && route.proof_tag() == "mir_json_function_field"
            && route.key_const_text() == Some("name")
    }));
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_collection_or_void_phi_for_mir_json_function_blocks() {
    let mut module = MirModule::new("global_call_collection_or_void_phi_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_function/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_function = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_function/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_function.params = vec![ValueId::new(1)];

    let entry = emit_function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("blocks".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Bool(false),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_assign = BasicBlock::new(BasicBlockId::new(1));
    null_assign.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Void,
    });
    null_assign.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut present_assign = BasicBlock::new(BasicBlockId::new(2));
    present_assign.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge = BasicBlock::new(BasicBlockId::new(3));
    merge.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(5)),
                (BasicBlockId::new(2), ValueId::new(3)),
            ],
            type_hint: None,
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(6),
            rhs: ValueId::new(7),
        },
    ]);
    merge.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(4),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_return = BasicBlock::new(BasicBlockId::new(4));
    null_return.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::String("[]".to_string()),
    });
    null_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });

    let mut present_return = BasicBlock::new(BasicBlockId::new(5));
    present_return.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(10),
            inputs: vec![(BasicBlockId::new(3), ValueId::new(6))],
            type_hint: None,
        },
        method_call(
            Some(ValueId::new(11)),
            "RuntimeDataBox",
            "length",
            ValueId::new(10),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::String("[]".to_string()),
        },
    ]);
    present_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    emit_function
        .blocks
        .insert(BasicBlockId::new(1), null_assign);
    emit_function
        .blocks
        .insert(BasicBlockId::new(2), present_assign);
    emit_function.blocks.insert(BasicBlockId::new(3), merge);
    emit_function
        .blocks
        .insert(BasicBlockId::new(4), null_return);
    emit_function
        .blocks
        .insert(BasicBlockId::new(5), present_return);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_function/1".to_string(), emit_function);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_flags_keys_route() {
    let mut module = MirModule::new("global_call_mir_json_flags_keys_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_flags/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_flags = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_flags/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_flags.params = vec![ValueId::new(1)];
    let block = emit_flags
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        method_call(
            Some(ValueId::new(2)),
            "RuntimeDataBox",
            "keys",
            ValueId::new(1),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("{}".to_string()),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_flags/1".to_string(), emit_flags);

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_flags/1"];
    assert!(helper.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.keys"
            && route.proof_tag() == "mir_json_flags_keys"
            && route.route_kind_tag() == "map_keys_array"
    }));
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_flags_keys_null_guard() {
    let mut module = MirModule::new("global_call_mir_json_flags_keys_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_flags/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_flags = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_flags/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_flags.params = vec![ValueId::new(1)];

    let entry = emit_flags
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.instructions.extend([
        method_call(
            Some(ValueId::new(2)),
            "RuntimeDataBox",
            "keys",
            ValueId::new(1),
            vec![],
        ),
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
        value: ConstValue::String("{}".to_string()),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });

    let mut present_block = BasicBlock::new(BasicBlockId::new(2));
    present_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
            type_hint: Some(MirType::Box("ArrayBox".to_string())),
        },
        method_call(
            Some(ValueId::new(7)),
            "RuntimeDataBox",
            "length",
            ValueId::new(6),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("{}".to_string()),
        },
    ]);
    present_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });

    emit_flags.blocks.insert(BasicBlockId::new(1), null_block);
    emit_flags
        .blocks
        .insert(BasicBlockId::new(2), present_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_flags/1".to_string(), emit_flags);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

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

#[test]
fn refresh_module_semantic_metadata_accepts_string_indexof_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_indexof_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.has_token/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.has_token/1".to_string(),
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
            value: ConstValue::String("\"token\"".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(1),
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
            args: vec![ValueId::new(2), ValueId::new(4)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.has_token/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.has_token/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.indexOf"
            && route.method() == "indexOf"
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_indexof"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_string_lastindexof_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_lastindexof_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.has_last_token/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.has_last_token/1".to_string(),
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
            value: ConstValue::String("\"token\"".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "lastIndexOf".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.has_last_token/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    let callee = &module.functions["Helper.has_last_token/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.lastIndexOf"
            && route.method() == "lastIndexOf"
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_last_indexof"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_string_contains_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_contains_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.keep_dotted/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.keep_dotted/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(".".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "contains".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut yes = BasicBlock::new(BasicBlockId::new(1));
    yes.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut no = BasicBlock::new(BasicBlockId::new(2));
    no.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("fallback".to_string()),
    });
    no.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), yes);
    callee.blocks.insert(BasicBlockId::new(2), no);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.keep_dotted/1".to_string(), callee);

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
    let callee = &module.functions["Helper.keep_dotted/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.contains"
            && route.method() == "contains"
            && route.receiver_origin_box() == Some("StringBox")
            && route.route_kind_tag() == "string_contains"
    }));
}

#[test]
fn refresh_module_semantic_metadata_accepts_generic_string_select_flow() {
    let mut module = MirModule::new("global_call_generic_string_select_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.choose_text/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.choose_text/1".to_string(),
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
            value: ConstValue::Bool(true),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("\"fallback\"".to_string()),
        },
        MirInstruction::Select {
            dst: ValueId::new(4),
            cond: ValueId::new(2),
            then_val: ValueId::new(1),
            else_val: ValueId::new(3),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.choose_text/1".to_string(), callee);

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
    assert_eq!(route.return_shape(), Some("string_handle"));
}

#[test]
fn refresh_module_semantic_metadata_accepts_ordered_string_compare_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_ordered_string_compare_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.guard_digit/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.guard_digit/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("0".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Ge,
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
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("not_digit".to_string()),
    });
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.guard_digit/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_infers_unknown_lhs_from_string_compare() {
    let mut module = MirModule::new("global_call_unknown_lhs_string_compare_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.guard_unknown_digit/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.guard_unknown_digit/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("0".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Ge,
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
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("x".to_string()),
    });
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.guard_unknown_digit/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_pure_string_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_self_recursive_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_recursive_string_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.rec_digits/4",
        Some(ValueId::new(20)),
        vec![
            ValueId::new(10),
            ValueId::new(11),
            ValueId::new(12),
            ValueId::new(13),
        ],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.rec_digits/4".to_string(),
            params: vec![
                MirType::String,
                MirType::Integer,
                MirType::Unknown,
                MirType::Integer,
            ],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![
        ValueId::new(1),
        ValueId::new(2),
        ValueId::new(3),
        ValueId::new(4),
    ];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Compare {
        dst: ValueId::new(5),
        op: CompareOp::Ge,
        lhs: ValueId::new(2),
        rhs: ValueId::new(4),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut done_block = BasicBlock::new(BasicBlockId::new(1));
    done_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(11),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(3))],
        type_hint: None,
    });
    done_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let mut step_block = BasicBlock::new(BasicBlockId::new(2));
    step_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(12),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(3))],
            type_hint: None,
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(7),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(6),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(8)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(7)],
            effects: EffectMask::PURE,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(9),
            op: BinaryOp::Add,
            lhs: ValueId::new(12),
            rhs: ValueId::new(8),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.rec_digits/4".to_string())),
            args: vec![
                ValueId::new(1),
                ValueId::new(7),
                ValueId::new(9),
                ValueId::new(4),
            ],
            effects: EffectMask::PURE,
        },
    ]);
    step_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    callee.blocks.insert(BasicBlockId::new(1), done_block);
    callee.blocks.insert(BasicBlockId::new(2), step_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.rec_digits/4".to_string(), callee);

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
}
