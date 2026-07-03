use super::super::root::build_mir_json_root;
use super::super::{
    collect_array_record_storage_plan_values, collect_object_storage_plan_values,
    collect_record_layout_plan_values, collect_sorted_enum_decl_values,
    collect_sorted_record_decl_values, collect_sorted_user_box_decl_values,
    collect_typed_object_plan_values,
};
use crate::mir::function::{
    ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan, RecordDecl, RecordLayoutFieldPlan,
    RecordLayoutPlan, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::MirModule;
use serde_json::json;

mod packed_plans;

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
        RecordDecl {
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
fn collect_object_storage_plan_values_exports_flattened_nested_alignment_result() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "HakoAllocObjectLifecycleFacade".to_string(),
        type_id: 3,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "alignment_result".to_string(),
            slot: 3,
            declared_type_name: Some("HakoAllocObjectLifecycleAlignmentResult".to_string()),
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "HakoAllocObjectLifecycleAlignmentResult".to_string(),
        type_id: 1,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 4,
        fields: vec![
            TypedObjectFieldPlan {
                name: "last_requested".to_string(),
                slot: 0,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_normalized".to_string(),
                slot: 1,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_reason".to_string(),
                slot: 2,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_supported".to_string(),
                slot: 3,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
        ],
    });

    let plans = collect_object_storage_plan_values(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["representation"], "flattened_nested_fields");
    assert_eq!(plans[0]["source_evidence"], "296x-726");
    assert_eq!(plans[0]["owner_box"], "HakoAllocObjectLifecycleFacade");
    assert_eq!(plans[0]["owner_field"], "alignment_result");
    assert_eq!(
        plans[0]["nested_box"],
        "HakoAllocObjectLifecycleAlignmentResult"
    );
    assert_eq!(plans[0]["flattened_field_count"], 4);
    assert_eq!(plans[0]["backend_lowering_enabled"], false);
    assert_eq!(plans[0]["boundary_driver_flattened_nested_consumer"], false);
    assert_eq!(plans[0]["mirbuilder_object_management_enabled"], false);
    assert_eq!(plans[0]["product_default_changed"], false);

    let fields = plans[0]["fields"].as_array().expect("fields");
    let flattened_names: Vec<_> = fields
        .iter()
        .map(|field| field["flattened_field"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(
        flattened_names,
        vec![
            "alignment_result.last_requested",
            "alignment_result.last_normalized",
            "alignment_result.last_reason",
            "alignment_result.last_supported",
        ]
    );
    let key_globals: Vec<_> = fields
        .iter()
        .map(|field| field["key_global"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(
        key_globals,
        vec![
            "@.objstore_alignment_result_last_requested",
            "@.objstore_alignment_result_last_normalized",
            "@.objstore_alignment_result_last_reason",
            "@.objstore_alignment_result_last_supported",
        ]
    );
    assert!(fields
        .iter()
        .all(|field| field["scalar_type"].as_str() == Some("i64")));
    let methods = plans[0]["methods"].as_array().expect("methods");
    let method_names: Vec<_> = methods
        .iter()
        .map(|method| method["method"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(
        method_names,
        vec![
            "requested",
            "normalized",
            "reason",
            "supported",
            "reset",
            "recordFailure",
            "recordSuccess",
        ]
    );
}

#[test]
fn collect_object_storage_plan_values_is_empty_when_nested_proof_is_missing() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "HakoAllocObjectLifecycleFacade".to_string(),
        type_id: 3,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "alignment_result".to_string(),
            slot: 3,
            declared_type_name: Some("OtherBox".to_string()),
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });

    let plans = collect_object_storage_plan_values(&module);

    assert!(plans.is_empty());
}

#[test]
fn build_mir_json_root_includes_object_storage_plans_surface() {
    let mut module = MirModule::new("test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "HakoAllocObjectLifecycleFacade".to_string(),
        type_id: 3,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "alignment_result".to_string(),
            slot: 3,
            declared_type_name: Some("HakoAllocObjectLifecycleAlignmentResult".to_string()),
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "HakoAllocObjectLifecycleAlignmentResult".to_string(),
        type_id: 1,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 4,
        fields: vec![
            TypedObjectFieldPlan {
                name: "last_requested".to_string(),
                slot: 0,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_normalized".to_string(),
                slot: 1,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_reason".to_string(),
                slot: 2,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "last_supported".to_string(),
                slot: 3,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
        ],
    });

    let root = build_mir_json_root(&module).expect("MIR JSON root");
    let plans = root["object_storage_plans"]
        .as_array()
        .expect("object_storage_plans array");

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["representation"], "flattened_nested_fields");
    assert_eq!(plans[0]["flattened_field_count"], 4);
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
