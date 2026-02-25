//! Stage 3: Apply - Apply rewritten blocks to MirBuilder
//!
//! Phase 287 P3: Extracted from instruction_rewriter.rs::apply_rewrites()
//!
//! Mutates the builder:
//! - Adds new blocks to current function
//! - Injects boundary copies (if boundary provided)
//! - Updates RewriteContext with carrier/phi inputs
//!
//! # Phase 286C-4 Step 3
//!
//! This function extracts ~180 lines from merge_and_rewrite():
//! - Block addition (lines 1738-1751)
//! - Boundary injection (lines 1755-1857)
//! - Context updates (carrier_inputs, exit_phi_inputs)

use super::super::{
    rewrite_context::RewriteContext,
    plan_box::RewrittenBlocks,
};
use super::super::super::{
    loop_header_phi_info::LoopHeaderPhiInfo,
    trace,
};
use crate::mir::{MirModule, ValueId};
use crate::mir::builder::{MirBuilder, joinir_id_remapper::JoinIrIdRemapper};
use crate::mir::builder::joinir_inline_boundary_injector::BoundaryInjector;
use crate::mir::join_ir::lowering::{
    inline_boundary::JoinInlineBoundary,
    canonical_names,
};

/// Stage 3: Apply - Apply rewritten blocks to MirBuilder
///
/// Mutates the builder:
/// - Adds new blocks to current function
/// - Injects boundary copies (if boundary provided)
/// - Updates RewriteContext with carrier/phi inputs
///
/// # Phase 286C-4 Step 3
///
/// This function extracts ~180 lines from merge_and_rewrite():
/// - Block addition (lines 1738-1751)
/// - Boundary injection (lines 1755-1857)
/// - Context updates (carrier_inputs, exit_phi_inputs)
///
/// # Phase 287 P5: Re-exported through stages/mod.rs
/// Access via stages::{apply_rewrites} for unified API.
pub(in crate::mir::builder::control_flow::joinir::merge) fn apply_rewrites(
    builder: &mut MirBuilder,
    blocks: RewrittenBlocks,
    boundary: Option<&JoinInlineBoundary>,
    remapper: &JoinIrIdRemapper,
    loop_header_phi_info: &LoopHeaderPhiInfo,
    mir_module: &MirModule,
    ctx: &mut RewriteContext,
    debug: bool,
) -> Result<(), String> {
    let trace = trace::trace();
    // Only verbose if explicitly requested via debug flag (not env var - causes test failures)
    let verbose = debug;
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    // Add new blocks to current function
    if let Some(ref mut current_func) = builder.scope_ctx.current_function {
        for new_block in blocks.new_blocks {
            if debug && new_block.instructions.len() >= 4 {
                log!(
                    true,
                    "[apply_rewrites] Adding block {:?} with {} instructions",
                    new_block.id, new_block.instructions.len()
                );
                for (idx, inst) in new_block.instructions.iter().enumerate() {
                    log!(true, "[apply_rewrites]   [{}] {:?}", idx, inst);
                }
            }
            current_func.add_block(new_block);
        }
    }

    // Inject boundary copies (if boundary provided)
    if let Some(boundary) = boundary {
        use canonical_names as cn;

        // Get entry function's entry block
        let (entry_func_name, entry_func) = {
            if let Some(main) = mir_module.functions.get(cn::MAIN) {
                if main.params == boundary.join_inputs {
                    (cn::MAIN, main)
                } else {
                    mir_module
                        .functions
                        .iter()
                        .find(|(_, func)| func.params == boundary.join_inputs)
                        .or_else(|| mir_module.functions.iter().next())
                        .map(|(name, func)| (name.as_str(), func))
                        .ok_or("JoinIR module has no functions")?
                }
            } else {
                mir_module
                    .functions
                    .iter()
                    .find(|(_, func)| func.params == boundary.join_inputs)
                    .or_else(|| mir_module.functions.iter().next())
                    .map(|(name, func)| (name.as_str(), func))
                    .ok_or("JoinIR module has no functions")?
            }
        };

        let entry_block_remapped = remapper
            .get_block(entry_func_name, entry_func.entry_block)
            .ok_or_else(|| format!("Entry block not found for {}", entry_func_name))?;

        log!(
            verbose,
            "[apply_rewrites] Boundary entry: func='{}' entry_block={:?} remapped={:?}",
            entry_func_name, entry_func.entry_block, entry_block_remapped
        );

        // Build value map for BoundaryInjector
        let mut value_map_for_injector = std::collections::BTreeMap::new();

        // Add join_inputs to value_map
        for join_in in &boundary.join_inputs {
            if let Some(remapped) = remapper.get_value(*join_in) {
                value_map_for_injector.insert(*join_in, remapped);
            }
        }

        // Add condition_bindings to value_map
        for binding in &boundary.condition_bindings {
            if let Some(remapped) = remapper.get_value(binding.join_value) {
                value_map_for_injector.insert(binding.join_value, remapped);
                log!(
                    verbose,
                    "[apply_rewrites] Condition binding '{}': JoinIR {:?} → remapped {:?}",
                    binding.name, binding.join_value, remapped
                );
            }
        }

        // Collect PHI dst IDs from loop_header_phi_info
        let phi_dst_ids: std::collections::HashSet<ValueId> = loop_header_phi_info
            .carrier_phis
            .values()
            .map(|entry| entry.phi_dst)
            .collect();

        // Inject boundary copies
        if let Some(ref mut current_func) = builder.scope_ctx.current_function {
            let _reallocations = BoundaryInjector::inject_boundary_copies(
                current_func,
                entry_block_remapped,
                boundary,
                &value_map_for_injector,
                &phi_dst_ids,
                debug,
            )?;
        }
    }

    // Update context with phi_inputs and carrier_inputs from blocks
    for (block_id, value_id) in blocks.phi_inputs {
        ctx.add_exit_phi_input(block_id, value_id);
    }

    for (carrier_name, inputs) in blocks.carrier_inputs {
        for (block_id, value_id) in inputs {
            ctx.add_carrier_input(carrier_name.clone(), block_id, value_id);
        }
    }

    if debug {
        log!(
            true,
            "[apply_rewrites] Applied {} blocks, {} phi_inputs, {} carriers",
            builder.scope_ctx.current_function
                .as_ref()
                .map(|f| f.blocks.len())
                .unwrap_or(0),
            ctx.exit_phi_inputs.len(),
            ctx.carrier_inputs.len()
        );
    }

    Ok(())
}
