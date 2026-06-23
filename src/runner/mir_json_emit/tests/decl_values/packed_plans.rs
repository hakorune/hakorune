use super::super::super::{
    collect_array_record_autouse_eligibility_plan_values,
    collect_array_record_materialization_boundary_plan_values,
    collect_array_record_packed_autouse_pilot_plan_values,
    collect_hako_alloc_aligned_small_packed_store_pilot_plan_values,
    collect_hako_alloc_huge_page_packed_store_pilot_plan_values,
    collect_source_packed_array_autouse_pilot_plan_values, collect_static_data_plan_values,
};
use crate::mir::function::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, HakoAllocAlignedSmallPackedStorePilotPlan,
    HakoAllocHugePagePackedStorePilotPlan, SourcePackedArrayAutoUsePilotPlan, StaticDataPlan,
};
use crate::mir::MirModule;
use serde_json::json;

#[test]
fn collect_array_record_autouse_eligibility_plan_values_preserves_gate_truth() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.array_record_autouse_eligibility_plans.push(
        ArrayRecordAutoUseEligibilityPlan {
            record_name: "Meta".to_string(),
            layout_id: 1,
            storage_kind: "inline_record_columns_v0".to_string(),
            decision: "eligible".to_string(),
            reason: "integer-lane-non-escaping-candidate".to_string(),
            field_count: 2,
            integer_lane_columns: 2,
            required_backend_capability: Some("arraybox.inline_record_columns_v0".to_string()),
            production_auto_use_enabled: false,
        },
    );

    let plans = collect_array_record_autouse_eligibility_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(plans[0]["storage_kind"], "inline_record_columns_v0");
    assert_eq!(plans[0]["decision"], "eligible");
    assert_eq!(plans[0]["reason"], "integer-lane-non-escaping-candidate");
    assert_eq!(plans[0]["field_count"], 2);
    assert_eq!(plans[0]["integer_lane_columns"], 2);
    assert_eq!(
        plans[0]["required_backend_capability"],
        "arraybox.inline_record_columns_v0"
    );
    assert_eq!(plans[0]["production_auto_use_enabled"], false);
}

#[test]
fn collect_array_record_materialization_boundary_plan_values_preserves_stop_line() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .array_record_materialization_boundary_plans
        .push(ArrayRecordMaterializationBoundaryPlan {
            record_name: "Meta".to_string(),
            layout_id: 1,
            boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            source_decision: "eligible".to_string(),
            direct_indexed_field_reads_allowed: true,
            visible_record_materialization_enabled: false,
            public_array_get_action: "fail_fast_unmaterialized_record_value".to_string(),
            returned_element_action: "fail_fast_unmaterialized_record_value".to_string(),
            host_backend_escape_action: "fail_fast_unmaterialized_record_value".to_string(),
            diagnostic:
                "[array/inline-record/unmaterialized] record value materialization is not enabled"
                    .to_string(),
            runtime_auto_use_enabled: false,
        });

    let plans = collect_array_record_materialization_boundary_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(
        plans[0]["boundary_kind"],
        "non_escaping_direct_field_reads_v0"
    );
    assert_eq!(plans[0]["source_decision"], "eligible");
    assert_eq!(plans[0]["direct_indexed_field_reads_allowed"], true);
    assert_eq!(plans[0]["visible_record_materialization_enabled"], false);
    assert_eq!(
        plans[0]["public_array_get_action"],
        "fail_fast_unmaterialized_record_value"
    );
    assert_eq!(
        plans[0]["returned_element_action"],
        "fail_fast_unmaterialized_record_value"
    );
    assert_eq!(
        plans[0]["host_backend_escape_action"],
        "fail_fast_unmaterialized_record_value"
    );
    assert_eq!(
        plans[0]["diagnostic"],
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
    assert_eq!(plans[0]["runtime_auto_use_enabled"], false);
}

#[test]
fn collect_array_record_packed_autouse_pilot_plan_values_preserves_pilot_limits() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .push(ArrayRecordPackedAutoUsePilotPlan {
            record_name: "Meta".to_string(),
            layout_id: 1,
            pilot_kind: "integer_lane_direct_reads_v0".to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            integer_lane_columns: 2,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            hako_alloc_migration_enabled: false,
            backend_lowering_enabled: false,
        });

    let plans = collect_array_record_packed_autouse_pilot_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(plans[0]["pilot_kind"], "integer_lane_direct_reads_v0");
    assert_eq!(
        plans[0]["source_boundary_kind"],
        "non_escaping_direct_field_reads_v0"
    );
    assert_eq!(plans[0]["integer_lane_columns"], 2);
    assert_eq!(plans[0]["direct_indexed_field_reads_enabled"], true);
    assert_eq!(plans[0]["private_runtime_storage_enabled"], true);
    assert_eq!(plans[0]["public_array_get_materialization_enabled"], false);
    assert_eq!(plans[0]["hako_alloc_migration_enabled"], false);
    assert_eq!(plans[0]["backend_lowering_enabled"], false);
}

