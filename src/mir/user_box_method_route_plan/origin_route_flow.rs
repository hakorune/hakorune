use std::collections::{BTreeSet, HashMap};

use crate::mir::value_origin::{ValueDefMap, ValueOriginQueryContext};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

use super::{box_name_from_type, route_result_box_name_cached, value_box_name};

type RouteFlowMemo = HashMap<ValueId, Option<String>>;

pub(super) fn phi_input_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    inputs: &[(BasicBlockId, ValueId)],
    value_origin_context: &mut ValueOriginQueryContext<'_>,
) -> Option<String> {
    let mut inferred = None;
    let mut visiting = BTreeSet::new();
    let mut memo = RouteFlowMemo::new();
    for (_, input) in inputs {
        let box_name = route_flow_value_box_name(
            function,
            def_map,
            route_result_lookup,
            *input,
            &mut visiting,
            &mut memo,
            value_origin_context,
        )?;
        inferred = match inferred {
            None => Some(box_name),
            Some(existing) if existing == box_name => Some(existing),
            _ => return None,
        };
    }
    inferred
}

fn route_flow_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    value: ValueId,
    visiting: &mut BTreeSet<ValueId>,
    memo: &mut RouteFlowMemo,
    value_origin_context: &mut ValueOriginQueryContext<'_>,
) -> Option<String> {
    let origin = value_origin_context.origin(value);
    if let Some(cached) = memo.get(&origin) {
        return cached.clone();
    }
    if !visiting.insert(origin) {
        memo.insert(origin, None);
        return None;
    }

    let result = value_box_name(function, origin)
        .or_else(|| route_result_box_name_cached(route_result_lookup, origin))
        .map(str::to_string)
        .or_else(|| route_flow_origin_instruction_box_name(
            function,
            def_map,
            route_result_lookup,
            origin,
            visiting,
            memo,
            value_origin_context,
        ));

    visiting.remove(&origin);
    memo.insert(origin, result.clone());
    result
}

fn route_flow_origin_instruction_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route_result_lookup: &HashMap<ValueId, String>,
    origin: ValueId,
    visiting: &mut BTreeSet<ValueId>,
    memo: &mut RouteFlowMemo,
    value_origin_context: &mut ValueOriginQueryContext<'_>,
) -> Option<String> {
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let instruction = function
        .blocks
        .get(&block_id)?
        .instructions
        .get(instruction_index)?;
    match instruction {
        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
        MirInstruction::Phi {
            inputs, type_hint, ..
        } => type_hint
            .as_ref()
            .and_then(box_name_from_type)
            .map(str::to_string)
            .or_else(|| {
                let mut inferred = None;
                for (_, input) in inputs {
                    let box_name = route_flow_value_box_name(
                        function,
                        def_map,
                        route_result_lookup,
                        *input,
                        visiting,
                        memo,
                        value_origin_context,
                    )?;
                    inferred = match inferred {
                        None => Some(box_name),
                        Some(existing) if existing == box_name => Some(existing),
                        _ => return None,
                    };
                }
                inferred
            }),
        _ => None,
    }
}
