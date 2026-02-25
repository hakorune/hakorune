use crate::mir::function::MirFunction;
use crate::mir::verification::utils;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{HashMap, HashSet};

/// Verify CFG references and reachability
pub fn check_control_flow(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for (block_id, block) in &function.blocks {
        let expected_successors = block.successors_from_terminator();
        if expected_successors != block.successors {
            errors.push(VerificationError::ControlFlowError {
                block: *block_id,
                reason: format!(
                    "Successors cache mismatch: cached={:?}, expected={:?}",
                    block.successors, expected_successors
                ),
            });
        }
        for successor in &block.successors {
            if !function.blocks.contains_key(successor) {
                errors.push(VerificationError::ControlFlowError {
                    block: *block_id,
                    reason: format!("References non-existent block {}", successor),
                });
            }
        }
    }
    // Unreachable blocks are allowed in MIR.
    // They are created intentionally by break/continue/return statements via
    // switch_to_unreachable_block_with_void() to continue SSA construction after
    // control flow terminators. This is standard practice (see LLVM's `unreachable`).
    // Dead code elimination pass (TODO) will remove them during optimization.

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Verify that merge blocks do not use predecessor-defined values directly (must go through Phi)
pub fn check_merge_uses(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    if crate::config::env::verify_allow_no_phi() {
        return Ok(());
    }
    let mut errors = Vec::new();
    let preds = utils::compute_predecessors(function);
    let def_block = utils::compute_def_blocks(function);
    let dominators = utils::compute_dominators(function);
    check_merge_uses_with(function, &preds, &def_block, &dominators).map_err(|mut e| {
        errors.append(&mut e);
        errors
    })
}

pub fn check_merge_uses_with(
    function: &MirFunction,
    preds: &HashMap<BasicBlockId, Vec<BasicBlockId>>,
    def_block: &HashMap<ValueId, BasicBlockId>,
    dominators: &utils::DominatorTree,
) -> Result<(), Vec<VerificationError>> {
    if crate::config::env::verify_allow_no_phi() {
        return Ok(());
    }
    let mut errors = Vec::new();
    let mut phi_dsts_in_block: HashMap<BasicBlockId, HashSet<ValueId>> = HashMap::new();
    for (bid, block) in &function.blocks {
        let set = phi_dsts_in_block.entry(*bid).or_default();
        for sp in block.all_spanned_instructions() {
            if let crate::mir::MirInstruction::Phi { dst, .. } = sp.inst {
                set.insert(*dst);
            }
        }
    }
    for (bid, block) in &function.blocks {
        let Some(pred_list) = preds.get(bid) else {
            continue;
        };
        if pred_list.len() < 2 {
            continue;
        }
        let phi_dsts = phi_dsts_in_block.get(bid);
        for sp in block.all_spanned_instructions() {
            if let crate::mir::MirInstruction::Phi { .. } = sp.inst {
                continue;
            }
            for used in sp.inst.used_values() {
                if let Some(&db) = def_block.get(&used) {
                    if !dominators.dominates(db, *bid) {
                        let is_phi_dst = phi_dsts.map(|s| s.contains(&used)).unwrap_or(false);
                        if !is_phi_dst {
                            errors.push(VerificationError::MergeUsesPredecessorValue {
                                value: used,
                                merge_block: *bid,
                                pred_block: db,
                            });
                        }
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

/// Phase 257 P1-1: Verify PHI inputs reference actual CFG predecessors
///
/// Checks:
/// 1. Each PHI input references an actual CFG predecessor (no phantom predecessors)
/// 2. All reachable predecessors have corresponding PHI inputs (no missing inputs)
pub(super) fn check_phi_predecessors(
    function: &crate::mir::MirFunction,
) -> Result<(), Vec<crate::mir::verification_types::VerificationError>> {
    use crate::mir::verification::utils::compute_predecessors;
    use crate::mir::verification_types::VerificationError;
    use crate::mir::MirInstruction;
    use std::collections::HashSet;

    let mut errors = Vec::new();
    let preds = compute_predecessors(function);

    // Compute reachable blocks to filter out unreachable ones
    // (Unreachable blocks may have incomplete PHIs, which is OK)
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(function);

    for (block_id, block) in &function.blocks {
        // Skip unreachable blocks
        if !reachable.contains(block_id) {
            continue;
        }

        for instr in &block.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = instr {
                let expected_preds = match preds.get(block_id) {
                    Some(p) => p,
                    None => {
                        errors.push(VerificationError::InvalidPhi {
                            phi_value: *dst,
                            block: *block_id,
                            reason: format!("Block bb{} has PHI but no predecessors", block_id.0),
                        });
                        continue;
                    }
                };

                // Collect PHI input predecessor blocks
                let phi_input_preds: HashSet<_> = inputs.iter().map(|(bb, _)| *bb).collect();

                // Check 1: Each PHI input block is actually a predecessor (no phantom preds)
                for (pred_block, _value) in inputs {
                    if !expected_preds.contains(pred_block) {
                        errors.push(VerificationError::InvalidPhi {
                            phi_value: *dst,
                            block: *block_id,
                            reason: format!(
                                "PHI dst={:?} has input from non-predecessor bb{} (actual preds: {:?})",
                                dst, pred_block.0, expected_preds
                            ),
                        });
                    }
                }

                // Check 2: All reachable predecessors have PHI inputs (no missing inputs)
                // This is CRITICAL - catches the "no input for predecessor" runtime error
                for &expected_pred in expected_preds {
                    // Only check reachable predecessors
                    if reachable.contains(&expected_pred) && !phi_input_preds.contains(&expected_pred) {
                        errors.push(VerificationError::InvalidPhi {
                            phi_value: *dst,
                            block: *block_id,
                            reason: format!(
                                "PHI dst={:?} missing input from reachable predecessor bb{} (has inputs from: {:?})",
                                dst, expected_pred.0, phi_input_preds
                            ),
                        });
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
