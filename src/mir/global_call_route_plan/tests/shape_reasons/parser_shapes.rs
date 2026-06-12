use super::*;

#[test]
fn refresh_module_global_call_routes_marks_parser_known_receiver_method_blocker() {
    let mut module = MirModule::new("global_call_parser_known_receiver_boundary_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.parse/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.parse/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "ParserBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "parse_program2".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.parse/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_known_receiver_method")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("ParserBox.parse_program2")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_unsupported_known_receiver_method")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_parser_program_json_contract_direct_target() {
    let mut module = MirModule::new("global_call_parser_program_json_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "BuildBox._parse_program_json/2",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BuildBox._parse_program_json/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "ParserBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(3),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "stage3_enable".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(8),
            src: ValueId::new(1),
        },
        MirInstruction::Copy {
            dst: ValueId::new(9),
            src: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "set_enum_inventory_from_source".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(9)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "parse_program2".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("BuildBox._parse_program_json/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_parser_program_json");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.definition_owner(), "diagnostics_only");
    assert_eq!(
        route.emit_trace_consumer(),
        "mir_call_global_diagnostics_only_emit"
    );
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_program_json_emit_body_direct_target() {
    let mut module = MirModule::new("global_call_program_json_emit_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
                .to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
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
            value: ConstValue::Void,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                "BuildBox.emit_program_json_v0/2".to_string(),
            )),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1".to_string(),
        callee,
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.definition_owner(), "module_generic");
    assert_eq!(
        route.emit_trace_consumer(),
        "mir_call_global_module_generic_emit"
    );
    assert_eq!(route.reason(), None);
}
