use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{
    ArrayRmwAdd1LeafSeedProof, ArrayRmwAdd1LeafSeedRoute, ArrayRmwWindowProof,
    ExactSeedBackendRoute, ExactSeedBackendRouteKind, MirModule, SumLocalAggregateLayout,
    SumVariantProjectSeedKind, SumVariantProjectSeedPayload, SumVariantProjectSeedProof,
    SumVariantProjectSeedRoute, SumVariantTagSeedKind, SumVariantTagSeedProof,
    SumVariantTagSeedRoute, ValueId,
};
use hakorune_mir_core::BasicBlockId;

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
fn build_mir_json_root_emits_array_rmw_add1_leaf_seed_and_exact_route() {
    let mut function = make_function("main", true);
    function.metadata.array_rmw_add1_leaf_seed_route = Some(ArrayRmwAdd1LeafSeedRoute {
        size: 128,
        ops: 2_000_000,
        init_push_count: 1,
        final_get_count: 2,
        selected_rmw_block: BasicBlockId::new(23),
        selected_rmw_instruction_index: 8,
        selected_rmw_set_instruction_index: 13,
        proof: ArrayRmwAdd1LeafSeedProof::KiloLeafArrayRmwAdd1SevenBlock,
        rmw_proof: ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot,
    });
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf,
        source_route: "array_rmw_add1_leaf_seed_route".to_string(),
        proof: "kilo_leaf_array_rmw_add1_7block".to_string(),
        selected_value: None,
    });
    let mut module = MirModule::new("json_array_rmw_add1_leaf_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let seed_route = &metadata["array_rmw_add1_leaf_seed_route"];
    assert_eq!(seed_route["size"], 128);
    assert_eq!(seed_route["ops"], 2_000_000);
    assert_eq!(seed_route["init_push_count"], 1);
    assert_eq!(seed_route["final_get_count"], 2);
    assert_eq!(seed_route["selected_rmw_block"], 23);
    assert_eq!(seed_route["selected_rmw_instruction_index"], 8);
    assert_eq!(seed_route["selected_rmw_set_instruction_index"], 13);
    assert_eq!(seed_route["proof"], "kilo_leaf_array_rmw_add1_7block");
    assert_eq!(seed_route["rmw_proof"], "array_get_add1_set_same_slot");

    let route = &metadata["exact_seed_backend_route"];
    assert_eq!(route["tag"], "array_rmw_add1_leaf");
    assert_eq!(route["source_route"], "array_rmw_add1_leaf_seed_route");
    assert_eq!(route["proof"], "kilo_leaf_array_rmw_add1_7block");
    assert!(route["selected_value"].is_null());
}

#[test]
fn build_mir_json_root_emits_sum_variant_tag_seed_and_exact_route() {
    let mut function = make_function("main", true);
    function.metadata.sum_variant_tag_seed_route = Some(SumVariantTagSeedRoute {
        kind: SumVariantTagSeedKind::LocalI64,
        enum_name: "Result".to_string(),
        variant: "Ok".to_string(),
        subject: "Result::Ok".to_string(),
        layout: SumLocalAggregateLayout::TagI64Payload,
        variant_tag: 0,
        make_block: BasicBlockId::new(0),
        make_instruction_index: 1,
        tag_block: BasicBlockId::new(0),
        tag_instruction_index: 2,
        sum_value: ValueId::new(2),
        tag_value: ValueId::new(3),
        tag_source_value: ValueId::new(2),
        copy_value: None,
        payload_value: Some(ValueId::new(1)),
        proof: SumVariantTagSeedProof::LocalAggregateTagSeed,
    });
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::SumVariantTagLocal,
        source_route: "sum_variant_tag_seed_route".to_string(),
        proof: "sum_variant_tag_local_aggregate_seed".to_string(),
        selected_value: None,
    });
    let mut module = MirModule::new("json_sum_variant_tag_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let seed_route = &metadata["sum_variant_tag_seed_route"];
    assert_eq!(seed_route["kind"], "variant_tag_local_i64");
    assert_eq!(seed_route["enum"], "Result");
    assert_eq!(seed_route["variant"], "Ok");
    assert_eq!(seed_route["subject"], "Result::Ok");
    assert_eq!(seed_route["layout"], "tag_i64_payload");
    assert_eq!(seed_route["variant_tag"], 0);
    assert_eq!(seed_route["make_block"], 0);
    assert_eq!(seed_route["make_instruction_index"], 1);
    assert_eq!(seed_route["tag_block"], 0);
    assert_eq!(seed_route["tag_instruction_index"], 2);
    assert_eq!(seed_route["sum_value"], 2);
    assert_eq!(seed_route["tag_value"], 3);
    assert_eq!(seed_route["tag_source_value"], 2);
    assert!(seed_route["copy_value"].is_null());
    assert_eq!(seed_route["payload_value"], 1);
    assert_eq!(seed_route["proof"], "sum_variant_tag_local_aggregate_seed");
    assert_eq!(
        seed_route["consumer_capability"],
        "direct_sum_variant_tag_local"
    );

    let route = &metadata["exact_seed_backend_route"];
    assert_eq!(route["tag"], "sum_variant_tag_local");
    assert_eq!(route["source_route"], "sum_variant_tag_seed_route");
    assert_eq!(route["proof"], "sum_variant_tag_local_aggregate_seed");
    assert!(route["selected_value"].is_null());
}

