use super::super::{
    collect_array_record_autouse_eligibility_plan_values,
    collect_array_record_materialization_boundary_plan_values,
    collect_array_record_packed_autouse_pilot_plan_values,
    collect_array_record_storage_plan_values,
    collect_hako_alloc_aligned_small_packed_store_pilot_plan_values,
    collect_hako_alloc_huge_page_packed_store_pilot_plan_values, collect_record_layout_plan_values,
    collect_sorted_enum_decl_values, collect_sorted_record_decl_values,
    collect_sorted_user_box_decl_values, collect_static_data_plan_values,
    collect_typed_object_plan_values,
};
use crate::mir::function::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan,
    HakoAllocAlignedSmallPackedStorePilotPlan, HakoAllocHugePagePackedStorePilotPlan,
    RecordLayoutFieldPlan, RecordLayoutPlan, StaticDataPlan, TypedObjectFieldPlan,
    TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::MirModule;
use serde_json::json;

#[test]
fn collect_sorted_user_box_decl_values_sorts_by_box_name() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Stage1ProgramResultValidationBox".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Main".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Stage1InputContractBox".to_string(), Vec::new());

    let decls = collect_sorted_user_box_decl_values(&module);
    let names: Vec<_> = decls
        .iter()
        .map(|decl| {
            decl.get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
                .to_string()
        })
        .collect();

    assert_eq!(
        names,
        vec![
            "Main".to_string(),
            "Stage1InputContractBox".to_string(),
            "Stage1ProgramResultValidationBox".to_string(),
        ]
    );
}

#[test]
fn collect_sorted_user_box_decl_values_includes_typed_field_decls() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Point".to_string(), vec!["x".to_string(), "y".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Point".to_string(),
        vec![
            crate::mir::UserBoxFieldDecl {
                name: "x".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            },
            crate::mir::UserBoxFieldDecl {
                name: "y".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: true,
            },
        ],
    );
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Point".to_string(),
        type_id: 7,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "x".to_string(),
            slot: 0,
            declared_type_name: Some("IntegerBox".to_string()),
            storage: TypedObjectFieldStorage::I64,
            is_weak: false,
        }],
    });

    let decls = collect_sorted_user_box_decl_values(&module);
    let point = decls
        .iter()
        .find(|decl| decl.get("name").and_then(serde_json::Value::as_str) == Some("Point"))
        .expect("Point decl");
    let field_decls = point
        .get("field_decls")
        .and_then(serde_json::Value::as_array)
        .expect("field_decls array");

    assert_eq!(field_decls.len(), 2);
    assert_eq!(
        field_decls[0]
            .get("name")
            .and_then(serde_json::Value::as_str),
        Some("x")
    );
    assert_eq!(
        field_decls[0]
            .get("declared_type")
            .and_then(serde_json::Value::as_str),
        Some("IntegerBox")
    );
    assert_eq!(
        field_decls[1]
            .get("is_weak")
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        field_decls[0]
            .get("field_index_fast_path")
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        field_decls[0]
            .get("layout_id")
            .and_then(serde_json::Value::as_u64),
        Some(7)
    );
    assert_eq!(
        field_decls[0]
            .get("field_index")
            .and_then(serde_json::Value::as_u64),
        Some(0)
    );
    assert_eq!(
        field_decls[0]
            .get("storage")
            .and_then(serde_json::Value::as_str),
        Some("i64")
    );
    assert_eq!(
        field_decls[1]
            .get("field_index_fast_path")
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
}

#[test]
fn collect_sorted_record_decl_values_preserves_record_lane() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.record_decls.insert(
        "Meta".to_string(),
        crate::mir::RecordDecl {
            name: "Meta".to_string(),
            type_parameters: vec!["T".to_string()],
            fields: vec![
                crate::mir::UserBoxFieldDecl {
                    name: "ptr".to_string(),
                    declared_type_name: Some("i64".to_string()),
                    is_weak: false,
                },
                crate::mir::UserBoxFieldDecl {
                    name: "payload".to_string(),
                    declared_type_name: Some("T".to_string()),
                    is_weak: false,
                },
            ],
        },
    );
    module
        .metadata
        .user_box_decls
        .insert("Ordinary".to_string(), vec!["x".to_string()]);

    let record_decls = collect_sorted_record_decl_values(&module);
    assert_eq!(record_decls.len(), 1);
    assert_eq!(record_decls[0]["name"], "Meta");
    assert_eq!(record_decls[0]["type_parameters"], json!(["T"]));
    assert_eq!(record_decls[0]["fields"], json!(["ptr", "payload"]));
    assert_eq!(record_decls[0]["field_decls"][0]["name"], "ptr");
    assert_eq!(record_decls[0]["field_decls"][0]["declared_type"], "i64");
    assert_eq!(record_decls[0]["field_decls"][0]["field_index"], 0);
    assert_eq!(record_decls[0]["field_decls"][1]["field_index"], 1);
}

