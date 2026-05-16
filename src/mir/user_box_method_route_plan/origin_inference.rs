use std::collections::{BTreeMap, BTreeSet};

use super::return_shape::{UserBoxFieldReturnHints, UserBoxMethodInferredReturn};
use super::target_collection::{
    method_target_symbol, parse_method_symbol, UserBoxMethodTargetFacts,
};
use super::{
    BoxOriginInference, FieldBoxOriginKey, FieldBoxOriginMap, ParamBoxOriginKey, ParamBoxOriginMap,
    UserBoxMethodRoute,
};
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

pub(super) fn infer_user_box_method_param_box_origins(
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

    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for (param_index, box_name) in
                infer_param_box_origins_from_field_uses(function, &def_map, &typed_plan_fields)
            {
                if !user_box_names.contains(&box_name) {
                    continue;
                }
                changed |= merge_param_box_origin(
                    &mut origins,
                    (function.signature.name.clone(), param_index),
                    box_name,
                );
            }
            for block_id in sorted_block_ids(function) {
                let Some(block) = function.blocks.get(&block_id) else {
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
                    let Some(route_box_name) = user_box_route_receiver_box_name(
                        function,
                        &def_map,
                        &user_box_names,
                        box_name,
                        *certainty,
                        *receiver,
                        &current,
                        field_box_origins,
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
                        let Some(arg_box_name) = user_box_value_box_name(
                            function,
                            &def_map,
                            *arg,
                            &current,
                            field_box_origins,
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
                        let Some(caller_param_index) = value_param_index(function, &def_map, *arg)
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
    typed_plan_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<(usize, String)> {
    let mut param_fields = BTreeMap::<usize, BTreeSet<String>>::new();
    for block_id in sorted_block_ids(function) {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::FieldGet { base, field, .. }
                | MirInstruction::FieldSet { base, field, .. } => {
                    let Some(param_index) = value_param_index(function, def_map, *base) else {
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

pub(super) fn infer_user_box_field_box_origins(
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

    for _ in 0..module.functions.len().saturating_mul(2).max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
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
                    let Some(base_box) = user_box_value_box_name(
                        function,
                        &def_map,
                        *base,
                        param_box_origins,
                        &current,
                    ) else {
                        continue;
                    };
                    let Some(value_box) = user_box_value_box_name(
                        function,
                        &def_map,
                        *value,
                        param_box_origins,
                        &current,
                    ) else {
                        continue;
                    };
                    if !user_box_names.contains(&base_box)
                        || !(user_box_names.contains(&value_box) || value_box == "StringBox")
                    {
                        continue;
                    }
                    changed |=
                        merge_field_box_origin(&mut origins, (base_box, field.clone()), value_box);
                }
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
                    if method != "birth" {
                        continue;
                    }
                    let Some(route_box_name) = user_box_route_receiver_box_name(
                        function,
                        &def_map,
                        &user_box_names,
                        box_name,
                        *certainty,
                        *receiver,
                        param_box_origins,
                        &current,
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
                        let Some(value_box) = user_box_value_box_name(
                            function,
                            &def_map,
                            *arg,
                            param_box_origins,
                            &current,
                        ) else {
                            continue;
                        };
                        if !(user_box_names.contains(&value_box) || value_box == "StringBox") {
                            continue;
                        }
                        changed |= merge_field_box_origin(
                            &mut origins,
                            (route_box_name.clone(), field.clone()),
                            value_box,
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
                if Some(resolve_value_origin(function, &def_map, *base)) != receiver {
                    continue;
                }
                let value_origin = resolve_value_origin(function, &def_map, *value);
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

pub(super) fn build_user_box_field_return_hints(
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

fn merge_param_box_origin(
    origins: &mut ParamBoxOriginMap,
    key: ParamBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}

fn merge_field_box_origin(
    origins: &mut FieldBoxOriginMap,
    key: FieldBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}

pub(super) fn user_box_route_receiver_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    user_box_names: &BTreeSet<String>,
    callee_box_name: &str,
    certainty: TypeCertainty,
    receiver: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    if certainty == TypeCertainty::Known && user_box_names.contains(callee_box_name) {
        return Some(callee_box_name.to_string());
    }
    user_box_value_box_name(
        function,
        def_map,
        receiver,
        param_box_origins,
        field_box_origins,
    )
    .filter(|box_name| user_box_names.contains(box_name))
}

pub(super) fn user_box_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    if let Some(box_name) = value_box_name(function, origin).map(str::to_string) {
        return Some(box_name);
    }
    if let Some(box_name) = route_result_box_name(function, origin).map(str::to_string) {
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
                if let Some(box_name) = phi_input_box_name(function, def_map, inputs) {
                    return Some(box_name);
                }
            }
            MirInstruction::FieldGet { base, field, .. } => {
                let base_box = user_box_value_box_name(
                    function,
                    def_map,
                    *base,
                    param_box_origins,
                    field_box_origins,
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

fn value_param_index(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<usize> {
    let mut visiting = BTreeSet::new();
    value_param_index_inner(function, def_map, value, &mut visiting)
}

fn value_param_index_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<usize> {
    if !visiting.insert(value) {
        return None;
    }
    if let Some(index) = function.params.iter().position(|param| *param == value) {
        visiting.remove(&value);
        return Some(index);
    }
    let result = def_map
        .get(&value)
        .and_then(|(block_id, instruction_index)| {
            function
                .blocks
                .get(block_id)
                .and_then(|block| block.instructions.get(*instruction_index))
        })
        .and_then(|instruction| match instruction {
            MirInstruction::Copy { src, .. } => {
                value_param_index_inner(function, def_map, *src, visiting)
            }
            MirInstruction::Phi { inputs, .. } => {
                let mut inferred = None;
                for (_incoming_block, incoming_value) in inputs {
                    let index =
                        value_param_index_inner(function, def_map, *incoming_value, visiting)?;
                    inferred = match inferred {
                        None => Some(index),
                        Some(existing) if existing == index => Some(existing),
                        Some(_) => return None,
                    };
                }
                inferred
            }
            _ => None,
        });
    visiting.remove(&value);
    result
}

fn phi_input_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<String> {
    let mut inferred = None;
    for (_, input) in inputs {
        let origin = resolve_value_origin(function, def_map, *input);
        let box_name = value_box_name(function, origin)
            .or_else(|| route_result_box_name(function, origin))?
            .to_string();
        inferred = match inferred {
            None => Some(box_name),
            Some(existing) if existing == box_name => Some(existing),
            _ => return None,
        };
    }
    inferred
}

pub(super) fn route_result_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.reason().is_none() && route.result_value() == Some(value))
        .and_then(UserBoxMethodRoute::target_result_box_name)
        .or_else(|| {
            function
                .metadata
                .generic_method_routes
                .iter()
                .find(|route| route.result_value() == Some(value))
                .and_then(generic_method_route_result_box_name)
        })
        .or_else(|| {
            function
                .metadata
                .global_call_routes
                .iter()
                .find(|route| route.result_value() == Some(value))
                .and_then(global_call_route_result_box_name)
        })
}

pub(super) fn generic_method_route_result_box_name(
    route: &crate::mir::generic_method_route_plan::GenericMethodRoute,
) -> Option<&str> {
    route
        .result_origin_box()
        .or_else(|| match route.route_kind_tag() {
            "string_substring" => Some("StringBox"),
            "map_keys_array" => Some("ArrayBox"),
            _ => None,
        })
}

pub(super) fn value_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
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

pub(super) fn box_name_from_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::String => Some("StringBox"),
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

fn global_call_route_result_box_name(
    route: &crate::mir::global_call_route_plan::GlobalCallRoute,
) -> Option<&'static str> {
    if route.result_origin() == "string" {
        return Some("StringBox");
    }
    match route.return_shape() {
        Some("string_handle" | "string_handle_or_null") => Some("StringBox"),
        Some("array_handle") => Some("ArrayBox"),
        Some("map_handle") => Some("MapBox"),
        _ => None,
    }
}

pub(super) fn param_box_origin(
    param_box_origins: &ParamBoxOriginMap,
    function_name: &str,
    index: usize,
) -> Option<String> {
    match param_box_origins.get(&(function_name.to_string(), index)) {
        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

pub(super) fn field_box_origin(
    field_box_origins: &FieldBoxOriginMap,
    box_name: &str,
    field: &str,
) -> Option<String> {
    match field_box_origins.get(&(box_name.to_string(), field.to_string())) {
        Some(BoxOriginInference::Known(field_box)) => Some(field_box.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

fn box_origin_known(origin: &BoxOriginInference) -> Option<&str> {
    match origin {
        BoxOriginInference::Known(box_name) => Some(box_name.as_str()),
        BoxOriginInference::Conflict => None,
    }
}

fn method_receiver_box_name(symbol: &str) -> Option<String> {
    let (owner_and_method, _arity) = symbol.rsplit_once('/')?;
    let (box_name, _method) = owner_and_method.rsplit_once('.')?;
    Some(box_name.to_string())
}

pub(super) fn sorted_block_ids(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    block_ids
}
