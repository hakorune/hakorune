use super::common::make_function_with_call;
use super::*;
use crate::mir::ValueId;

#[test]
fn refresh_function_extern_call_routes_requires_dst_and_matching_arity() {
    let mut missing_dst = make_function_with_call("env.get/1", vec![ValueId::new(1)], None);
    refresh_function_extern_call_routes(&mut missing_dst);
    assert!(missing_dst.metadata.extern_call_routes.is_empty());

    let mut missing_arg = make_function_with_call("env.get/1", vec![], Some(ValueId::new(2)));
    refresh_function_extern_call_routes(&mut missing_arg);
    assert!(missing_arg.metadata.extern_call_routes.is_empty());

    let mut missing_value =
        make_function_with_call("env.set/2", vec![ValueId::new(1)], Some(ValueId::new(2)));
    refresh_function_extern_call_routes(&mut missing_value);
    assert!(missing_value.metadata.extern_call_routes.is_empty());
}
