use std::collections::BTreeMap;

use crate::mir::{BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction};

pub(super) fn user_box_method_body_supported(
    function: &MirFunction,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> bool {
    if function.blocks.is_empty() {
        return false;
    }
    function.blocks.iter().all(|(block_id, block)| {
        let instructions_supported =
            block
                .instructions
                .iter()
                .enumerate()
                .all(|(instruction_index, instruction)| {
                    user_box_method_instruction_supported(
                        function,
                        *block_id,
                        instruction_index,
                        instruction,
                        typed_plan_type_ids,
                    )
                });
        instructions_supported
            && block
                .terminator
                .as_ref()
                .map(user_box_method_terminator_supported)
                .unwrap_or(false)
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
            ConstValue::Integer(_)
                | ConstValue::Bool(_)
                | ConstValue::String(_)
                | ConstValue::Void
                | ConstValue::Null
        ),
        MirInstruction::Copy { .. } => true,
        MirInstruction::NewBox { box_type, .. } => {
            matches!(box_type.as_str(), "ArrayBox" | "MapBox")
                || typed_plan_type_ids.contains_key(box_type)
        }
        MirInstruction::FieldGet { .. } | MirInstruction::FieldSet { .. } => true,
        MirInstruction::Phi { inputs, .. } => !inputs.is_empty(),
        MirInstruction::BinOp { op, .. } => matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        ),
        MirInstruction::Compare { .. } | MirInstruction::Select { .. } => true,
        MirInstruction::KeepAlive { .. } | MirInstruction::ReleaseStrong { .. } => true,
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
        } => {
            function.metadata.generic_method_routes.iter().any(|route| {
                route.block() == block_id && route.instruction_index() == instruction_index
            }) || function
                .metadata
                .user_box_method_routes
                .iter()
                .any(|route| {
                    route.block() == block_id
                        && route.instruction_index() == instruction_index
                        && route.reason().is_none()
                })
        }
        _ => false,
    }
}

fn user_box_method_terminator_supported(instruction: &MirInstruction) -> bool {
    matches!(
        instruction,
        MirInstruction::Branch { .. } | MirInstruction::Jump { .. } | MirInstruction::Return { .. }
    )
}
