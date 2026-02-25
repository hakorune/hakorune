use crate::mir::function::MirFunction;
use crate::mir::verification::utils;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::HashMap;

/// Verify dominance: def must dominate use across blocks (Phi inputs excluded)
pub fn check_dominance(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    if crate::config::env::verify_allow_no_phi() {
        return Ok(());
    }
    let mut errors = Vec::new();
    let def_block = utils::compute_def_blocks(function);
    let dominators = utils::compute_dominators(function);
    check_dominance_with(function, &def_block, &dominators).map_err(|mut e| {
        errors.append(&mut e);
        errors
    })
}

pub fn check_dominance_with(
    function: &MirFunction,
    def_block: &HashMap<ValueId, BasicBlockId>,
    dominators: &utils::DominatorTree,
) -> Result<(), Vec<VerificationError>> {
    if crate::config::env::verify_allow_no_phi() {
        return Ok(());
    }
    let mut errors = Vec::new();
    for (use_block_id, block) in &function.blocks {
        for sp in block.all_spanned_instructions() {
            if let crate::mir::MirInstruction::Phi { .. } = sp.inst {
                continue;
            }
            for used_value in sp.inst.used_values() {
                if let Some(&def_bb) = def_block.get(&used_value) {
                    if def_bb != *use_block_id {
                        if !dominators.dominates(def_bb, *use_block_id) {
                            errors.push(VerificationError::DominatorViolation {
                                value: used_value,
                                use_block: *use_block_id,
                                def_block: def_bb,
                            });
                        }
                    }
                }
            }
        }
    }

    // PHI inputs are semantically "used" on their predecessor edge. Ensure each PHI incoming
    // value dominates the predecessor block it is associated with; otherwise, a value can be
    // referenced on an edge where it is not available at runtime.
    for (_phi_block_id, block) in &function.blocks {
        for sp in block.all_spanned_instructions() {
            let crate::mir::MirInstruction::Phi { inputs, .. } = &sp.inst else {
                continue;
            };
            for (pred_bb, incoming) in inputs {
                let Some(&def_bb) = def_block.get(incoming) else {
                    continue;
                };
                if def_bb == *pred_bb {
                    continue;
                }
                if !dominators.dominates(def_bb, *pred_bb) {
                    errors.push(VerificationError::DominatorViolation {
                        value: *incoming,
                        use_block: *pred_bb,
                        def_block: def_bb,
                    });
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
