use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_array_slot_append_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_append_hh",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_append_any");
    assert_eq!(route.core_op(), "ArraySlotAppendAny");
    assert_eq!(route.symbol(), "nyash.array.slot_append_hh");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_append_hh");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_append"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_len_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_len_h",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_len_i64");
    assert_eq!(route.core_op(), "ArraySlotLenI64");
    assert_eq!(route.symbol(), "nyash.array.slot_len_h");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_len_h");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_len"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_load_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_load_hi",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_load_i64");
    assert_eq!(route.core_op(), "ArraySlotLoadI64");
    assert_eq!(route.symbol(), "nyash.array.slot_load_hi");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_load_hi");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_load"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_store_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_store_hii",
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
        Some(ValueId::new(4)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_store_i64");
    assert_eq!(route.core_op(), "ArraySlotStoreI64");
    assert_eq!(route.symbol(), "nyash.array.slot_store_hii");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_store_hii");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(3)));
    assert_eq!(route.result_value(), ValueId::new(4));
    assert_eq!(route.arity(), 3);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_store_i64"]);
}
