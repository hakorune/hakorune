use std::collections::BTreeMap;

use crate::mir::{BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction};

pub(crate) fn supported_backend_global(name: &str) -> bool {
    matches!(name, "print")
}

pub(crate) fn same_module_body_supported(
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
                    same_module_instruction_supported(
                        function,
                        *block_id,
                        instruction_index,
                        instruction,
                        typed_plan_type_ids,
                    )
                });
        let terminator_supported = block
            .terminator
            .as_ref()
            .map(same_module_terminator_supported)
            .or_else(|| {
                block
                    .instructions
                    .last()
                    .map(same_module_terminator_supported)
            })
            .unwrap_or(false);
        instructions_supported && terminator_supported
    })
}

fn same_module_instruction_supported(
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
        instruction if same_module_terminator_supported(instruction) => true,
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } => {
            supported_backend_global(name)
                || function.metadata.global_call_routes.iter().any(|route| {
                    route.block() == block_id
                        && route.instruction_index() == instruction_index
                        && route.reason().is_none()
                })
        }
        MirInstruction::Call {
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            args,
            ..
        } => {
            function.metadata.generic_method_routes.iter().any(|route| {
                route.block() == block_id && route.instruction_index() == instruction_index
            }) || is_self_recursive_method_call(function, box_name, method, args.len())
                || function
                    .metadata
                    .user_box_method_routes
                    .iter()
                    .any(|route| {
                        route.block() == block_id
                            && route.instruction_index() == instruction_index
                            && (route.reason().is_none()
                                || route.target_symbol() == function.signature.name)
                    })
        }
        _ => false,
    }
}

fn is_self_recursive_method_call(
    function: &MirFunction,
    box_name: &str,
    method: &str,
    arity: usize,
) -> bool {
    function.signature.name == format!("{box_name}.{method}/{arity}")
}

fn same_module_terminator_supported(instruction: &MirInstruction) -> bool {
    matches!(
        instruction,
        MirInstruction::Branch { .. } | MirInstruction::Jump { .. } | MirInstruction::Return { .. }
    )
}
