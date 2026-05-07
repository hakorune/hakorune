use std::collections::BTreeMap;

use crate::mir::{BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction};

pub(super) fn user_box_method_body_supported(
    function: &MirFunction,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> bool {
    if function.blocks.len() != 1 {
        return false;
    }
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return false;
    };
    if !matches!(block.terminator, Some(MirInstruction::Return { .. })) {
        return false;
    }
    block
        .instructions
        .iter()
        .enumerate()
        .all(|(instruction_index, instruction)| {
            user_box_method_instruction_supported(
                function,
                function.entry_block,
                instruction_index,
                instruction,
                typed_plan_type_ids,
            )
        })
}

fn user_box_method_instruction_supported(
    function: &MirFunction,
    block_id: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> bool {
    match instruction {
        MirInstruction::Const { value, .. } => matches!(
            value,
            ConstValue::Integer(_) | ConstValue::Bool(_) | ConstValue::Void | ConstValue::Null
        ),
        MirInstruction::Copy { .. } => true,
        MirInstruction::NewBox { box_type, .. } => {
            matches!(box_type.as_str(), "ArrayBox" | "MapBox")
                || typed_plan_type_ids.contains_key(box_type)
        }
        MirInstruction::FieldGet { .. } | MirInstruction::FieldSet { .. } => true,
        MirInstruction::BinOp { op, .. } => matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        ),
        MirInstruction::Call {
            callee: Some(Callee::Global(_)),
            ..
        } => function.metadata.global_call_routes.iter().any(|route| {
            route.block() == block_id
                && route.instruction_index() == instruction_index
                && route.reason().is_none()
        }),
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        } => function.metadata.user_box_method_routes.iter().any(|route| {
            route.block() == block_id
                && route.instruction_index() == instruction_index
                && route.reason().is_none()
        }),
        _ => false,
    }
}
