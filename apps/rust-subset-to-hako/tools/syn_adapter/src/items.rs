use serde_json::{json, Value};
use syn::{Fields, ImplItem, Item};

use crate::functions::function_to_json;
use crate::types::{item_kind, type_name};

pub(crate) fn file_to_json(file: &syn::File, module: String) -> Value {
    json!({
        "schema_version": 0,
        "kind": "RustSubsetModule",
        "module": module,
        "items": file.items.iter().map(item_to_json).collect::<Vec<_>>(),
    })
}

fn item_to_json(item: &Item) -> Value {
    match item {
        Item::Struct(item) => {
            let identity = has_hako_identity(&item.attrs);
            let mut value = json!({
                "kind": "Struct",
                "name": item.ident.to_string(),
                "identity": identity,
                "fields": fields_to_json(&item.fields),
            });
            if identity {
                value["identity_reason"] = json!("resource_or_mutable_state");
            }
            value
        }
        Item::Enum(item) => json!({
            "kind": "Enum",
            "name": item.ident.to_string(),
            "variants": item.variants.iter().map(|variant| {
                let fields = match &variant.fields {
                    Fields::Unit => Vec::new(),
                    Fields::Named(named) => named.named.iter().map(|field| {
                        json!({"type": type_name(&field.ty)})
                    }).collect(),
                    Fields::Unnamed(unnamed) => unnamed.unnamed.iter().map(|field| {
                        json!({"type": type_name(&field.ty)})
                    }).collect(),
                };
                json!({"name": variant.ident.to_string(), "fields": fields})
            }).collect::<Vec<_>>(),
        }),
        Item::Fn(item) => function_to_json(&item.sig, &item.block),
        Item::Impl(item) => {
            let target = type_name(item.self_ty.as_ref());
            let methods = item
                .items
                .iter()
                .filter_map(|impl_item| match impl_item {
                    ImplItem::Fn(method) => Some(function_to_json(&method.sig, &method.block)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            json!({
                "kind": "Impl",
                "target": target,
                "methods": methods,
            })
        }
        _ => json!({
            "kind": "Unsupported",
            "rust_kind": item_kind(item),
            "summary": format!("{} items are out of v0 scope", item_kind(item)),
        }),
    }
}

fn fields_to_json(fields: &Fields) -> Vec<Value> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|field| {
                json!({
                    "name": field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default(),
                    "type": type_name(&field.ty),
                })
            })
            .collect(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| json!({"name": index.to_string(), "type": type_name(&field.ty)}))
            .collect(),
    }
}

fn has_hako_identity(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("hako_identity"))
}
