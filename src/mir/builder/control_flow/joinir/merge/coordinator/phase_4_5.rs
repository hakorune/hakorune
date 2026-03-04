//! Phase 4-4.5: Merge, Rewrite, and Header PHI Finalization
//!
//! This module handles:
//! - Phase 4: Merge blocks and rewrite instructions
//! - Phase 4.5: Finalize loop header PHIs (insert into header block)
//! - Contract verification for terminator targets

use super::super::{
    config::MergeConfig, contract_checks, loop_header_phi_builder, loop_header_phi_info,
    merge_result, rewriter, MergeContracts,
};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, MirModule, ValueId};
use std::collections::BTreeMap;

/// Execute Phase 4-4.5: Merge, rewrite, and finalize header PHIs
pub fn execute(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut loop_header_phi_info::LoopHeaderPhiInfo,
    exit_block_id: BasicBlockId,
    config: &MergeConfig,
    trace: &trace::JoinLoopTrace,
    debug: bool,
) -> Result<merge_result::MergeResult, String> {
    // Collect value_to_func_name from mir_module (needed by rewriter)
    let value_to_func_name = collect_value_to_func_name(mir_module);

    // Phase 4: Merge blocks and rewrite instructions
    // Phase 33-16: Pass mutable loop_header_phi_info for latch_incoming tracking
    // Phase 177-3: Pass exit_block_id from allocator to avoid conflicts
    // Phase 260 P0.1: Use rewriter module (re-exports instruction_rewriter)
    let merge_result = rewriter::merge_and_rewrite(
        builder,
        mir_module,
        remapper,
        &value_to_func_name,
        function_params,
        boundary,
        loop_header_phi_info,
        exit_block_id,
        debug,
    )?;

    // Phase 4.5: Finalize loop header PHIs (insert into header block)
    //
    // By now, rewriter has set latch_incoming for all carriers.
    // We can finalize the PHIs and insert them into the header block.
    if !loop_header_phi_info.carrier_phis.is_empty() {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 4.5: Finalizing {} header PHIs",
                loop_header_phi_info.carrier_phis.len()
            ),
            debug,
        );
        loop_header_phi_builder::LoopHeaderPhiBuilder::finalize(
            builder,
            loop_header_phi_info,
            debug,
        )?;
    }

    // Contract check (Fail-Fast): ensure we didn't leave dangling Jump/Branch targets.
    // Phase 131 Task 6: Use MergeConfig.strict_mode instead of env checks
    if config.strict_mode || config.dev_log {
        if let Some(ref current_func) = builder.scope_ctx.current_function {
            // Note: exit_block_id may be allocated but not inserted yet (it becomes the
            // current block after merge, and subsequent AST lowering fills it).
            // We still want to catch truly dangling targets (e.g., jumps to skipped k_exit).
            let allowed_missing_jump_targets = if config.allow_missing_exit_block {
                vec![merge_result.exit_block_id]
            } else {
                Vec::new()
            };
            let contracts = MergeContracts {
                allowed_missing_jump_targets,
            };
            contract_checks::verify_all_terminator_targets_exist(current_func, &contracts)?;
        }
    }

    Ok(merge_result)
}

/// Collect value_to_func_name mapping from mir_module
fn collect_value_to_func_name(mir_module: &MirModule) -> BTreeMap<ValueId, String> {
    let mut value_to_func_name = BTreeMap::new();
    for (_name, func) in &mir_module.functions {
        for param_value_id in &func.params {
            value_to_func_name.insert(*param_value_id, func.signature.name.clone());
        }
    }
    value_to_func_name
}
