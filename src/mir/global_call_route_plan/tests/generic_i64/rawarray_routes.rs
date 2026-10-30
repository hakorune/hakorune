use super::*;

fn assert_array_slot_contract(
    route: &crate::mir::extern_call_route_plan::ExternCallRoute,
    expected: (&str, &str, usize, &str, u32, Option<u32>, u32),
) {
    let (core_op, symbol, arity, effect, key, value, result) = expected;
    assert_eq!(route.core_op(), core_op);
    assert_eq!(route.symbol(), symbol);
    assert_eq!(route.source_symbol(), symbol);
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.key_value(), ValueId::new(key));
    assert_eq!(route.value_value(), value.map(ValueId::new));
    assert_eq!(route.result_value(), ValueId::new(result));
    assert_eq!(route.arity(), arity);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &[effect]);
}

#[test]
fn generic_i64_body_accepts_any_handle_live_and_array_slot_append_extern_routes() {
    let mut module = MirModule::new("global_call_generic_i64_rawarray_append_test".to_string());
    let main = make_function_with_global_call_args(
        "RawArrayLike.slot_append_any/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );

    let mut live = MirFunction::new(
        FunctionSignature {
            name: "OwnershipLike._handle_live_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    live.params = vec![ValueId::new(10)];
    let live_block = live.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    live_block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.any.handle_live_h".to_string())),
        args: vec![ValueId::new(10)],
        effects: EffectMask::IO,
    });
    live_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let mut ensure = MirFunction::new(
        FunctionSignature {
            name: "OwnershipLike.ensure_handle_writable_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    ensure.params = vec![ValueId::new(20)];
    let ensure_block = ensure.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    ensure_block
        .instructions
        .push(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "OwnershipLike._handle_live_i64/1".to_string(),
            ))),
            args: vec![ValueId::new(20)],
            effects: EffectMask::IO,
        });
    ensure_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(21)),
    });

    let mut ptr_append = MirFunction::new(
        FunctionSignature {
            name: "PtrLike.slot_append_any/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    ptr_append.params = vec![ValueId::new(30), ValueId::new(31)];
    let ptr_block = ptr_append.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    ptr_block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(32)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.array.slot_append_hh".to_string())),
        args: vec![ValueId::new(30), ValueId::new(31)],
        effects: EffectMask::IO,
    });
    ptr_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(32)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "RawArrayLike.slot_append_any/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(40), ValueId::new(41)];
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(42)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "OwnershipLike.ensure_handle_writable_i64/1".to_string(),
            ))),
            args: vec![ValueId::new(40)],
            effects: EffectMask::IO,
        },
        MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(43)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "PtrLike.slot_append_any/2".to_string(),
            ))),
            args: vec![ValueId::new(40), ValueId::new(41)],
            effects: EffectMask::IO,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(43)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("OwnershipLike._handle_live_i64/1".to_string(), live);
    module.functions.insert(
        "OwnershipLike.ensure_handle_writable_i64/1".to_string(),
        ensure,
    );
    module
        .functions
        .insert("PtrLike.slot_append_any/2".to_string(), ptr_append);
    module
        .functions
        .insert("RawArrayLike.slot_append_any/2".to_string(), wrapper);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));

    let live_extern = &module.functions["OwnershipLike._handle_live_i64/1"]
        .metadata
        .extern_call_routes[0];
    assert_eq!(live_extern.route_id(), "extern.any.handle_live");
    let append_routes = &module.functions["PtrLike.slot_append_any/2"]
        .metadata
        .extern_call_routes;
    assert_eq!(append_routes.len(), 1);
    let append_extern = &append_routes[0];
    assert_eq!(append_extern.route_id(), "extern.array.slot_append_any");
    assert_array_slot_contract(
        append_extern,
        ("ArraySlotAppendAny", "nyash.array.slot_append_hh", 2, "array.slot_append", 30, Some(31), 32),
    );
}

#[test]
fn generic_i64_body_accepts_array_slot_len_extern_route() {
    let mut module = MirModule::new("global_call_generic_i64_rawarray_len_test".to_string());
    let main = make_function_with_global_call_args(
        "RawArrayLike.slot_len_i64/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );

    let mut ptr_len = MirFunction::new(
        FunctionSignature {
            name: "PtrLike.slot_len_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    ptr_len.params = vec![ValueId::new(10)];
    let ptr_block = ptr_len.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    ptr_block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.array.slot_len_h".to_string())),
        args: vec![ValueId::new(10)],
        effects: EffectMask::IO,
    });
    ptr_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "RawArrayLike.slot_len_i64/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(20)];
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block
        .instructions
        .push(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "PtrLike.slot_len_i64/1".to_string(),
            ))),
            args: vec![ValueId::new(20)],
            effects: EffectMask::IO,
        });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(21)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("PtrLike.slot_len_i64/1".to_string(), ptr_len);
    module
        .functions
        .insert("RawArrayLike.slot_len_i64/1".to_string(), wrapper);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));

    let ptr_routes = &module.functions["PtrLike.slot_len_i64/1"].metadata.extern_call_routes;
    assert_eq!(ptr_routes.len(), 1);
    let ptr_extern = &ptr_routes[0];
    assert_eq!(ptr_extern.route_id(), "extern.array.slot_len_i64");
    assert_eq!(ptr_extern.core_op(), "ArraySlotLenI64");
    assert_array_slot_contract(
        ptr_extern,
        ("ArraySlotLenI64", "nyash.array.slot_len_h", 1, "array.slot_len", 10, None, 11),
    );
}

