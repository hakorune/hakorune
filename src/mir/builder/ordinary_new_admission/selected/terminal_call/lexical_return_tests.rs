use super::*;

#[test]
fn direct_return_emitter_rejects_taken_row_before_builder_effects() {
    let text = "box Token {} box Maker { make(size: i64) { return new Token() } relay() { return me.make(7) } } static box Main { main() { return 0 } }";
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed fixture")
    };
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = crate::mir::builder::NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(997).unwrap();
    let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver, source, Some(&brands),
    ).unwrap();
    let mut state = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .any(|(_, call)| call.selector() == "make")
                        .then(|| CallableSemanticLoweringState::from_exact_source(input).unwrap())
                })
                .unwrap()
        })
        .expect("original relay source");
    let owner = state.owner();
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .map_err(|(_, error)| error)
        .unwrap()
        .commit();
    let ledger = installed.ordinary_new_claim_ledger();
    let (exit, row) =
        ledger.take_direct_return_row_for_test(owner, Some(InvokeCallResultKind::I64));
    let call = row.call_site().clone();
    let mut builder = MirBuilder::new();
    assert!(builder.function_state.current_function.is_none());
    assert!(emit(&mut builder, &mut state, &ledger, owner, &exit, row)
        .unwrap_err()
        .contains("taken-row-drift"));
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
    assert!(builder.function_state.type_ctx.value_types.is_empty());
    assert!(ledger
        .take_lexical_instance_call(owner, call.site())
        .is_err());
}

#[test]
fn direct_return_emitter_type_uses_original_result_class() {
    for result in [
        InvokeCallResultKind::Handle,
        InvokeCallResultKind::NullableHandle,
    ] {
        assert_eq!(
            result_type(result, Some("Token")).unwrap(),
            MirType::Box("Token".into())
        );
        assert!(result_type(result, None).is_err());
    }
    assert_eq!(
        result_type(InvokeCallResultKind::I64, None).unwrap(),
        MirType::Integer
    );
}
