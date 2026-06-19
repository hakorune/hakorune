use serde_json::{json, Value};
use syn::{Fields, ImplItem, Item};

use crate::functions::function_to_json;
use crate::names::{assert_unique_names, insert_name_metadata};
use crate::types::{item_kind, type_name};

pub(crate) fn file_to_json(file: &syn::File, module: String) -> Value {
    file_to_json_inner(file, module, true)
}

pub(crate) fn file_to_json_for_crate(file: &syn::File, module: String) -> Value {
    file_to_json_inner(file, module, false)
}

fn file_to_json_inner(file: &syn::File, module: String, include_external_mods: bool) -> Value {
    let items = file.items.iter().map(item_to_json).collect::<Vec<_>>();
    let items = if include_external_mods {
        items
    } else {
        file.items
            .iter()
            .filter(|item| !is_external_mod_decl(item))
            .map(item_to_json)
            .collect::<Vec<_>>()
    };
    assert_unique_names(&items, "module items");
    json!({
        "schema_version": 0,
        "kind": "RustSubsetModule",
        "module": module,
        "items": items,
    })
}

fn is_external_mod_decl(item: &Item) -> bool {
    matches!(item, Item::Mod(module) if module.content.is_none())
}

fn item_to_json(item: &Item) -> Value {
    match item {
        Item::Struct(item) => {
            let identity = has_hako_identity(&item.attrs);
            let mut value = json!({
                "kind": "Struct",
                "identity": identity,
                "fields": fields_to_json(&item.fields),
            });
            insert_name_metadata(&mut value, &item.ident.to_string());
            if identity {
                value["identity_reason"] = json!("resource_or_mutable_state");
            }
            value
        }
        Item::Enum(item) => {
            let variants = item
                .variants
                .iter()
                .map(|variant| {
                    let fields = match &variant.fields {
                        Fields::Unit => Vec::new(),
                        Fields::Named(named) => named
                            .named
                            .iter()
                            .map(|field| json!({"type": type_name(&field.ty)}))
                            .collect(),
                        Fields::Unnamed(unnamed) => unnamed
                            .unnamed
                            .iter()
                            .map(|field| json!({"type": type_name(&field.ty)}))
                            .collect(),
                    };
                    let mut variant_value = json!({"fields": fields});
                    insert_name_metadata(&mut variant_value, &variant.ident.to_string());
                    variant_value
                })
                .collect::<Vec<_>>();
            assert_unique_names(&variants, "enum variants");
            let mut value = json!({
                "kind": "Enum",
                "variants": variants,
            });
            insert_name_metadata(&mut value, &item.ident.to_string());
            value
        }
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
            assert_unique_names(&methods, "impl methods");
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
    let values = match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|field| {
                let source = field
                    .ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                let mut value = json!({"type": type_name(&field.ty)});
                insert_name_metadata(&mut value, &source);
                value
            })
            .collect(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                json!({
                    "name": format!("_{index}"),
                    "source_name": index.to_string(),
                    "emitted_name": format!("_{index}"),
                    "type": type_name(&field.ty),
                })
            })
            .collect(),
    };
    assert_unique_names(&values, "struct fields");
    values
}

fn has_hako_identity(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("hako_identity"))
}
