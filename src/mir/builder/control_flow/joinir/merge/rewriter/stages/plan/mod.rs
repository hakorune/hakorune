//! Stage 2: Plan - Transform plan into concrete rewritten blocks
//!
//! Phase 287 P3: Extracted from instruction_rewriter.rs::plan_rewrites()
//!
//! Generates new BasicBlocks based on the scan plan:
//! - Processes each function and block
//! - Filters instructions using InstructionFilterBox
//! - Converts terminators using ReturnConverterBox
//! - Builds parameter bindings using ParameterBindingBox
//! - Prepares exit PHI inputs and carrier inputs
//!
//! Updates RewriteContext but does NOT touch MirBuilder.

// Rewriter siblings (2 levels up: stages/ → rewriter/)
use super::super::{
    plan_helpers::{build_local_block_map, sync_spans},
    rewrite_context::RewriteContext,
    plan_box::RewrittenBlocks,
    helpers::is_skippable_continuation,
};

// Merge level (3 levels up: stages/ → rewriter/ → merge/)
use super::super::super::{
    loop_header_phi_info::LoopHeaderPhiInfo,
    trace,
};

// Crate-level imports
use crate::mir::{BasicBlock, MirModule, ValueId};
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use std::collections::{BTreeMap, BTreeSet, HashSet};

// Phase 287 P4: Import extracted modules
mod entry_resolver;
mod instruction_rewrite;
mod tail_call_rewrite;
mod terminator_rewrite;

