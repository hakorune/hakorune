use crate::mir::route_value_type_publication::{
    helper_param_type_publication_policy, route_return_shape_value_type,
    HelperParamTypePublicationPolicy,
};
use crate::mir::value_origin::build_value_def_map;
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};

use super::origin_inference::{
    build_route_result_box_lookup, field_box_origin, generic_method_route_result_box_name,
    param_box_origin, sorted_block_ids, user_box_value_box_name,
};
use super::{FieldBoxOriginMap, ParamBoxOriginMap};

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
        let route_result_lookup = build_route_result_box_lookup(function);
        for route in function
            .metadata
            .user_box_method_routes
            .iter()
            .filter(|route| route.reason().is_none())
        {
            let target_symbol = route.target_symbol();
            if helper_param_type_publication_policy(target_symbol, 0)
                != HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation
            {
                facts.push((target_symbol.to_string(), 0, route.box_name().to_string()));
            }
            let Some(block) = function.blocks.get(&route.block()) else {
                continue;
            };
            let Some(MirInstruction::Call { args, .. }) =
                block.instructions.get(route.instruction_index())
            else {
                continue;
            };
            for (arg_index, arg) in args.iter().enumerate() {
                let target_param_index = arg_index + 1;
                if helper_param_type_publication_policy(target_symbol, target_param_index)
                    == HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation
                {
                    continue;
                }
                let Some(box_name) = user_box_value_box_name(
                    function,
                    &def_map,
                    &route_result_lookup,
                    *arg,
                    param_box_origins,
                    field_box_origins,
                ) else {
                    continue;
                };
                facts.push((target_symbol.to_string(), target_param_index, box_name));
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
                    route_return_shape_value_type(route.return_shape())?,
                ))
            })
            .collect::<Vec<_>>();
        for (value, ty) in facts {
            changed |= publish_value_type(function, value, ty);
        }
    }
    changed
}

pub(super) fn publish_generic_route_result_value_types(module: &mut MirModule) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        let shape_facts = function
            .metadata
            .generic_method_routes
            .iter()
            .filter_map(|route| {
                Some((
                    route.result_value()?,
                    route_return_shape_value_type(Some(route.return_shape()?.as_metadata_name()))?,
                ))
            })
            .collect::<Vec<_>>();
        for (value, ty) in shape_facts {
            changed |= publish_value_type(function, value, ty);
        }

        let box_facts = function
            .metadata
            .generic_method_routes
            .iter()
            .filter_map(|route| {
                Some((
                    route.result_value()?,
                    generic_method_route_result_box_name(route)?.to_string(),
                ))
            })
            .collect::<Vec<_>>();
        for (value, box_name) in box_facts {
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
        let route_result_lookup = build_route_result_box_lookup(function);
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
                    &route_result_lookup,
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
    publish_value_type(function, value, MirType::Box(box_name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::generic_method_route_plan::test_support;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature};

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn generic_scalar_return_shapes_publish_integer_value_type() {
        let mut module = MirModule::new("generic-scalar-publication-test".to_string());
        let mut function = make_function();
        function.metadata.generic_method_routes.push(
            test_support::runtime_data_map_get_scalar_i64_same_key(0, 0, 1, 2, 3),
        );
        function.metadata.generic_method_routes.push(
            test_support::runtime_data_map_get_mixed_i64_key(0, 1, 1, 2, 4),
        );
        module.add_function(function);

        assert!(publish_generic_route_result_value_types(&mut module));
        let function = module.get_function("main").unwrap();
        assert_eq!(
            function.metadata.value_types.get(&ValueId::new(3)),
            Some(&MirType::Integer)
        );
        assert_eq!(function.metadata.value_types.get(&ValueId::new(4)), None);
    }
}

fn publish_value_type(function: &mut MirFunction, value: ValueId, ty: MirType) -> bool {
    match function.metadata.value_types.get(&value) {
        Some(existing) if existing == &ty => false,
        Some(existing) if can_refine_placeholder_to_type(existing, &ty) => {
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

fn can_refine_placeholder_to_type(existing: &MirType, ty: &MirType) -> bool {
    let MirType::Box(box_name) = ty else {
        return matches!(existing, MirType::Unknown);
    };
    match existing {
        // Some front paths seed unannotated handle-carrier params/results as
        // scalar placeholders. User-box/generic route facts are the MIR owner
        // for the public ABI shape and may refine those placeholders.
        MirType::Integer | MirType::Bool => true,
        // Some object-return methods still carry a void placeholder from an
        // unannotated source signature while their route contract already
        // owns `return_shape=object_handle` and `target_result_box_name`.
        MirType::Void => true,
        MirType::String => box_name == "StringBox",
        _ => false,
    }
}
