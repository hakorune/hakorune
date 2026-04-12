use super::super::{collect_sorted_enum_decl_values, collect_sorted_user_box_decl_values};
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
