use std::collections::{BTreeMap, BTreeSet};

use super::{lookup_global_call_target, BinaryOp, Callee, ConstValue, GlobalCallTargetFacts};
use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn is_void_side_effect_body_function(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if function.signature.return_type != MirType::Void {
        return false;
    }
    if function.params.len() != function.signature.params.len() {
        return false;
    }

    let mut void_values = BTreeSet::<ValueId>::new();
    let mut saw_return = false;
    let mut saw_route_backed_effect = false;
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..8 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                match instruction {
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Void | ConstValue::Null,
                    } => {
                        if void_values.insert(*dst) {
                            changed = true;
                        }
                    }
                    MirInstruction::Copy { dst, src } if void_values.contains(src) => {
                        if void_values.insert(*dst) {
                            changed = true;
                        }
                    }
                    MirInstruction::Phi { dst, inputs, .. }
                        if !inputs.is_empty()
                            && inputs
                                .iter()
                                .all(|(_, value)| void_values.contains(value)) =>
                    {
                        if void_values.insert(*dst) {
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

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .enumerate()
        {
            if !void_side_effect_instruction_supported(
                function,
                block_id,
                instruction_index,
                instruction,
                targets,
                &void_values,
                &mut saw_route_backed_effect,
                &mut saw_return,
            ) {
                return false;
            }
        }
    }

    saw_return && saw_route_backed_effect
}

fn void_side_effect_instruction_supported(
    function: &MirFunction,
    block_id: crate::mir::BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    void_values: &BTreeSet<ValueId>,
    saw_route_backed_effect: &mut bool,
    saw_return: &mut bool,
) -> bool {
    match instruction {
        MirInstruction::Const { value, .. } => matches!(
            value,
            ConstValue::Integer(_) | ConstValue::Bool(_) | ConstValue::Void | ConstValue::Null
        ),
        MirInstruction::Copy { .. }
        | MirInstruction::FieldGet { .. }
        | MirInstruction::FieldSet { .. }
        | MirInstruction::Compare { .. }
        | MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::Phi { .. } => true,
        MirInstruction::BinOp { op, .. } => matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        ),
        MirInstruction::Return { value } => {
            *saw_return = true;
            value.map(|value| void_values.contains(&value)).unwrap_or(true)
        }
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        } => {
            let supported = function.metadata.generic_method_routes.iter().any(|route| {
                route.block() == block_id
                    && route.instruction_index() == instruction_index
                    && matches!(route.route_id(), "generic_method.push" | "generic_method.set")
            }) || function.metadata.user_box_method_routes.iter().any(|route| {
                route.block() == block_id
                    && route.instruction_index() == instruction_index
                    && route.return_shape() == Some("void_sentinel_i64_zero")
                    && route.reason().is_none()
            });
            if supported {
                *saw_route_backed_effect = true;
            }
            supported
        }
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } => {
            let supported = lookup_global_call_target(name, targets)
                .map(|target| target.return_contract().is_some())
                .unwrap_or(false);
            if supported {
                *saw_route_backed_effect = true;
            }
            supported
        }
        _ => false,
    }
}
