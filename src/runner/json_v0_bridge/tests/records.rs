use super::super::*;
use super::common::*;
use crate::mir::{MirInstruction, MirType, ValueId};
use serde_json::json;

#[test]
fn parse_json_v0_to_module_lowers_enum_ctor_to_variant_make() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "enum_decls": option_enum_decls(),
        "body": [
            {
                "type": "Return",
                "expr": option_some_ctor(7)
            }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("enum ctor lowers");
    let insts = main_instructions(&module);

    assert!(matches!(
        insts.iter().find(|inst| matches!(inst, MirInstruction::VariantMake { .. })),
        Some(MirInstruction::VariantMake {
            enum_name,
            variant,
            tag,
            payload: Some(payload),
            payload_type: Some(MirType::Integer),
            ..
        }) if enum_name == "Option"
            && variant == "Some"
            && *tag == 1
            && *payload == ValueId::new(1)
    ));
    assert!(module.metadata.enum_decls.contains_key("Option"));
    assert!(
        !insts.iter().any(|inst| matches!(
            inst,
            MirInstruction::NewBox { .. }
                | MirInstruction::FieldGet { .. }
                | MirInstruction::FieldSet { .. }
        )),
        "enum ctor must stay on sum lane, not box/field lane"
    );
}

#[test]
fn parse_json_v0_to_module_preserves_static_data_plans() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "static_data_plans": [
            {
                "source_name": "SIZE_CLASS",
                "symbol": ".hako.static.SIZE_CLASS",
                "element": "u16",
                "align": 2,
                "linkage": "private",
                "unnamed_addr": true,
                "values": [8, 16, 24, 32]
            }
        ],
        "body": [
            {
                "type": "Return",
                "expr": { "type": "Int", "value": 0 }
            }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("module");
    let plans = &module.metadata.static_data_plans;

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].source_name, "SIZE_CLASS");
    assert_eq!(plans[0].symbol, ".hako.static.SIZE_CLASS");
    assert_eq!(plans[0].element, "u16");
    assert_eq!(plans[0].align, 2);
    assert_eq!(plans[0].linkage, "private");
    assert!(plans[0].unnamed_addr);
    assert_eq!(plans[0].values, vec![8, 16, 24, 32]);
}

#[test]
fn parse_json_v0_to_module_preserves_record_decls_metadata_only() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "record_decls": [
            {
                "name": "Meta",
                "type_parameters": ["T"],
                "fields": ["ptr", "payload"],
                "field_decls": [
                    { "name": "ptr", "declared_type": "i64", "is_weak": false },
                    { "name": "payload", "declared_type": "T", "is_weak": false }
                ]
            }
        ],
        "body": [
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("record metadata module");
    assert!(!module.metadata.user_box_decls.contains_key("Meta"));
    assert!(module.metadata.typed_object_plans.is_empty());
    assert!(module.metadata.record_layout_plans.is_empty());
    let decl = module
        .metadata
        .record_decls
        .get("Meta")
        .expect("record decl");
    assert_eq!(decl.type_parameters, vec!["T".to_string()]);
    assert_eq!(decl.fields.len(), 2);
    assert_eq!(decl.fields[0].name, "ptr");
    assert_eq!(decl.fields[0].declared_type_name.as_deref(), Some("i64"));
}

#[test]
fn parse_json_v0_to_module_derives_concrete_record_layout_plans() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "record_decls": [
            {
                "name": "Meta",
                "fields": ["ptr", "size"],
                "field_decls": [
                    { "name": "ptr", "declared_type": "i64", "is_weak": false },
                    { "name": "size", "declared_type": "usize", "is_weak": false }
                ]
            }
        ],
        "body": [
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("record layout module");
    assert!(module.metadata.typed_object_plans.is_empty());
    let plans = &module.metadata.record_layout_plans;
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].record_name, "Meta");
    assert_eq!(plans[0].layout_kind, "record_value_aggregate_v0");
    assert_eq!(plans[0].fields[0].storage.as_str(), "i64");
    assert_eq!(plans[0].fields[1].storage.as_str(), "usize");
    let storage_plans = &module.metadata.array_record_storage_plans;
    assert_eq!(storage_plans.len(), 1);
    assert_eq!(storage_plans[0].record_name, "Meta");
    assert_eq!(storage_plans[0].storage_kind, "inline_record_columns_v0");
    assert_eq!(storage_plans[0].columns[0].name, "ptr");
    assert_eq!(storage_plans[0].columns[1].storage.as_str(), "usize");
}

#[test]
fn parse_json_v0_to_module_derives_source_packed_array_autouse_pilot_plans() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "record_decls": [
            {
                "name": "Meta",
                "fields": ["ptr", "size"],
                "field_decls": [
                    { "name": "ptr", "declared_type": "i64", "is_weak": false },
                    { "name": "size", "declared_type": "usize", "is_weak": false }
                ]
            }
        ],
        "user_box_decls": [
            {
                "name": "Store",
                "fields": ["metas"],
                "field_decls": [
                    { "name": "metas", "declared_type": "PackedArray<Meta>", "is_weak": false }
                ]
            }
        ],
        "body": [
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("source packed array module");
    let plans = &module.metadata.source_packed_array_autouse_pilot_plans;
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].owner_box, "Store");
    assert_eq!(plans[0].field_name, "metas");
    assert_eq!(plans[0].declared_type_name, "PackedArray<Meta>");
    assert_eq!(plans[0].record_name, "Meta");
    assert!(plans[0].source_declared_packed);
    assert!(plans[0].private_runtime_storage_enabled);
    assert!(!plans[0].public_array_get_materialization_enabled);
    assert!(!plans[0].backend_lowering_enabled);
    assert!(!plans[0].boxed_fallback_enabled);
}

#[test]
fn parse_json_v0_to_module_rejects_option_some_null_payload() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "enum_decls": option_enum_decls(),
        "body": [
            {
                "type": "Return",
                "expr": option_some_null_ctor()
            }
        ]
    })
    .to_string();

    let err = parse_json_v0_to_module(&json).expect_err("Option::Some(null) should be rejected");
    assert!(err.contains("[freeze:contract][option/some_nullish]"));
    assert!(err.contains("Option::Some payload must not be null or void"));
}
