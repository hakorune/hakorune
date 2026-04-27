/*!
 * Shared array receiver proof helpers for MIR-owned backend route plans.
 *
 * RuntimeDataBox call surfaces need a narrow provenance proof before backend
 * route planners may treat them as ArrayBox operations. This module owns that
 * proof so each route family can focus on its own window shape.
 */

use super::definitions::Callee;
use super::value_origin::{resolve_value_origin, ValueDefMap};
use super::{MirFunction, MirInstruction, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArrayGetCall<'a> {
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub output_value: ValueId,
    pub receiver_box_name: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArraySetCall {
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub input_value: ValueId,
}

pub(crate) fn value_root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

pub(crate) fn same_value_root(
    function: &MirFunction,
    def_map: &ValueDefMap,
    lhs: ValueId,
    rhs: ValueId,
) -> bool {
    value_root(function, def_map, lhs) == value_root(function, def_map, rhs)
}

pub(crate) fn match_array_get_call(inst: &MirInstruction) -> Option<ArrayGetCall<'_>> {
    match inst {
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
        } if args.len() == 1
            && method == "get"
            && matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") =>
        {
            Some(ArrayGetCall {
                array_value: *receiver,
                index_value: args[0],
                output_value: *dst,
                receiver_box_name: box_name.as_str(),
            })
        }
        _ => None,
    }
}

pub(crate) fn match_array_set_call(inst: &MirInstruction) -> Option<ArraySetCall> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if args.len() == 2
            && method == "set"
            && matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") =>
        {
            Some(ArraySetCall {
                array_value: *receiver,
                index_value: args[0],
                input_value: args[1],
            })
        }
        _ => None,
    }
}

pub(crate) fn receiver_is_proven_array(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    receiver_box_name: &str,
) -> bool {
    if receiver_box_name == "ArrayBox" {
        return true;
    }

    let receiver_root = value_root(function, def_map, receiver);
    if value_is_array_box_type(function.metadata.value_types.get(&receiver_root)) {
        return true;
    }

    if value_is_array_box_newbox(function, def_map, receiver_root) {
        return true;
    }

    function
        .params
        .iter()
        .position(|param| *param == receiver_root)
        .and_then(|index| function.signature.params.get(index))
        .is_some_and(type_is_array_box)
}

fn value_is_array_box_newbox(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    let Some(inst) = def_instruction(function, def_map, value) else {
        return false;
    };
    matches!(inst, MirInstruction::NewBox { box_type, .. } if box_type == "ArrayBox")
}

fn def_instruction<'a>(
    function: &'a MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<&'a MirInstruction> {
    let (block, index) = def_map.get(&value).copied()?;
    function.blocks.get(&block)?.instructions.get(index)
}

fn value_is_array_box_type(ty: Option<&MirType>) -> bool {
    ty.is_some_and(type_is_array_box)
}

fn type_is_array_box(ty: &MirType) -> bool {
    matches!(ty, MirType::Box(name) if name == "ArrayBox")
}
