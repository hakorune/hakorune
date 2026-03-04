//! JoinIR Instruction Rewriter
//!
//! Rewrites JoinIR instructions with remapped IDs and merges blocks
//! into the host MIR builder.
//!
//! Phase 4 Extraction: Separated from merge_joinir_mir_blocks (lines 260-546)
//! Phase 33-17: Further modularization - extracted TailCallClassifier and MergeResult
//! Phase 179-A Step 3: Named constants for magic values
//! Phase 284 P1: Use block_remapper SSOT for block ID remapping
//! Phase 286C-2: Box extraction - InstructionFilterBox, ReturnConverterBox, etc.
//! Phase 286C-4: 3-stage pipeline (Scan → Plan → Apply)
//! Phase 287 P3: Physical modularization - extracted stages to separate files

 // Phase 284 P1: SSOT
use super::loop_header_phi_info::LoopHeaderPhiInfo;
use super::merge_result::MergeResult;
use super::super::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, MirModule, ValueId};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// Phase 287 P3: Import 3-stage pipeline functions
use super::rewriter::helpers::is_skippable_continuation;
use super::rewriter::rewrite_context::RewriteContext;
use super::contract_checks;

// Phase 287 P5: Unified import through stages facade (SSOT)
use super::rewriter::stages::{plan_rewrites, apply_rewrites};


