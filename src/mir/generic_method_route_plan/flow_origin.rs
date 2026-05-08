use std::collections::{BTreeMap, BTreeSet};

use crate::mir::generic_method_route_facts::{const_string_value, receiver_origin_box_name};
use crate::mir::global_call_route_plan::GlobalCallRoute;
use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId,
};

use super::{
    method_args_without_redundant_receiver, typed_object_value_box_name, FieldHandleOriginMap,
};

pub(super) fn generic_runtime_data_contains_param_text_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    box_name: &str,
    receiver: ValueId,
    needle: ValueId,
) -> Option<String> {
    if box_name != "RuntimeDataBox" {
        return None;
    }
    if generic_pure_string_value_origin_box_name(function, def_map, needle).as_deref()
        != Some("StringBox")
    {
        return None;
    }
    let mut visited = BTreeSet::new();
    generic_value_flows_from_text_param(function, def_map, receiver, &mut visited)
        .then(|| "StringBox".to_string())
}

pub(super) fn generic_string_receiver_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    box_name: &str,
) -> Option<String> {
    receiver_origin_box_name(function, def_map, receiver)
        .or_else(|| generic_pure_string_value_origin_box_name(function, def_map, receiver))
        .or_else(|| {
            string_corridor_value_origin_box_name(
                function,
                def_map,
                receiver,
                StringCorridorOp::StrSlice,
            )
        })
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()))
}

fn string_corridor_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    op: StringCorridorOp,
) -> Option<String> {
    let mut visited = BTreeSet::new();
    string_corridor_value_has_op_flow(function, def_map, value, op, &mut visited)
        .then(|| "StringBox".to_string())
}

fn string_corridor_value_has_op_flow(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    op: StringCorridorOp,
    visited: &mut BTreeSet<ValueId>,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    if !visited.insert(origin) {
        return false;
    }
    if function
        .metadata
        .string_corridor_facts
        .get(&origin)
        .is_some_and(|fact| fact.op == op)
    {
        return true;
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Phi { inputs, .. }) if !inputs.is_empty() => {
            inputs.iter().all(|(_, input)| {
                let mut branch_visited = visited.clone();
                string_corridor_value_has_op_flow(
                    function,
                    def_map,
                    *input,
                    op,
                    &mut branch_visited,
                )
            })
        }
        _ => false,
    }
}

pub(super) fn string_corridor_method_origin_box_name(
    function: &MirFunction,
    dst: Option<ValueId>,
    op: StringCorridorOp,
) -> Option<String> {
    let dst = dst?;
    let fact = function.metadata.string_corridor_facts.get(&dst)?;
    (fact.op == op).then(|| "StringBox".to_string())
}

fn generic_pure_string_global_call_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, receiver);
    let (block, instruction_index) = def_map.get(&origin).copied()?;
    function
        .metadata
        .global_call_routes
        .iter()
        .any(|route| {
            route.block() == block
                && route.instruction_index() == instruction_index
                && route.result_value() == Some(origin)
                && global_call_route_returns_string_like_handle(route)
        })
        .then(|| "StringBox".to_string())
}

fn global_call_route_returns_string_like_handle(route: &GlobalCallRoute) -> bool {
    matches!(
        route.proof(),
        "typed_global_call_generic_pure_string"
            | "typed_global_call_generic_string_or_void_sentinel"
    ) && matches!(
        route.return_shape(),
        Some("string_handle" | "string_handle_or_null")
    )
}

pub(super) fn generic_pure_string_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    generic_pure_string_signature_param_origin_box_name(function, def_map, receiver)
        .or_else(|| generic_pure_string_global_call_origin_box_name(function, def_map, receiver))
        .or_else(|| generic_pure_string_flow_origin_box_name(function, receiver))
}

fn generic_pure_string_signature_param_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, receiver);
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(|index| function.signature.params.get(index))
        .and_then(|ty| match ty {
            MirType::String => Some("StringBox".to_string()),
            MirType::Box(name) if name == "StringBox" => Some("StringBox".to_string()),
            _ => None,
        })
}

fn generic_value_flows_from_text_param(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visited: &mut BTreeSet<ValueId>,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    if !visited.insert(origin) {
        return false;
    }
    if generic_value_is_text_param(function, origin) {
        return true;
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Phi { inputs, .. }) if !inputs.is_empty() => {
            inputs.iter().all(|(_, input)| {
                let mut branch_visited = visited.clone();
                generic_value_flows_from_text_param(function, def_map, *input, &mut branch_visited)
            })
        }
        _ => false,
    }
}

fn generic_value_is_text_param(function: &MirFunction, value: ValueId) -> bool {
    function
        .params
        .iter()
        .position(|param| *param == value)
        .and_then(|index| function.signature.params.get(index))
        .is_some_and(generic_param_type_can_flow_as_text)
}

