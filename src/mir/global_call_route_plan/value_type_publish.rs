//! MIR-owned value metadata publication for same-module global calls.
//!
//! This module only moves already-proven handle facts across the public global
//! call seam. It does not infer app-specific `JsonLine` / `StringHelpers`
//! semantics or teach the backend new source-level rules.

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId};

pub(super) fn publish_global_call_route_param_value_types(module: &mut MirModule) -> bool {
    let mut facts = Vec::<(String, usize, String)>::new();
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
            let Some(MirInstruction::Call { args, .. }) =
                route_instruction(function, route.block(), route.instruction_index())
            else {
                continue;
            };
            for (arg_index, arg) in args.iter().enumerate() {
                let Some(box_name) = value_box_name(function, &def_map, *arg) else {
                    continue;
                };
                facts.push((target_symbol.to_string(), arg_index, box_name));
            }
        }
    }

    let mut changed = false;
    for (function_name, index, box_name) in facts {
        let Some(function) = module.functions.get_mut(&function_name) else {
            continue;
        };
        changed |= publish_function_param_box_type(function, index, &box_name);
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
    let Some(param) = function.params.get(index).copied() else {
        return false;
    };
    publish_value_box_type(function, param, box_name)
}

fn publish_value_box_type(function: &mut MirFunction, value: ValueId, box_name: &str) -> bool {
    let ty = MirType::Box(box_name.to_string());
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
