use crate::mir::function::TypedObjectFieldStorage;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{ConstValue, MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn value_box_origin_for(function: &MirFunction, value: ValueId) -> Option<String> {
    function
        .metadata
        .value_types
        .get(&value)
        .and_then(box_origin_from_mir_type)
        .or_else(|| {
            function
                .params
                .iter()
                .position(|param| *param == value)
                .and_then(|index| function.signature.params.get(index))
                .and_then(box_origin_from_mir_type)
                .or_else(|| method_receiver_box_from_param(function, value))
        })
}

pub(super) fn method_receiver_box_from_param(
    function: &MirFunction,
    value: ValueId,
) -> Option<String> {
    let param_index = function.params.iter().position(|param| *param == value)?;
    method_receiver_box_from_param_index(function, param_index)
}

pub(super) fn method_receiver_box_from_param_index(
    function: &MirFunction,
    param_index: usize,
) -> Option<String> {
    if param_index != 0 {
        return None;
    }
    let (box_name, method_part) = function.signature.name.split_once('.')?;
    if method_part.contains('/') {
        Some(box_name.to_string())
    } else {
        None
    }
}

pub(super) fn value_type_for(function: &MirFunction, value: ValueId) -> Option<&MirType> {
    function
        .metadata
        .value_types
        .get(&value)
        .or_else(|| function.signature.params.get(value.to_usize()))
}

pub(super) fn storage_for_const(value: &ConstValue) -> Option<TypedObjectFieldStorage> {
    match value {
        ConstValue::Integer(_) | ConstValue::Bool(_) => Some(TypedObjectFieldStorage::I64),
        ConstValue::String(_) => Some(TypedObjectFieldStorage::Handle),
        ConstValue::Null | ConstValue::Void | ConstValue::Float(_) => None,
    }
}

pub(super) fn is_null_or_void_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return value_type_for(function, origin).is_some_and(|ty| matches!(ty, MirType::Void));
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    matches!(
        block.instructions.get(instruction_index),
        Some(MirInstruction::Const {
            value: ConstValue::Null | ConstValue::Void,
            ..
        })
    )
}

pub(super) fn storage_for_mir_type(ty: &MirType) -> Option<TypedObjectFieldStorage> {
    match ty {
        MirType::Integer | MirType::Bool => Some(TypedObjectFieldStorage::I64),
        MirType::Box(name) if matches!(name.as_str(), "IntegerBox" | "BoolBox") => {
            Some(TypedObjectFieldStorage::I64)
        }
        MirType::String | MirType::Box(_) | MirType::Array(_) | MirType::Future(_) => {
            Some(TypedObjectFieldStorage::Handle)
        }
        MirType::Float | MirType::WeakRef | MirType::Void | MirType::Unknown => None,
    }
}

pub(super) fn box_origin_from_mir_type(ty: &MirType) -> Option<String> {
    match ty {
        MirType::String => Some("StringBox".to_string()),
        MirType::Box(name) => Some(name.clone()),
        MirType::Array(_) => Some("ArrayBox".to_string()),
        MirType::Integer
        | MirType::Bool
        | MirType::Float
        | MirType::Future(_)
        | MirType::WeakRef
        | MirType::Void
        | MirType::Unknown => None,
    }
}

pub(super) fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}
