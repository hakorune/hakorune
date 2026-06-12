use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_hako_osvm_routes() {
    let mut reserve = make_function_with_call(
        "hako_osvm_reserve_bytes_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );
    refresh_function_extern_call_routes(&mut reserve);
    assert_eq!(reserve.metadata.extern_call_routes.len(), 1);
    let route = &reserve.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.reserve_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmReserveBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_reserve_bytes_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_osvm_reserve_bytes_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.osvm.reserve"]);

    let mut commit = make_function_with_call(
        "hako_osvm_commit_bytes_i64",
        vec![ValueId::new(3), ValueId::new(4)],
        Some(ValueId::new(5)),
    );
    refresh_function_extern_call_routes(&mut commit);
    assert_eq!(commit.metadata.extern_call_routes.len(), 1);
    let route = &commit.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.commit_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmCommitBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_commit_bytes_i64");
    assert_eq!(route.key_value(), ValueId::new(3));
    assert_eq!(route.value_value(), Some(ValueId::new(4)));
    assert_eq!(route.result_value(), ValueId::new(5));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.osvm.commit"]);

    let mut decommit = make_function_with_call(
        "hako_osvm_decommit_bytes_i64/2",
        vec![ValueId::new(6), ValueId::new(7)],
        Some(ValueId::new(8)),
    );
    refresh_function_extern_call_routes(&mut decommit);
    assert_eq!(decommit.metadata.extern_call_routes.len(), 1);
    let route = &decommit.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.decommit_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmDecommitBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_decommit_bytes_i64");
    assert_eq!(route.key_value(), ValueId::new(6));
    assert_eq!(route.value_value(), Some(ValueId::new(7)));
    assert_eq!(route.result_value(), ValueId::new(8));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.osvm.decommit"]);

    let mut unreserve = make_function_with_call(
        "hako_osvm_unreserve_bytes_i64/2",
        vec![ValueId::new(9), ValueId::new(10)],
        Some(ValueId::new(11)),
    );
    refresh_function_extern_call_routes(&mut unreserve);
    assert_eq!(unreserve.metadata.extern_call_routes.len(), 1);
    let route = &unreserve.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.unreserve_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmUnreserveBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_unreserve_bytes_i64");
    assert_eq!(route.key_value(), ValueId::new(9));
    assert_eq!(route.value_value(), Some(ValueId::new(10)));
    assert_eq!(route.result_value(), ValueId::new(11));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.osvm.unreserve"]);
}
