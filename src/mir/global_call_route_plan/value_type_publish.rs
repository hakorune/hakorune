//! MIR-owned value metadata publication for same-module global calls.
//!
//! This module only moves already-proven handle facts across the public global
//! call seam. It does not infer app-specific `JsonLine` / `StringHelpers`
//! semantics or teach the backend new source-level rules.

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Default)]
struct ParamObservation {
    box_name: Option<String>,
    scalar_type: Option<MirType>,
    has_scalar_or_mixed: bool,
}

pub(super) fn publish_global_call_route_param_value_types(module: &mut MirModule) -> bool {
    let mut observations = BTreeMap::<(String, usize), ParamObservation>::new();
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for route in function
            .metadata
            .global_call_routes
            .iter()
            .filter(|route| route.target_exists() && route.arity_matches() == Some(true))
        {
            let Some(target_symbol) = route.target_symbol() else {
                continue;
            };
            // `to_i64` is intentionally polymorphic: callers pass both i64
            // values and numeric strings. Do not freeze its param as StringBox
            // from scanner-only observations.
            if target_symbol == "StringHelpers.to_i64/1" {
                continue;
            }
            let Some(MirInstruction::Call { args, .. }) =
                route_instruction(function, route.block(), route.instruction_index())
            else {
                continue;
            };
            for (arg_index, arg) in args.iter().enumerate() {
                let observation = observations
                    .entry((target_symbol.to_string(), arg_index))
                    .or_default();
                if let Some(box_name) = value_box_name(function, &def_map, *arg) {
                    if observation.scalar_type.is_some() {
                        observation.has_scalar_or_mixed = true;
                    }
                    match &observation.box_name {
                        None => observation.box_name = Some(box_name),
                        Some(existing) if existing == &box_name => {}
                        Some(_) => observation.has_scalar_or_mixed = true,
                    }
                } else if let Some(scalar_type) =
                    value_scalar_or_mixed_type_for_value(function, &def_map, *arg)
                {
                    if observation.box_name.is_some() {
                        observation.has_scalar_or_mixed = true;
                    }
                    match &observation.scalar_type {
                        None => observation.scalar_type = Some(scalar_type),
                        Some(existing) if existing == &scalar_type => {}
                        Some(_) => observation.has_scalar_or_mixed = true,
                    }
                }
            }
        }
    }

    let mut changed = false;
    for ((function_name, index), observation) in observations {
        if observation.has_scalar_or_mixed {
            continue;
        }
        let Some(function) = module.functions.get_mut(&function_name) else {
            continue;
        };
        if let Some(box_name) = observation.box_name {
            changed |= publish_function_param_box_type(function, index, &box_name);
        } else if let Some(scalar_type) = observation.scalar_type {
            changed |= publish_function_param_type(function, index, scalar_type);
        }
    }
    changed
}

pub(super) fn publish_global_call_route_result_value_types(module: &mut MirModule) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        let mut facts = Vec::<(ValueId, MirType)>::new();
        for route in function
            .metadata
            .global_call_routes
            .iter()
            .chain(function.metadata.builtin_global_call_routes.iter())
        {
            let Some(value) = route.result_value() else {
                continue;
            };
            let Some(ty) = value_type_from_return_shape(route.return_shape()) else {
                continue;
            };
            facts.push((value, ty));
        }
        for (value, ty) in facts {
            changed |= publish_value_type(function, value, ty);
        }
    }
    changed
}

pub(super) fn propagate_global_call_box_value_types(module: &mut MirModule) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        for _ in 0..function.blocks.len().saturating_mul(4).max(4) {
            let mut pending = Vec::<(ValueId, MirType)>::new();
            for block in function.blocks.values() {
                for instruction in &block.instructions {
                    match instruction {
                        MirInstruction::Copy { dst, src } => {
                            if value_has_concrete_box_type(function, *dst) {
                                continue;
                            }
                            if let Some(ty) = concrete_box_value_type(function, *src) {
                                pending.push((*dst, ty));
                            }
                        }
                        MirInstruction::Phi { dst, inputs, .. } => {
                            if value_has_concrete_box_type(function, *dst) {
                                continue;
                            }
                            if let Some(ty) = concrete_phi_box_value_type(function, inputs) {
                                pending.push((*dst, ty));
                            }
                        }
                        _ => {}
                    }
                }
            }
            if pending.is_empty() {
                break;
            }

            let mut pass_changed = false;
            for (value, ty) in pending {
                match function.metadata.value_types.get(&value) {
                    Some(existing) if existing == &ty => {}
                    Some(existing) if can_refine_placeholder_to_box_type(existing, &ty) => {
                        function.metadata.value_types.insert(value, ty);
                        pass_changed = true;
                    }
                    Some(MirType::Unknown) | None => {
                        function.metadata.value_types.insert(value, ty);
                        pass_changed = true;
                    }
                    Some(_) => {}
                }
            }
            if !pass_changed {
                break;
            }
            changed = true;
        }
    }
    changed
}

