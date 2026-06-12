use super::model::GlobalCallTargetFacts;
use crate::mir::{MirFunction, MirInstruction};
use std::collections::BTreeMap;

mod refine;
mod route;
mod string_methods;
mod value_class;

use refine::generic_i64_body_refine_instruction;
use value_class::{
    generic_i64_abi_type_is_i64_word_compatible, generic_i64_return_type_is_scalar,
    seed_generic_i64_values, GenericI64ValueClass,
};

pub(super) fn is_generic_i64_body_function(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if !generic_i64_return_type_is_scalar(&function.signature.return_type) {
        return false;
    }
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_i64_abi_type_is_i64_word_compatible)
    {
        return false;
    }

    let mut values = BTreeMap::<crate::mir::ValueId, GenericI64ValueClass>::new();
    if !seed_generic_i64_values(function, &mut values) {
        return false;
    }
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if !generic_i64_body_refine_instruction(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    targets,
                    &mut values,
                    &mut changed,
                ) {
                    return false;
                }
            }
            if let Some(terminator) = block.terminator.as_ref() {
                if !generic_i64_body_refine_instruction(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    targets,
                    &mut values,
                    &mut changed,
                ) {
                    return false;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let mut saw_return = false;
    let mut saw_scalar_return = false;
    let mut saw_void_sentinel_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    saw_return = true;
                    match values
                        .get(value)
                        .copied()
                        .unwrap_or(GenericI64ValueClass::Unknown)
                    {
                        GenericI64ValueClass::I64 | GenericI64ValueClass::Bool => {
                            saw_scalar_return = true;
                        }
                        GenericI64ValueClass::VoidSentinel => saw_void_sentinel_return = true,
                        _ => return false,
                    }
                    if saw_void_sentinel_return && !saw_scalar_return {
                        continue;
                    }
                    if !matches!(
                        values
                            .get(value)
                            .copied()
                            .unwrap_or(GenericI64ValueClass::Unknown),
                        GenericI64ValueClass::I64
                            | GenericI64ValueClass::Bool
                            | GenericI64ValueClass::VoidSentinel
                    ) {
                        return false;
                    }
                }
                MirInstruction::Return { value: None } => return false,
                _ => {}
            }
        }
    }
    saw_return && (!saw_void_sentinel_return || saw_scalar_return)
}
