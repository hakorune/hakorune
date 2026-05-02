use super::*;

#[test]
fn refresh_function_global_call_routes_records_unsupported_global_call() {
    let mut function = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    refresh_function_global_call_routes(&mut function);

    assert_eq!(function.metadata.global_call_routes.len(), 1);
    let route = &function.metadata.global_call_routes[0];
    assert_eq!(route.block(), BasicBlockId::new(0));
    assert_eq!(route.instruction_index(), 0);
    assert_eq!(route.callee_name(), "Stage1ModeContractBox.resolve_mode/0");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.result_value(), Some(ValueId::new(7)));
    assert_eq!(route.tier(), "Unsupported");
    assert!(!route.target_exists());
    assert_eq!(route.target_arity(), None);
    assert_eq!(route.target_return_type(), None);
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.reason(), Some("unknown_global_callee"));
}

#[test]
fn refresh_function_global_call_routes_skips_print_surface() {
    let mut function = make_function_with_global_call("print", None);
    refresh_function_global_call_routes(&mut function);
    assert!(function.metadata.global_call_routes.is_empty());
}

#[test]
fn refresh_module_global_call_routes_records_target_facts() {
    let mut module = MirModule::new("global_call_target_test".to_string());
    let caller = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Stage1ModeContractBox.resolve_mode/0".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Stage1ModeContractBox.resolve_mode/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(
        route.target_symbol(),
        Some("Stage1ModeContractBox.resolve_mode/0")
    );
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_no_string_surface")
    );
    assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
}
