//! Runtime type tag (entry classification for runtime type identity)
//!
//! This is NOT the semantic SSOT. Semantic logic lives in runtime_type_spec.rs.
//! SSOT: docs/reference/language/types.md (Decision: provisional)

use crate::backend::vm_types::VMValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTypeTag {
    Integer,
    Float,
    Bool,
    String,
    Void,
    BoxRef,
    WeakRef,
    Future,
}

pub fn tag_from_vmvalue(v: &VMValue) -> RuntimeTypeTag {
    match v {
        VMValue::Integer(_) => RuntimeTypeTag::Integer,
        VMValue::Float(_) => RuntimeTypeTag::Float,
        VMValue::Bool(_) => RuntimeTypeTag::Bool,
        VMValue::String(_) => RuntimeTypeTag::String,
        VMValue::Void => RuntimeTypeTag::Void,
        VMValue::BoxRef(_) => RuntimeTypeTag::BoxRef,
        VMValue::WeakBox(_) => RuntimeTypeTag::WeakRef,
        VMValue::Future(_) => RuntimeTypeTag::Future,
    }
}

/// Convert RuntimeTypeTag to human-readable string (SSOT for tag names)
pub fn tag_to_str(tag: RuntimeTypeTag) -> &'static str {
    match tag {
        RuntimeTypeTag::Integer => "Integer",
        RuntimeTypeTag::Float => "Float",
        RuntimeTypeTag::Bool => "Bool",
        RuntimeTypeTag::String => "String",
        RuntimeTypeTag::Void => "Void",
        RuntimeTypeTag::BoxRef => "BoxRef",
        RuntimeTypeTag::WeakRef => "WeakRef",
        RuntimeTypeTag::Future => "Future",
    }
}
