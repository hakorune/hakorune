use crate::mir::ValueId;
use std::collections::BTreeMap;

use super::{generic_i64_value_class, set_generic_i64_value_class, GenericI64ValueClass};

pub(super) fn generic_i64_accepts_length_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "length"
        && args.is_empty()
        && receiver_class == GenericI64ValueClass::String
}

pub(super) fn generic_i64_substring_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
) -> Option<bool> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || method != "substring"
        || args.len() != 2
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    let mut ready = true;
    for arg in args {
        match generic_i64_value_class(values, *arg) {
            GenericI64ValueClass::I64 => {}
            GenericI64ValueClass::Unknown => ready = false,
            _ => return None,
        }
    }
    Some(ready)
}

pub(super) fn generic_i64_indexof_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> Option<bool> {
    let supports_args = match method {
        "indexOf" => matches!(args.len(), 1 | 2),
        "lastIndexOf" => args.len() == 1,
        _ => false,
    };
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || !supports_args
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    let mut ready = true;
    match generic_i64_value_class(values, args[0]) {
        GenericI64ValueClass::String => {}
        GenericI64ValueClass::Unknown => {
            if !set_generic_i64_value_class(values, args[0], GenericI64ValueClass::String, changed)
            {
                return None;
            }
            ready = false;
        }
        _ => return None,
    }
    if args.len() == 2 {
        match generic_i64_value_class(values, args[1]) {
            GenericI64ValueClass::I64 => {}
            GenericI64ValueClass::Unknown => {
                if !set_generic_i64_value_class(values, args[1], GenericI64ValueClass::I64, changed)
                {
                    return None;
                }
                ready = false;
            }
            _ => return None,
        }
    }
    Some(ready)
}

pub(super) fn generic_i64_contains_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> Option<bool> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || method != "contains"
        || args.len() != 1
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    match generic_i64_value_class(values, args[0]) {
        GenericI64ValueClass::String => Some(true),
        GenericI64ValueClass::Unknown => {
            if !set_generic_i64_value_class(values, args[0], GenericI64ValueClass::String, changed)
            {
                return None;
            }
            Some(false)
        }
        _ => None,
    }
}
