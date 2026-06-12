use super::*;
use super::common::make_function_with_call;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_records_env_get_plan_source() {
    let mut function =
        make_function_with_call("env.get/1", vec![ValueId::new(1)], Some(ValueId::new(2)));

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.env.get");
    assert_eq!(route.core_op(), "EnvGet");
    assert_eq!(route.symbol(), "nyash.env.get");
    assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(
        route.lowering_emit_kind(),
        LoweringPlanEmitKind::RuntimeCall
    );
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "env.get/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "string_handle_or_null");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["read.env"]);
}

#[test]
fn refresh_function_extern_call_routes_records_env_set_plan_source() {
    let mut function = make_function_with_call(
        "env.set/2",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.env.set");
    assert_eq!(route.core_op(), "EnvSet");
    assert_eq!(route.symbol(), "nyash.env.set");
    assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(
        route.lowering_emit_kind(),
        LoweringPlanEmitKind::RuntimeCall
    );
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "env.set/2");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["write.env"]);
}
