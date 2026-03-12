//! Phase 287 P4: Instruction filtering and remapping logic
//!
//! Extracted from plan.rs lines 228-342
//!
//! Responsibilities:
//! - Filter instructions (skip PHI overwrites, function name consts, boundary input consts)
//! - Detect tail calls (intra-module Call instructions)
//! - Remap instruction ValueIds and BlockIds
//! - Remap Branch/Phi block references

// Rewriter siblings (2 super:: up from plan/ to stages/, then 1 more to rewriter/)
use super::super::super::{
    instruction_filter_box::InstructionFilterBox, rewrite_context::RewriteContext,
};

// Merge level (3 super:: up from plan/ to stages/, then 1 more to rewriter/, then 1 more to merge/)
use super::super::super::super::{
    block_remapper::remap_block_id, loop_header_phi_info::LoopHeaderPhiInfo,
    phi_block_remapper::remap_phi_instruction, trace,
};

use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Process block instructions: filter, remap, detect tail calls
///
/// Returns (filtered_instructions, tail_call_target)
pub(super) fn process_block_instructions(
    old_block: &BasicBlock,
    remapper: &mut JoinIrIdRemapper,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
    loop_header_phi_info: &LoopHeaderPhiInfo,
    is_loop_header_with_phi: bool,
    is_loop_header_entry_block: bool,
    boundary_input_set: &std::collections::HashSet<ValueId>,
    phi_dst_ids_for_block: &std::collections::HashSet<ValueId>,
    value_to_func_name: &BTreeMap<ValueId, String>,
    ctx: &RewriteContext,
    verbose: bool,
) -> (Vec<MirInstruction>, Option<(BasicBlockId, Vec<ValueId>)>) {
    let trace_obj = trace::trace();
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace_obj.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    let mut instructions = Vec::new();
    let mut tail_call_target: Option<(BasicBlockId, Vec<ValueId>)> = None;

    // First pass: Filter instructions
    for inst in &old_block.instructions {
        // Skip Copy instructions that overwrite PHI dsts
        if is_loop_header_with_phi && is_loop_header_entry_block {
            if let MirInstruction::Copy { dst, src } = inst {
                let dst_remapped = remapper.get_value(*dst).unwrap_or(*dst);
                let is_boundary_input = boundary_input_set.contains(src);
                if is_boundary_input
                    && InstructionFilterBox::should_skip_copy_overwriting_phi(
                        dst_remapped,
                        phi_dst_ids_for_block,
                    )
                {
                    log!(
                        verbose,
                        "[plan_rewrites] Skipping loop header Copy to PHI dst {:?}",
                        dst_remapped
                    );
                    continue;
                }
            }
        }

        // Skip function name Const String instructions
        if let MirInstruction::Const { dst, value } = inst {
            if InstructionFilterBox::should_skip_function_name_const(value)
                && value_to_func_name.contains_key(dst)
            {
                log!(
                    verbose,
                    "[plan_rewrites] Skipping function name const: {:?}",
                    inst
                );
                continue;
            }

            // Skip boundary input Const instructions
            let boundary_inputs: Vec<ValueId> = boundary_input_set.iter().cloned().collect();
            if InstructionFilterBox::should_skip_boundary_input_const(
                *dst,
                &boundary_inputs,
                is_loop_header_entry_block,
            ) {
                log!(
                    verbose,
                    "[plan_rewrites] Skipping boundary input const: {:?}",
                    inst
                );
                continue;
            }
        }

        // Detect tail calls
        if let MirInstruction::Call { func, args, .. } = inst {
            if let Some(callee_name) = value_to_func_name.get(func) {
                if let Some(&target_block) = ctx.function_entry_map.get(callee_name) {
                    // This is a tail call
                    let remapped_args: Vec<ValueId> = args
                        .iter()
                        .map(|&v| remapper.get_value(v).unwrap_or(v))
                        .collect();
                    tail_call_target = Some((target_block, remapped_args));

                    log!(
                        verbose,
                        "[plan_rewrites] Detected tail call to '{}' (args={:?})",
                        callee_name,
                        args
                    );
                    continue; // Skip the Call instruction itself
                }
            }
        }

        // Skip Copy instructions that overwrite header PHI dsts
        if is_loop_header_entry_block {
            if let MirInstruction::Copy { dst, src } = inst {
                let remapped_dst = remapper.get_value(*dst).unwrap_or(*dst);
                let is_header_phi_dst = loop_header_phi_info
                    .carrier_phis
                    .values()
                    .any(|entry| entry.phi_dst == remapped_dst);
                let is_boundary_input = boundary_input_set.contains(src);

                if is_header_phi_dst && is_boundary_input {
                    log!(
                        verbose,
                        "[plan_rewrites] Skipping Copy that overwrites header PHI dst {:?}",
                        remapped_dst
                    );
                    continue;
                }
            }
        }

        // Remap instruction
        let remapped = remapper.remap_instruction(inst);

        // Remap block IDs in Branch/Phi
        let remapped_with_blocks = match remapped {
            MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                then_edge_args,
                else_edge_args,
            } => {
                let remapped_then =
                    remap_block_id(then_bb, local_block_map, &ctx.skipped_entry_redirects);
                let remapped_else =
                    remap_block_id(else_bb, local_block_map, &ctx.skipped_entry_redirects);
                MirInstruction::Branch {
                    condition,
                    then_bb: remapped_then,
                    else_bb: remapped_else,
                    then_edge_args,
                    else_edge_args,
                }
            }
            MirInstruction::Phi {
                dst,
                inputs,
                type_hint,
            } => remap_phi_instruction(dst, &inputs, type_hint, local_block_map),
            other => other,
        };

        // TODO: Type propagation should be in apply stage, not plan stage
        // For now, keep it here to match original behavior
        // propagate_value_type_for_inst(builder, func, inst, &remapped_with_blocks);

        instructions.push(remapped_with_blocks);
    }

    (instructions, tail_call_target)
}
