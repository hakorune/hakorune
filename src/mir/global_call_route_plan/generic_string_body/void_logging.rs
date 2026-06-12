use std::collections::BTreeMap;

use crate::mir::{Callee, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeSet;

use super::super::generic_string_abi::generic_pure_string_abi_type_is_handle_compatible;
use super::super::generic_string_body_analysis::generic_pure_string_instruction_reject_reason;
use super::super::generic_string_facts::{
    generic_pure_string_iteration_limit, seed_generic_pure_values, value_class,
    GenericPureValueClass,
};
use super::super::generic_string_guards::generic_pure_string_non_void_guard_phi_values;
use super::super::generic_string_reject::GenericPureStringReject;
use super::super::model::{
    GlobalCallReturnContract, GlobalCallTargetFacts, GlobalCallTargetShapeReason,
};

pub(super) fn generic_string_void_logging_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if function.signature.return_type != MirType::Void {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnAbiNotHandleCompatible,
        ));
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringParamAbiNotHandleCompatible,
        ));
    }
    if function.params.len() != function.signature.params.len() {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        ));
    }

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    let non_void_string_values = generic_pure_string_non_void_guard_phi_values(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    let max_iterations = generic_pure_string_iteration_limit(function);
    for _ in 0..max_iterations {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
        }
        if !changed {
            break;
        }
    }

    if !has_string_surface || !generic_string_void_logging_has_logging_call(function, targets) {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringNoStringSurface,
        ));
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    saw_return = true;
                    if value_class(&values, *value) != GenericPureValueClass::VoidSentinel {
                        return Some(GenericPureStringReject::new(
                            GlobalCallTargetShapeReason::GenericStringReturnNotString,
                        ));
                    }
                }
                MirInstruction::Return { value: None } => saw_return = true,
                _ => {}
            }
        }
    }
    if saw_return {
        None
    } else {
        Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnNotString,
        ))
    }
}

fn generic_string_void_logging_has_logging_call(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Global(name)),
                        ..
                    } if name == "print"
                ) || matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Global(name)),
                        ..
                    } if super::super::lookup_global_call_target(name, targets)
                        .map(|target| {
                            target.return_contract()
                                == Some(GlobalCallReturnContract::VoidSentinelI64Zero)
                        })
                        .unwrap_or(false)
                )
            })
    })
}
