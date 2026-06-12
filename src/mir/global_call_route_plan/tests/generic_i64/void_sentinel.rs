use super::*;

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
