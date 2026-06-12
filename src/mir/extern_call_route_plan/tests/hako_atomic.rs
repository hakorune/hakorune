use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_slot_cas_route() {
    let mut function = make_function_with_call(
        "hako_atomic_slot_cas_i64/3",
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
        Some(ValueId::new(4)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.slot_cas_i64");
    assert_eq!(route.core_op(), "HakoAtomicSlotCasI64");
    assert_eq!(route.symbol(), "hako_atomic_slot_cas_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_slot_cas_i64/3");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(3)));
    assert_eq!(route.result_value(), ValueId::new(4));
    assert_eq!(route.arity(), 3);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.atomic.slot_cas"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_slot_load_route() {
    let mut function = make_function_with_call(
        "hako_atomic_slot_load_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.slot_load_i64");
    assert_eq!(route.core_op(), "HakoAtomicSlotLoadI64");
    assert_eq!(route.symbol(), "hako_atomic_slot_load_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_slot_load_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.atomic.slot_load"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_slot_store_route() {
    let mut function = make_function_with_call(
        "hako_atomic_slot_store_i64/2",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.slot_store_i64");
    assert_eq!(route.core_op(), "HakoAtomicSlotStoreI64");
    assert_eq!(route.symbol(), "hako_atomic_slot_store_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_slot_store_i64/2");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.atomic.slot_store"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_ptr_cas_ordered_route() {
    let mut function = make_function_with_call(
        "hako_atomic_ptr_cas_ordered/5",
        vec![
            ValueId::new(1),
            ValueId::new(2),
            ValueId::new(3),
            ValueId::new(4),
            ValueId::new(5),
        ],
        Some(ValueId::new(6)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.ptr_cas_ordered");
    assert_eq!(route.core_op(), "HakoAtomicPtrCasOrdered");
    assert_eq!(route.symbol(), "hako_atomic_ptr_cas_ordered");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_ptr_cas_ordered/5");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(3)));
    assert_eq!(route.result_value(), ValueId::new(6));
    assert_eq!(route.arity(), 5);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.atomic.ptr_cas"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_ptr_load_ordered_route() {
    let mut function = make_function_with_call(
        "hako_atomic_ptr_load_ordered/2",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.ptr_load_ordered");
    assert_eq!(route.core_op(), "HakoAtomicPtrLoadOrdered");
    assert_eq!(route.symbol(), "hako_atomic_ptr_load_ordered");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_ptr_load_ordered/2");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.atomic.ptr_load"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_ptr_store_ordered_route() {
    let mut function = make_function_with_call(
        "hako_atomic_ptr_store_ordered/3",
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
        Some(ValueId::new(4)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.ptr_store_ordered");
    assert_eq!(route.core_op(), "HakoAtomicPtrStoreOrdered");
    assert_eq!(route.symbol(), "hako_atomic_ptr_store_ordered");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_ptr_store_ordered/3");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(4));
    assert_eq!(route.arity(), 3);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.atomic.ptr_store"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_atomic_slot_fetch_add_route() {
    let mut function = make_function_with_call(
        "hako_atomic_slot_fetch_add_i64/2",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_atomic.slot_fetch_add_i64");
    assert_eq!(route.core_op(), "HakoAtomicSlotFetchAddI64");
    assert_eq!(route.symbol(), "hako_atomic_slot_fetch_add_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_atomic_slot_fetch_add_i64/2");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.atomic.slot_fetch_add"]);
}
