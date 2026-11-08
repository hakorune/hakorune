//! Loop PHI Processing Utilities
//!
//! This module contains PHI lifecycle management logic for loop lowering.
//! Phase 29bq+: Extracted from loop_lowering.rs for better modularity.
//!
//! Responsibilities:
//! - Step 3.5: Merge deferred PHI inputs from loop frame
//! - Step 4: Patch PHI inputs
//! - Error path: Validate no unpatched provisional PHIs

use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::emission::phi_lifecycle;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::HashMap;

use super::LoopFrame;

/// Step 3.5: Merge deferred PHI inputs into continue_target PHIs
///
/// This function handles the case where continue statements may have accumulated
/// PHI inputs that need to be merged into the loop's continue_target PHIs.
/// When debug_enabled is true, includes detailed eprintln! output with def_blocks.
pub fn merge_deferred_phi_inputs(
    loop_plan: &mut CoreLoopPlan,
    loop_stack: &mut Vec<LoopFrame>,
    debug_enabled: bool,
    fn_name: Option<&str>,
    debug_def_blocks: Option<&HashMap<ValueId, BasicBlockId>>,
) -> Result<(), String> {
    // Step 3.5: Merge deferred continue-phi inputs into continue_target PHIs (strict/dev helper).
    // This supports multi-continue edges where the "next" carrier values differ per edge.
    if let Some(frame) = loop_stack.last_mut() {
        // Merge step_phi_inputs (from continue statements)
        if !frame.step_phi_inputs.is_empty() {
            let merge_target = loop_plan.continue_target;
            for phi in loop_plan.phis.iter_mut() {
                if phi.block != merge_target {
                    continue;
                }

                let Some(inputs_by_pred) = frame.step_phi_inputs.get(&phi.dst) else {
                    continue;
                };

                // Generic-loop seeded step PHIs start with a body_bb default input.
                // If deferred inputs come only from different predecessors, drop the
                // seeded body input before merging to avoid non-predecessor PHI entries.
                if phi.tag.starts_with("loop_step_in_")
                    && !inputs_by_pred.is_empty()
                    && !inputs_by_pred.contains_key(&loop_plan.body_bb)
                {
                    phi.inputs.retain(|(pred, _)| *pred != loop_plan.body_bb);
                }

                for (pred, val) in inputs_by_pred {
                    if !phi.inputs.iter().any(|(p, _)| p == pred) {
                        if debug_enabled {
                            let def_bb = debug_def_blocks.and_then(|m| m.get(val).copied());
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "{} fn={} origin=loop_lowering_merge pred_bb={:?} dst=%{} incoming=%{} incoming_def_bb={:?} phi_tag={}",
                                super::debug_tags::TAG_STEP_PHI_INPUT_ADD,
                                fn_name.unwrap_or("<none>"),
                                pred,
                                phi.dst.0,
                                val.0,
                                def_bb,
                                phi.tag
                            ));
                        }
                        phi.inputs.push((*pred, *val));
                    }
                }
            }
        }

        // Merge break_phi_inputs (from break statements)
        if !frame.break_phi_inputs.is_empty() {
            let merge_target = loop_plan.after_bb;

            for phi in loop_plan.phis.iter_mut() {
                if phi.block != merge_target {
                    continue;
                }

                let Some(inputs_by_pred) = frame.break_phi_inputs.get(&phi.dst) else {
                    continue;
                };

                // LoopTrueBreak routes seed after-merge PHIs with the structural
                // header->after edge. Keep that seed: even for `loop(true)` the
                // false branch remains a real CFG predecessor of after_bb.
                // Concrete break exits are appended below.

                for (pred, val) in inputs_by_pred {
                    if !phi.inputs.iter().any(|(p, _)| p == pred) {
                        phi.inputs.push((*pred, *val));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Step 4: Update PHI inputs (patch provisional PHIs)
///
/// PHIs were inserted provisionally in Step 1.5 with empty inputs.
/// Now update them with the actual inputs merged in Step 3.5.
pub(super) fn patch_all_phis_with_transaction(
    builder: &mut MirBuilder, loop_plan: &CoreLoopPlan,
    transaction: &mut phi_lifecycle::PhiTxn,
) -> Result<(), String> {
    for phi in &loop_plan.phis {
        let inputs = phi.inputs.clone();

        // Some generic-loop routes can leave a provisional loop_step_in_* PHI
        // structurally present but unused (no predecessor-driven inputs).
        // When it is not referenced by any other PHI input, remove it eagerly
        // instead of leaving an empty provisional PHI to trip strict gate checks.
        let is_unreferenced_empty_step_phi = inputs.is_empty()
            && phi.tag.starts_with("loop_step_in_")
            && !loop_plan.phis.iter().any(|other| {
                other
                    .inputs
                    .iter()
                    .any(|(_, incoming)| *incoming == phi.dst)
            });
        let token = transaction
            .pending_tokens_v1()
            .find_map(|(block, token)| {
                (block == phi.block && token.dst() == phi.dst).then_some(token)
            })
            .ok_or_else(|| "[freeze:contract][source-loop-phi/patch-token-missing]".to_owned())?;
        if is_unreferenced_empty_step_phi {
            transaction.rollback_pending_phi(
                builder,
                token,
                &format!("loop_lowerer:step4:drop_unused:{}", phi.tag),
            )?;
        } else {
            transaction.patch_phi_inputs(
                builder,
                token,
                inputs,
                &format!("loop_lowerer:step4:{}", phi.tag),
            )?;
        }
    }

    Ok(())
}
