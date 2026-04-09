use crate::ast::{ASTNode, EnumVariantDecl};

pub(super) fn collect_enum_record_payload_box_decls(
    statements: &[ASTNode],
) -> Vec<serde_json::Value> {
    let mut decls = Vec::new();
    for statement in statements {
        let ASTNode::EnumDeclaration { name, variants, .. } = statement else {
            continue;
        };
        for variant in variants {
            if !variant.is_record_payload() {
                continue;
            }
            decls.push(enum_record_payload_box_decl(name, variant));
        }
    }
    decls
}

pub(super) fn enum_variant_payload_type_name(
    enum_name: &str,
    variant: &EnumVariantDecl,
) -> Option<String> {
    if variant.is_record_payload() {
        Some(enum_record_payload_box_name(enum_name, &variant.name))
    } else {
        variant.payload_type_name.clone()
    }
}

pub(super) fn enum_record_payload_box_name(enum_name: &str, variant_name: &str) -> String {
    format!("__NyEnumPayload_{}_{}", enum_name, variant_name)
}

fn enum_record_payload_box_decl(enum_name: &str, variant: &EnumVariantDecl) -> serde_json::Value {
    serde_json::json!({
        "name": enum_record_payload_box_name(enum_name, &variant.name),
        "fields": variant
            .record_field_decls
            .iter()
            .map(|field| field.name.clone())
            .collect::<Vec<_>>(),
        "field_decls": variant.record_field_decls.iter().map(|field| serde_json::json!({
            "name": field.name,
            "declared_type": field.declared_type_name,
            "is_weak": field.is_weak,
        })).collect::<Vec<_>>(),
    })
}
