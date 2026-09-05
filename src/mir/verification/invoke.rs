//! Structural Normal/Fault verification, independent of compatibility env flags.
//! Runtime admission remains closed until the common Fault ABI consumer lands.

use super::{cfg, dom, ssa, utils};
use crate::mir::instruction::InvokeOperation;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction};

pub(super) fn check_function(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let has_control = function.blocks.values().any(|block| {
        block.all_instructions().any(|inst| {
            matches!(
                inst,
                MirInstruction::Invoke { .. }
                    | MirInstruction::InvokeNormalResult { .. }
                    | MirInstruction::ReturnFault { .. }
            )
        })
    });
    if !has_control {
        return Ok(());
    }
    let mut errors = Vec::new();
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
                InvokeOperation::NewBox { .. } => true,
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

fn error(block: BasicBlockId, reason: &str) -> VerificationError {
    VerificationError::ControlFlowError {
        block,
        reason: format!("[freeze:contract][mir/invoke/{reason}]"),
    }
}
