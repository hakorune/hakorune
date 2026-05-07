use std::collections::BTreeMap;

use crate::mir::{BinaryOp, ConstValue, MirFunction, MirInstruction};

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
        .all(|instruction| user_box_method_instruction_supported(instruction, typed_plan_type_ids))
}

fn user_box_method_instruction_supported(
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
        _ => false,
    }
}
