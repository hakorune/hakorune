use super::*;

#[test]
fn detects_runtime_data_has_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "RuntimeDataBox", "has", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    assert_eq!(
        function.metadata.generic_method_routes[0].route_kind(),
        GenericMethodRouteKind::RuntimeDataContainsAny
    );
    assert!(function.metadata.generic_method_routes[0]
        .core_method()
        .is_none());
    assert_eq!(
        function.metadata.generic_method_routes[0].return_shape(),
        None
    );
    assert_eq!(
        function.metadata.generic_method_routes[0].publication_policy(),
        None
    );
}

#[test]
fn rejects_unknown_generic_method_surface() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "MapBox", "unknown", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert!(function.metadata.generic_method_routes.is_empty());
}
