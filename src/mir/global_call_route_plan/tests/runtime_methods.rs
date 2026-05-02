use super::*;
use crate::mir::semantic_refresh::refresh_module_semantic_metadata;

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
