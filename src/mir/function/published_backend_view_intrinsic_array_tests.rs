// Physical consumer witnesses only; source acceptance has separate host tests.
fn intrinsic_array_module() -> MirModule {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(0),
        target: crate::mir::ConstructionTarget::IntrinsicArray,
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: crate::mir::ConstValue::Integer(30),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut module = MirModule::new("intrinsic-allocation-only".into());
    module.add_function(function);
    module
}

#[test]
fn intrinsic_array_alone_selects_rows_and_emits_object() {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let mut module = intrinsic_array_module();
            crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
                &mut module,
                crate::mir::ContractRefreshBoundary::Verifier,
            )
            .unwrap();
            let view = PublishedMirBackendView::try_new(&module).unwrap();
            assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
            assert!(view.static_method_calls().is_empty());
            assert!(view.free_function_calls().is_empty());
            assert!(view.builtin_print_calls().is_empty());
            let frame = PublishedStaticMethodCFrameV1::from_view(&view).unwrap();
            let [row] = frame.as_slice() else {
                panic!("allocation alone must issue one row")
            };
            assert_eq!(row.kind, 8);
            assert_eq!(row.dst, 0);
            assert_eq!(row.flags, 1);
            assert!(row.target_symbol.is_null());
            let body = crate::runner::mir_json_emit::emit_published_view_body(&view).unwrap();
            let body: serde_json::Value = serde_json::from_str(&body).unwrap();
            let allocation = &body["functions"][0]["blocks"][0]["instructions"][0];
            assert_eq!(allocation["target"]["kind"], "intrinsic_array");
            assert!(allocation.get("type").is_none());
            let output =
                std::env::temp_dir().join(format!("intrinsic-array-only-{}.o", std::process::id()));
            let result =
                crate::host_providers::llvm_codegen::try_compile_published_static_method_object(
                    &module,
                    output.to_str().unwrap(),
                );
            assert_eq!(result, Ok(true));
            assert!(std::fs::metadata(&output).unwrap().len() > 0);
            std::fs::remove_file(output).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn intrinsic_array_rejects_arguments_and_invalid_destination() {
    for invalid_dst in [false, true] {
        let mut module = intrinsic_array_module();
        let function = module.functions.get_mut("main").unwrap();
        let instruction = &mut function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .instructions[0];
        if let MirInstruction::NewBox { args, dst, .. } = instruction {
            if invalid_dst {
                *dst = ValueId::INVALID;
            } else {
                args.push(ValueId::new(1));
            }
        }
        assert!(matches!(
            PublishedMirBackendView::try_new(&module),
            Err(PublishedMirBackendViewErrorV1::IntrinsicArrayShapeMismatch { .. })
        ));
    }
}

#[test]
fn intrinsic_array_survives_value_remap_and_core13_without_provider_name() {
    let mut module = intrinsic_array_module();
    let before = module.functions["main"].blocks[&BasicBlockId::new(0)].instructions[0].clone();
    let mut remapper = crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper::new();
    remapper.set_value(ValueId::new(0), ValueId::new(17));
    assert_eq!(
        remapper.remap_instruction(&before),
        MirInstruction::NewBox {
            dst: ValueId::new(17),
            target: crate::mir::ConstructionTarget::IntrinsicArray,
            args: vec![],
        }
    );
    let mut optimizer = crate::mir::optimizer::MirOptimizer::new();
    crate::mir::optimizer_passes::normalize_core13_pure::normalize_pure_core13(
        &mut optimizer,
        &mut module,
    );
    assert_eq!(
        module.functions["main"].blocks[&BasicBlockId::new(0)].instructions[0],
        before
    );
}
