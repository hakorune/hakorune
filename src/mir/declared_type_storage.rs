//! Shared declared-type to storage-class classification.
//!
//! This helper is deliberately metadata-only. It classifies source type names
//! for layout planning without making ordinary boxes and records share an
//! identity model.

use crate::mir::function::{ModuleMetadata, TypedObjectFieldStorage};
use crate::mir::numeric_substrate::{classify_numeric_type_name, is_inline_i64_storage_type_name};

pub(crate) fn storage_for_declared_type(
    metadata: &ModuleMetadata,
    type_name: Option<&str>,
) -> Option<TypedObjectFieldStorage> {
    match type_name {
        Some(name) => exact_numeric_storage_for_declared_type(name)
            .or_else(|| legacy_storage_for_declared_type(metadata, name)),
        None => None,
    }
}

fn exact_numeric_storage_for_declared_type(type_name: &str) -> Option<TypedObjectFieldStorage> {
    classify_numeric_type_name(type_name)?;
    Some(match type_name {
        "i8" => TypedObjectFieldStorage::I8,
        "i16" => TypedObjectFieldStorage::I16,
        "i32" => TypedObjectFieldStorage::I32,
        "i64" => TypedObjectFieldStorage::I64,
        "isize" => TypedObjectFieldStorage::ISize,
        "u8" => TypedObjectFieldStorage::U8,
        "u16" => TypedObjectFieldStorage::U16,
        "u32" => TypedObjectFieldStorage::U32,
        "u64" => TypedObjectFieldStorage::U64,
        "usize" => TypedObjectFieldStorage::USize,
        _ => return None,
    })
}

fn legacy_storage_for_declared_type(
    metadata: &ModuleMetadata,
    type_name: &str,
) -> Option<TypedObjectFieldStorage> {
    match type_name {
        name if is_inline_i64_storage_type_name(name) => Some(TypedObjectFieldStorage::I64),
        "StringBox" | "String" | "str" | "ArrayBox" | "MapBox" => {
            Some(TypedObjectFieldStorage::Handle)
        }
        name if metadata.user_box_decls.contains_key(name) => Some(TypedObjectFieldStorage::Handle),
        name if metadata.user_box_field_decls.contains_key(name) => {
            Some(TypedObjectFieldStorage::Handle)
        }
        _ => None,
    }
}
