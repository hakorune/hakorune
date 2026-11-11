use crate::mir::function::MirFunction;
use crate::mir::verification::utils;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, ValueId};

pub(super) fn check_dominance_by(
    function: &MirFunction,
    dominators: &utils::DominatorTree,
    definition_block: impl Fn(ValueId) -> Option<BasicBlockId>,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for (use_block_id, block) in &function.blocks {
        for sp in block.all_spanned_instructions() {
            if let crate::mir::MirInstruction::Phi { .. } = sp.inst {
                continue;
            }
            for used_value in sp.inst.used_values() {
                if let Some(def_bb) = definition_block(used_value) {
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
                let Some(def_bb) = definition_block(*incoming) else {
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