#[test]
fn build_mir_json_root_emits_sum_variant_project_seed_and_exact_route() {
    let mut function = make_function("main", true);
    function.metadata.sum_variant_project_seed_route = Some(SumVariantProjectSeedRoute {
        kind: SumVariantProjectSeedKind::CopyLocalHandle,
        enum_name: "ResultHandle".to_string(),
        variant: "Ok".to_string(),
        subject: "ResultHandle::Ok".to_string(),
        layout: SumLocalAggregateLayout::TagHandlePayload,
        variant_tag: 0,
        make_block: BasicBlockId::new(0),
        make_instruction_index: 1,
        project_block: BasicBlockId::new(0),
        project_instruction_index: 3,
        sum_value: ValueId::new(2),
        project_value: ValueId::new(4),
        project_source_value: ValueId::new(3),
        copy_value: Some(ValueId::new(3)),
        payload_value: ValueId::new(1),
        payload: SumVariantProjectSeedPayload::String("hako".to_string()),
        proof: SumVariantProjectSeedProof::LocalAggregateProjectSeed,
    });
    function.metadata.exact_seed_backend_route = Some(ExactSeedBackendRoute {
        tag: ExactSeedBackendRouteKind::SumVariantProjectLocal,
        source_route: "sum_variant_project_seed_route".to_string(),
        proof: "sum_variant_project_local_aggregate_seed".to_string(),
        selected_value: None,
    });
    let mut module = MirModule::new("json_sum_variant_project_route_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let seed_route = &metadata["sum_variant_project_seed_route"];
    assert_eq!(seed_route["kind"], "variant_project_copy_local_handle");
    assert_eq!(seed_route["enum"], "ResultHandle");
    assert_eq!(seed_route["variant"], "Ok");
    assert_eq!(seed_route["subject"], "ResultHandle::Ok");
    assert_eq!(seed_route["layout"], "tag_handle_payload");
    assert_eq!(seed_route["variant_tag"], 0);
    assert_eq!(seed_route["make_instruction_index"], 1);
    assert_eq!(seed_route["project_instruction_index"], 3);
    assert_eq!(seed_route["sum_value"], 2);
    assert_eq!(seed_route["project_value"], 4);
    assert_eq!(seed_route["project_source_value"], 3);
    assert_eq!(seed_route["copy_value"], 3);
    assert_eq!(seed_route["payload_value"], 1);
    assert_eq!(seed_route["payload_literal_kind"], "string");
    assert!(seed_route["payload_i64"].is_null());
    assert!(seed_route["payload_f64"].is_null());
    assert_eq!(seed_route["payload_string"], "hako");
    assert_eq!(
        seed_route["proof"],
        "sum_variant_project_local_aggregate_seed"
    );
    assert_eq!(
        seed_route["consumer_capability"],
        "direct_sum_variant_project_local"
    );

    let route = &metadata["exact_seed_backend_route"];
    assert_eq!(route["tag"], "sum_variant_project_local");
    assert_eq!(route["source_route"], "sum_variant_project_seed_route");
    assert_eq!(route["proof"], "sum_variant_project_local_aggregate_seed");
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
