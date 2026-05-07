use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{Callee, MirFunction, MirInstruction, MirType, ValueId};

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
use super::model::{
    GlobalCallReturnContract, GlobalCallTargetFacts, GlobalCallTargetShapeReason,
};
use super::string_return_profile::{
    generic_string_return_object_boundary_candidate, generic_string_void_sentinel_return_candidate,
    generic_string_void_sentinel_return_global_blocker,
};

pub(super) fn generic_pure_string_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        if function.signature.return_type == MirType::Void {
            if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets)
            {
                return Some(reject);
            }
            if generic_string_return_object_boundary_candidate(function, targets) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
                ));
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
    if let Some(reject) = generic_pure_string_known_receiver_return_blocker(function) {
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

fn generic_pure_string_known_receiver_return_blocker(
    function: &MirFunction,
) -> Option<GenericPureStringReject> {
    let mut blockers = BTreeMap::<ValueId, String>::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                update_generic_pure_string_known_receiver_return_blockers(
                    instruction,
                    &mut blockers,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let Some(symbol) = blockers.get(value) else {
                continue;
            };
            return Some(GenericPureStringReject::with_blocker(
                GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod,
                symbol.clone(),
                Some(GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod),
            ));
        }
    }
    None
}

fn update_generic_pure_string_known_receiver_return_blockers(
    instruction: &MirInstruction,
    blockers: &mut BTreeMap<ValueId, String>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            ..
        } if box_name == "ParserBox" => {
            let symbol = format!("{}.{}", box_name, method);
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(symbol) = blockers.get(src).cloned() {
                if blockers.get(dst) != Some(&symbol) {
                    blockers.insert(*dst, symbol);
                    *changed = true;
                }
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            let Some(symbol) = inputs
                .iter()
                .filter_map(|(_, value)| blockers.get(value))
                .next()
                .cloned()
            else {
                return;
            };
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        _ => {}
    }
}

pub(super) fn generic_string_void_sentinel_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_string_void_sentinel_return_candidate(function, targets) {
        if let Some(reject) = generic_string_void_sentinel_return_global_blocker(function, targets)
        {
            return Some(reject);
        }
        if generic_string_return_object_boundary_candidate(function, targets) {
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
                    } if super::lookup_global_call_target(name, targets)
                        .map(|target| {
                            target.return_contract()
                                == Some(GlobalCallReturnContract::VoidSentinelI64Zero)
                        })
                        .unwrap_or(false)
                )
            })
    })
}
