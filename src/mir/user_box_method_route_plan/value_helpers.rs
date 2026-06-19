use std::collections::{HashMap, HashSet};

use super::{BoxOriginInference, FieldBoxOriginMap, ParamBoxOriginMap};
use crate::mir::global_call_route_plan::GlobalCallRoute;
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};

pub(crate) fn build_route_result_box_lookup(function: &MirFunction) -> HashMap<ValueId, String> {
    let mut lookup = HashMap::new();
    for route in &function.metadata.user_box_method_routes {
        if route.reason().is_none() {
            if let (Some(value), Some(box_name)) =
                (route.result_value(), route.target_result_box_name())
            {
                lookup.entry(value).or_insert_with(|| box_name.to_string());
            }
        }
    }
    for route in &function.metadata.generic_method_routes {
        if let (Some(value), Some(box_name)) = (
            route.result_value(),
            generic_method_route_result_box_name(route),
        ) {
            lookup.entry(value).or_insert_with(|| box_name.to_string());
        }
    }
    for route in &function.metadata.global_call_routes {
        if let (Some(value), Some(box_name)) = (
            route.result_value(),
            global_call_route_result_box_name(route),
        ) {
            lookup.entry(value).or_insert_with(|| box_name.to_string());
        }
    }
    lookup
}

pub(crate) fn generic_method_route_result_box_name(
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

pub(crate) fn value_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
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

pub(crate) fn box_name_from_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::String => Some("StringBox"),
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

pub(crate) fn global_call_route_result_box_name(route: &GlobalCallRoute) -> Option<&str> {
    if let Some(box_name) = route.target_result_box_name() {
        return Some(box_name);
    }
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

pub(crate) fn param_box_origin(
    param_box_origins: &ParamBoxOriginMap,
    function_name: &str,
    index: usize,
) -> Option<String> {
    match param_box_origins.get(&(function_name.to_string(), index)) {
        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

pub(crate) fn field_box_origin(
    field_box_origins: &FieldBoxOriginMap,
    box_name: &str,
    field: &str,
) -> Option<String> {
    match field_box_origins.get(&(box_name.to_string(), field.to_string())) {
        Some(BoxOriginInference::Known(field_box)) => Some(field_box.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

pub(crate) fn route_result_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.reason().is_none() && route.result_value() == Some(value))
        .and_then(|route| route.target_result_box_name())
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

pub(crate) fn route_result_box_name_cached(
    route_result_lookup: &HashMap<ValueId, String>,
    value: ValueId,
) -> Option<&str> {
    route_result_lookup.get(&value).map(String::as_str)
}

pub(crate) fn box_origin_known(origin: &BoxOriginInference) -> Option<&str> {
    match origin {
        BoxOriginInference::Known(box_name) => Some(box_name.as_str()),
        BoxOriginInference::Conflict => None,
    }
}

pub(crate) fn method_receiver_box_name(symbol: &str) -> Option<String> {
    let (owner_and_method, _arity) = symbol.rsplit_once('/')?;
    let (box_name, _method) = owner_and_method.rsplit_once('.')?;
    Some(box_name.to_string())
}

pub(crate) fn sorted_block_ids(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    block_ids
}

pub(crate) fn value_param_index(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    cache: &mut HashMap<ValueId, Option<usize>>,
) -> Option<usize> {
    if let Some(cached) = cache.get(&value) {
        return *cached;
    }
    let mut visiting = HashSet::new();
    let resolved = value_param_index_inner(function, def_map, value, &mut visiting, cache);
    cache.insert(value, resolved);
    resolved
}

fn value_param_index_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visiting: &mut HashSet<ValueId>,
    cache: &mut HashMap<ValueId, Option<usize>>,
) -> Option<usize> {
    if let Some(cached) = cache.get(&value) {
        return *cached;
    }
    if !visiting.insert(value) {
        return None;
    }
    if let Some(index) = function.params.iter().position(|param| *param == value) {
        visiting.remove(&value);
        cache.insert(value, Some(index));
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
                value_param_index_inner(function, def_map, *src, visiting, cache)
            }
            MirInstruction::Phi { inputs, .. } => {
                let mut inferred = None;
                for (_incoming_block, incoming_value) in inputs {
                    let index = value_param_index_inner(
                        function,
                        def_map,
                        *incoming_value,
                        visiting,
                        cache,
                    )?;
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
    cache.insert(value, result);
    result
}
