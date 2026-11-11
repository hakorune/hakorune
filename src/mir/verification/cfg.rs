use crate::mir::function::MirFunction;
use crate::mir::verification_types::VerificationError;
use crate::mir::BasicBlockId;
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
        for successor in &expected_successors {
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

/// Phase 257 P1-1: Verify PHI inputs reference actual CFG predecessors
///
/// Checks:
/// 1. Each PHI input references an actual CFG predecessor (no phantom predecessors)
/// 2. All reachable predecessors have corresponding PHI inputs (no missing inputs)
pub(super) fn check_phi_predecessors_with(
    function: &MirFunction,
    preds: &HashMap<BasicBlockId, Vec<BasicBlockId>>,
    reachable: &HashSet<BasicBlockId>,
) -> Result<(), Vec<VerificationError>> {
    use crate::mir::MirInstruction;
    let mut errors = Vec::new();
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
                    if reachable.contains(&expected_pred)
                        && !phi_input_preds.contains(&expected_pred)
                    {
                        let input_pairs: Vec<String> = inputs
                            .iter()
                            .map(|(pred, value)| format!("bb{}:%{}", pred.0, value.0))
                            .collect();
                        errors.push(VerificationError::InvalidPhi {
                            phi_value: *dst,
                            block: *block_id,
                            reason: format!(
                                "PHI dst={:?} missing input from reachable predecessor bb{} (has inputs from: {:?}; inputs=[{}])",
                                dst, expected_pred.0, phi_input_preds, input_pairs.join(",")
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
