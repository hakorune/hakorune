use super::*;

#[test]
fn refresh_module_semantic_metadata_accepts_void_side_effect_array_push_body() {
    let mut module = MirModule::new("global_call_void_side_effect_array_push".to_string());
    let caller = make_function_with_global_call_args(
        "Seeder.seed/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut seeder = MirFunction::new(
        FunctionSignature {
            name: "Seeder.seed/1".to_string(),
            params: vec![MirType::Box("ArrayBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    seeder.params = vec![ValueId::new(0)];
    let entry = seeder.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(42),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Seeder.seed/1".to_string(), seeder);

    crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut module);

    let seeder_routes = &module.functions["Seeder.seed/1"]
        .metadata
        .generic_method_routes;
    assert_eq!(seeder_routes.len(), 1);
    assert_eq!(seeder_routes[0].route_id(), "generic_method.push");
    assert_eq!(seeder_routes[0].receiver_origin_box(), Some("ArrayBox"));

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Seeder.seed/1"));
    assert_eq!(route.proof(), "typed_global_call_void_side_effect");
    assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(
        route.emit_trace_consumer(),
        "mir_call_global_uniform_mir_emit"
    );
    assert_eq!(route.reason(), None);
}
