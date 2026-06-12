use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};

use super::generic_string_abi::{
    generic_pure_string_abi_type_is_handle_compatible,
    generic_pure_string_return_allows_param_passthrough,
};
use super::generic_string_body_analysis::generic_pure_string_instruction_reject_reason;
use super::generic_string_corridor::seed_generic_pure_string_corridor_method_values;
use super::generic_string_facts::{
    generic_pure_string_iteration_limit, seed_generic_pure_string_return_param_values,
    seed_generic_pure_values, value_class, GenericPureValueClass,
};
use super::generic_string_guards::generic_pure_string_non_void_guard_phi_values;
use super::generic_string_reject::GenericPureStringReject;
use super::model::{GlobalCallTargetFacts, GlobalCallTargetShapeReason};
use super::string_return_profile::{
    generic_string_return_object_boundary_candidate_from_profile,
    generic_string_void_sentinel_profile_may_apply,
    generic_string_void_sentinel_return_candidate_from_profile,
    generic_string_void_sentinel_return_global_blocker_from_profile,
    GenericStringReturnProfileCache,
};

mod known_receiver;
mod void_logging;

pub(super) fn generic_string_void_logging_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    void_logging::generic_string_void_logging_body_reject_reason(function, targets)
}

pub(super) fn generic_pure_string_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    string_return_profiles: &mut GenericStringReturnProfileCache,
) -> Option<GenericPureStringReject> {
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        if function.signature.return_type == MirType::Void {
            if let Some(reject) = generic_string_void_sentinel_body_reject_reason(
                function,
                targets,
                string_return_profiles,
            ) {
                return Some(reject);
            }
        }
        if matches!(&function.signature.return_type, MirType::Box(name) if name != "StringBox") {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
            ));
        }
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
    if let Some(reject) =
        known_receiver::generic_pure_string_known_receiver_return_blocker(function)
    {
        return Some(reject);
    }

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    seed_generic_pure_string_return_param_values(function, &mut return_param_values);
    seed_generic_pure_string_corridor_method_values(function, &mut values, &mut has_string_surface);
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

    if !has_string_surface {
        if has_void_sentinel_const {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
            ));
        }
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringNoStringSurface,
        ));
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Return { value: Some(value) } = instruction {
                saw_return = true;
                let class = value_class(&values, *value);
                if class == GenericPureValueClass::VoidSentinel {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                    ));
                }
                if class != GenericPureValueClass::String
                    && !generic_pure_string_return_allows_param_passthrough(
                        function,
                        *value,
                        &return_param_values,
                    )
                {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringReturnNotString,
                    ));
                }
            } else if matches!(instruction, MirInstruction::Return { value: None }) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringReturnNotString,
                ));
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

pub(super) fn generic_string_void_sentinel_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    string_return_profiles: &mut GenericStringReturnProfileCache,
) -> Option<GenericPureStringReject> {
    if function.signature.return_type == MirType::Integer
        && !generic_string_void_sentinel_profile_may_apply(function, targets)
    {
        return None;
    }
    let profile = string_return_profiles.profile_for(function, targets);
    if !generic_string_void_sentinel_return_candidate_from_profile(function, &profile) {
        if let Some(reject) =
            generic_string_void_sentinel_return_global_blocker_from_profile(function, &profile)
        {
            return Some(reject);
        }
        if generic_string_return_object_boundary_candidate_from_profile(function, &profile) {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
            ));
        }
        return None;
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

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    seed_generic_pure_string_corridor_method_values(function, &mut values, &mut has_string_surface);
    let non_void_string_values = generic_pure_string_non_void_guard_phi_values(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
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

    Some(GenericPureStringReject::new(
        GlobalCallTargetShapeReason::GenericStringReturnVoidSentinelCandidate,
    ))
}
