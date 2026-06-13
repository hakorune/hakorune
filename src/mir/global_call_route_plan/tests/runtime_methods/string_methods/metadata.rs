use super::*;

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
