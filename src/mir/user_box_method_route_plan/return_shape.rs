use std::collections::BTreeMap;

use crate::mir::generic_method_route_facts::GenericMethodReturnShape;
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UserBoxMethodInferredReturn {
    ScalarI64,
    StringHandle,
    ObjectHandle,
    VoidSentinel,
}

pub(super) type UserBoxFieldReturnHints = BTreeMap<(String, String), UserBoxMethodInferredReturn>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueClass {
    ScalarI64,
    StringHandle,
    ObjectHandle,
    VoidSentinel,
}

pub(super) fn infer_user_box_method_return(
    function: &MirFunction,
    field_return_hints: &UserBoxFieldReturnHints,
) -> Option<UserBoxMethodInferredReturn> {
    let mut values = BTreeMap::<ValueId, ValueClass>::new();
    let mut fields = BTreeMap::<String, ValueClass>::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..function.blocks.len().saturating_mul(8).max(8) {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                observe_instruction(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    &mut values,
                    &mut fields,
                    field_return_hints,
                    &mut changed,
                );
            }
            if let Some(terminator) = &block.terminator {
                observe_instruction(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    &mut values,
                    &mut fields,
                    field_return_hints,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    let mut result = None;
    let mut saw_void_sentinel = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let class = values.get(value).copied()?;
            if class == ValueClass::VoidSentinel {
                saw_void_sentinel = true;
                continue;
            }
            let inferred = inferred_return_from_value_class(class);
            result = match result {
                None => Some(inferred),
                Some(existing) if existing == inferred => Some(existing),
                _ => return None,
            };
        }
    }
    result.or_else(|| saw_void_sentinel.then_some(UserBoxMethodInferredReturn::VoidSentinel))
}

fn observe_instruction(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    values: &mut BTreeMap<ValueId, ValueClass>,
    fields: &mut BTreeMap<String, ValueClass>,
    field_return_hints: &UserBoxFieldReturnHints,
    changed: &mut bool,
) {
    if let Some(dst) = instruction.dst_value() {
        if let Some(class) = value_metadata_class(function, dst) {
            set_value(values, dst, class, changed);
        }
    }

    match instruction {
        MirInstruction::Const { dst, value } => {
            if let Some(class) = const_value_class(value) {
                set_value(values, *dst, class, changed);
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(class) = values.get(src).copied() {
                set_value(values, *dst, class, changed);
            } else if let Some(class) = values.get(dst).copied() {
                set_value(values, *src, class, changed);
            }
        }
        MirInstruction::NewBox { dst, .. } => {
            set_value(values, *dst, ValueClass::ObjectHandle, changed);
        }
        MirInstruction::FieldSet { field, value, .. } => {
            if let Some(class) = values.get(value).copied() {
                if fields.get(field).copied() != Some(class) {
                    fields.insert(field.clone(), class);
                    *changed = true;
                }
            }
        }
        MirInstruction::FieldGet {
            dst,
            base,
            field,
            declared_type,
            ..
        } => {
            if let Some(class) = declared_type.as_ref().and_then(value_class_from_type) {
                set_value(values, *dst, class, changed);
            } else if let Some(class) = fields.get(field).copied() {
                set_value(values, *dst, class, changed);
            } else if let Some(class) =
                field_return_hint_class(function, *base, field, field_return_hints)
            {
                set_value(values, *dst, class, changed);
            }
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = values.get(lhs).copied();
            let rhs_class = values.get(rhs).copied();
            if *op == BinaryOp::Add
                && matches!(
                    (lhs_class, rhs_class),
                    (Some(ValueClass::StringHandle), _) | (_, Some(ValueClass::StringHandle))
                )
            {
                set_value(values, *dst, ValueClass::StringHandle, changed);
            } else if matches!(
                (lhs_class, rhs_class),
                (Some(ValueClass::ScalarI64), Some(ValueClass::ScalarI64))
            ) {
                set_value(values, *dst, ValueClass::ScalarI64, changed);
            }
        }
        MirInstruction::Compare { dst, .. } => {
            set_value(values, *dst, ValueClass::ScalarI64, changed);
        }
        MirInstruction::Select {
            dst,
            then_val,
            else_val,
            ..
        } => {
            if let (Some(then_class), Some(else_class)) =
                (values.get(then_val).copied(), values.get(else_val).copied())
            {
                if then_class == else_class {
                    set_value(values, *dst, then_class, changed);
                }
            }
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let mut input_class = None;
            for (_, value) in inputs {
                let Some(class) = values.get(value).copied() else {
                    input_class = None;
                    break;
                };
                input_class = match input_class {
                    None => Some(class),
                    Some(existing) if existing == class => Some(existing),
                    _ => None,
                };
                if input_class.is_none() {
                    break;
                }
            }
            if let Some(class) = input_class {
                set_value(values, *dst, class, changed);
            } else if let Some(class) = type_hint.as_ref().and_then(value_class_from_type) {
                set_value(values, *dst, class, changed);
            }
        }
        MirInstruction::Call { dst: Some(dst), .. } => {
            if let Some(class) = route_value_class(function, block, instruction_index) {
                set_value(values, *dst, class, changed);
            }
        }
        _ => {}
    }
}

fn value_metadata_class(function: &MirFunction, value: ValueId) -> Option<ValueClass> {
    function
        .metadata
        .value_types
        .get(&value)
        .and_then(value_class_from_type)
}

fn field_return_hint_class(
    function: &MirFunction,
    base: ValueId,
    field: &str,
    field_return_hints: &UserBoxFieldReturnHints,
) -> Option<ValueClass> {
    let box_name = value_box_name(function, base)?;
    field_return_hints
        .get(&(box_name.to_string(), field.to_string()))
        .copied()
        .and_then(value_class_from_inferred_return)
}

fn value_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .value_types
        .get(&value)
        .and_then(box_name_from_type)
        .or_else(|| {
            function
                .params
                .iter()
                .position(|param| *param == value)
                .and_then(|index| function.signature.params.get(index))
                .and_then(box_name_from_type)
        })
}

