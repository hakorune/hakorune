fn canonical_static_function(key: &CanonicalSameModuleCallableKeyV1) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: key.mir_symbol_projection(),
            params: vec![MirType::Integer; key.arity() as usize],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let target = key
        .canonical_global_target_v1()
        .expect("static key must project to global target");
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::call(
            Some(ValueId::new(10)),
            Callee::Global(target),
            vec![ValueId::new(1), ValueId::new(2)],
            EffectMask::PURE,
        ));
    function
}

fn canonical_static_function_with_two_calls(key: &CanonicalSameModuleCallableKeyV1) -> MirFunction {
    let mut function = canonical_static_function(key);
    let target = key
        .canonical_global_target_v1()
        .expect("static key must project to global target");
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::call(
            Some(ValueId::new(11)),
            Callee::Global(target),
            vec![ValueId::new(1), ValueId::new(2)],
            EffectMask::PURE,
        ));
    function
}

#[test]
fn selected_normal_admission_accepts_canonical_typed_calls_only() {
    let key = static_key();
    let mut module = MirModule::new("selected-normal-typed".to_owned());
    module
        .add_cataloged_box_method(key.clone(), canonical_static_function(&key))
        .expect("publish relation");

    let view =
        PublishedMirBackendView::try_new_selected_normal(&module).expect("canonical selected view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert_eq!(view.static_method_calls().len(), 1);
}

#[test]
fn selected_normal_admission_keeps_legacy_only_input_on_existing_route() {
    let key = static_key();
    let mut module = MirModule::new("selected-normal-legacy-only".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");

    let view = PublishedMirBackendView::try_new_selected_normal(&module)
        .expect("legacy-only selected view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
}

#[test]
fn selected_normal_admission_rejects_one_legacy_mutation_before_artifact() {
    let key = static_key();
    let mut module = MirModule::new("selected-normal-mixed".to_owned());
    module
        .add_cataloged_box_method(key.clone(), canonical_static_function_with_two_calls(&key))
        .expect("publish relation");
    let function = module
        .functions
        .get_mut(&key.mir_symbol_projection())
        .expect("published definition");
    let instruction = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .instructions
        .get_mut(1)
        .expect("second selected call");
    let MirInstruction::Call(call) = instruction.clone() else {
        panic!("fixture must start with a canonical selected call");
    };
    *instruction = MirInstruction::LegacyCallV0 {
        dst: call.dst,
        func: ValueId::INVALID,
        callee: Some(call.callee),
        args: call.args,
        effects: call.effects,
    };

    let error = PublishedMirBackendView::try_new_selected_normal(&module)
        .expect_err("mixed call shapes must stop before artifact");
    assert!(matches!(
        error,
        PublishedMirBackendViewErrorV1::SelectedNormalUsesLegacyCallV0 { .. }
    ));
    assert!(error.to_string().contains("SelectedNormalUsesLegacyCallV0"));
}
