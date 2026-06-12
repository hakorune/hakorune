use crate::mir::{MirFunction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GenericI64ValueClass {
    Unknown,
    I64,
    Bool,
    String,
    StringOrVoid,
    Object,
    VoidSentinel,
}

pub(super) fn seed_generic_i64_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
) -> bool {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_i64_value_class_from_type)
        {
            if !set_generic_i64_value_class(values, *param, class, &mut changed) {
                return false;
            }
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_i64_value_class_from_type(ty) {
            if !set_generic_i64_value_class(values, *value, class, &mut changed) {
                return false;
            }
        }
    }
    true
}

pub(super) fn generic_i64_value_class_from_type(ty: &MirType) -> Option<GenericI64ValueClass> {
    match ty {
        MirType::Integer => Some(GenericI64ValueClass::I64),
        MirType::Bool => Some(GenericI64ValueClass::Bool),
        MirType::String => Some(GenericI64ValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericI64ValueClass::I64),
            "BoolBox" => Some(GenericI64ValueClass::Bool),
            "StringBox" => Some(GenericI64ValueClass::String),
            _ => Some(GenericI64ValueClass::Object),
        },
        MirType::Array(_) | MirType::WeakRef => Some(GenericI64ValueClass::Object),
        MirType::Void => Some(GenericI64ValueClass::VoidSentinel),
        _ => None,
    }
}

pub(super) fn generic_i64_abi_type_is_i64_word_compatible(ty: &MirType) -> bool {
    matches!(
        ty,
        MirType::Integer
            | MirType::Bool
            | MirType::String
            | MirType::Unknown
            | MirType::Void
            | MirType::Array(_)
            | MirType::WeakRef
    ) || matches!(ty, MirType::Box(_))
}

pub(super) fn generic_i64_return_type_is_scalar(ty: &MirType) -> bool {
    matches!(
        ty,
        MirType::Integer | MirType::Bool | MirType::Unknown | MirType::Void
    )
}

pub(super) fn generic_i64_select_value_class(
    then_class: GenericI64ValueClass,
    else_class: GenericI64ValueClass,
) -> Option<GenericI64ValueClass> {
    if then_class == else_class {
        Some(then_class)
    } else {
        None
    }
}

pub(super) fn generic_i64_value_class(
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
) -> GenericI64ValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericI64ValueClass::Unknown)
}

pub(super) fn set_generic_i64_value_class(
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
    class: GenericI64ValueClass,
    changed: &mut bool,
) -> bool {
    if class == GenericI64ValueClass::Unknown {
        return true;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => true,
        Some(GenericI64ValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::VoidSentinel)
            if matches!(
                class,
                GenericI64ValueClass::String | GenericI64ValueClass::StringOrVoid
            ) =>
        {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::String) if class == GenericI64ValueClass::StringOrVoid => {
            values.insert(value, GenericI64ValueClass::StringOrVoid);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::StringOrVoid)
            if matches!(
                class,
                GenericI64ValueClass::String | GenericI64ValueClass::VoidSentinel
            ) =>
        {
            true
        }
        Some(GenericI64ValueClass::I64)
            if matches!(
                class,
                GenericI64ValueClass::String
                    | GenericI64ValueClass::StringOrVoid
                    | GenericI64ValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(_) => false,
    }
}

pub(super) fn set_generic_i64_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
    changed: &mut bool,
) -> bool {
    match values.get(&value).copied() {
        Some(GenericI64ValueClass::String) => true,
        Some(GenericI64ValueClass::StringOrVoid) => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::Unknown) | None => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        // String handles are raw i64 at the ABI layer. For `String + ...`, the
        // operation itself is the semantic proof that this value is a string.
        Some(GenericI64ValueClass::I64) => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        Some(_) => false,
    }
}