fn box_name_from_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::String => Some("StringBox"),
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

fn route_value_class(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<ValueClass> {
    if let Some(route) = function
        .metadata
        .global_call_routes
        .iter()
        .find(|route| route.block() == block && route.instruction_index() == instruction_index)
    {
        if route.reason().is_none() {
            return value_class_from_return_shape(route.return_shape()?);
        }
    }
    if let Some(route) = function
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.block() == block && route.instruction_index() == instruction_index)
    {
        if route.reason().is_none() {
            return value_class_from_return_shape(route.return_shape()?);
        }
    }
    if let Some(route) = function
        .metadata
        .generic_method_routes
        .iter()
        .find(|route| route.block() == block && route.instruction_index() == instruction_index)
    {
        if let Some(box_name) = route.result_origin_box() {
            return if box_name == "StringBox" {
                Some(ValueClass::StringHandle)
            } else {
                Some(ValueClass::ObjectHandle)
            };
        }
        return match route.return_shape() {
            Some(GenericMethodReturnShape::ScalarI64)
            | Some(GenericMethodReturnShape::ScalarI64OrMissingZero) => Some(ValueClass::ScalarI64),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle) => None,
            None => match route.route_kind_tag() {
                "string_substring" => Some(ValueClass::StringHandle),
                "string_len" | "array_slot_len" | "string_indexof" | "string_lastindexof" => {
                    Some(ValueClass::ScalarI64)
                }
                "string_contains" => Some(ValueClass::ScalarI64),
                _ => None,
            },
        };
    }
    None
}

fn set_value(
    values: &mut BTreeMap<ValueId, ValueClass>,
    value: ValueId,
    class: ValueClass,
    changed: &mut bool,
) {
    if values.get(&value).copied() == Some(class) {
        return;
    }
    values.insert(value, class);
    *changed = true;
}

fn const_value_class(value: &ConstValue) -> Option<ValueClass> {
    match value {
        ConstValue::Integer(_) | ConstValue::Bool(_) => Some(ValueClass::ScalarI64),
        ConstValue::String(_) => Some(ValueClass::StringHandle),
        ConstValue::Null | ConstValue::Void => Some(ValueClass::VoidSentinel),
        ConstValue::Float(_) => None,
    }
}

fn value_class_from_type(ty: &MirType) -> Option<ValueClass> {
    match ty {
        MirType::Integer | MirType::Bool => Some(ValueClass::ScalarI64),
        MirType::String => Some(ValueClass::StringHandle),
        MirType::Box(name) if name == "StringBox" => Some(ValueClass::StringHandle),
        MirType::Box(_) | MirType::Array(_) | MirType::WeakRef => Some(ValueClass::ObjectHandle),
        MirType::Void => Some(ValueClass::VoidSentinel),
        MirType::Float | MirType::Future(_) | MirType::Unknown => None,
    }
}

fn value_class_from_return_shape(shape: &str) -> Option<ValueClass> {
    match shape {
        "scalar_i64" | "ScalarI64" | "scalar_i64_or_missing_zero" => Some(ValueClass::ScalarI64),
        "string_handle" | "string_handle_or_null" => Some(ValueClass::StringHandle),
        "object_handle" | "array_handle" | "map_handle" => Some(ValueClass::ObjectHandle),
        "void_sentinel_i64_zero" => Some(ValueClass::VoidSentinel),
        _ => None,
    }
}

fn value_class_from_inferred_return(inferred: UserBoxMethodInferredReturn) -> Option<ValueClass> {
    match inferred {
        UserBoxMethodInferredReturn::ScalarI64 => Some(ValueClass::ScalarI64),
        UserBoxMethodInferredReturn::StringHandle => Some(ValueClass::StringHandle),
        UserBoxMethodInferredReturn::ObjectHandle => Some(ValueClass::ObjectHandle),
        UserBoxMethodInferredReturn::VoidSentinel => None,
    }
}

fn inferred_return_from_value_class(class: ValueClass) -> UserBoxMethodInferredReturn {
    match class {
        ValueClass::ScalarI64 => UserBoxMethodInferredReturn::ScalarI64,
        ValueClass::StringHandle => UserBoxMethodInferredReturn::StringHandle,
        ValueClass::ObjectHandle => UserBoxMethodInferredReturn::ObjectHandle,
        ValueClass::VoidSentinel => UserBoxMethodInferredReturn::VoidSentinel,
    }
}