fn value_type_from_return_shape(return_shape: Option<&str>) -> Option<MirType> {
    match return_shape {
        Some("ScalarI64") | Some("void_sentinel_i64_zero") => Some(MirType::Integer),
        Some("string_handle") | Some("string_handle_or_null") => {
            Some(MirType::Box("StringBox".to_string()))
        }
        Some("array_handle") => Some(MirType::Box("ArrayBox".to_string())),
        Some("map_handle") => Some(MirType::Box("MapBox".to_string())),
        Some("object_handle") | Some("mixed_runtime_i64_or_handle") => None,
        _ => None,
    }
}

fn value_scalar_or_mixed_type_for_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<MirType> {
    value_scalar_or_mixed_type(function.metadata.value_types.get(&value)).or_else(|| {
        let origin = resolve_value_origin(function, def_map, value);
        value_scalar_or_mixed_type(function.metadata.value_types.get(&origin))
    })
}

fn route_instruction(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
) -> Option<&MirInstruction> {
    function
        .blocks
        .get(&block)?
        .instructions
        .get(instruction_index)
}

fn value_box_name(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    concrete_box_name_from_value_type(function.metadata.value_types.get(&value))
        .or_else(|| {
            let origin = resolve_value_origin(function, def_map, value);
            concrete_box_name_from_value_type(function.metadata.value_types.get(&origin))
                .or_else(|| param_box_name(function, origin))
                .or_else(|| box_name_from_origin_instruction(function, def_map, origin))
        })
        .or_else(|| param_box_name(function, value))
}

fn box_name_from_origin_instruction(
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
) -> Option<String> {
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let instruction = function
        .blocks
        .get(&block_id)?
        .instructions
        .get(instruction_index)?;
    match instruction {
        MirInstruction::Const {
            value: ConstValue::String(_),
            ..
        } => Some("StringBox".to_string()),
        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
        MirInstruction::Phi {
            type_hint: Some(ty),
            ..
        } => concrete_box_name_from_value_type(Some(ty)),
        _ => None,
    }
}

fn param_box_name(function: &MirFunction, value: ValueId) -> Option<String> {
    function
        .params
        .iter()
        .position(|param| *param == value)
        .and_then(|index| function.signature.params.get(index))
        .and_then(|ty| concrete_box_name_from_value_type(Some(ty)))
}

fn publish_function_param_box_type(
    function: &mut MirFunction,
    index: usize,
    box_name: &str,
) -> bool {
    publish_function_param_type(function, index, MirType::Box(box_name.to_string()))
}

fn publish_function_param_type(function: &mut MirFunction, index: usize, ty: MirType) -> bool {
    let Some(param) = function.params.get(index).copied() else {
        return false;
    };
    publish_value_type(function, param, ty)
}

fn publish_value_type(function: &mut MirFunction, value: ValueId, ty: MirType) -> bool {
    match function.metadata.value_types.get(&value) {
        Some(existing) if existing == &ty => false,
        Some(existing) if can_refine_placeholder_to_box_type(existing, &ty) => {
            function.metadata.value_types.insert(value, ty);
            true
        }
        Some(MirType::Unknown) | None => {
            function.metadata.value_types.insert(value, ty);
            true
        }
        Some(_) => false,
    }
}

fn value_scalar_or_mixed_type(ty: Option<&MirType>) -> Option<MirType> {
    match ty {
        Some(MirType::Integer) => Some(MirType::Integer),
        Some(MirType::Float) => Some(MirType::Float),
        Some(MirType::Bool) => Some(MirType::Bool),
        Some(MirType::Void) => Some(MirType::Void),
        Some(MirType::WeakRef) | Some(MirType::Array(_)) | Some(MirType::Future(_)) => {
            Some(MirType::Unknown)
        }
        Some(MirType::String) | Some(MirType::Box(_)) | Some(MirType::Unknown) | None => None,
    }
}

fn concrete_phi_box_value_type(
    function: &MirFunction,
    inputs: &[(crate::mir::BasicBlockId, ValueId)],
) -> Option<MirType> {
    let mut inferred = None::<MirType>;
    for (_block, value) in inputs {
        let ty = concrete_box_value_type(function, *value)?;
        inferred = match inferred {
            None => Some(ty),
            Some(existing) if existing == ty => Some(existing),
            Some(_) => return None,
        };
    }
    inferred
}

fn value_has_concrete_box_type(function: &MirFunction, value: ValueId) -> bool {
    concrete_box_value_type(function, value).is_some()
}

fn concrete_box_value_type(function: &MirFunction, value: ValueId) -> Option<MirType> {
    concrete_box_name_from_value_type(function.metadata.value_types.get(&value)).map(MirType::Box)
}

fn concrete_box_name_from_value_type(ty: Option<&MirType>) -> Option<String> {
    match ty {
        Some(MirType::Box(name)) => Some(name.clone()),
        Some(MirType::String) => Some("StringBox".to_string()),
        _ => None,
    }
}

fn can_refine_placeholder_to_box_type(existing: &MirType, ty: &MirType) -> bool {
    let MirType::Box(box_name) = ty else {
        return false;
    };
    match existing {
        MirType::Integer | MirType::Bool => true,
        MirType::String => box_name == "StringBox",
        _ => false,
    }
}
