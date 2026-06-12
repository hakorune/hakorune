use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_hako_mem_alloc_route() {
    let mut function = make_function_with_call(
        "hako_mem_alloc",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_mem.alloc");
    assert_eq!(route.core_op(), "HakoMemAlloc");
    assert_eq!(route.symbol(), "hako_mem_alloc");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_mem_alloc");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.mem.alloc"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_mem_free_route() {
    let mut function = make_function_with_call(
        "hako_mem_free/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_mem.free");
    assert_eq!(route.core_op(), "HakoMemFree");
    assert_eq!(route.symbol(), "hako_mem_free");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_mem_free/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "void_sentinel_i64_zero");
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.effect_tags(), &["hako.mem.free"]);
}
