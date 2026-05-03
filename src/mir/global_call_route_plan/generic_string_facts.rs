use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GenericPureValueClass {
    Unknown,
    I64,
    Bool,
    ScalarOrVoid,
    String,
    Array,
    Map,
    StringOrVoid,
    VoidSentinel,
}

pub(super) fn seed_generic_pure_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
) {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_pure_value_class_from_type)
        {
            set_value_class(values, *param, class, &mut changed);
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_pure_metadata_value_class_from_type(ty) {
            set_value_class(values, *value, class, &mut changed);
        }
    }
}

pub(super) fn seed_generic_pure_string_return_param_values(
    function: &MirFunction,
    values: &mut BTreeSet<ValueId>,
) {
    for (index, param) in function.params.iter().enumerate() {
        let Some(ty) = function.signature.params.get(index) else {
            continue;
        };
        if generic_pure_string_return_param_passthrough_type_is_candidate(ty) {
            values.insert(*param);
        }
    }
}

pub(super) fn generic_pure_value_class_from_type(ty: &MirType) -> Option<GenericPureValueClass> {
    match ty {
        MirType::Integer => Some(GenericPureValueClass::I64),
        MirType::Bool => Some(GenericPureValueClass::Bool),
        MirType::String => Some(GenericPureValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericPureValueClass::I64),
            "BoolBox" => Some(GenericPureValueClass::Bool),
            "StringBox" => Some(GenericPureValueClass::String),
            _ => None,
        },
        MirType::Void => Some(GenericPureValueClass::VoidSentinel),
        _ => None,
    }
}

fn generic_pure_metadata_value_class_from_type(ty: &MirType) -> Option<GenericPureValueClass> {
    match ty {
        MirType::String => Some(GenericPureValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericPureValueClass::I64),
            "BoolBox" => Some(GenericPureValueClass::Bool),
            "StringBox" => Some(GenericPureValueClass::String),
            _ => None,
        },
        MirType::Void => Some(GenericPureValueClass::VoidSentinel),
        _ => None,
    }
}

fn generic_pure_string_return_param_passthrough_type_is_candidate(ty: &MirType) -> bool {
    matches!(ty, MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

pub(super) fn generic_pure_string_iteration_limit(function: &MirFunction) -> usize {
    function
        .blocks
        .values()
        .map(|block| block.instructions.len() + usize::from(block.terminator.is_some()))
        .sum::<usize>()
        .saturating_add(1)
}

pub(super) fn update_generic_pure_string_return_param_values(
    instruction: &MirInstruction,
    values: &mut BTreeSet<ValueId>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Copy { dst, src } if values.contains(src) => {
            if values.insert(*dst) {
                *changed = true;
            }
        }
        MirInstruction::Phi { dst, inputs, .. }
            if !inputs.is_empty() && inputs.iter().all(|(_, value)| values.contains(value)) =>
        {
            if values.insert(*dst) {
                *changed = true;
            }
        }
        _ => {}
    }
}

pub(super) fn value_class(
    values: &BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
) -> GenericPureValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericPureValueClass::Unknown)
}

pub(super) fn generic_pure_value_class_is_void_like(class: GenericPureValueClass) -> bool {
    matches!(
        class,
        GenericPureValueClass::ScalarOrVoid
            | GenericPureValueClass::StringOrVoid
            | GenericPureValueClass::VoidSentinel
    )
}

pub(super) fn generic_pure_void_sentinel_compare_is_supported(
    lhs_class: GenericPureValueClass,
    rhs_class: GenericPureValueClass,
) -> bool {
    matches!(
        (lhs_class, rhs_class),
        (
            GenericPureValueClass::ScalarOrVoid,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::ScalarOrVoid
        ) | (
            GenericPureValueClass::String,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::String
        ) | (
            GenericPureValueClass::StringOrVoid,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::StringOrVoid
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::Array,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::Array
        ) | (
            GenericPureValueClass::Map,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::Map
        )
    )
}

pub(super) fn set_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    class: GenericPureValueClass,
    changed: &mut bool,
) {
    if class == GenericPureValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(_) => {}
    }
}

pub(super) fn set_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericPureValueClass::String) => {}
        Some(GenericPureValueClass::Unknown)
        | Some(GenericPureValueClass::I64)
        | Some(GenericPureValueClass::StringOrVoid)
        | Some(GenericPureValueClass::VoidSentinel)
        | None => {
            values.insert(value, GenericPureValueClass::String);
            *changed = true;
        }
        Some(_) => {}
    }
}

pub(super) fn set_guarded_non_void_string_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericPureValueClass::String) => {}
        Some(GenericPureValueClass::StringOrVoid) | Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, GenericPureValueClass::String);
            *changed = true;
        }
        Some(_) => {}
    }
}

pub(super) fn set_guarded_non_void_scalar_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericPureValueClass::I64) => {}
        Some(GenericPureValueClass::ScalarOrVoid) | Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, GenericPureValueClass::I64);
            *changed = true;
        }
        Some(_) => {}
    }
}

pub(super) fn set_proven_flow_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    class: GenericPureValueClass,
    changed: &mut bool,
) {
    if class == GenericPureValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(GenericPureValueClass::VoidSentinel)
            if matches!(
                class,
                GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
            ) =>
        {
            values.insert(value, GenericPureValueClass::StringOrVoid);
            *changed = true;
        }
        Some(GenericPureValueClass::String)
            if matches!(
                class,
                GenericPureValueClass::StringOrVoid | GenericPureValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, GenericPureValueClass::StringOrVoid);
            *changed = true;
        }
        Some(GenericPureValueClass::StringOrVoid) if class == GenericPureValueClass::String => {}
        Some(GenericPureValueClass::StringOrVoid)
            if class == GenericPureValueClass::VoidSentinel => {}
        Some(GenericPureValueClass::I64)
            if matches!(
                class,
                GenericPureValueClass::String
                    | GenericPureValueClass::StringOrVoid
                    | GenericPureValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, class);
            *changed = true;
        }
        Some(_) => {}
    }
}