/// Stage 2: Plan - Transform plan into concrete rewritten blocks
///
/// Generates new BasicBlocks based on the scan plan:
/// - Processes each function and block
/// - Filters instructions using InstructionFilterBox
/// - Converts terminators using ReturnConverterBox
/// - Builds parameter bindings using ParameterBindingBox
/// - Prepares exit PHI inputs and carrier inputs
///
/// Updates RewriteContext but does NOT touch MirBuilder.
///
/// # Phase 286C-4 Step 2
///
/// This function extracts ~550 lines from merge_and_rewrite():
/// - Function/block initialization (lines 399-468)
/// - First pass: instruction filtering (lines 581-760)
/// - Terminator conversion (lines 1163-1435)
/// - Span synchronization (lines 1436-1452)
///
/// # Phase 287 P5: Re-exported through stages/mod.rs
/// Access via stages::{plan_rewrites} for unified API.
pub(in crate::mir::builder::control_flow::joinir::merge) fn plan_rewrites(
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
    ctx: &mut RewriteContext,
    value_to_func_name: &BTreeMap<ValueId, String>,
    debug: bool,
) -> Result<RewrittenBlocks, String> {
    let trace = trace::trace();
    // Only verbose if explicitly requested via debug flag (not env var - causes test failures)
    let verbose = debug;
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    let mut result = RewrittenBlocks {
        new_blocks: Vec::new(),
        block_replacements: BTreeMap::new(),
        phi_inputs: Vec::new(),
        carrier_inputs: BTreeMap::new(),
    };

    // Phase 256 P1.7: Build continuation candidate set
    let continuation_candidates: BTreeSet<String> = boundary
        .map(|b| b.continuation_func_ids.clone())
        .unwrap_or_default();

    let skippable_continuation_func_names: BTreeSet<String> = mir_module
        .functions
        .iter()
        .filter_map(|(func_name, func)| {
            if continuation_candidates.contains(func_name) && is_skippable_continuation(func) {
                Some(func_name.clone())
            } else {
                None
            }
        })
        .collect();

    // Build boundary input set for filtering
    let boundary_input_set: HashSet<ValueId> = boundary
        .map(|b| b.join_inputs.iter().copied().collect())
        .unwrap_or_default();

    // Sort functions for deterministic iteration
    let mut functions_merge: Vec<_> = mir_module.functions.iter().collect();
    functions_merge.sort_by_key(|(name, _)| name.as_str());

    // Phase 287 P4: Resolve entry function using extracted module
    let entry_func_name_str = entry_resolver::resolve_entry_func_name(
        &functions_merge,
        boundary,
        &continuation_candidates,
    );
    let entry_func_name = entry_func_name_str.as_deref();

    // Process each function
    for (func_name, func) in functions_merge {
        let is_continuation_candidate = continuation_candidates.contains(func_name);
        let is_skippable_continuation = skippable_continuation_func_names.contains(func_name);

        if debug {
            log!(
                true,
                "[plan_rewrites] Processing function '{}' with {} blocks (continuation_candidate={}, skippable={})",
                func_name,
                func.blocks.len(),
                is_continuation_candidate,
                is_skippable_continuation
            );
        }

        // Skip structurally skippable continuation functions
        if is_skippable_continuation {
            if debug {
                log!(
                    true,
                    "[plan_rewrites] Skipping skippable continuation function '{}'",
                    func_name
                );
            }
            continue;
        }

        // Build local block map for this function
        let local_block_map = build_local_block_map(func_name, func, remapper)?;

        // Sort blocks for deterministic iteration
        let mut blocks_merge: Vec<_> = func.blocks.iter().collect();
        blocks_merge.sort_by_key(|(id, _)| id.0);

        // Determine if this function is the loop header (loop_step).
        let is_loop_header_func = entry_func_name == Some(func_name.as_str());

        // Check if loop header has PHIs
        let is_loop_header_with_phi =
            is_loop_header_func && !loop_header_phi_info.carrier_phis.is_empty();

        // Collect PHI dst IDs for this block (if loop header)
        let phi_dst_ids_for_block: HashSet<ValueId> =
            if is_loop_header_with_phi {
                loop_header_phi_info
                    .carrier_phis
                    .values()
                    .map(|entry| entry.phi_dst)
                    .collect()
            } else {
                HashSet::new()
            };

        // Process each block in the function
        for (old_block_id, old_block) in blocks_merge {
            let new_block_id = remapper
                .get_block(func_name, *old_block_id)
                .ok_or_else(|| format!("Block {:?} not found for {}", old_block_id, func_name))?;

            if debug {
                log!(
                    true,
                    "[plan_rewrites] Block mapping: func='{}' old={:?} → new={:?} (inst_count={})",
                    func_name, old_block_id, new_block_id, old_block.instructions.len()
                );
            }

            // Phase 287 P4: Initialize new block and process instructions
            let mut new_block = BasicBlock::new(new_block_id);

            // PHASE 2: Instruction rewriting (extracted)
            let is_loop_header_entry_block =
                is_loop_header_func && *old_block_id == func.entry_block;

            let (filtered_insts, tail_target) = instruction_rewrite::process_block_instructions(
                old_block,
                remapper,
                &local_block_map,
                loop_header_phi_info,
                is_loop_header_with_phi,
                is_loop_header_entry_block,
                &boundary_input_set,
                &phi_dst_ids_for_block,
                value_to_func_name,
                ctx,
                verbose,
            );
            new_block.instructions = filtered_insts;
            let found_tail_call = tail_target.is_some();
            let tail_call_target = tail_target;

            // PHASE 3: Tail call parameter binding (extracted)
            if let Some((target_block, ref args)) = tail_call_target {
                tail_call_rewrite::process_tail_call_params(
                    &mut new_block,
                    (target_block, args),
                    func_name,
                    func,
                    *old_block_id,
                    function_params,
                    entry_func_name,
                    &continuation_candidates,
                    is_loop_header_entry_block,
                    is_loop_header_with_phi,
                    boundary,
                    loop_header_phi_info,
                    remapper,
                    ctx,
                    new_block_id,
                    verbose,
                )?;
            }

            // Span synchronization
            new_block.instruction_spans = sync_spans(&new_block.instructions, old_block);

            // PHASE 4: Terminator rewriting (extracted)
            terminator_rewrite::process_block_terminator(
                &mut new_block,
                old_block,
                found_tail_call,
                tail_call_target.as_ref().map(|(b, v)| (*b, v.as_slice())),
                func_name,
                func,
                *old_block_id,
                new_block_id,
                remapper,
                &local_block_map,
                entry_func_name,
                &continuation_candidates,
                &skippable_continuation_func_names,
                is_continuation_candidate,
                is_skippable_continuation,
                boundary,
                loop_header_phi_info,
                ctx,
                &mut result,
                verbose,
            )?;

            // Add block to result
            result.new_blocks.push(new_block);
        }
    }

    if debug {
        log!(
            true,
            "[plan_rewrites] Generated {} new blocks",
            result.new_blocks.len()
        );
    }

    Ok(result)
}
