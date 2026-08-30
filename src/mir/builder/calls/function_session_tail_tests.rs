#[test]
fn box_compilation_session_preserves_the_existing_partial_type_context_action() {
    let mut builder = seeded_builder();
    builder.comp_ctx.compilation_context = Some(BoxCompilationContext::new());

    let error = builder
        .with_function_lowering_session("Injected.box_context/0", Vec::new(), |child| {
            assert!(child.function_state.type_ctx.value_types.is_empty());
            assert!(child.function_state.type_ctx.value_kinds.is_empty());
            assert!(child.function_state.type_ctx.value_origin_newbox.is_empty());
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .string_literals
                    .get(&ValueId::new(722)),
                Some(&"outer literal".into())
            );
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .map_value_types
                    .get(&ValueId::new(723)),
                Some(&MirType::Integer)
            );
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .map_literal_value_types
                    .get(&(ValueId::new(724), "outer-key".into())),
                Some(&MirType::String)
            );
            Err("injected:box_context".into())
        })
        .unwrap_err();

    assert!(error.contains("injected:box_context"));
    assert!(builder.function_state.type_ctx.value_types.is_empty());
    assert!(builder.function_state.type_ctx.value_kinds.is_empty());
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .is_empty());
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .string_literals
            .get(&ValueId::new(722)),
        Some(&"outer literal".into())
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_value_types
            .get(&ValueId::new(723)),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_literal_value_types
            .get(&(ValueId::new(724), "outer-key".into())),
        Some(&MirType::String)
    );
}

#[test]
fn panic_backstop_restores_without_publishing() {
    let mut builder = seeded_builder();
    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = builder.with_function_lowering_session(
            "Injected.panic/0",
            Vec::new(),
            |_builder| -> Result<MirFunction, String> {
                panic!("injected panic");
            },
        );
    }));

    assert!(panic.is_err());
    assert_outer_state(&builder);
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .get_function("Injected.panic/0")
        .is_none());
}
