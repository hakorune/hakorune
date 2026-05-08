use super::*;

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

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_numeric_value_field_proof() {
    let mut module = MirModule::new("global_call_mir_json_numeric_field_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._expect_i64/2",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2)],
    );

    let mut to_i64 = MirFunction::new(
        FunctionSignature {
            name: "StringHelpers.to_i64/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    to_i64.params = vec![ValueId::new(1)];
    let to_i64_entry = to_i64.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    to_i64_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(1),
    });
    to_i64_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut expect_i64 = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._expect_i64/2".to_string(),
            params: vec![MirType::Unknown, MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    expect_i64.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = expect_i64.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("value".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(5),
            src: ValueId::new(4),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(7),
            op: CompareOp::Ne,
            lhs: ValueId::new(5),
            rhs: ValueId::new(6),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(7),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(8),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(4))],
            type_hint: None,
        },
        MirInstruction::Copy {
            dst: ValueId::new(9),
            src: ValueId::new(8),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("StringHelpers.to_i64/1".to_string())),
            args: vec![ValueId::new(9)],
            effects: EffectMask::PURE,
        },
    ]);
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(0),
    });
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    expect_i64.blocks.insert(BasicBlockId::new(1), then_block);
    expect_i64.blocks.insert(BasicBlockId::new(2), else_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("StringHelpers.to_i64/1".to_string(), to_i64);
    module
        .functions
        .insert("MirJsonEmitBox._expect_i64/2".to_string(), expect_i64);

    crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut module);

    let expect_i64 = &module.functions["MirJsonEmitBox._expect_i64/2"];
    let field_route = expect_i64
        .metadata
        .generic_method_routes
        .iter()
        .find(|route| route.proof_tag() == "mir_json_numeric_value_field")
        .expect("exact field-read proof");
    assert_eq!(field_route.key_const_text(), Some("value"));

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}
