use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_hako_tls_cache_slot_routes() {
    let mut get = make_function_with_call(
        "hako_tls_cache_slot_get_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );
    refresh_function_extern_call_routes(&mut get);
    assert_eq!(get.metadata.extern_call_routes.len(), 1);
    let route = &get.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_tls.cache_slot_get_i64");
    assert_eq!(route.core_op(), "HakoTlsCacheSlotGetI64");
    assert_eq!(route.symbol(), "hako_tls_cache_slot_get_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_tls_cache_slot_get_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.tls.cache_slot_get"]);

    let mut set = make_function_with_call(
        "hako_tls_cache_slot_set_i64",
        vec![ValueId::new(3), ValueId::new(4)],
        Some(ValueId::new(5)),
    );
    refresh_function_extern_call_routes(&mut set);
    assert_eq!(set.metadata.extern_call_routes.len(), 1);
    let route = &set.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_tls.cache_slot_set_i64");
    assert_eq!(route.core_op(), "HakoTlsCacheSlotSetI64");
    assert_eq!(route.symbol(), "hako_tls_cache_slot_set_i64");
    assert_eq!(route.key_value(), ValueId::new(3));
    assert_eq!(route.value_value(), Some(ValueId::new(4)));
    assert_eq!(route.result_value(), ValueId::new(5));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.tls.cache_slot_set"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_worker_current_id_route() {
    let mut function = make_function_with_call(
        "hako_worker_current_id_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );
    refresh_function_extern_call_routes(&mut function);
    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_worker.current_id_i64");
    assert_eq!(route.core_op(), "HakoWorkerCurrentIdI64");
    assert_eq!(route.symbol(), "hako_worker_current_id_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_worker_current_id_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.worker.current_id"]);
}
