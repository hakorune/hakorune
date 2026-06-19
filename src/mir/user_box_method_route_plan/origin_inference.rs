use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::return_shape::{UserBoxFieldReturnHints, UserBoxMethodInferredReturn};
use super::target_collection::{
    method_target_symbol, parse_method_symbol, UserBoxMethodTargetFacts,
};
use super::{
    BoxOriginInference, FieldBoxOriginKey, FieldBoxOriginMap, ParamBoxOriginKey, ParamBoxOriginMap,
};
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::value_origin::{
    build_value_def_map, ValueDefMap, ValueOriginQueryContext,
};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

#[path = "merge.rs"]
mod merge;
#[path = "origin_route_flow.rs"]
mod origin_route_flow;
#[path = "value_helpers.rs"]
pub(super) mod value_helpers;

use self::merge::{merge_field_box_origin, merge_param_box_origin};
use self::origin_route_flow::phi_input_box_name;
pub(crate) use self::value_helpers::{
    box_name_from_type, box_origin_known, build_route_result_box_lookup, field_box_origin,
    generic_method_route_result_box_name, method_receiver_box_name, param_box_origin,
    route_result_box_name, route_result_box_name_cached, sorted_block_ids, value_box_name,
    value_param_index,
};

pub(crate) fn infer_user_box_method_param_box_origins(
    module: &MirModule,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    field_box_origins: &FieldBoxOriginMap,
) -> ParamBoxOriginMap {
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(
        module
            .metadata
            .typed_object_plans
            .iter()
            .map(|plan| plan.box_name.clone()),
    );
    let typed_plan_fields = typed_object_plan_field_sets(module);
    let mut origins = ParamBoxOriginMap::new();
    let mut function_def_maps = BTreeMap::<String, ValueDefMap>::new();
    let mut function_block_ids = BTreeMap::<String, Vec<BasicBlockId>>::new();
    let mut function_route_result_lookups = BTreeMap::<String, HashMap<ValueId, String>>::new();
    let mut function_param_index_caches =
        BTreeMap::<String, HashMap<ValueId, Option<usize>>>::new();

    for function in module.functions.values() {
        let function_name = function.signature.name.clone();
        function_def_maps.insert(function_name.clone(), build_value_def_map(function));
        function_block_ids.insert(function_name.clone(), sorted_block_ids(function));
        function_route_result_lookups.insert(
            function_name.clone(),
            build_route_result_box_lookup(function),
        );
        function_param_index_caches.insert(function_name, HashMap::new());
    }

    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let function_name = function.signature.name.as_str();
            let Some(def_map) = function_def_maps.get(function_name) else {
                continue;
            };
            let Some(param_index_cache) = function_param_index_caches.get_mut(function_name) else {
                continue;
            };
            let Some(block_ids) = function_block_ids.get(function_name) else {
                continue;
            };
            let Some(route_result_lookup) = function_route_result_lookups.get(function_name) else {
                continue;
            };
            let mut value_origin_context = ValueOriginQueryContext::new(function, def_map);

            for (param_index, box_name) in infer_param_box_origins_from_field_uses(
                function,
                def_map,
                block_ids,
                &typed_plan_fields,
                param_index_cache,
            ) {
                if !user_box_names.contains(&box_name) {
                    continue;
                }
                changed |= merge_param_box_origin(
                    &mut origins,
                    (function.signature.name.clone(), param_index),
                    box_name,
                );
            }
            for block_id in block_ids {
                let Some(block) = function.blocks.get(block_id) else {
                    continue;
                };
                for instruction in &block.instructions {
                    let MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                certainty,
                                ..
                            }),
                        args,
                        ..
                    } = instruction
                    else {
                        continue;
                    };
                    let Some(route_box_name) = user_box_route_receiver_box_name_with_origin_context(
                        function,
                        def_map,
                        route_result_lookup,
                        &user_box_names,
                        box_name,
                        *certainty,
                        *receiver,
                        &current,
                        field_box_origins,
                        &mut value_origin_context,
                    ) else {
                        continue;
                    };
                    let target_symbol = method_target_symbol(&route_box_name, method, args.len());
                    if !targets.contains_key(&target_symbol) {
                        continue;
                    }

                    changed |= merge_param_box_origin(
                        &mut origins,
                        (target_symbol.clone(), 0),
                        route_box_name,
                    );
                    for (arg_index, arg) in args.iter().enumerate() {
                        let Some(arg_box_name) = user_box_value_box_name_with_origin_context(
                            function,
                            def_map,
                            route_result_lookup,
                            *arg,
                            &current,
                            field_box_origins,
                            &mut value_origin_context,
                        ) else {
                            continue;
                        };
                        if !user_box_names.contains(&arg_box_name) {
                            continue;
                        }
                        changed |= merge_param_box_origin(
                            &mut origins,
                            (target_symbol.clone(), arg_index + 1),
                            arg_box_name,
                        );
                    }
                    for (arg_index, arg) in args.iter().enumerate() {
                        let Some(target_param_box_name) =
                            param_box_origin(&current, &target_symbol, arg_index + 1)
                        else {
                            continue;
                        };
                        let Some(caller_param_index) =
                            value_param_index(function, def_map, *arg, param_index_cache)
                        else {
                            continue;
                        };
                        if !param_accepts_inferred_box_origin(function, caller_param_index) {
                            continue;
                        }
                        changed |= merge_param_box_origin(
                            &mut origins,
                            (function.signature.name.clone(), caller_param_index),
                            target_param_box_name,
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    origins
}

fn typed_object_plan_field_sets(module: &MirModule) -> BTreeMap<String, BTreeSet<String>> {
    module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| {
            (
                plan.box_name.clone(),
                plan.fields
                    .iter()
                    .map(|field| field.name.clone())
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect()
}

fn infer_param_box_origins_from_field_uses(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_ids: &[BasicBlockId],
    typed_plan_fields: &BTreeMap<String, BTreeSet<String>>,
    param_index_cache: &mut HashMap<ValueId, Option<usize>>,
) -> Vec<(usize, String)> {
    let mut param_fields = BTreeMap::<usize, BTreeSet<String>>::new();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(block_id) else {
            continue;
        };
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::FieldGet { base, field, .. }
                | MirInstruction::FieldSet { base, field, .. } => {
                    let Some(param_index) =
                        value_param_index(function, def_map, *base, param_index_cache)
                    else {
                        continue;
                    };
                    if !param_accepts_inferred_box_origin(function, param_index) {
                        continue;
                    }
                    param_fields
                        .entry(param_index)
                        .or_default()
                        .insert(field.clone());
                }
                _ => {}
            }
        }
    }

    param_fields
        .into_iter()
        .filter_map(|(param_index, fields)| {
            let box_name = unique_typed_object_plan_for_fields(&fields, typed_plan_fields)?;
            Some((param_index, box_name))
        })
        .collect()
}