/// Phase 4: Merge ALL functions and rewrite instructions
///
/// Returns:
/// - MergeResult containing exit_block_id, exit_phi_inputs, and carrier_inputs
///
/// # Phase 33-16
///
/// The `loop_header_phi_info` parameter is used to:
/// 1. Set latch_incoming when processing tail calls
/// 2. Provide PHI dsts for exit value collection (instead of undefined parameters)
///
/// # Phase 33-17
///
/// Uses extracted modules:
/// - tail_call_classifier: TailCallKind classification
/// - merge_result: MergeResult data structure
///
/// # Phase 177-3
///
/// `exit_block_id` is now passed in from block_allocator to ensure it doesn't
/// conflict with JoinIR function blocks.
pub(super) fn merge_and_rewrite(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    value_to_func_name: &BTreeMap<ValueId, String>, // Phase 222.5-E: HashMap → BTreeMap for determinism
    function_params: &BTreeMap<String, Vec<ValueId>>, // Phase 222.5-E: HashMap → BTreeMap for determinism
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<MergeResult, String> {
    // Phase 286C-4: pipeline orchestrator
    // 1. plan_rewrites() - Generate rewritten blocks (pure transformation)
    // 2. apply_rewrites() - Mutate builder (apply changes)

    let trace = trace::trace();
    // Only verbose if explicitly requested via debug flag (not env var - causes test failures)
    let verbose = debug;
    macro_rules! log {
        ($enabled:expr, $($arg:tt)*) => {
            trace.stderr_if(&format!($($arg)*), $enabled);
        };
    }

    // ===== METADATA SETUP =====
    // Build continuation candidates and policy
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

    log!(
        verbose,
        "[merge_and_rewrite] Phase 286C-4: Using exit_block_id = {:?}",
        exit_block_id
    );

    // Phase 286C-3: Use RewriteContext to consolidate scattered state
    let mut ctx = RewriteContext::new(exit_block_id, boundary, debug);

    // Build function_entry_map for Call→Jump conversion
    for (func_name, func) in &mir_module.functions {
        let entry_block_new = remapper
            .get_block(func_name, func.entry_block)
            .ok_or_else(|| format!("Entry block not found for {}", func_name))?;
        ctx.register_function_entry(func_name.clone(), entry_block_new);
    }

    // Phase 259 P0 FIX: Build redirect map for skipped continuation entry blocks → exit_block_id
    //
    // When a continuation function (e.g., k_exit) is skipped during merge, its blocks are not
    // included in the output. However, Branch/Jump terminators in other functions may still
    // reference k_exit's entry block. We need to redirect those references to exit_block_id.
    //
    // This map is GLOBAL across all functions, unlike local_block_map which is function-local.
    for func_name in &skippable_continuation_func_names {
        if let Some(&entry_block) = ctx.function_entry_map.get(func_name) {
            ctx.register_skipped_redirect(entry_block, exit_block_id);
        }
    }

    if debug && !ctx.skipped_entry_redirects.is_empty() {
        log!(
            true,
            "[cf_loop/joinir] Phase 259 P0: Built skipped_entry_redirects: {:?}",
            ctx.skipped_entry_redirects
        );
    }

    // ===== STAGE 1: PLAN (Generate rewritten blocks) =====
    if debug {
        log!(
            true,
            "[merge_and_rewrite] Phase 286C-4 Stage 1: Planning rewrites"
        );
    }

    let blocks = plan_rewrites(
        mir_module,
        remapper,
        function_params,
        boundary,
        loop_header_phi_info,
        &mut ctx,
        value_to_func_name,
        debug,
    )?;

    if debug {
        log!(
            true,
            "[merge_and_rewrite] Phase 286C-4: Plan complete - {} blocks generated",
            blocks.new_blocks.len()
        );
    }

    // Phase 286C-4.2: Verify carrier_inputs completeness (Fail-Fast)
    if let Some(b) = boundary {
        contract_checks::verify_carrier_inputs_complete(b, &blocks.carrier_inputs)?;
    }

    // ===== STAGE 2: APPLY (Mutate builder) =====
    if debug {
        log!(
            true,
            "[merge_and_rewrite] Phase 286C-4 Stage 2: Applying rewrites"
        );
    }

    apply_rewrites(
        builder,
        blocks,
        boundary,
        remapper,
        loop_header_phi_info,
        mir_module,
        &mut ctx,
        debug,
    )?;

    if debug {
        log!(
            true,
            "[merge_and_rewrite] Phase 286C-4: Apply complete"
        );
    }
    // Phase 131 P2: DirectValue mode remapped_exit_values SSOT
    // Phase 286C-3: Use ctx.set_remapped_exit_value() for state management
    //
    // Contract (DirectValue):
    // - boundary.exit_bindings[*].join_exit_value is a JoinIR-side ValueId that must be defined
    //   in the merged MIR (e.g., final env value produced by the loop body).
    // - remapper owns JoinIR→Host mapping, so merge_and_rewrite is responsible for producing
    //   carrier_name → host ValueId.
    if let Some(b) = boundary {
        if b.exit_reconnect_mode == crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode::DirectValue {
            for binding in &b.exit_bindings {
                if binding.role == crate::mir::join_ir::lowering::carrier_info::CarrierRole::ConditionOnly {
                    continue;
                }

                let host_vid = remapper.get_value(binding.join_exit_value).ok_or_else(|| {
                    error_tags::freeze_with_hint(
                        "phase131/directvalue/remap_missing",
                        &format!(
                            "DirectValue: join_exit_value {:?} for carrier '{}' was not remapped",
                            binding.join_exit_value, binding.carrier_name
                        ),
                        "ensure exit_bindings.join_exit_value is included in merge used_values and references a value defined by the fragment",
                    )
                })?;
                ctx.set_remapped_exit_value(binding.carrier_name.clone(), host_vid);
            }
        }
    }

    // Phase 286C-2.1: Step 3 complete - scan_blocks() integrated
    // Step 4-5 future work: Extract plan/apply logic into separate functions
    //
    // Target orchestrator structure:
    // 1. let plan = scan_blocks(...)?;              // ✅ Done (Step 3)
    // 2. let blocks = plan_rewrites(plan, &mut ctx)?;  // TODO (Step 4)
    // 3. apply_rewrites(builder, blocks)?;             // TODO (Step 5)
    //
    // Current state: Main loop (lines 380-1469) contains both plan and apply logic.
    // This needs to be separated into 2 stages for clean architecture.

    Ok(MergeResult {
        exit_block_id: ctx.exit_block_id,
        exit_phi_inputs: ctx.exit_phi_inputs,
        carrier_inputs: ctx.carrier_inputs,
        remapped_exit_values: ctx.remapped_exit_values, // Phase 131 P1.5
    })
}
