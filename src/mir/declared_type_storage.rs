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

pub(crate) fn exact_numeric_storage_for_declared_type(
    type_name: &str,
) -> Option<TypedObjectFieldStorage> {
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
        "StringBox" | "String" | "str" | "ArrayBox" | "DirectArrayI64" | "MapBox" => {
            Some(TypedObjectFieldStorage::Handle)
        }
        name if declared_object_field_type_v1(name).is_some_and(|ty|
            metadata.user_box_decls.contains_key(ty.class_name())
                || metadata.user_box_field_decls.contains_key(ty.class_name())) =>
            Some(TypedObjectFieldStorage::Handle),
        _ => None,
    }
}

/// Nominal object class and nullable syntax are distinct from original spelling.
/// No scalar, builtin or applied type can masquerade as an owned user object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclaredObjectFieldTypeV1 {
    class: String,
    nullable: bool,
}
impl DeclaredObjectFieldTypeV1 {
    pub(crate) fn class_name(&self) -> &str { &self.class }
    pub(crate) fn explicitly_nullable(&self) -> bool { self.nullable }
}
pub(crate) fn declared_object_field_type_v1(text: &str) -> Option<DeclaredObjectFieldTypeV1> {
    use crate::ast::TypeReferenceV1 as Type;
    let (class, nullable) = match Type::parse(text).ok()? {
        Type::Name(class) => (class, false),
        Type::Nullable(inner) => match *inner { Type::Name(class) => (class, true), _ => return None },
        _ => return None,
    };
    if classify_numeric_type_name(&class).is_some() || crate::box_trait::is_builtin_box(&class) {
        return None;
    }
    Some(DeclaredObjectFieldTypeV1 { class, nullable })
}
pub(crate) fn declared_object_field_matches_v1(declared: Option<&str>, class: Option<&str>) -> bool {
    match (declared, class) {
        (None, None) => true,
        (Some(declared), Some(class)) => declared == class || declared_object_field_type_v1(declared)
            .is_some_and(|ty| ty.class_name() == class),
        _ => false,
    }
}
