use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::exact_seed_backend_route::test_support as exact_seed_backend_route_fixture;
use crate::mir::{MirModule, ValueId};

#[test]
fn build_mir_json_root_emits_substring_concat_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route =
        Some(exact_seed_backend_route_fixture::substring_concat_loop_ascii(ValueId::new(35)));
    let mut module =
        MirModule::new("json_substring_concat_exact_seed_backend_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["exact_seed_backend_route"];
    assert_eq!(route["tag"], "substring_concat_loop_ascii");
    assert_eq!(route["source_route"], "string_kernel_plans.loop_payload");
    assert_eq!(
        route["proof"],
        "string_kernel_plan_concat_triplet_loop_payload"
    );
    assert_eq!(route["selected_value"], 35);
}