fn generic_param_type_can_flow_as_text(ty: &MirType) -> bool {
    matches!(ty, MirType::Unknown | MirType::String)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

pub(super) fn generic_array_flow_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
    receiver: ValueId,
) -> Option<String> {
    let mut array_values = BTreeMap::<ValueId, &'static str>::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, inst) in block.instructions.iter().enumerate() {
                match inst {
                    MirInstruction::NewBox { dst, box_type, .. } => {
                        let Some(origin_box) = collection_origin_box_name(box_type) else {
                            continue;
                        };
                        if array_values.insert(*dst, origin_box) != Some(origin_box) {
                            changed = true;
                        }
                    }
                    MirInstruction::Copy { dst, src } => {
                        if let Some(origin) = array_values.get(src).copied() {
                            if array_values.insert(*dst, origin) != Some(origin) {
                                changed = true;
                            }
                        }
                    }
                    MirInstruction::FieldGet {
                        dst, base, field, ..
                    } => {
                        let Some(box_name) = typed_object_value_box_name(function, def_map, *base)
                        else {
                            continue;
                        };
                        let Some(origin_box) = field_handle_origins
                            .get(&(box_name, field.clone()))
                            .and_then(|origin| collection_origin_box_name(origin))
                        else {
                            continue;
                        };
                        if array_values.insert(*dst, origin_box) != Some(origin_box) {
                            changed = true;
                        }
                    }
                    MirInstruction::Phi { dst, inputs, .. } if !inputs.is_empty() => {
                        let mut origin = None;
                        let mut all_same = true;
                        for (_, value) in inputs {
                            let Some(input_origin) = array_values.get(value).copied() else {
                                all_same = false;
                                break;
                            };
                            if let Some(existing) = origin {
                                if existing != input_origin {
                                    all_same = false;
                                    break;
                                }
                            } else {
                                origin = Some(input_origin);
                            }
                        }
                        if all_same {
                            if let Some(origin) = origin {
                                if array_values.insert(*dst, origin) != Some(origin) {
                                    changed = true;
                                }
                            }
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(_receiver),
                                ..
                            }),
                        args,
                        ..
                    } if function.signature.name == "MirJsonEmitBox._emit_flags/1"
                        && box_name == "RuntimeDataBox"
                        && method == "keys"
                        && args.is_empty() =>
                    {
                        if array_values.insert(*dst, "ArrayBox") != Some("ArrayBox") {
                            changed = true;
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                ..
                            }),
                        args,
                        ..
                    } if method == "keys"
                        && method_args_without_redundant_receiver(
                            function, def_map, *receiver, args, 0,
                        )
                        .is_some()
                        && matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox")
                        && (box_name == "MapBox"
                            || receiver_origin_box_name(function, def_map, *receiver)
                                .as_deref()
                                == Some("MapBox")) =>
                    {
                        if array_values.insert(*dst, "ArrayBox") != Some("ArrayBox") {
                            changed = true;
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(Callee::Method {
                                box_name, method, ..
                            }),
                        args,
                        ..
                    } if function.signature.name == "MirJsonEmitBox._emit_function/1"
                        && box_name == "RuntimeDataBox"
                        && method == "get"
                        && args.len() == 1
                        && const_string_value(function, def_map, args[0])
                            .is_some_and(|key| key == "params" || key == "blocks") =>
                    {
                        if array_values.insert(*dst, "ArrayBox") != Some("ArrayBox") {
                            changed = true;
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Global(_)),
                        ..
                    } => {
                        let is_static_array =
                            function.metadata.global_call_routes.iter().any(|route| {
                                route.block() == *block_id
                                    && route.instruction_index() == instruction_index
                                    && route.result_value() == Some(*dst)
                                    && route.proof() == "typed_global_call_static_string_array"
                                    && route.return_shape() == Some("array_handle")
                            });
                        if is_static_array
                            && array_values.insert(*dst, "ArrayBox") != Some("ArrayBox")
                        {
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        if !changed {
            break;
        }
    }

    array_values.get(&receiver).map(|name| (*name).to_string())
}

fn collection_origin_box_name(box_name: &str) -> Option<&'static str> {
    match box_name {
        "ArrayBox" => Some("ArrayBox"),
        "MapBox" => Some("MapBox"),
        _ => None,
    }
}

fn generic_pure_string_flow_origin_box_name(
    function: &MirFunction,
    receiver: ValueId,
) -> Option<String> {
    let mut string_values = BTreeSet::<ValueId>::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, inst) in block.instructions.iter().enumerate() {
                if generic_pure_string_flow_marks_instruction(
                    function,
                    &mut string_values,
                    *block_id,
                    instruction_index,
                    inst,
                ) {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    string_values
        .contains(&receiver)
        .then(|| "StringBox".to_string())
}

fn generic_pure_string_flow_marks_instruction(
    function: &MirFunction,
    string_values: &mut BTreeSet<ValueId>,
    block_id: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> bool {
    let mark = |string_values: &mut BTreeSet<ValueId>, value| string_values.insert(value);
    match inst {
        MirInstruction::Const {
            dst,
            value: ConstValue::String(_),
        } => mark(string_values, *dst),
        MirInstruction::NewBox { dst, box_type, .. } if box_type == "StringBox" => {
            mark(string_values, *dst)
        }
        MirInstruction::Copy { dst, src } if string_values.contains(src) => {
            mark(string_values, *dst)
        }
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } if string_values.contains(lhs) || string_values.contains(rhs) => {
            mark(string_values, *dst)
        }
        MirInstruction::Phi { dst, inputs, .. }
            if !inputs.is_empty()
                && inputs
                    .iter()
                    .all(|(_, value)| string_values.contains(value)) =>
        {
            mark(string_values, *dst)
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(_)),
            ..
        } if function.metadata.global_call_routes.iter().any(|route| {
            route.block() == block_id
                && route.instruction_index() == instruction_index
                && route.result_value() == Some(*dst)
                && global_call_route_returns_string_like_handle(route)
        }) =>
        {
            mark(string_values, *dst)
        }
        _ => false,
    }
}
