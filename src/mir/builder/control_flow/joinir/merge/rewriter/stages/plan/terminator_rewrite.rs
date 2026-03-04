//! Phase 287 P4: Terminator conversion and routing logic
//!
//! Extracted from plan.rs lines 469-727
//!
//! Responsibilities:
//! - Convert Return to Jump (with exit args collection)
//! - Remap Jump/Branch terminators
//! - Route tail calls (BackEdge → header, LoopEntry → target, ExitJump → exit/target)
//! - Collect carrier inputs for exit jumps

// Import helpers from entry_resolver
use super::entry_resolver::resolve_target_func_name;

// Rewriter siblings (2 super:: up from plan/ to stages/, then 1 more to rewriter/)
use super::super::super::{
    rewrite_context::RewriteContext,
    plan_box::RewrittenBlocks,
    return_converter_box::ReturnConverterBox,
    carrier_inputs_collector::CarrierInputsCollector,
    tail_call_policy,
    terminator::{remap_branch, remap_jump},
};

// Merge level (3 super:: up from plan/ to stages/, then 1 more to rewriter/, then 1 more to merge/)
use super::super::super::super::{
    tail_call_classifier::{classify_tail_call, TailCallKind},
    exit_args_collector::ExitArgsCollectorBox,
    loop_header_phi_info::LoopHeaderPhiInfo,
    trace,
};

use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, MirFunction, ValueId};
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use std::collections::{BTreeMap, BTreeSet};

