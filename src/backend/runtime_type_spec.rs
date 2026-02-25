//! Runtime type specification (semantic SSOT for type matching)
//!
//! This is THE semantic SSOT. MirType is NOT used here.
//! SSOT: docs/reference/language/types.md (Decision: provisional)
//!
//! Note: MirType → RuntimeTypeSpec 変換は呼び出し側（type_ops.rs）で行う。
//! このファイルは VMValue と RuntimeTypeSpec のみを扱い、MirType に依存しない。

use crate::backend::vm_types::VMValue;

/// Runtime type specification (MirType に依存しない意味論 SSOT)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeTypeSpec {
    Unknown,
    Void,
    Bool,
    Integer,
    Float,
    String,
    WeakRef,
    Future,
    Array,
    Box(String), // Box名
}

/// VMValue が RuntimeTypeSpec にマッチするか判定（意味論 SSOT）
pub fn matches_spec(value: &VMValue, spec: &RuntimeTypeSpec) -> bool {
    match spec {
        RuntimeTypeSpec::Unknown => true,
        RuntimeTypeSpec::Void => match value {
            VMValue::Void => true,
            VMValue::BoxRef(bx) => {
                bx.as_any()
                    .downcast_ref::<crate::box_trait::VoidBox>()
                    .is_some()
                    || bx
                        .as_any()
                        .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                        .is_some()
            }
            _ => false,
        },
        RuntimeTypeSpec::Bool => match value {
            VMValue::Bool(_) => true,
            VMValue::BoxRef(bx) => bx
                .as_any()
                .downcast_ref::<crate::box_trait::BoolBox>()
                .is_some(),
            _ => false,
        },
        RuntimeTypeSpec::Integer => match value {
            VMValue::Integer(_) => true,
            VMValue::BoxRef(bx) => bx
                .as_any()
                .downcast_ref::<crate::box_trait::IntegerBox>()
                .is_some(),
            _ => false,
        },
        RuntimeTypeSpec::Float => match value {
            VMValue::Float(_) => true,
            VMValue::BoxRef(bx) => bx
                .as_any()
                .downcast_ref::<crate::boxes::FloatBox>()
                .is_some(),
            _ => false,
        },
        RuntimeTypeSpec::String => match value {
            VMValue::String(_) => true,
            VMValue::BoxRef(bx) => bx
                .as_any()
                .downcast_ref::<crate::box_trait::StringBox>()
                .is_some(),
            _ => false,
        },
        RuntimeTypeSpec::WeakRef => matches!(value, VMValue::WeakBox(_)), // Phase 285A1
        RuntimeTypeSpec::Future => matches!(value, VMValue::Future(_)),
        RuntimeTypeSpec::Array => {
            // Current VM representation is BoxRef(ArrayBox) (not a distinct VMValue variant).
            // Keep this as a conservative name match to avoid guessing.
            match value {
                VMValue::BoxRef(bx) => {
                    if let Some(inst) =
                        bx.as_any().downcast_ref::<crate::instance_v2::InstanceBox>()
                    {
                        inst.class_name == "ArrayBox"
                    } else {
                        bx.type_name() == "ArrayBox"
                    }
                }
                _ => false,
            }
        }
        RuntimeTypeSpec::Box(name) => match value {
            VMValue::BoxRef(bx) => {
                // User-defined boxes are represented as InstanceBox (type_name is not stable for user boxes).
                if let Some(inst) = bx.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
                    if inst.class_name == *name {
                        return true;
                    }
                    // Builtin inner box name match (best-effort).
                    if let Some(inner) = inst.inner_content.as_ref() {
                        if inner.type_name() == name {
                            return true;
                        }
                    }
                    false
                } else {
                    bx.type_name() == name
                }
            }
            // Allow primitive to satisfy core "Box types" by name when user writes "IntegerBox" etc.
            VMValue::Integer(_) => name == "IntegerBox",
            VMValue::Float(_) => name == "FloatBox",
            VMValue::Bool(_) => name == "BoolBox",
            VMValue::String(_) => name == "StringBox",
            VMValue::Void => name == "VoidBox" || name == "NullBox",
            _ => false,
        },
    }
}
