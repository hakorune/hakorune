use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{ExactSeedBackendRoute, ExactSeedBackendRouteKind, MirModule, ValueId};

#[test]
fn build_mir_json_root_emits_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::ArrayStringStoreMicro,
        source_route: "array_string_store_micro_seed_route".to_string(),
        proof: "kilo_micro_array_string_store_8block".to_string(),
        selected_value: None,
    });
    let mut module = MirModule::new("json_exact_seed_backend_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["exact_seed_backend_route"];
    assert_eq!(route["tag"], "array_string_store_micro");
    assert_eq!(route["source_route"], "array_string_store_micro_seed_route");
    assert_eq!(route["proof"], "kilo_micro_array_string_store_8block");
    assert!(route["selected_value"].is_null());
}

#[test]
fn build_mir_json_root_emits_concat_const_suffix_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::ConcatConstSuffixMicro,
        source_route: "concat_const_suffix_micro_seed_route".to_string(),
        proof: "kilo_micro_concat_const_suffix_5block".to_string(),
        selected_value: None,
    });
    let mut module = MirModule::new("json_concat_exact_seed_backend_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["exact_seed_backend_route"];
    assert_eq!(route["tag"], "concat_const_suffix_micro");
    assert_eq!(
        route["source_route"],
        "concat_const_suffix_micro_seed_route"
    );
    assert_eq!(route["proof"], "kilo_micro_concat_const_suffix_5block");
    assert!(route["selected_value"].is_null());
}

#[test]
fn build_mir_json_root_emits_substring_views_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::SubstringViewsOnlyMicro,
        source_route: "substring_views_micro_seed_route".to_string(),
        proof: "kilo_micro_substring_views_only_5block".to_string(),
        selected_value: None,
    });
    let mut module =
        MirModule::new("json_substring_views_exact_seed_backend_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["exact_seed_backend_route"];
    assert_eq!(route["tag"], "substring_views_only_micro");
    assert_eq!(route["source_route"], "substring_views_micro_seed_route");
    assert_eq!(route["proof"], "kilo_micro_substring_views_only_5block");
    assert!(route["selected_value"].is_null());
}

#[test]
fn build_mir_json_root_emits_substring_concat_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::SubstringConcatLoopAscii,
        source_route: "string_kernel_plans.loop_payload".to_string(),
        proof: "string_kernel_plan_concat_triplet_loop_payload".to_string(),
        selected_value: Some(ValueId::new(35)),
    });
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