#[test]
fn generic_i64_body_accepts_array_slot_load_extern_route() {
    let mut module = MirModule::new("global_call_generic_i64_rawarray_load_test".to_string());
    let main = make_function_with_global_call_args(
        "RawArrayLike.slot_load_i64/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );

    let mut ptr_load = MirFunction::new(
        FunctionSignature {
            name: "PtrLike.slot_load_i64/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    ptr_load.params = vec![ValueId::new(10), ValueId::new(11)];
    let ptr_block = ptr_load.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    ptr_block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(12)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.array.slot_load_hi".to_string())),
        args: vec![ValueId::new(10), ValueId::new(11)],
        effects: EffectMask::IO,
    });
    ptr_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "RawArrayLike.slot_load_i64/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(20), ValueId::new(21)];
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block
        .instructions
        .push(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(22)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "PtrLike.slot_load_i64/2".to_string(),
            ))),
            args: vec![ValueId::new(20), ValueId::new(21)],
            effects: EffectMask::IO,
        });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(22)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("PtrLike.slot_load_i64/2".to_string(), ptr_load);
    module
        .functions
        .insert("RawArrayLike.slot_load_i64/2".to_string(), wrapper);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));

    let ptr_routes = &module.functions["PtrLike.slot_load_i64/2"].metadata.extern_call_routes;
    assert_eq!(ptr_routes.len(), 1);
    let ptr_extern = &ptr_routes[0];
    assert_eq!(ptr_extern.route_id(), "extern.array.slot_load_i64");
    assert_eq!(ptr_extern.core_op(), "ArraySlotLoadI64");
    assert_array_slot_contract(
        ptr_extern,
        ("ArraySlotLoadI64", "nyash.array.slot_load_hi", 2, "array.slot_load", 10, Some(11), 12),
    );
}

#[test]
fn generic_i64_body_accepts_array_slot_store_extern_route() {
    let mut module = MirModule::new("global_call_generic_i64_rawarray_store_test".to_string());
    let main = make_function_with_global_call_args(
        "RawArrayLike.slot_store_i64/3",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );

    let mut ptr_store = MirFunction::new(
        FunctionSignature {
            name: "PtrLike.slot_store_i64/3".to_string(),
            params: vec![MirType::Integer, MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    ptr_store.params = vec![ValueId::new(10), ValueId::new(11), ValueId::new(12)];
    let ptr_block = ptr_store.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    ptr_block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(13)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.array.slot_store_hii".to_string())),
        args: vec![ValueId::new(10), ValueId::new(11), ValueId::new(12)],
        effects: EffectMask::IO,
    });
    ptr_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(13)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "RawArrayLike.slot_store_i64/3".to_string(),
            params: vec![MirType::Integer, MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(20), ValueId::new(21), ValueId::new(22)];
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block
        .instructions
        .push(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(23)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "PtrLike.slot_store_i64/3".to_string(),
            ))),
            args: vec![ValueId::new(20), ValueId::new(21), ValueId::new(22)],
            effects: EffectMask::IO,
        });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(23)),
    });

    module.functions.insert("main".to_string(), main);
    module
        .functions
        .insert("PtrLike.slot_store_i64/3".to_string(), ptr_store);
    module
        .functions
        .insert("RawArrayLike.slot_store_i64/3".to_string(), wrapper);

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
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));

    let ptr_routes = &module.functions["PtrLike.slot_store_i64/3"].metadata.extern_call_routes;
    assert_eq!(ptr_routes.len(), 1);
    let ptr_extern = &ptr_routes[0];
    assert_eq!(ptr_extern.route_id(), "extern.array.slot_store_i64");
    assert_eq!(ptr_extern.core_op(), "ArraySlotStoreI64");
    assert_array_slot_contract(
        ptr_extern,
        ("ArraySlotStoreI64", "nyash.array.slot_store_hii", 3, "array.slot_store_i64", 10, Some(12), 13),
    );
}
