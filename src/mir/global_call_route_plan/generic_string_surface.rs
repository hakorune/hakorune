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

pub(super) fn generic_pure_string_accepts_array_push_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "ArrayBox")
        && method == "push"
        && args.len() == 1
        && receiver_class == GenericPureValueClass::Array
        && value_class(values, args[0]) == GenericPureValueClass::String
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
