//! JoinIR MIR Block Merging Coordinator
//!
//! Phase 49-3.2: Merge JoinIR-generated MIR blocks into current_function
//!
//! # Module Structure (Refactored)
//!
//! The coordinator is split into focused submodules:
//! - `phase_1_2` - Block allocation and value collection
//! - `phase_3_remap` - Value remapping and debug logging
//! - `phase_3_5_phi_override` - Parameter-to-PHI override logic
//! - `phase_4_5` - Merge, rewrite, and header PHI finalization
//! - `phase_5_6` - Exit PHI building and boundary reconnection
//! - `finalize` - Final jump, block switch, and return handling

mod finalize;
mod phase_1_2;
mod phase_3_5_phi_override;
mod phase_3_remap;
mod phase_4_5;
mod phase_5_6;

use super::config::MergeConfig;
use super::{boundary_logging, contract_checks, header_phi_prebuild};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{MirModule, ValueId};

/// Phase 49-3.2: Merge JoinIR-generated MIR blocks into current_function
///
/// # Phase 189: Multi-Function MIR Merge
///
/// This merges JoinIR-generated blocks by:
/// 1. Remapping all block IDs across ALL functions to avoid conflicts
/// 2. Remapping all value IDs across ALL functions to avoid conflicts
/// 3. Adding all blocks from all functions to current_function
/// 4. Jumping from current_block to the entry block
/// 5. Converting Return → Jump to exit block for all functions
///
/// **Multi-Function Support** (Phase 189):
/// - Pattern 1 (Simple While) generates 3 functions: entry + loop_step + k_exit
/// - All functions are flattened into current_function with global ID remapping
/// - Single exit block receives all Return instructions from all functions
///
/// # Phase 188-Impl-3: JoinInlineBoundary Support
///
/// When `boundary` is provided, injects Copy instructions at the entry block
/// to connect host ValueIds to JoinIR local ValueIds:
///
/// ```text
/// entry_block:
///   // Injected by boundary
///   ValueId(100) = Copy ValueId(4)  // join_input → host_input
///   // Original JoinIR instructions follow...
/// ```
///
/// This enables clean separation: JoinIR uses local IDs (0,1,2...),
/// host uses its own IDs, and Copy instructions bridge the gap.
///
/// # Returns
///
/// Returns `Ok(Some(exit_phi_id))` if the merged JoinIR functions have return values
/// that were collected into an exit block PHI. Exit reconnection is driven by
/// explicit exit_bindings on the boundary (no legacy host_outputs).
pub(in crate::mir::builder) fn merge_joinir_mir_blocks(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // Phase 131 Task 6: Use MergeConfig for consolidated configuration
    let config = MergeConfig::with_debug(debug);
    let verbose = config.dev_log;
    let trace = trace::trace();

    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] merge_joinir_mir_blocks called with {} functions",
            mir_module.functions.len()
        ),
        debug,
    );

    // Validate boundary contracts
    if let Some(boundary) = boundary {
        if let Err(msg) = boundary.validate_jump_args_layout() {
            return Err(error_tags::freeze_with_hint(
                "phase256/jump_args_layout",
                &msg,
                "set JoinInlineBoundary.jump_args_layout via builder and avoid expr_result/carrier mismatch",
            ));
        }
    }

    // Phase 286 P3: Validate boundary contract BEFORE merge begins
    if let Some(boundary) = boundary {
        let host_fn = builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.as_str())
            .unwrap_or("<unknown>");

        let cont_count = boundary.continuation_func_ids.len();
        let join_summary = format!(
            "conts={} exits={} conds={}",
            cont_count,
            boundary.exit_bindings.len(),
            boundary.condition_bindings.len()
        );

        let context = format!(
            "merge_joinir_mir_blocks host={} join={} phase=<unknown> [{}]",
            host_fn, cont_count, join_summary
        );

        if let Err(msg) = contract_checks::verify_boundary_contract_at_creation(boundary, &context)
        {
            return Err(msg); // Fail-Fast: [joinir/contract:B*] error
        }
    }

    // Phase 287 P0.5: Delegated to boundary_logging module
    boundary_logging::log_boundary_info(boundary, &trace, verbose);

    // Phase 1-2: Allocate blocks and collect values
    let phase_1_2_result =
        phase_1_2::execute(builder, mir_module, boundary, &trace, debug, verbose)?;

    let mut remapper = phase_1_2_result.remapper;
    let exit_block_id = phase_1_2_result.exit_block_id;
    let used_values = phase_1_2_result.used_values;
    let function_params = phase_1_2_result.function_params;

    // Phase 201-A + Phase 287 P0.4: Pre-build loop header PHIs BEFORE Phase 3
    let (mut loop_header_phi_info, merge_entry_block, reserved_value_ids) =
        header_phi_prebuild::prebuild_header_phis(
            builder,
            mir_module,
            boundary,
            &remapper,
            &function_params,
            debug,
        )?;
    if let Some(boundary) = boundary {
        contract_checks::verify_header_phi_layout(boundary, &loop_header_phi_info)?;
    }

    // Phase 3: Remap ValueIds (with reserved PHI dsts protection)
    phase_3_remap::execute(
        builder,
        &used_values,
        &mut remapper,
        &reserved_value_ids,
        boundary,
        &trace,
        debug,
        verbose,
    )?;

    // Phase 3.5: Override remapper for function parameters to use PHI dsts
    if let Some(boundary) = boundary {
        phase_3_5_phi_override::execute(
            boundary,
            &function_params,
            &mut remapper,
            &mut loop_header_phi_info,
            &trace,
            debug,
            verbose,
        )?;
    }

    // Phase 4-4.5: Merge blocks, rewrite instructions, and finalize header PHIs
    let merge_result = phase_4_5::execute(
        builder,
        mir_module,
        &mut remapper,
        &function_params,
        boundary,
        &mut loop_header_phi_info,
        exit_block_id,
        &config,
        &trace,
        debug,
    )?;

    // Phase 5-6: Build exit PHIs and reconnect boundary
    let (exit_phi_result_id, carrier_phis) = phase_5_6::execute(
        builder,
        boundary,
        &merge_result,
        &config,
        &trace,
        debug,
    )?;

    // Final: Jump to entry, switch to exit block, handle return
    finalize::execute(
        builder,
        boundary,
        &merge_result,
        &loop_header_phi_info,
        merge_entry_block,
        exit_phi_result_id,
        &carrier_phis,
        &remapper,
        &trace,
        debug,
    )
}