#[test]
fn collect_sorted_enum_decl_values_preserves_variant_inventory() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.enum_decls.insert(
        "Option".to_string(),
        crate::mir::MirEnumDecl {
            type_parameters: vec!["T".to_string()],
            variants: vec![
                crate::mir::MirEnumVariantDecl {
                    name: "None".to_string(),
                    payload_type_name: None,
                },
                crate::mir::MirEnumVariantDecl {
                    name: "Some".to_string(),
                    payload_type_name: Some("T".to_string()),
                },
            ],
        },
    );

    let decls = collect_sorted_enum_decl_values(&module);
    assert_eq!(decls.len(), 1);
    assert_eq!(decls[0]["name"], "Option");
    assert_eq!(decls[0]["type_parameters"], json!(["T"]));
    assert_eq!(decls[0]["variants"][1]["name"], "Some");
    assert_eq!(decls[0]["variants"][1]["payload_type"], "T");
}

#[test]
fn collect_typed_object_plan_values_preserves_backend_layout_truth() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Pair".to_string(),
        type_id: 1,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 2,
        fields: vec![
            TypedObjectFieldPlan {
                name: "left".to_string(),
                slot: 0,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::USize,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "right".to_string(),
                slot: 1,
                declared_type_name: Some("ArrayBox".to_string()),
                storage: TypedObjectFieldStorage::Handle,
                is_weak: false,
            },
        ],
    });

    let plans = collect_typed_object_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["box_name"], "Pair");
    assert_eq!(plans[0]["type_id"], 1);
    assert_eq!(plans[0]["layout_kind"], "runtime_slot_object_v0");
    assert_eq!(plans[0]["field_count"], 2);
    assert_eq!(plans[0]["fields"][0]["name"], "left");
    assert_eq!(plans[0]["fields"][0]["slot"], 0);
    assert_eq!(plans[0]["fields"][0]["storage"], "usize");
    assert_eq!(plans[0]["fields"][0]["weak"], false);
    assert_eq!(plans[0]["fields"][1]["storage"], "handle");
}

#[test]
fn collect_record_layout_plan_values_preserves_record_layout_truth() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.record_layout_plans.push(RecordLayoutPlan {
        record_name: "Meta".to_string(),
        layout_id: 1,
        layout_kind: "record_value_aggregate_v0".to_string(),
        field_count: 2,
        fields: vec![
            RecordLayoutFieldPlan {
                name: "ptr".to_string(),
                slot: 0,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
            },
            RecordLayoutFieldPlan {
                name: "size".to_string(),
                slot: 1,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::USize,
            },
        ],
    });

    let plans = collect_record_layout_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(plans[0]["layout_kind"], "record_value_aggregate_v0");
    assert_eq!(plans[0]["field_count"], 2);
    assert_eq!(plans[0]["fields"][0]["name"], "ptr");
    assert_eq!(plans[0]["fields"][0]["slot"], 0);
    assert_eq!(plans[0]["fields"][0]["storage"], "i64");
    assert_eq!(plans[0]["fields"][1]["storage"], "usize");
}

#[test]
fn collect_array_record_storage_plan_values_preserves_column_truth() {
    let mut module = MirModule::new("test".to_string());
    module
        .metadata
        .array_record_storage_plans
        .push(ArrayRecordStoragePlan {
            record_name: "Meta".to_string(),
            layout_id: 1,
            storage_kind: "inline_record_columns_v0".to_string(),
            field_count: 2,
            columns: vec![
                ArrayRecordStorageColumnPlan {
                    name: "ptr".to_string(),
                    column: 0,
                    storage: TypedObjectFieldStorage::I64,
                },
                ArrayRecordStorageColumnPlan {
                    name: "size".to_string(),
                    column: 1,
                    storage: TypedObjectFieldStorage::USize,
                },
            ],
        });

    let plans = collect_array_record_storage_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["record_name"], "Meta");
    assert_eq!(plans[0]["layout_id"], 1);
    assert_eq!(plans[0]["storage_kind"], "inline_record_columns_v0");
    assert_eq!(plans[0]["field_count"], 2);
    assert_eq!(plans[0]["columns"][0]["name"], "ptr");
    assert_eq!(plans[0]["columns"][0]["column"], 0);
    assert_eq!(plans[0]["columns"][0]["storage"], "i64");
    assert_eq!(plans[0]["columns"][1]["storage"], "usize");
}

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
