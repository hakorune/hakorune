use super::*;

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
