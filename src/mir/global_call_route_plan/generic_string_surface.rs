use std::collections::BTreeMap;

use crate::mir::{CompareOp, ValueId};

use super::generic_string_facts::{value_class, GenericPureValueClass};

pub(super) fn generic_pure_string_accepts_length_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "length"
        && receiver_class == GenericPureValueClass::String
        && (args.is_empty()
            || (box_name == "StringBox"
                && args.len() == 1
                && value_class(values, args[0]) == GenericPureValueClass::String))
}

pub(super) fn generic_pure_string_accepts_array_len_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "ArrayBox")
        && matches!(method, "len" | "length" | "size")
        && args.is_empty()
        && receiver_class == GenericPureValueClass::Array
}

pub(super) fn generic_pure_string_accepts_collection_birth_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
) -> bool {
    args.is_empty()
        && method == "birth"
        && matches!(
            (box_name, receiver_class),
            ("ArrayBox" | "RuntimeDataBox", GenericPureValueClass::Array)
                | ("MapBox" | "RuntimeDataBox", GenericPureValueClass::Map)
        )
}

pub(super) fn generic_pure_string_accepts_array_push_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    if !matches!(box_name, "RuntimeDataBox" | "ArrayBox")
        || method != "push"
        || receiver_class != GenericPureValueClass::Array
    {
        return false;
    }
    let payload = match args {
        [value] => *value,
        [receiver_arg, value]
            if value_class(values, *receiver_arg) == GenericPureValueClass::Array =>
        {
            *value
        }
        _ => return false,
    };
    generic_pure_string_accepts_write_any_payload(value_class(values, payload))
}

fn generic_pure_string_accepts_write_any_payload(class: GenericPureValueClass) -> bool {
    matches!(
        class,
        GenericPureValueClass::Unknown
            | GenericPureValueClass::I64
            | GenericPureValueClass::Bool
            | GenericPureValueClass::ScalarOrVoid
            | GenericPureValueClass::String
            | GenericPureValueClass::Array
            | GenericPureValueClass::Map
            | GenericPureValueClass::StringOrVoid
            | GenericPureValueClass::VoidSentinel
    )
}

pub(super) fn generic_pure_string_accepts_map_set_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    if !matches!(box_name, "RuntimeDataBox" | "MapBox")
        || method != "set"
        || receiver_class != GenericPureValueClass::Map
    {
        return false;
    }
    let (key, value) = match args {
        [key, value] => (*key, *value),
        [receiver_arg, key, value]
            if value_class(values, *receiver_arg) == GenericPureValueClass::Map =>
        {
            (*key, *value)
        }
        _ => return false,
    };
    value_class(values, key) == GenericPureValueClass::String
        && generic_pure_string_accepts_write_any_payload(value_class(values, value))
}

pub(super) fn generic_pure_string_accepts_indexof_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "indexOf"
        && matches!(args.len(), 1 | 2)
        && receiver_class == GenericPureValueClass::String
        && value_class(values, args[0]) == GenericPureValueClass::String
        && (args.len() == 1 || value_class(values, args[1]) == GenericPureValueClass::I64)
}

pub(super) fn generic_pure_string_accepts_lastindexof_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "lastIndexOf"
        && args.len() == 1
        && receiver_class == GenericPureValueClass::String
        && value_class(values, args[0]) == GenericPureValueClass::String
}

pub(super) fn generic_pure_string_accepts_contains_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "contains"
        && args.len() == 1
        && receiver_class == GenericPureValueClass::String
        && value_class(values, args[0]) == GenericPureValueClass::String
}

pub(super) fn generic_pure_string_accepts_env_set(
    name: &str,
    args: &[ValueId],
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(
        name,
        "env.set/2" | "env.set" | "nyash.env.set/2" | "nyash.env.set"
    ) && args.len() == 2
        && args
            .iter()
            .all(|arg| value_class(values, *arg) == GenericPureValueClass::String)
}

pub(super) fn generic_pure_compare_proves_i64(op: CompareOp) -> bool {
    !matches!(op, CompareOp::Eq | CompareOp::Ne)
}

pub(super) fn generic_pure_string_accepts_string_compare(
    op: CompareOp,
    lhs_class: GenericPureValueClass,
    rhs_class: GenericPureValueClass,
) -> bool {
    match op {
        CompareOp::Eq | CompareOp::Ne => true,
        CompareOp::Lt | CompareOp::Le | CompareOp::Gt | CompareOp::Ge => {
            lhs_class == GenericPureValueClass::String && rhs_class == GenericPureValueClass::String
        }
    }
}

pub(super) fn generic_pure_string_compare_can_infer_string(op: CompareOp) -> bool {
    matches!(
        op,
        CompareOp::Eq
            | CompareOp::Ne
            | CompareOp::Lt
            | CompareOp::Le
            | CompareOp::Gt
            | CompareOp::Ge
    )
}

pub(super) fn generic_pure_string_accepts_substring_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "substring"
        && matches!(args.len(), 1 | 2)
        && receiver_class == GenericPureValueClass::String
        && args
            .iter()
            .all(|arg| value_class(values, *arg) == GenericPureValueClass::I64)
}

pub(super) fn generic_pure_string_global_name_is_self(
    name: &str,
    current_function_name: &str,
) -> bool {
    name == current_function_name
        || crate::mir::naming::normalize_static_global_name(name) == current_function_name
}
