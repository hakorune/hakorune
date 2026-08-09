use nyash_rust::ast::{
    ASTNode, BoxMethodCompatibilityOriginV1, BoxMethodInventoryRoundtripRowV2,
    BoxMethodInventoryV1, DelegateDecl, DelegateExposeDecl, EnumVariantDecl, FieldDecl, Span,
};
use serde_json::Value;
use std::collections::HashMap;

use super::{AstJsonDecoder, DecodeMode};
use crate::r#macro::ast_json::box_inventory_v2;
use crate::r#macro::ast_json::shared::{self, json_to_attrs};

pub(super) fn decode(decoder: &AstJsonDecoder, kind: &str, value: &Value) -> Option<ASTNode> {
    match kind {
        "BoxDeclaration" => decode_box(decoder, value),
        "EnumDeclaration" => decode_enum(decoder, value),
        "BrandDeclaration" => Some(ASTNode::BrandDeclaration {
            name: value.get("name")?.as_str()?.to_string(),
            underlying_type_name: value.get("underlying_type")?.as_str()?.to_string(),
            span: Span::unknown(),
        }),
        "TypeAliasDeclaration" => Some(ASTNode::TypeAliasDeclaration {
            name: value.get("name")?.as_str()?.to_string(),
            target_type_name: value.get("target_type")?.as_str()?.to_string(),
            span: Span::unknown(),
        }),
        _ => None,
    }
}

fn decode_box(decoder: &AstJsonDecoder, value: &Value) -> Option<ASTNode> {
    let methods = match decoder.mode {
        DecodeMode::Legacy => {
            let methods = value
                .get("methods")?
                .as_array()?
                .iter()
                .filter_map(|method| {
                    Some((
                        method.get("key")?.as_str()?.to_string(),
                        decoder.decode(method.get("decl")?)?,
                    ))
                })
                .collect::<HashMap<String, ASTNode>>();
            BoxMethodInventoryV1::try_from_compatibility_map(
                methods,
                BoxMethodCompatibilityOriginV1::LegacyJsonV1,
            )
            .ok()?
        }
        DecodeMode::RoundtripV2 => {
            let rows = value
                .get("methods")?
                .as_array()?
                .iter()
                .map(|method| {
                    box_inventory_v2::decode_method_entry_v2(method, |decl| decoder.decode(decl))
                })
                .collect::<Option<Vec<BoxMethodInventoryRoundtripRowV2>>>()?;
            nyash_rust::ast::PreparedBoxMethodInventoryRoundtripV2::try_new(rows)
                .ok()?
                .commit()
        }
    };
    let constructors = value
        .get("constructors")?
        .as_array()?
        .iter()
        .filter_map(|constructor| {
            Some((
                constructor.get("key")?.as_str()?.to_string(),
                decoder.decode(constructor.get("decl")?)?,
            ))
        })
        .collect::<HashMap<String, ASTNode>>();
    let static_init = value.get("static_init").and_then(|static_init| {
        static_init.as_array().map(|items| {
            items
                .iter()
                .filter_map(|node| decoder.decode(node))
                .collect::<Vec<ASTNode>>()
        })
    });
    let fields: Vec<String> = value
        .get("fields")?
        .as_array()?
        .iter()
        .filter_map(|field| field.as_str().map(str::to_string))
        .collect();
    let weak_fields: Vec<String> = value
        .get("weak_fields")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|field| field.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let field_decls = value
        .get("field_decls")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    Some(FieldDecl {
                        name: item.get("name")?.as_str()?.to_string(),
                        declared_type_name: item
                            .get("declared_type")
                            .and_then(Value::as_str)
                            .map(str::to_string),
                        is_weak: item
                            .get("is_weak")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                        default_value: item
                            .get("default_value")
                            .and_then(|node| decoder.decode(node))
                            .map(Box::new),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            fields
                .iter()
                .cloned()
                .map(|name| FieldDecl {
                    is_weak: weak_fields.contains(&name),
                    name,
                    declared_type_name: None,
                    default_value: None,
                })
                .collect()
        });

    Some(ASTNode::BoxDeclaration {
        name: value.get("name")?.as_str()?.to_string(),
        fields,
        field_decls,
        public_fields: string_array(value, "public_fields"),
        private_fields: string_array(value, "private_fields"),
        methods,
        constructors,
        init_fields: string_array(value, "init_fields"),
        weak_fields,
        delegates: value
            .get("delegates")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        Some(DelegateDecl::compatibility_only(
                            item.get("field_name")?.as_str()?.to_string(),
                            item.get("exposes")
                                .and_then(Value::as_array)
                                .map(|exposes| {
                                    exposes
                                        .iter()
                                        .filter_map(|expose| {
                                            Some(DelegateExposeDecl {
                                                source_name: expose
                                                    .get("source_name")?
                                                    .as_str()?
                                                    .to_string(),
                                                exposed_name: expose
                                                    .get("exposed_name")?
                                                    .as_str()?
                                                    .to_string(),
                                            })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            BoxMethodCompatibilityOriginV1::LegacyJsonV1,
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default(),
        invariants: value
            .get("invariants")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|node| decoder.decode(node))
                    .collect()
            })
            .unwrap_or_default(),
        transitions: shared::json_to_transition_decls(value.get("transitions")).unwrap_or_default(),
        is_interface: bool_field(value, "is_interface"),
        is_record: bool_field(value, "is_record"),
        extends: string_array(value, "extends"),
        implements: string_array(value, "implements"),
        type_parameters: string_array(value, "type_parameters"),
        is_sync: bool_field(value, "is_sync"),
        is_static: bool_field(value, "is_static"),
        static_init,
        attrs: json_to_attrs(value.get("attrs")),
        span: Span::unknown(),
    })
}

fn decode_enum(decoder: &AstJsonDecoder, value: &Value) -> Option<ASTNode> {
    Some(ASTNode::EnumDeclaration {
        name: value.get("name")?.as_str()?.to_string(),
        variants: value
            .get("variants")?
            .as_array()?
            .iter()
            .filter_map(|item| {
                Some(EnumVariantDecl {
                    name: item.get("name")?.as_str()?.to_string(),
                    payload_type_name: item
                        .get("payload_type")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    tuple_payload_type_names: item
                        .get("tuple_payload_types")
                        .and_then(Value::as_array)
                        .map(|types| {
                            types
                                .iter()
                                .filter_map(|item| item.as_str().map(str::to_string))
                                .collect()
                        })
                        .unwrap_or_default(),
                    record_field_decls: item
                        .get("record_fields")
                        .and_then(Value::as_array)
                        .map(|fields| {
                            fields
                                .iter()
                                .filter_map(|field| {
                                    Some(FieldDecl {
                                        name: field.get("name")?.as_str()?.to_string(),
                                        declared_type_name: field
                                            .get("declared_type")
                                            .and_then(Value::as_str)
                                            .map(str::to_string),
                                        is_weak: field
                                            .get("is_weak")
                                            .and_then(Value::as_bool)
                                            .unwrap_or(false),
                                        default_value: field
                                            .get("default_value")
                                            .and_then(|node| decoder.decode(node))
                                            .map(Box::new),
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                })
            })
            .collect(),
        type_parameters: string_array(value, "type_parameters"),
        attrs: json_to_attrs(value.get("attrs")),
        span: Span::unknown(),
    })
}

fn string_array(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn bool_field(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}
