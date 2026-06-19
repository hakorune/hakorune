use serde_json::Value;
use syn::{GenericArgument, Item, Pat, PathArguments, ReturnType, Type};

use crate::names::{emitted_ident, emitted_path, insert_name_metadata};

pub(crate) fn type_name(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| {
                let base = if path.path.segments.len() == 1 {
                    emitted_ident(&segment.ident.to_string())
                } else {
                    emitted_path(&path.path)
                };
                let args = type_path_args(&segment.arguments);
                if args.is_empty() {
                    base
                } else {
                    format!("{base}<{}>", args.join(", "))
                }
            })
            .unwrap_or_else(|| "Unknown".to_string()),
        Type::Reference(reference) => {
            if let Type::Path(path) = reference.elem.as_ref() {
                if path
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident == "str")
                    .unwrap_or(false)
                {
                    return "String".to_string();
                }
            }
            type_name(reference.elem.as_ref())
        }
        Type::Tuple(tuple) if tuple.elems.is_empty() => "void".to_string(),
        _ => "UnsupportedType".to_string(),
    }
}

pub(crate) fn type_target_emitted_name(ty: &Type) -> String {
    match ty {
        Type::Path(path) => {
            if path.path.segments.len() == 1 {
                path.path
                    .segments
                    .last()
                    .map(|segment| emitted_ident(&segment.ident.to_string()))
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                emitted_path(&path.path)
            }
        }
        Type::Reference(reference) => type_target_emitted_name(reference.elem.as_ref()),
        _ => emitted_ident(&type_name(ty)),
    }
}

fn type_path_args(args: &PathArguments) -> Vec<String> {
    match args {
        PathArguments::AngleBracketed(bracketed) => bracketed
            .args
            .iter()
            .filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(type_name(ty)),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub(crate) fn return_type(output: &ReturnType) -> String {
    match output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => type_name(ty),
    }
}

pub(crate) fn pat_name(pat: &Pat) -> Option<String> {
    match pat {
        Pat::Ident(ident) => Some(emitted_ident(&ident.ident.to_string())),
        Pat::Type(pat_type) => pat_name(pat_type.pat.as_ref()),
        _ => None,
    }
}

pub(crate) fn insert_pat_name_metadata(value: &mut Value, pat: &Pat) -> bool {
    match pat {
        Pat::Ident(ident) => {
            insert_name_metadata(value, &ident.ident.to_string());
            true
        }
        Pat::Type(pat_type) => insert_pat_name_metadata(value, pat_type.pat.as_ref()),
        _ => false,
    }
}

pub(crate) fn field_name(member: &syn::Member) -> String {
    match member {
        syn::Member::Named(ident) => emitted_ident(&ident.to_string()),
        syn::Member::Unnamed(index) => format!("_{}", index.index),
    }
}

pub(crate) fn item_kind(item: &Item) -> &'static str {
    match item {
        Item::Const(_) => "Const",
        Item::Enum(_) => "Enum",
        Item::ExternCrate(_) => "ExternCrate",
        Item::Fn(_) => "Fn",
        Item::ForeignMod(_) => "ForeignMod",
        Item::Impl(_) => "Impl",
        Item::Macro(_) => "Macro",
        Item::Mod(_) => "Mod",
        Item::Static(_) => "Static",
        Item::Struct(_) => "Struct",
        Item::Trait(_) => "Trait",
        Item::TraitAlias(_) => "TraitAlias",
        Item::Type(_) => "Type",
        Item::Union(_) => "Union",
        Item::Use(_) => "Use",
        _ => "Other",
    }
}
