use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{ExactSeedBackendRoute, ExactSeedBackendRouteKind, MirModule};

#[test]
fn build_mir_json_root_emits_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::ArrayStringStoreMicro,
        source_route: "array_string_store_micro_seed_route".to_string(),
        proof: "kilo_micro_array_string_store_8block".to_string(),
    });
    let mut module = MirModule::new("json_exact_seed_backend_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["exact_seed_backend_route"];
    assert_eq!(route["tag"], "array_string_store_micro");
    assert_eq!(route["source_route"], "array_string_store_micro_seed_route");
    assert_eq!(route["proof"], "kilo_micro_array_string_store_8block");
}

#[test]
fn build_mir_json_root_emits_concat_const_suffix_exact_seed_backend_route() {
    let mut function = make_function("main", true);
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::ConcatConstSuffixMicro,
        source_route: "concat_const_suffix_micro_seed_route".to_string(),
        proof: "kilo_micro_concat_const_suffix_5block".to_string(),
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
}
