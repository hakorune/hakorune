use super::*;

#[test]
fn refresh_module_global_call_routes_marks_generic_i64_body_direct_target_with_env_get_canonical_spelling(
) {
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
            callee: Some(Callee::Extern("env.get".to_string())),
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

    crate::mir::extern_call_route_plan::refresh_module_extern_call_routes(&mut module);
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
fn generic_i64_body_accepts_hako_mem_alloc_free_extern_routes() {
    let mut module = MirModule::new("global_call_hako_mem_generic_i64_test".to_string());
    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let main_block = main.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    main_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(64),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("MemCoreBox.alloc_i64/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("MemCoreBox.free_i64/1".to_string())),
            args: vec![ValueId::new(2)],
            effects: EffectMask::IO,
        },
    ]);
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut alloc = MirFunction::new(
        FunctionSignature {
            name: "MemCoreBox.alloc_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    alloc.params = vec![ValueId::new(10)];
    let alloc_block = alloc.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    alloc_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("hako_mem_alloc".to_string())),
        args: vec![ValueId::new(10)],
        effects: EffectMask::IO,
    });
    alloc_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let mut free = MirFunction::new(
        FunctionSignature {
            name: "MemCoreBox.free_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    free.params = vec![ValueId::new(20)];
    let free_block = free.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    free_block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("hako_mem_free".to_string())),
            args: vec![ValueId::new(20)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(22),
            value: ConstValue::Integer(0),
        },
    ]);
    free_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(22)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("MemCoreBox.alloc_i64/1".to_string(), alloc);
    module
        .functions
        .insert("MemCoreBox.free_i64/1".to_string(), free);

    crate::mir::extern_call_route_plan::refresh_module_extern_call_routes(&mut module);
    refresh_module_global_call_routes(&mut module);

    let main_routes = &module.functions["main"].metadata.global_call_routes;
    assert_eq!(main_routes.len(), 2);
    for route in main_routes {
        assert_eq!(
            route.target_shape(),
            Some("generic_i64_body"),
            "callee={} reason={:?} blocker={:?}/{:?}",
            route.callee_name(),
            route.target_shape_reason(),
            route.target_shape_blocker_symbol(),
            route.target_shape_blocker_reason()
        );
        assert_eq!(route.proof(), "typed_global_call_generic_i64");
        assert_eq!(route.return_shape(), Some("ScalarI64"));
        assert_eq!(route.value_demand(), "scalar_i64");
    }
}

#[test]
fn generic_i64_body_accepts_void_sentinel_global_side_call() {
    let mut module =
        MirModule::new("global_call_generic_i64_void_sentinel_side_call_test".to_string());
    let main = make_function_with_global_call_args(
        "RawBufLike.alloc_bytes_i64/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );

    let mut alloc = MirFunction::new(
        FunctionSignature {
            name: "MemLike.alloc_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    alloc.params = vec![ValueId::new(10)];
    alloc
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });

    let mut trace = MirFunction::new(
        FunctionSignature {
            name: "Helper.log/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    trace.params = vec![ValueId::new(20)];
    let trace_block = trace.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    trace_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(21),
            value: ConstValue::String("log:".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(22),
            op: BinaryOp::Add,
            lhs: ValueId::new(21),
            rhs: ValueId::new(20),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(22)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(23),
            value: ConstValue::Void,
        },
    ]);
    trace_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(23)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "RawBufLike.alloc_bytes_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(0)];
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(1), MirType::Integer);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(5), MirType::Void);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(6), MirType::String);
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("MemLike.alloc_i64/1".to_string())),
            args: vec![ValueId::new(0)],
            effects: EffectMask::IO,
        },
        MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("[rawbuf-like:alloc]".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.log/1".to_string())),
            args: vec![ValueId::new(6)],
            effects: EffectMask::IO,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("MemLike.alloc_i64/1".to_string(), alloc);
    module.functions.insert("Helper.log/1".to_string(), trace);
    module
        .functions
        .insert("RawBufLike.alloc_bytes_i64/1".to_string(), wrapper);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_typed_object_field_i64_body() {
    let mut module = MirModule::new("global_call_typed_object_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "HakoAllocHeap.outstandingBlocks/0",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "HakoAllocHeap.outstandingBlocks/0".to_string(),
            params: vec![MirType::Box("HakoAllocHeap".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Box("HakoAllocHeap".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Box("HakoAllocPage".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(4), MirType::Box("HakoAllocPage".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(5), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(6), MirType::Integer);
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(1),
            field: "small_page".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(3),
            base: ValueId::new(2),
            field: "current_used".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(4),
            base: ValueId::new(1),
            field: "medium_page".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(5),
            base: ValueId::new(4),
            field: "current_used".to_string(),
            declared_type: None,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("HakoAllocHeap.outstandingBlocks/0".to_string(), callee);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.definition_owner(), "generic_i64_or_leaf");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_i64_or_null_return_as_zero_sentinel() {
    let mut module = MirModule::new("global_call_generic_i64_or_null_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.get_or_null/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.get_or_null/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
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
    let mut int_block = BasicBlock::new(BasicBlockId::new(1));
    int_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(42),
    });
    int_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), int_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.get_or_null/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_void_typed_i64_or_null_return_as_zero_sentinel() {
    let mut module = MirModule::new("global_call_generic_i64_or_null_void_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.get_or_null/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.get_or_null/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
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
    let mut int_block = BasicBlock::new(BasicBlockId::new(1));
    int_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(42),
    });
    int_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), int_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.get_or_null/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), Some("generic_i64_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
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