#[test]
fn collect_source_packed_array_autouse_pilot_plan_values_preserves_source_limits() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .source_packed_array_autouse_pilot_plans
        .push(SourcePackedArrayAutoUsePilotPlan {
            owner_box: "Store".to_string(),
            field_name: "metas".to_string(),
            declared_type_name: "PackedArray<Meta>".to_string(),
            record_name: "Meta".to_string(),
            layout_id: 1,
            pilot_kind: "declared_packed_record_array_v0".to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            source_declared_packed: true,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
            boxed_fallback_enabled: false,
        });

    let plans = collect_source_packed_array_autouse_pilot_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["owner_box"], "Store");
    assert_eq!(plans[0]["field_name"], "metas");
    assert_eq!(plans[0]["declared_type"], "PackedArray<Meta>");
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(plans[0]["pilot_kind"], "declared_packed_record_array_v0");
    assert_eq!(plans[0]["source_declared_packed"], true);
    assert_eq!(plans[0]["direct_indexed_field_reads_enabled"], true);
    assert_eq!(plans[0]["private_runtime_storage_enabled"], true);
    assert_eq!(plans[0]["public_array_get_materialization_enabled"], false);
    assert_eq!(plans[0]["backend_lowering_enabled"], false);
    assert_eq!(plans[0]["boxed_fallback_enabled"], false);
}

#[test]
fn collect_hako_alloc_aligned_small_packed_store_pilot_plan_values_preserves_pilot_limits() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .hako_alloc_aligned_small_packed_store_pilot_plans
        .push(HakoAllocAlignedSmallPackedStorePilotPlan {
            record_name: "HakoAllocAlignedSmallMeta".to_string(),
            store_owner: "HakoAllocAlignedSmallMetaStore".to_string(),
            layout_id: 7,
            pilot_kind: "aligned_small_metadata_i64_columns_v0".to_string(),
            ptr_column: 0,
            alignment_column: 1,
            padded_size_column: 2,
            private_runtime_storage_enabled: true,
            hako_alloc_source_mentions_compiler: false,
            live_scalar_columns_retained: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
        });

    let plans = collect_hako_alloc_aligned_small_packed_store_pilot_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "HakoAllocAlignedSmallMeta");
    assert_eq!(plans[0]["store_owner"], "HakoAllocAlignedSmallMetaStore");
    assert_eq!(plans[0]["layout_id"], 7);
    assert_eq!(
        plans[0]["pilot_kind"],
        "aligned_small_metadata_i64_columns_v0"
    );
    assert_eq!(plans[0]["ptr_column"], 0);
    assert_eq!(plans[0]["alignment_column"], 1);
    assert_eq!(plans[0]["padded_size_column"], 2);
    assert_eq!(plans[0]["private_runtime_storage_enabled"], true);
    assert_eq!(plans[0]["hako_alloc_source_mentions_compiler"], false);
    assert_eq!(plans[0]["live_scalar_columns_retained"], true);
    assert_eq!(plans[0]["public_array_get_materialization_enabled"], false);
    assert_eq!(plans[0]["backend_lowering_enabled"], false);
}

#[test]
fn collect_hako_alloc_huge_page_packed_store_pilot_plan_values_preserves_pilot_limits() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .hako_alloc_huge_page_packed_store_pilot_plans
        .push(HakoAllocHugePagePackedStorePilotPlan {
            record_name: "HakoAllocHugePageMeta".to_string(),
            store_owner: "HakoAllocHugePageMetaStore".to_string(),
            layout_id: 9,
            pilot_kind: "huge_page_metadata_i64_columns_v0".to_string(),
            page_id_column: 0,
            ptr_column: 1,
            requested_size_column: 2,
            committed_size_column: 3,
            live_column: 4,
            released_page_id_sentinel: -1,
            released_size_sentinel: 0,
            private_runtime_storage_enabled: true,
            hako_alloc_source_mentions_compiler: false,
            live_scalar_columns_retained: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
        });

    let plans = collect_hako_alloc_huge_page_packed_store_pilot_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "HakoAllocHugePageMeta");
    assert_eq!(plans[0]["store_owner"], "HakoAllocHugePageMetaStore");
    assert_eq!(plans[0]["layout_id"], 9);
    assert_eq!(plans[0]["pilot_kind"], "huge_page_metadata_i64_columns_v0");
    assert_eq!(plans[0]["page_id_column"], 0);
    assert_eq!(plans[0]["ptr_column"], 1);
    assert_eq!(plans[0]["requested_size_column"], 2);
    assert_eq!(plans[0]["committed_size_column"], 3);
    assert_eq!(plans[0]["live_column"], 4);
    assert_eq!(plans[0]["released_page_id_sentinel"], -1);
    assert_eq!(plans[0]["released_size_sentinel"], 0);
    assert_eq!(plans[0]["private_runtime_storage_enabled"], true);
    assert_eq!(plans[0]["hako_alloc_source_mentions_compiler"], false);
    assert_eq!(plans[0]["live_scalar_columns_retained"], true);
    assert_eq!(plans[0]["public_array_get_materialization_enabled"], false);
    assert_eq!(plans[0]["backend_lowering_enabled"], false);
}

#[test]
fn collect_static_data_plan_values_preserves_backend_row_truth() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.static_data_plans.push(StaticDataPlan {
        source_name: "SIZE_CLASS".to_string(),
        symbol: ".hako.static.SIZE_CLASS".to_string(),
        element: "u16".to_string(),
        align: 2,
        linkage: "private".to_string(),
        unnamed_addr: true,
        values: vec![8, 16, 24, 32],
    });

    let plans = collect_static_data_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["source_name"], "SIZE_CLASS");
    assert_eq!(plans[0]["symbol"], ".hako.static.SIZE_CLASS");
    assert_eq!(plans[0]["element"], "u16");
    assert_eq!(plans[0]["align"], 2);
    assert_eq!(plans[0]["linkage"], "private");
    assert_eq!(plans[0]["unnamed_addr"], true);
    assert_eq!(plans[0]["values"], json!([8, 16, 24, 32]));
}