/// Process block terminator: convert Return, remap Jump/Branch, or insert tail call Jump
///
/// Handles:
/// - Return → Jump conversion (with exit args collection)
/// - Jump/Branch remapping (block ID updates)
/// - Tail call routing:
///   - BackEdge → header block
///   - LoopEntry → target block
///   - ExitJump → exit_block_id (if skippable) or target_block
///
/// CRITICAL: CarrierInputsCollector calls at lines 574, 696 - DO NOT MOVE
#[allow(clippy::too_many_arguments)]
pub(super) fn process_block_terminator(
    new_block: &mut BasicBlock,
    old_block: &BasicBlock,
    found_tail_call: bool,
    tail_call_target: Option<(BasicBlockId, &[ValueId])>,
    func_name: &str,
    func: &MirFunction,
    old_block_id: BasicBlockId,
    new_block_id: BasicBlockId,
    remapper: &JoinIrIdRemapper,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
    entry_func_name: Option<&str>,
    continuation_candidates: &BTreeSet<String>,
    skippable_continuation_func_names: &BTreeSet<String>,
    is_continuation_candidate: bool,
    is_skippable_continuation: bool,
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
    ctx: &RewriteContext,
    result: &mut RewrittenBlocks,
    verbose: bool,
) -> Result<(), String> {
    let trace_obj = trace::trace();
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace_obj.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    // Terminator conversion
    if !found_tail_call {
        if let Some(ref term) = old_block.terminator {
            match term {
                MirInstruction::Return { value } => {
                    // Check if we should keep Return or convert to Jump
                    if ReturnConverterBox::should_keep_return(is_continuation_candidate, is_skippable_continuation) {
                        // Non-skippable continuation: keep Return
                        let remapped_value = ReturnConverterBox::remap_return_value(*value, |v| remapper.remap_value(v));
                        new_block.set_terminator(MirInstruction::Return { value: remapped_value });
                        log!(
                            verbose,
                            "[plan_rewrites] Keeping Return for non-skippable continuation '{}' (value={:?})",
                            func_name, remapped_value
                        );
                    } else {
                        // Convert Return to Jump to exit block
                        let mut exit_edge_args: Option<crate::mir::EdgeArgs> = None;
                        if value.is_some() {
                            if let Some(b) = boundary {
                                // Use terminator edge-args from old block
                                if let Some(edge_args) = old_block.edge_args_from_terminator() {
                                    if edge_args.layout != b.jump_args_layout {
                                        let msg = format!(
                                            "[plan_rewrites] exit edge-args layout mismatch: block={:?} edge={:?} boundary={:?}",
                                            old_block.id, edge_args.layout, b.jump_args_layout
                                        );
                                        if ctx.strict_exit {
                                            return Err(msg);
                                        } else if verbose {
                                            log!(true, "[DEBUG] {}", msg);
                                        }
                                    }

                                    // Remap jump_args to HOST value space
                                    let remapped_args: Vec<ValueId> = edge_args
                                        .values
                                        .iter()
                                        .map(|&arg| remapper.remap_value(arg))
                                        .collect();

                                    log!(
                                        verbose,
                                        "[plan_rewrites] Remapped exit jump_args: {:?}",
                                        remapped_args
                                    );

                                    // Collect exit values using ExitArgsCollectorBox
                                    let edge_args = crate::mir::EdgeArgs {
                                        layout: edge_args.layout,
                                        values: remapped_args,
                                    };
                                    exit_edge_args = Some(edge_args.clone());

                                    let collector = ExitArgsCollectorBox::new();
                                    let collection_result = collector.collect(
                                        &b.exit_bindings,
                                        &edge_args.values,
                                        new_block_id,
                                        ctx.strict_exit,
                                        edge_args.layout,
                                    )?;

                                    // Add expr_result to exit_phi_inputs
                                    if let Some(expr_result_val) = collection_result.expr_result_value {
                                        result
                                            .phi_inputs
                                            .push((collection_result.block_id, expr_result_val));
                                        log!(
                                            verbose,
                                            "[plan_rewrites] exit_phi_inputs: ({:?}, {:?})",
                                            collection_result.block_id, expr_result_val
                                        );
                                    }

                                    // Add carrier values to carrier_inputs
                                    for (carrier_name, (block_id, value_id)) in collection_result.carrier_values {
                                        log!(
                                            verbose,
                                            "[plan_rewrites] Collecting carrier '{}': from {:?} value {:?}",
                                            carrier_name, block_id, value_id
                                        );
                                        result.carrier_inputs
                                            .entry(carrier_name)
                                            .or_insert_with(Vec::new)
                                            .push((block_id, value_id));
                                    }
                                } else {
                                    // Fallback: Collect return value directly if present
                                    // NOTE: value が Some の場合のみ phi_inputs に追加し、header PHI fallback は使わない（二重 push 回避）
                                    log!(
                                        verbose,
                                        "[plan_rewrites] Block {:?} has NO jump_args, using fallback",
                                        old_block.id
                                    );

                                    if let Some(ret_val) = value {
                                        // ReturnConverterBox を使って既存契約に揃える
                                        let remapped_val = ReturnConverterBox::remap_return_value(Some(*ret_val), |v| remapper.remap_value(v));
                                        if let Some(val) = remapped_val {
                                            result.phi_inputs.push((new_block_id, val));
                                            log!(
                                                verbose,
                                                "[plan_rewrites] Using Return value {:?} (remapped to {:?}) for exit PHI",
                                                ret_val, val
                                            );
                                        }
                                    } else if let Some(loop_var_name) = &b.loop_var_name {
                                        // value が None の場合のみ header PHI fallback を使用
                                        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
                                            result.phi_inputs.push((new_block_id, phi_dst));
                                            log!(
                                                verbose,
                                                "[plan_rewrites] Using header PHI dst {:?} for exit (loop_var='{}')",
                                                phi_dst, loop_var_name
                                            );
                                        }
                                    }

                                    // Phase 286C-5 Step 1: Use CarrierInputsCollector to eliminate duplication
                                    // CRITICAL: Do not move this call - exact location matters (line 574 equivalent)
                                    let collector = CarrierInputsCollector::new(b, loop_header_phi_info);
                                    let carrier_inputs = collector.collect(new_block_id);
                                    for (carrier_name, block_id, value_id) in carrier_inputs {
                                        result.carrier_inputs
                                            .entry(carrier_name.clone())
                                            .or_insert_with(Vec::new)
                                            .push((block_id, value_id));
                                        log!(
                                            verbose,
                                            "[plan_rewrites] Carrier '{}': from {:?} value {:?}",
                                            carrier_name, block_id, value_id
                                        );
                                    }
                                }
                            }
                        }

                        // Set Jump terminator with edge args
                        if let Some(edge_args) = exit_edge_args {
                            new_block.set_jump_with_edge_args(ctx.exit_block_id, Some(edge_args));
                        } else {
                            new_block.set_terminator(MirInstruction::Jump {
                                target: ctx.exit_block_id,
                                edge_args: None,
                            });
                        }
                    }
                }
                MirInstruction::Jump { target, edge_args } => {
                    let remapped_term = remap_jump(
                        remapper,
                        *target,
                        edge_args,
                        &ctx.skipped_entry_redirects,
                        local_block_map,
                    );
                    new_block.set_terminator(remapped_term);
                }
                MirInstruction::Branch {
                    condition,
                    then_bb,
                    else_bb,
                    then_edge_args,
                    else_edge_args,
                } => {
                    let remapped_term = remap_branch(
                        remapper,
                        *condition,
                        *then_bb,
                        *else_bb,
                        then_edge_args,
                        else_edge_args,
                        &ctx.skipped_entry_redirects,
                        local_block_map,
                    );
                    new_block.set_terminator(remapped_term);
                }
                _ => {
                    let remapped = remapper.remap_instruction(term);
                    new_block.set_terminator(remapped);
                }
            }
        }
    } else if let Some((target_block, args)) = tail_call_target {
        // Tail call: Set Jump terminator
        // Classify tail call and determine actual target
        let target_func_name = resolve_target_func_name(&ctx.function_entry_map, target_block);

        let is_target_continuation = target_func_name
            .map(|name| continuation_candidates.contains(name))
            .unwrap_or(false);

        // Phase 287 P2: Compute is_target_loop_entry for classify_tail_call
        let is_target_loop_entry = target_func_name
            .map(|name| entry_func_name == Some(name))
            .unwrap_or(false);

        // Phase 287 P2: host entry block からの呼び出しを LoopEntry 扱いにする
        // (loop header func の entry block は含めない)
        let is_entry_like_block =
            tail_call_policy::is_loop_entry_source(func_name, old_block_id, func.entry_block);

        // CRITICAL: Argument order must match merge level classify_tail_call()
        let tail_call_kind = classify_tail_call(
            is_entry_like_block,
            !loop_header_phi_info.carrier_phis.is_empty(),
            boundary.is_some(),
            is_target_continuation,
            is_target_loop_entry,
        );

        // SSOT: record latch incoming only when BackEdge is confirmed
        tail_call_policy::record_latch_incoming_if_backedge(
            tail_call_kind,
            boundary,
            new_block_id,
            args,
            new_block,
            loop_header_phi_info,
        );

        let actual_target = match tail_call_kind {
            TailCallKind::BackEdge => {
                log!(
                    verbose,
                    "[plan_rewrites] BackEdge: redirecting from {:?} to header {:?}",
                    target_block, loop_header_phi_info.header_block
                );
                loop_header_phi_info.header_block
            }
            TailCallKind::LoopEntry => {
                log!(
                    verbose,
                    "[plan_rewrites] LoopEntry: using direct target {:?}",
                    target_block
                );
                target_block
            }
            TailCallKind::ExitJump => {
                // Check if target is skippable continuation
                let is_target_skippable = resolve_target_func_name(&ctx.function_entry_map, target_block)
                    .map(|name| skippable_continuation_func_names.contains(name))
                    .unwrap_or(false);

                if is_target_skippable {
                    log!(
                        verbose,
                        "[plan_rewrites] ExitJump (skippable): redirecting from {:?} to exit_block_id {:?}",
                        target_block, ctx.exit_block_id
                    );

                    // Phase 286C-5 Step 1: Use CarrierInputsCollector to eliminate duplication
                    // This replaces Phase 286C-4.1 inline code
                    // CRITICAL: Do not move this call - exact location matters (line 696 equivalent)
                    if let Some(b) = boundary {
                        let collector = CarrierInputsCollector::new(b, loop_header_phi_info);
                        let carrier_inputs = collector.collect(new_block_id);
                        for (carrier_name, block_id, value_id) in carrier_inputs {
                            result.carrier_inputs
                                .entry(carrier_name.clone())
                                .or_insert_with(Vec::new)
                                .push((block_id, value_id));
                            log!(
                                verbose,
                                "[plan_rewrites] ExitJump carrier '{}': from {:?} value {:?}",
                                carrier_name, block_id, value_id
                            );
                        }
                    }

                    ctx.exit_block_id
                } else {
                    log!(
                        verbose,
                        "[plan_rewrites] ExitJump (non-skippable): to target_block {:?}",
                        target_block
                    );
                    target_block
                }
            }
        };

        new_block.set_terminator(MirInstruction::Jump {
            target: actual_target,
            edge_args: None,
        });
    }

    Ok(())
}
