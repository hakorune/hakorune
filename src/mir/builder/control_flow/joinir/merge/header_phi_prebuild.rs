//! Phase 287 P0.4: Loop header PHI pre-build orchestration
//!
//! Coordinates the pre-building of loop header PHIs BEFORE value remapping.
//! This prevents ValueId conflicts where a Const instruction could get a
//! ValueId that will later be used as a PHI dst, causing carrier corruption.
//!
//! This is orchestration logic, not a pure function - it coordinates:
//! - Entry function selection (via entry_selector)
//! - Block remapping
//! - Carrier extraction from boundary
//! - LoopHeaderPhiBuilder invocation
//! - Reserved ValueId collection

use super::{entry_selector, LoopHeaderPhiBuilder, LoopHeaderPhiInfo};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, MirModule, ValueId};
use std::collections::{BTreeMap, HashSet};

/// Pre-build loop header PHIs and reserve ValueIds
///
/// Returns:
/// - LoopHeaderPhiInfo: Pre-built PHI metadata with allocated dst ValueIds
/// - BasicBlockId: Merge entry block (where host will jump to)
/// - HashSet<ValueId>: All reserved ValueIds (PHI dsts + function params)
pub(super) fn prebuild_header_phis(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    remapper: &crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    debug: bool,
) -> Result<(LoopHeaderPhiInfo, BasicBlockId, HashSet<ValueId>), String> {
    let trace = super::trace::trace();

    // Build loop header PHI info if we have a boundary with loop_var_name
    let (loop_header_phi_info, merge_entry_block) = if let Some(boundary) = boundary {
        if let Some(loop_var_name) = &boundary.loop_var_name {
            // Phase 287 P0.4: Delegate entry selection to entry_selector (SSOT)
            let loop_step_func_name =
                entry_selector::select_loop_step_func_name(mir_module, boundary)?;
            let loop_step_func =
                entry_selector::get_function(&mir_module.functions, loop_step_func_name)?;

            let (entry_func_name, entry_func) =
                entry_selector::select_merge_entry_func(mir_module, boundary, loop_step_func_name)?;

            // Loop header block (for PHI placement)
            let loop_header_block = remapper
                .get_block(loop_step_func_name, loop_step_func.entry_block)
                .ok_or_else(|| {
                    format!(
                        "Loop header block not found for {} (Phase 287 P0.4)",
                        loop_step_func_name
                    )
                })?;

            // Merge entry block (for Jump target and PHI entry edge)
            let entry_block_remapped = remapper
                .get_block(entry_func_name, entry_func.entry_block)
                .ok_or_else(|| {
                    format!(
                        "Entry block not found for {} (Phase 287 P0.4)",
                        entry_func_name
                    )
                })?;

            // Get host's current block as the entry edge
            let _host_entry_block = builder
                .current_block
                .ok_or("Phase 287 P0.4: No current block when building header PHIs")?;

            // Phase 256.7-fix: Get loop variable's initial value from carrier_info if available
            let loop_var_init = if let Some(ref carrier_info) = boundary.carrier_info {
                carrier_info.loop_var_id
            } else {
                boundary.host_inputs.first().copied().ok_or(
                    "Phase 287 P0.4: No host_inputs or carrier_info in boundary for loop_var_init",
                )?
            };

            // Extract carriers with their initialization strategy
            let other_carriers: Vec<(
                String,
                ValueId,
                crate::mir::join_ir::lowering::carrier_info::CarrierInit,
                crate::mir::join_ir::lowering::carrier_info::CarrierRole,
            )> = if let Some(ref carrier_info) = boundary.carrier_info {
                carrier_info
                    .carriers
                    .iter()
                    .filter(|c| c.name != *loop_var_name)
                    .map(|c| (c.name.clone(), c.host_id, c.init, c.role))
                    .collect()
            } else {
                boundary
                    .exit_bindings
                    .iter()
                    .filter(|b| b.carrier_name != *loop_var_name)
                    .map(|b| {
                        (
                            b.carrier_name.clone(),
                            b.host_slot,
                            crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
                            crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                        )
                    })
                    .collect()
            };

            // Log entry function and block separation
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 287 P0.4: merge_entry_func='{}', merge_entry_block={:?}, loop_header_block={:?}",
                    entry_func_name, entry_block_remapped, loop_header_block
                ),
                debug,
            );
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 287 P0.4: Pre-building header PHIs for loop_var='{}' at {:?}",
                    loop_var_name, loop_header_block
                ),
                debug,
            );
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir]   loop_var_init={:?}, carriers={:?}",
                    loop_var_init,
                    other_carriers
                        .iter()
                        .map(|(n, _, _, _)| n.as_str())
                        .collect::<Vec<_>>()
                ),
                debug,
            );

            // Build PHI info (this allocates PHI dst ValueIds)
            let phi_info = LoopHeaderPhiBuilder::build(
                builder,
                loop_header_block,
                entry_block_remapped,
                loop_var_name,
                loop_var_init,
                &other_carriers,
                &boundary.loop_invariants,
                boundary.expr_result.is_some(),
                debug,
            )?;

            (phi_info, entry_block_remapped)
        } else {
            let default_block = get_default_entry_block(mir_module, remapper)?;
            (LoopHeaderPhiInfo::empty(default_block), default_block)
        }
    } else {
        let default_block = get_default_entry_block(mir_module, remapper)?;
        (LoopHeaderPhiInfo::empty(default_block), default_block)
    };

    // Collect reserved PHI dst ValueIds
    let reserved_phi_dsts = loop_header_phi_info.reserved_value_ids();
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 287 P0.4: Reserved PHI dsts: {:?}",
            reserved_phi_dsts
        ),
        debug && !reserved_phi_dsts.is_empty(),
    );

    // Also reserve JoinIR parameter ValueIds to avoid collisions
    let mut reserved_value_ids = reserved_phi_dsts.clone();
    for params in function_params.values() {
        for &param in params {
            reserved_value_ids.insert(param);
        }
    }

    // Set reserved IDs in MirBuilder so next_value_id() skips them
    builder.comp_ctx.reserved_value_ids = reserved_value_ids.clone();
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 287 P0.4: Set builder.comp_ctx.reserved_value_ids = {:?}",
            builder.comp_ctx.reserved_value_ids
        ),
        debug && !builder.comp_ctx.reserved_value_ids.is_empty(),
    );

    Ok((loop_header_phi_info, merge_entry_block, reserved_value_ids))
}

/// Get default entry block when no boundary or loop_var_name is present
fn get_default_entry_block(
    mir_module: &MirModule,
    remapper: &crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper,
) -> Result<BasicBlockId, String> {
    let (first_func_name, first_func) = mir_module
        .functions
        .iter()
        .next()
        .ok_or("JoinIR module has no functions (Phase 287 P0.4)")?;

    remapper
        .get_block(first_func_name, first_func.entry_block)
        .ok_or_else(|| {
            format!(
                "Entry block not found for {} (Phase 287 P0.4)",
                first_func_name
            )
        })
}
