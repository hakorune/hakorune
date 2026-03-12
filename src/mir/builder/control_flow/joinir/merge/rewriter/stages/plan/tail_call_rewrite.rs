//! Phase 287 P4: Tail call parameter binding logic
//!
//! Extracted from plan.rs lines 344-464
//!
//! Responsibilities:
//! - Classify tail calls (continuation, recursive, loop entry, normal)
//! - Insert parameter binding Copy instructions
//! - Record latch incoming for loop header PHI

// Import helpers from entry_resolver
use super::entry_resolver::resolve_target_func_name;

// Rewriter siblings (2 super:: up from plan/ to stages/, then 1 more to rewriter/)
use super::super::super::rewrite_context::RewriteContext;

// Merge level (3 super:: up from plan/ to stages/, then 1 more to rewriter/, then 1 more to merge/)
use super::super::super::super::{loop_header_phi_info::LoopHeaderPhiInfo, trace};

use crate::mir::builder::emission::copy_emitter::{self, CopyEmitReason};
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// Process tail call parameter bindings
///
/// Inserts Copy instructions to bind call arguments to target function parameters,
/// with special handling for:
/// - Loop entry blocks (skip bindings, PHIs define carriers)
/// - Recursive/loop entry calls with PHIs (update remapper only)
/// - Continuation calls (copy to original params)
/// - Normal tail calls (copy with header PHI dst checks)
///
/// Also records latch incoming for loop header PHI updates.
#[allow(clippy::too_many_arguments)]
pub(super) fn process_tail_call_params(
    new_block: &mut BasicBlock,
    tail_call_target: (BasicBlockId, &[ValueId]),
    func_name: &str,
    _func: &MirFunction,
    _old_block_id: BasicBlockId,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    entry_func_name: Option<&str>,
    continuation_candidates: &BTreeSet<String>,
    is_loop_header_entry_block: bool,
    is_loop_header_with_phi: bool,
    _boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
    remapper: &mut JoinIrIdRemapper,
    ctx: &RewriteContext,
    _new_block_id: BasicBlockId,
    verbose: bool,
) -> Result<(), String> {
    let trace_obj = trace::trace();
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace_obj.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    let (target_block, args) = tail_call_target;

    let target_func_name = resolve_target_func_name(&ctx.function_entry_map, target_block);

    // Check if target is continuation/recursive/loop entry
    let is_target_continuation = target_func_name
        .map(|name| continuation_candidates.contains(name))
        .unwrap_or(false);

    let is_recursive_call = target_func_name
        .map(|name| name == func_name)
        .unwrap_or(false);

    // Phase 188.3: Define is_target_loop_entry early for latch incoming logic
    let is_target_loop_entry = target_func_name
        .map(|name| entry_func_name == Some(name))
        .unwrap_or(false);

    if let Some(target_func_name) = target_func_name {
        if let Some(target_params) = function_params.get(target_func_name) {
            log!(
                verbose,
                "[plan_rewrites] Tail call param binding: from='{}' to='{}' (recursive={}, loop_entry={}, continuation={})",
                func_name, target_func_name, is_recursive_call, is_target_loop_entry, is_target_continuation
            );

            // Skip parameter binding in specific cases:
            // 1. Loop entry point (header PHIs define carriers)
            // 2. Recursive/entry call to loop header with PHIs (latch edge)
            // 3. Continuation call (handled separately below)
            // Phase 287 P1: Skip ONLY when target is loop header
            // (not when source is entry func but target is non-entry like inner_step)
            if is_loop_header_entry_block && is_target_loop_entry {
                log!(
                    verbose,
                    "[plan_rewrites] Skip param bindings in header block (PHIs define carriers)"
                );
            } else if (is_recursive_call || is_target_loop_entry)
                && is_loop_header_with_phi
                && is_loop_header_entry_block
            {
                // Update remapper mappings for continuation instructions
                for (i, arg_val_remapped) in args.iter().enumerate() {
                    if i < target_params.len() {
                        let param_val_original = target_params[i];
                        remapper.set_value(param_val_original, *arg_val_remapped);
                    }
                }
                log!(
                    verbose,
                    "[plan_rewrites] Skip Copy bindings for {} call (remapper updated)",
                    if is_recursive_call {
                        "recursive"
                    } else {
                        "entry"
                    }
                );
            } else if is_target_continuation {
                // Continuation call: Copy args to original params
                for (i, arg_val_remapped) in args.iter().enumerate() {
                    if i < target_params.len() {
                        let param_val_original = target_params[i];
                        copy_emitter::emit_copy_into_detached_block(
                            new_block,
                            param_val_original,
                            *arg_val_remapped,
                            CopyEmitReason::JoinIrMergeRewriterTailCallParamsContinuation,
                        )?;
                        log!(
                            verbose,
                            "[plan_rewrites] Continuation param binding: {:?} = copy {:?}",
                            param_val_original,
                            arg_val_remapped
                        );
                    }
                }
            } else {
                // Normal tail call: Insert Copy instructions
                for (i, arg_val_remapped) in args.iter().enumerate() {
                    if i < target_params.len() {
                        let param_val_original = target_params[i];
                        let param_remap_result = remapper.get_value(param_val_original);
                        let param_val_dst = param_remap_result.unwrap_or(param_val_original);

                        // Check if this would overwrite a header PHI dst
                        let is_header_phi_dst = loop_header_phi_info
                            .carrier_phis
                            .values()
                            .any(|entry| entry.phi_dst == param_val_dst);

                        // Phase 29ae: Do not emit Copy that defines header-PHI dsts.
                        //
                        // These PHI dsts must be defined only by header PHIs (at the loop header),
                        // not by preheader (LoopEntry) or by the header entry block itself.
                        //
                        // This prevents SSA double-defs and avoids "undefined value" copies when
                        // the LoopEntry preheader tries to bind carrier params via remapped PHI ids.
                        // LoopEntry (host → loop_step) must NOT define header-PHI dsts.
                        // BackEdge (loop_step → loop_step) is allowed to define them (latch update).
                        let is_loop_entry_source = is_target_loop_entry && !is_recursive_call;
                        if is_header_phi_dst && (is_loop_header_entry_block || is_loop_entry_source)
                        {
                            log!(
                                verbose,
                                "[plan_rewrites] Skip param binding to PHI dst {:?}",
                                param_val_dst
                            );
                            continue;
                        }

                        copy_emitter::emit_copy_into_detached_block(
                            new_block,
                            param_val_dst,
                            *arg_val_remapped,
                            CopyEmitReason::JoinIrMergeRewriterTailCallParamsTailCall,
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}