fn unique_typed_object_plan_for_fields(
    fields: &BTreeSet<String>,
    typed_plan_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Option<String> {
    if fields.is_empty() {
        return None;
    }
    let mut candidates = typed_plan_fields
        .iter()
        .filter(|(_box_name, plan_fields)| fields.is_subset(plan_fields))
        .map(|(box_name, _plan_fields)| box_name.clone());
    let first = candidates.next()?;
    if candidates.next().is_none() {
        Some(first)
    } else {
        None
    }
}

fn param_accepts_inferred_box_origin(function: &MirFunction, param_index: usize) -> bool {
    matches!(
        function.signature.params.get(param_index),
        Some(MirType::Unknown) | None
    )
}

pub(crate) fn infer_user_box_field_box_origins(
    module: &MirModule,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    param_box_origins: &ParamBoxOriginMap,
) -> FieldBoxOriginMap {
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(
        module
            .metadata
            .typed_object_plans
            .iter()
            .map(|plan| plan.box_name.clone()),
    );
    let birth_field_params = collect_birth_field_param_bindings(module);
    let mut origins = FieldBoxOriginMap::new();
    let mut function_def_maps = BTreeMap::<String, ValueDefMap>::new();
    let mut function_block_ids = BTreeMap::<String, Vec<BasicBlockId>>::new();
    let mut function_route_result_lookups = BTreeMap::<String, HashMap<ValueId, String>>::new();
    for function in module.functions.values() {
        let function_name = function.signature.name.clone();
        function_def_maps.insert(function_name.clone(), build_value_def_map(function));
        function_block_ids.insert(function_name.clone(), sorted_block_ids(function));
        function_route_result_lookups
            .insert(function_name, build_route_result_box_lookup(function));
    }

    for _ in 0..module.functions.len().saturating_mul(2).max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let function_name = function.signature.name.as_str();
            let Some(def_map) = function_def_maps.get(function_name) else {
                continue;
            };
            let Some(block_ids) = function_block_ids.get(function_name) else {
                continue;
            };
            let Some(route_result_lookup) = function_route_result_lookups.get(function_name) else {
                continue;
            };
            let mut value_origin_context = ValueOriginQueryContext::new(function, def_map);
            for block_id in block_ids {
                let Some(block) = function.blocks.get(block_id) else {
                    continue;
                };
                for instruction in &block.instructions {
                    match instruction {
                        MirInstruction::FieldSet {
                            base, field, value, ..
                        } => {
                            let Some(base_box) = user_box_value_box_name_with_origin_context(
                                function,
                                def_map,
                                route_result_lookup,
                                *base,
                                param_box_origins,
                                &current,
                                &mut value_origin_context,
                            ) else {
                                continue;
                            };
                            let Some(value_box) = user_box_value_box_name_with_origin_context(
                                function,
                                def_map,
                                route_result_lookup,
                                *value,
                                param_box_origins,
                                &current,
                                &mut value_origin_context,
                            ) else {
                                continue;
                            };
                            if !user_box_names.contains(&base_box)
                                || !(user_box_names.contains(&value_box)
                                    || value_box == "StringBox")
                            {
                                continue;
                            }
                            changed |= merge_field_box_origin(
                                &mut origins,
                                (base_box, field.clone()),
                                value_box,
                            );
                        }
                        MirInstruction::Call {
                            callee:
                                Some(Callee::Method {
                                    box_name,
                                    method,
                                    receiver: Some(receiver),
                                    certainty,
                                    ..
                                }),
                            args,
                            ..
                        } if method == "birth" => {
                            let Some(route_box_name) = user_box_route_receiver_box_name_with_origin_context(
                                function,
                                def_map,
                                route_result_lookup,
                                &user_box_names,
                                box_name,
                                *certainty,
                                *receiver,
                                param_box_origins,
                                &current,
                                &mut value_origin_context,
                            ) else {
                                continue;
                            };
                            for ((birth_box, field), param_index) in &birth_field_params {
                                if birth_box != &route_box_name || *param_index == 0 {
                                    continue;
                                }
                                let Some(arg) = args.get(param_index - 1) else {
                                    continue;
                                };
                                let Some(value_box) = user_box_value_box_name_with_origin_context(
                                    function,
                                    def_map,
                                    route_result_lookup,
                                    *arg,
                                    param_box_origins,
                                    &current,
                                    &mut value_origin_context,
                                ) else {
                                    continue;
                                };
                                if !(user_box_names.contains(&value_box)
                                    || value_box == "StringBox")
                                {
                                    continue;
                                }
                                changed |= merge_field_box_origin(
                                    &mut origins,
                                    (route_box_name.clone(), field.clone()),
                                    value_box,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    origins
}

fn collect_birth_field_param_bindings(module: &MirModule) -> BTreeMap<(String, String), usize> {
    let mut bindings = BTreeMap::new();
    for (name, function) in &module.functions {
        let Some((box_name, method, _arity)) = parse_method_symbol(name) else {
            continue;
        };
        if method != "birth" {
            continue;
        }
        let def_map = build_value_def_map(function);
        let mut value_origin_context = ValueOriginQueryContext::new(function, &def_map);
        let receiver = function.params.first().copied();
        for block_id in sorted_block_ids(function) {
            let Some(block) = function.blocks.get(&block_id) else {
                continue;
            };
            for instruction in &block.instructions {
                let MirInstruction::FieldSet {
                    base, field, value, ..
                } = instruction
                else {
                    continue;
                };
                if Some(value_origin_context.origin(*base)) != receiver {
                    continue;
                }
                let value_origin = value_origin_context.origin(*value);
                let Some(param_index) = function
                    .params
                    .iter()
                    .position(|param| param == value)
                    .or_else(|| {
                        function
                            .params
                            .iter()
                            .position(|param| *param == value_origin)
                    })
                else {
                    continue;
                };
                bindings.insert((box_name.to_string(), field.clone()), param_index);
            }
        }
    }
    bindings
}

pub(crate) fn build_user_box_field_return_hints(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
) -> UserBoxFieldReturnHints {
    let mut hints = UserBoxFieldReturnHints::new();
    for plan in &module.metadata.typed_object_plans {
        for field in &plan.fields {
            let hint = if field.storage.uses_integer_lane() {
                UserBoxMethodInferredReturn::ScalarI64
            } else {
                UserBoxMethodInferredReturn::ObjectHandle
            };
            hints.insert((plan.box_name.clone(), field.name.clone()), hint);
        }
    }
    for ((box_name, field), origin) in field_box_origins {
        let Some(field_box) = box_origin_known(origin) else {
            continue;
        };
        let hint = if field_box == "StringBox" {
            UserBoxMethodInferredReturn::StringHandle
        } else {
            UserBoxMethodInferredReturn::ObjectHandle
        };
        hints.insert((box_name.clone(), field.clone()), hint);
    }
    hints
}

pub(crate) fn user_box_route_receiver_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    user_box_names: &BTreeSet<String>,
    callee_box_name: &str,
    certainty: TypeCertainty,
    receiver: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    let mut value_origin_context = ValueOriginQueryContext::new(function, def_map);
    user_box_route_receiver_box_name_with_origin_context(
        function,
        def_map,
        route_result_lookup,
        user_box_names,
        callee_box_name,
        certainty,
        receiver,
        param_box_origins,
        field_box_origins,
        &mut value_origin_context,
    )
}

fn user_box_route_receiver_box_name_with_origin_context(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    user_box_names: &BTreeSet<String>,
    callee_box_name: &str,
    certainty: TypeCertainty,
    receiver: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
    value_origin_context: &mut ValueOriginQueryContext<'_>,
) -> Option<String> {
    if certainty == TypeCertainty::Known && user_box_names.contains(callee_box_name) {
        return Some(callee_box_name.to_string());
    }
    user_box_value_box_name_with_origin_context(
        function,
        def_map,
        route_result_lookup,
        receiver,
        param_box_origins,
        field_box_origins,
        value_origin_context,
    )
    .filter(|box_name| user_box_names.contains(box_name))
}

pub(crate) fn user_box_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    value: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    let mut value_origin_context = ValueOriginQueryContext::new(function, def_map);
    user_box_value_box_name_with_origin_context(
        function,
        def_map,
        route_result_lookup,
        value,
        param_box_origins,
        field_box_origins,
        &mut value_origin_context,
    )
}

fn user_box_value_box_name_with_origin_context(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    value: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
    value_origin_context: &mut ValueOriginQueryContext<'_>,
) -> Option<String> {
    let origin = value_origin_context.origin(value);
    if let Some(box_name) = value_box_name(function, origin).map(str::to_string) {
        return Some(box_name);
    }
    if let Some(box_name) =
        route_result_box_name_cached(route_result_lookup, origin).map(str::to_string)
    {
        return Some(box_name);
    }
    if let Some((block_id, instruction_index)) = def_map.get(&origin).copied() {
        let block = function.blocks.get(&block_id)?;
        match block.instructions.get(instruction_index)? {
            MirInstruction::Const {
                value: ConstValue::String(_),
                ..
            } => return Some("StringBox".to_string()),
            MirInstruction::NewBox { box_type, .. } => return Some(box_type.clone()),
            MirInstruction::Phi {
                inputs, type_hint, ..
            } => {
                if let Some(box_name) = type_hint.as_ref().and_then(box_name_from_type) {
                    return Some(box_name.to_string());
                }
                if let Some(box_name) =
                    phi_input_box_name(
                        function,
                        def_map,
                        route_result_lookup,
                        inputs,
                        value_origin_context,
                    )
                {
                    return Some(box_name);
                }
            }
            MirInstruction::FieldGet { base, field, .. } => {
                let base_box = user_box_value_box_name_with_origin_context(
                    function,
                    def_map,
                    route_result_lookup,
                    *base,
                    param_box_origins,
                    field_box_origins,
                    value_origin_context,
                )?;
                if let Some(field_box) = field_box_origin(field_box_origins, &base_box, field) {
                    return Some(field_box);
                }
            }
            _ => {}
        }
    }
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(|index| {
            param_box_origin(param_box_origins, &function.signature.name, index).or_else(|| {
                (index == 0)
                    .then(|| method_receiver_box_name(&function.signature.name))
                    .flatten()
            })
        })
}
