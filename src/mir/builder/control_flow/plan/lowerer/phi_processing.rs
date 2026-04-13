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

fn compact_error_for_freeze(original_error: &str) -> String {
    if original_error.starts_with("[freeze:")
        && !original_error.contains('\n')
        && !original_error.contains('\r')
    {
        original_error.to_string()
    } else {
        let truncated: String = original_error.chars().take(80).collect();
        if original_error.chars().count() > 80 {
            format!("{}...", truncated)
        } else {
            truncated
        }
    }
}

fn rollback_unpatched_provisional_phis(
    builder: &mut MirBuilder,
    unpatched: &[(ValueId, BasicBlockId, String)],
    rollback_tag_prefix: &str,
) -> (usize, Option<String>) {
    let mut rollback_count = 0usize;
    for (phi_dst, phi_bb, phi_tag) in unpatched {
        match phi_lifecycle::rollback_provisional_phi(
            builder,
            *phi_bb,
            *phi_dst,
            &format!("{}:{}", rollback_tag_prefix, phi_tag),
        ) {
            Ok(removed) => {
                if removed {
                    rollback_count += 1;
                }
            }
            Err(e) => {
                return (
                    rollback_count,
                    Some(format!(
                        "dst=%{} bb={:?} tag={} rollback_err={}",
                        phi_dst.0, phi_bb, phi_tag, e
                    )),
                );
            }
        }
    }
    (rollback_count, None)
}

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

                // LoopTrueBreak routes seed after-merge PHIs with a placeholder
                // predecessor before concrete break exits are known. When every
                // real break exit comes from different predecessors, drop the
                // placeholder input before merging to avoid non-predecessor PHIs.
                if phi.tag.starts_with("loop_true_after_")
                    && !inputs_by_pred.is_empty()
                    && !phi
                        .inputs
                        .iter()
                        .any(|(pred, _)| inputs_by_pred.contains_key(pred))
                {
                    phi.inputs.clear();
                }

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
pub fn patch_all_phis(builder: &mut MirBuilder, loop_plan: &CoreLoopPlan) -> Result<(), String> {
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
        if is_unreferenced_empty_step_phi {
            phi_lifecycle::rollback_provisional_phi(
                builder,
                phi.block,
                phi.dst,
                &format!("loop_lowerer:step4:drop_unused:{}", phi.tag),
            )?;
            continue;
        }

        phi_lifecycle::patch_phi_inputs(
            builder,
            phi.block,
            phi.dst,
            inputs,
            &format!("loop_lowerer:step4:{}", phi.tag),
        )?;
    }

    Ok(())
}

/// Validate no unpatched provisional PHIs remain
///
/// This is a fail-fast check to ensure that all provisional PHIs created in Step 1.5
/// were successfully patched in Step 4. If any remain unpatched when an error occurs,
/// we detect this immediately instead of allowing the error to propagate downstream.
///
/// This check should only run in strict/dev+planner_required mode.
pub fn validate_no_unpatched_phis(
    builder: &mut MirBuilder,
    provisional_phis: &[(ValueId, BasicBlockId, String)],
    original_error: &str,
) -> Result<(), String> {
    use crate::config::env::joinir_dev;

    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    if strict_or_dev && joinir_dev::planner_required_enabled() {
        let (fn_name, unpatched) = if let Some(func) = builder.scope_ctx.current_function.as_ref() {
            let mut unpatched: Vec<(ValueId, BasicBlockId, String)> = Vec::new();
            for &(phi_dst, phi_bb, ref phi_tag) in provisional_phis {
                let Some(block) = func.get_block(phi_bb) else {
                    continue;
                };
                let has_empty_inputs = block.instructions.iter().any(|inst| {
                    matches!(inst, crate::mir::MirInstruction::Phi { dst, inputs, .. }
                        if *dst == phi_dst && inputs.is_empty())
                });
                if has_empty_inputs {
                    unpatched.push((phi_dst, phi_bb, phi_tag.clone()));
                }
            }
            (func.signature.name.clone(), unpatched)
        } else {
            return Ok(());
        };

        if !unpatched.is_empty() {
            let (rollback_count, rollback_err) =
                rollback_unpatched_provisional_phis(builder, &unpatched, "loop_lowerer:rollback");

            let reason = if rollback_err.is_some() {
                "rollback_failed"
            } else {
                "err_before_patch"
            };
            let orig_err = if let Some(ref rollback_err) = rollback_err {
                compact_error_for_freeze(rollback_err)
            } else {
                compact_error_for_freeze(original_error)
            };

            let (phi_dst, phi_bb, phi_tag) = &unpatched[0];
            return Err(format!(
                "[freeze:contract][phi_lifecycle/provisional_left_unpatched] fn={} phi_dst=%{} phi_bb={:?} tag={} orig_err=<{}> reason={} rollback_count={}",
                fn_name, phi_dst.0, phi_bb, phi_tag, orig_err, reason, rollback_count
            ));
        }
    }

    Ok(())
}

/// Validate no unpatched provisional PHIs remain after patching (success path).
///
/// This catches "we returned Ok but forgot to patch some provisional PHI" early.
pub fn validate_no_unpatched_phis_after_patch(
    builder: &mut MirBuilder,
    provisional_phis: &[(ValueId, BasicBlockId, String)],
) -> Result<(), String> {
    use crate::config::env::joinir_dev;

    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    if !(strict_or_dev && joinir_dev::planner_required_enabled()) {
        return Ok(());
    }

    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return Ok(());
    };

    let fn_name = func.signature.name.clone();
    let mut unpatched: Vec<(ValueId, BasicBlockId, String)> = Vec::new();
    for &(phi_dst, phi_bb, ref phi_tag) in provisional_phis {
        let Some(block) = func.get_block(phi_bb) else {
            continue;
        };
        let has_empty_inputs = block.instructions.iter().any(|inst| {
            matches!(inst, crate::mir::MirInstruction::Phi { dst, inputs, .. }
                if *dst == phi_dst && inputs.is_empty())
        });
        if has_empty_inputs {
            unpatched.push((phi_dst, phi_bb, phi_tag.clone()));
        }
    }

    if !unpatched.is_empty() {
        let (rollback_count, rollback_err) =
            rollback_unpatched_provisional_phis(builder, &unpatched, "loop_lowerer:rollback_ok");
        let reason = if rollback_err.is_some() {
            "rollback_failed"
        } else {
            "ok_after_patch"
        };
        let orig_err = rollback_err
            .as_deref()
            .map(compact_error_for_freeze)
            .unwrap_or_else(|| "none".to_string());

        let (phi_dst, phi_bb, phi_tag) = &unpatched[0];
        return Err(format!(
            "[freeze:contract][phi_lifecycle/provisional_left_unpatched] fn={} phi_dst=%{} phi_bb={:?} tag={} orig_err=<{}> reason={} rollback_count={}",
            fn_name, phi_dst.0, phi_bb, phi_tag, orig_err, reason, rollback_count
        ));
    }

    Ok(())
}
