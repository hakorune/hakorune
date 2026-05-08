use crate::mir::value_origin::build_value_def_map;
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};

use super::{
    field_box_origin, param_box_origin, sorted_block_ids, user_box_value_box_name,
    FieldBoxOriginMap, ParamBoxOriginMap,
};

pub(super) fn publish_user_box_param_origin_value_types(
    module: &mut MirModule,
    param_box_origins: &ParamBoxOriginMap,
) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        for index in 0..function.params.len() {
            let Some(box_name) =
                param_box_origin(param_box_origins, &function.signature.name, index)
            else {
                continue;
            };
            changed |= publish_function_param_box_type(function, index, &box_name);
        }
    }
    changed
}

pub(super) fn publish_user_box_route_param_value_types(
    module: &mut MirModule,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> bool {
    let mut facts = Vec::<(String, usize, String)>::new();
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for route in function
            .metadata
            .user_box_method_routes
            .iter()
            .filter(|route| route.reason().is_none())
        {
            facts.push((
                route.target_symbol().to_string(),
                0,
                route.box_name().to_string(),
            ));
            let Some(block) = function.blocks.get(&route.block()) else {
                continue;
            };
            let Some(MirInstruction::Call { args, .. }) =
                block.instructions.get(route.instruction_index())
            else {
                continue;
            };
            for (arg_index, arg) in args.iter().enumerate() {
                let Some(box_name) = user_box_value_box_name(
                    function,
                    &def_map,
                    *arg,
                    param_box_origins,
                    field_box_origins,
                ) else {
                    continue;
                };
                facts.push((route.target_symbol().to_string(), arg_index + 1, box_name));
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

pub(super) fn publish_user_box_route_result_value_types(module: &mut MirModule) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        let facts = function
            .metadata
            .user_box_method_routes
            .iter()
            .filter(|route| route.reason().is_none())
            .filter_map(|route| {
                Some((
                    route.result_value()?,
                    route.target_result_box_name()?.to_string(),
                ))
            })
            .collect::<Vec<_>>();
        for (value, box_name) in facts {
            changed |= publish_value_box_type(function, value, &box_name);
        }
    }
    changed
}

pub(super) fn publish_generic_route_result_value_types(module: &mut MirModule) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        let facts = function
            .metadata
            .generic_method_routes
            .iter()
            .filter_map(|route| {
                Some((
                    route.result_value()?,
                    super::generic_method_route_result_box_name(route)?.to_string(),
                ))
            })
            .collect::<Vec<_>>();
        for (value, box_name) in facts {
            changed |= publish_value_box_type(function, value, &box_name);
        }
    }
    changed
}

pub(super) fn propagate_user_box_box_value_types(module: &mut MirModule) -> bool {
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
                            let mut inferred = None::<MirType>;
                            let mut complete = true;
                            for (_block, value) in inputs {
                                let Some(ty) = concrete_box_value_type(function, *value) else {
                                    complete = false;
                                    break;
                                };
                                inferred = match inferred {
                                    None => Some(ty),
                                    Some(existing) if existing == ty => Some(existing),
                                    Some(_) => {
                                        complete = false;
                                        break;
                                    }
                                };
                            }
                            if complete {
                                if let Some(ty) = inferred {
                                    pending.push((*dst, ty));
                                }
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

pub(super) fn publish_user_box_field_get_value_types(
    module: &mut MirModule,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        let def_map = build_value_def_map(function);
        for block_id in sorted_block_ids(function) {
            let instructions = function
                .blocks
                .get(&block_id)
                .map(|block| block.instructions.clone())
                .unwrap_or_default();
            for instruction in instructions {
                let MirInstruction::FieldGet {
                    dst, base, field, ..
                } = instruction
                else {
                    continue;
                };
                let Some(base_box) = user_box_value_box_name(
                    function,
                    &def_map,
                    base,
                    param_box_origins,
                    field_box_origins,
                ) else {
                    continue;
                };
                let Some(field_box) = field_box_origin(field_box_origins, &base_box, &field) else {
                    continue;
                };
                changed |= publish_value_box_type(function, dst, &field_box);
            }
        }
    }
    changed
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
        Some(existing) if can_refine_placeholder_to_box_type(existing, box_name) => {
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

fn value_has_concrete_box_type(function: &MirFunction, value: ValueId) -> bool {
    concrete_box_value_type(function, value).is_some()
}

fn concrete_box_value_type(function: &MirFunction, value: ValueId) -> Option<MirType> {
    match function.metadata.value_types.get(&value) {
        Some(MirType::Box(name)) => Some(MirType::Box(name.clone())),
        Some(MirType::String) => Some(MirType::Box("StringBox".to_string())),
        _ => None,
    }
}

fn can_refine_placeholder_to_box_type(existing: &MirType, box_name: &str) -> bool {
    match existing {
        // Some front paths seed unannotated handle-carrier params/results as
        // scalar placeholders. User-box/generic route facts are the MIR owner
        // for the public ABI shape and may refine those placeholders.
        MirType::Integer | MirType::Bool => true,
        MirType::String => box_name == "StringBox",
        _ => false,
    }
}
