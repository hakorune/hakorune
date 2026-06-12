use super::*;

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
