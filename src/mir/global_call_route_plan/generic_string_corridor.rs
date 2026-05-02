use std::collections::BTreeMap;

use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::{Callee, MirFunction, MirInstruction, ValueId};

use super::generic_string_facts::{
    set_string_handle_value_class, set_value_class, GenericPureValueClass,
};

pub(super) fn seed_generic_pure_string_corridor_method_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    has_string_surface: &mut bool,
) {
    let mut changed = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Call {
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
            } = instruction
            else {
                continue;
            };
            let Some(expected_op) = generic_pure_string_corridor_method_op(box_name, method, args)
            else {
                continue;
            };
            let Some(fact) = function.metadata.string_corridor_facts.get(dst) else {
                continue;
            };
            if fact.op != expected_op {
                continue;
            }
            *has_string_surface = true;
            set_string_handle_value_class(values, *receiver, &mut changed);
            match fact.op {
                StringCorridorOp::StrLen => {
                    set_value_class(values, *dst, GenericPureValueClass::I64, &mut changed);
                }
                StringCorridorOp::StrSlice => {
                    for arg in args {
                        set_value_class(values, *arg, GenericPureValueClass::I64, &mut changed);
                    }
                    set_string_handle_value_class(values, *dst, &mut changed);
                }
                StringCorridorOp::FreezeStr => {}
            }
        }
    }
}

fn generic_pure_string_corridor_method_op(
    box_name: &str,
    method: &str,
    args: &[ValueId],
) -> Option<StringCorridorOp> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox") {
        return None;
    }
    match method {
        "length" if args.is_empty() => Some(StringCorridorOp::StrLen),
        "substring" if matches!(args.len(), 1 | 2) => Some(StringCorridorOp::StrSlice),
        _ => None,
    }
}
