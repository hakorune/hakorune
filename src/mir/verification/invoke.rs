//! Structural Normal/Fault verification, independent of compatibility env flags.
//! Runtime admission remains closed until the common Fault ABI consumer lands.

use super::{cfg, dom, ssa, utils};
use crate::mir::instruction::InvokeOperation;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction};

/// Declaration references survive publication; a backend must never recover
/// a missing field from a diagnostic name or the receiver's physical origin.
pub(super) fn check_module(module: &crate::mir::MirModule) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for function in module.functions.values() {
        for (id, block) in &function.blocks {
            for instruction in block.all_instructions() {
                if let MirInstruction::ObjectFieldGet { field, .. } = instruction {
                    if !module
                        .canonical_field_definition(*field)
                        .is_some_and(|definition| {
                            !definition.is_weak
                                && definition.declared_type_name.as_deref() == Some("i64")
                        })
                    {
                        errors.push(error(*id, "object-field-read-definition-invalid"));
                    }
                }
                if let MirInstruction::Invoke { operation, .. } = instruction {
                    match operation {
                        InvokeOperation::NewBox { object }
                        | InvokeOperation::HomeRelease { object, .. }
                        | InvokeOperation::ReclaimUnpublished { object, .. }
                            if module.canonical_object_definition(*object).is_none() =>
                        {
                            errors.push(error(*id, "object-definition-missing"));
                        }
                        InvokeOperation::HomeRelease { object, .. }
                            if !module.canonical_object_definition(*object).is_some_and(|definition|
                                definition.destruction_disposition()
                                    == crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook) =>
                        {
                            errors.push(error(*id, "home-destruction-unavailable"));
                        }
                        InvokeOperation::FieldSet { field, .. }
                            if module.canonical_field_definition(*field).is_none() =>
                        {
                            errors.push(error(*id, "field-definition-missing"));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(super) fn check_function(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let has_control = function.blocks.values().any(|block| {
        block.all_instructions().any(|inst| {
            matches!(
                inst,
                MirInstruction::Invoke { .. }
                    | MirInstruction::InvokeNormalResult { .. }
                    | MirInstruction::ReturnFault { .. }
                    | MirInstruction::FaultFrameEnter { .. }
                    | MirInstruction::ArrayResidenceRelease { .. }
            )
        })
    });
    if !has_control {
        return Ok(());
    }
    let mut errors = Vec::new();
    check_frame_entry(function, &mut errors);
    for result in [
        ssa::check_ssa_form(function),
        cfg::check_control_flow(function),
    ] {
        if let Err(mut found) = result {
            errors.append(&mut found);
        }
    }
    // Compute actual edges, not a potentially stale predecessor cache.
    let mut predecessors = std::collections::BTreeMap::<_, Vec<_>>::new();
    for (id, block) in &function.blocks {
        for target in block.successors_from_terminator() {
            predecessors.entry(target).or_default().push(*id);
        }
    }
    for (id, block) in &function.blocks {
        for instruction in &block.instructions {
            if matches!(
                instruction,
                MirInstruction::Invoke { .. } | MirInstruction::ReturnFault { .. }
            ) {
                errors.push(error(*id, "control-in-instruction-list"));
            }
        }
        if let Some(MirInstruction::Invoke {
            operation,
            normal_landing,
            fault_landing,
            ..
        }) = &block.terminator
        {
            if normal_landing == fault_landing {
                errors.push(error(*id, "identical-landings"));
            }
            // Entry has an implicit incoming execution edge; CFG predecessors
            // alone cannot prove that its result storage was initialized.
            if *normal_landing == function.entry_block || normal_landing == id {
                errors.push(error(*id, "normal-landing-before-invocation"));
            }
            let value_result = match operation {
                InvokeOperation::NewBox { .. } | InvokeOperation::IntrinsicArrayNew => true,
                InvokeOperation::FieldSet { .. }
                | InvokeOperation::ArrayStateContractClaim { .. }
                | InvokeOperation::ArrayElementWrite { .. }
                | InvokeOperation::HomeRelease { .. }
                | InvokeOperation::ReclaimUnpublished { .. } => false,
                InvokeOperation::Call(call) => {
                    if call.dst.is_some() {
                        errors.push(error(*id, "embedded-call-destination"));
                    }
                    // Birth's source contract is Unit. Other result ABI families
                    // require their canonical definition relation before opening.
                    if !matches!(call.callee, Callee::BirthConstructor { .. }) {
                        errors.push(error(*id, "call-result-contract-not-connected"));
                    }
                    false
                }
            };
            let projections = function
                .blocks
                .values()
                .flat_map(|b| b.all_instructions())
                .filter(|inst| {
                    matches!(inst, MirInstruction::InvokeNormalResult { invoke_block, .. }
                    if invoke_block == id)
                })
                .count();
            if projections != usize::from(value_result) {
                errors.push(error(*id, "normal-result-count"));
            }
            if predecessors.get(normal_landing).map(Vec::as_slice) != Some(&[*id]) {
                errors.push(error(*id, "normal-landing-not-exclusive"));
            }
        }
        for (index, instruction) in block.all_instructions().enumerate() {
            let MirInstruction::InvokeNormalResult { invoke_block, .. } = instruction else {
                continue;
            };
            let valid_origin = function.blocks.get(invoke_block).is_some_and(|origin| {
                matches!(&origin.terminator, Some(MirInstruction::Invoke { normal_landing, .. })
                    if normal_landing == id)
            });
            if !valid_origin {
                errors.push(error(*id, "foreign-normal-result-origin"));
            }
            if index >= block.instructions.len()
                || block.instructions[..index]
                    .iter()
                    .any(|inst| !matches!(inst, MirInstruction::Phi { .. }))
            {
                errors.push(error(*id, "normal-result-not-first"));
            }
        }
    }
    // Never let verify_allow_no_phi make a Fault-edge result use admissible.
    let definitions = utils::compute_def_blocks(function);
    let dominators = utils::compute_dominators(function);
    if let Err(mut found) =
        dom::check_dominance_with_policy(function, &definitions, &dominators, false)
    {
        errors.append(&mut found);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The entry definition is the operand sort. Source MirType metadata cannot
/// promote an integer/handle into a frame, and frame residence cannot escape.
fn check_frame_entry(function: &MirFunction, errors: &mut Vec<VerificationError>) {
    let mut frames = Vec::new();
    for (id, block) in &function.blocks {
        for (index, inst) in block.all_instructions().enumerate() {
            if let MirInstruction::FaultFrameEnter { dst, .. } = inst {
                frames.push(*dst);
                if *id != function.entry_block
                    || index >= block.instructions.len()
                    || block.instructions[..index]
                        .iter()
                        .any(|i| !matches!(i, MirInstruction::Phi { .. }))
                {
                    errors.push(error(*id, "frame-entry-position"));
                }
                if function.params.contains(dst) || function.metadata.value_types.contains_key(dst)
                {
                    errors.push(error(*id, "frame-source-value"));
                }
            }
        }
        if block
            .successors_from_terminator()
            .contains(&function.entry_block)
        {
            errors.push(error(*id, "frame-entry-reentered"));
        }
    }
    if frames.len() != 1 {
        errors.push(error(function.entry_block, "frame-entry-count"));
        return;
    }
    let frame = frames[0];
    for (id, block) in &function.blocks {
        for inst in block.all_instructions() {
            let ordinary_uses = match inst {
                MirInstruction::Invoke {
                    operation,
                    fault_frame,
                    ..
                } => {
                    if *fault_frame != frame {
                        errors.push(error(*id, "foreign-frame-operand"));
                    }
                    operation.used_values()
                }
                MirInstruction::ReturnFault { fault_frame } => {
                    if *fault_frame != frame {
                        errors.push(error(*id, "foreign-frame-operand"));
                    }
                    Vec::new()
                }
                _ => inst.used_values(),
            };
            if ordinary_uses.contains(&frame) {
                errors.push(error(*id, "frame-escaped-as-source-value"));
            }
        }
        if block
            .return_env
            .as_ref()
            .is_some_and(|values| values.contains(&frame))
        {
            errors.push(error(*id, "frame-escaped-as-source-value"));
        }
    }
}

fn error(block: BasicBlockId, reason: &str) -> VerificationError {
    VerificationError::ControlFlowError {
        block,
        reason: format!("[freeze:contract][mir/invoke/{reason}]"),
    }
}

#[cfg(test)]
#[path = "invoke_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "invoke_array_tests.rs"]
mod array_tests;
