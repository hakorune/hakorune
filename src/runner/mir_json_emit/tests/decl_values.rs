use super::super::{
    collect_array_record_storage_plan_values, collect_record_layout_plan_values,
    collect_sorted_enum_decl_values, collect_sorted_record_decl_values,
    collect_sorted_user_box_decl_values, collect_static_data_plan_values,
    collect_typed_object_plan_values,
};
use crate::mir::function::{
    ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan, RecordLayoutFieldPlan, RecordLayoutPlan,
    StaticDataPlan, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
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
