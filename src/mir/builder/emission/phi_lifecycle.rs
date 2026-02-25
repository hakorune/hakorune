//! PHI Lifecycle SSOT - Provisional PHI → Final PHI pattern
//!
//! **Purpose**: Unify PHI "Reserve → Define → Populate → Finalize" lifecycle
//! into a single entry point to prevent responsibility confusion.
//!
//! **Architecture**:
//! - SSOT for PHI operations: define_provisional_phi() and patch_phi_inputs()
//! - Prohibits direct calls to cf_common::insert_phi_at_head* and
//!   builder.update_phi_instruction from outside this module.
//!
//! **Background**: The selfhost blocker `call/arg_out_of_function_scope v=%36`
//! was caused by exposing a Reserve-only PHI dst to variable_map before it
//! was Defined (emitted as MIR instruction). This module enforces the
//! Reserve → Define → Populate contract to prevent recurrence.
//!
//! **Terms**:
//! - **Reserve**: alloc_typed() / next_value_id() to allocate ValueId (NOT a definition)
//! - **Define**: MIR instruction emitted with dst (PHI/Copy/Const/etc.)
//! - **Expose**: Publish ValueId to variable_map (or expression evaluation bindings)
//! - **Populate**: PHI inputs determination (pred→ValueId pairs)
//! - **Finalize/Seal**: CFG determined, no more "input hole filling"
//!
//! **Refactoring Context**:
//! - Before: `insert_phi_at_head*` and `update_phi_instruction` calls scattered
//!   across 5+ locations with temporal coupling (Step 1.5 → Step 4).
//! - After: Single entry point for all PHI lifecycle operations.
//!
//! **Contract (MUST)**:
//! 1. variable_map may only point to **Defined** ValueIds (not Reserve-only)
//! 2. Reserve-only PHI dst needed by body/effect must be Defined first via
//!    provisional PHI (even with empty inputs)
//! 3. PHI Insert/Update must go through this SSOT entry point (no direct writes)
//! 4. Failures must fail-fast with Result propagation (no silent no-ops)

use crate::mir::builder::MirBuilder;
use crate::mir::ssot::cf_common::insert_phi_at_head_spanned;
use crate::mir::{BasicBlockId, ValueId};

/// Define a provisional PHI with empty inputs.
///
/// **Purpose**: Define PHI dst early (before body emit) to ensure
/// the ValueId exists in def_blocks. This is the "Define" step only;
/// Populate comes later via patch_phi_inputs().
///
/// **Contract**:
/// - Calls cf_common::insert_phi_at_head_spanned(..., inputs=[], ...)
/// - PHI instruction is emitted with empty inputs (dst is now Defined)
/// - Inputs will be patched later via patch_phi_inputs()
///
/// **Errors**:
/// - Returns Err if current_function is None (fail-fast)
///
/// # Arguments
/// * `builder` - MirBuilder (for current_function, span extraction)
/// * `block` - Target block for PHI insertion
/// * `dst` - Destination ValueId for PHI result (already allocated)
/// * `tag` - Debug context string for error messages
///
/// # Example
/// ```ignore
/// // Step 1.5: Provisional PHI (Define only, no Populate yet)
/// phi_lifecycle::define_provisional_phi(
///     builder,
///     header_bb,
///     i_current,
///     "loop_lowerer:step1.5",
/// )?;
/// // i_current is now Defined (in def_blocks) but has no inputs yet
/// ```
#[track_caller]
pub(in crate::mir::builder) fn define_provisional_phi(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    tag: &str,
) -> Result<(), String> {
    let func = builder
        .scope_ctx
        .current_function
        .as_mut()
        .ok_or_else(|| format!("[freeze:contract][phi_lifecycle/define_no_function] tag={} No current function", tag))?;

    let span = builder.metadata_ctx.current_span();

    if crate::config::env::joinir_dev::debug_enabled() {
        builder
            .metadata_ctx
            .record_value_caller(dst, std::panic::Location::caller());
        if let Some(loc) = builder.metadata_ctx.value_origin_callers().get(&dst).cloned() {
            func.metadata.value_origin_callers.insert(dst, loc);
        }
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phi_lifecycle/define] fn={} bb={:?} dst=%{} tag={}",
            func.signature.name, block, dst.0, tag
        ));
    }

    // Insert PHI with empty inputs (provisional)
    // This ensures dst is Defined (in def_blocks) before body instructions are emitted
    insert_phi_at_head_spanned(func, block, dst, vec![], span).map_err(|e| {
        format!("{e} op=define_provisional_phi tag={tag}")
    })?;

    Ok(())
}

/// Define a final PHI with all inputs (single-step insertion).
///
/// **Purpose**: Insert a complete PHI instruction when all inputs are known
/// at insertion time. This is the "Define + Populate" single-step variant.
///
/// **Contract**:
/// - Calls cf_common::insert_phi_at_head_spanned(..., inputs, ...)
/// - PHI instruction is emitted with complete inputs (dst is now Defined)
/// - Use define_provisional_phi() + patch_phi_inputs() for two-step pattern
///
/// **Errors**:
/// - Returns Err if current_function is None (fail-fast)
///
/// # Arguments
/// * `builder` - MirBuilder (for current_function, span extraction)
/// * `block` - Target block for PHI insertion
/// * `dst` - Destination ValueId for PHI result (already allocated)
/// * `inputs` - Vec of (predecessor_block, value) pairs
/// * `tag` - Debug context string for error messages
///
/// # Example
/// ```ignore
/// // Single-step PHI insertion (all inputs known)
/// let inputs = vec![(preheader_bb, i_init), (step_bb, i_next)];
/// phi_lifecycle::define_phi_final(
///     builder,
///     header_bb,
///     i_current,
///     inputs,
///     "loop_lowerer:single_step",
/// )?;
/// ```
#[track_caller]
pub(in crate::mir::builder) fn define_phi_final(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    mut inputs: Vec<(BasicBlockId, ValueId)>,
    tag: &str,
) -> Result<(), String> {
    let func = builder
        .scope_ctx
        .current_function
        .as_mut()
        .ok_or_else(|| format!("[freeze:contract][phi_lifecycle/define_no_function] tag={} No current function", tag))?;

    let span = builder.metadata_ctx.current_span();

    if crate::config::env::joinir_dev::debug_enabled() {
        builder
            .metadata_ctx
            .record_value_caller(dst, std::panic::Location::caller());
        if let Some(loc) = builder.metadata_ctx.value_origin_callers().get(&dst).cloned() {
            func.metadata.value_origin_callers.insert(dst, loc);
        }
    }

    // Sort inputs by block ID (SSA invariant)
    inputs.sort_by_key(|(bb, _)| bb.0);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phi_lifecycle/define] fn={} bb={:?} dst=%{} tag={}",
            func.signature.name, block, dst.0, tag
        ));
    }

    // Insert PHI with complete inputs (single-step)
    insert_phi_at_head_spanned(func, block, dst, inputs, span).map_err(|e| {
        format!("{e} op=define_phi_final tag={tag}")
    })?;

    Ok(())
}

/// Define a final PHI with all inputs (function-level API).
///
/// **Purpose**: Function-level variant of define_phi_final for code that
/// operates directly on MirFunction instead of through MirBuilder.
///
/// **Use Case**: EdgeCFG emit layer that works at function level.
/// For builder-level code, use define_phi_final() instead.
///
/// **Contract**:
/// - Calls cf_common::insert_phi_at_head_spanned(..., inputs, ...)
/// - PHI instruction is emitted with complete inputs
///
/// **Errors**:
/// - Returns Err if block not found in function
///
/// # Arguments
/// * `function` - MirFunction (direct access, no builder wrapper)
/// * `block` - Target block for PHI insertion
/// * `dst` - Destination ValueId for PHI result
/// * `inputs` - Vec of (predecessor_block, value) pairs
/// * `span` - Source location span for diagnostics
///
/// # Example
/// ```ignore
/// // EdgeCFG BlockParams → PHI (function-level)
/// phi_lifecycle::define_phi_final_fn(
///     function,
///     target_block,
///     dst_value,
///     inputs,
///     Span::unknown(),
/// )?;
/// ```
#[track_caller]
pub(in crate::mir::builder) fn define_phi_final_fn(
    function: &mut crate::mir::MirFunction,
    block: BasicBlockId,
    dst: ValueId,
    mut inputs: Vec<(BasicBlockId, ValueId)>,
    span: crate::ast::Span,
) -> Result<(), String> {
    // Sort inputs by block ID (SSA invariant)
    inputs.sort_by_key(|(bb, _)| bb.0);

    if crate::config::env::joinir_dev::debug_enabled() {
        let caller = std::panic::Location::caller();
        let loc = format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
        function.metadata.value_origin_callers.insert(dst, loc);
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phi_lifecycle/define] fn={} bb={:?} dst=%{} tag=edgecfg_block_params",
            function.signature.name, block, dst.0
        ));
    }

    // Insert PHI with complete inputs (function-level)
    insert_phi_at_head_spanned(function, block, dst, inputs, span).map_err(|e| {
        format!("{e} op=define_phi_final_fn tag=edgecfg_block_params")
    })?;

    Ok(())
}

/// Patch PHI inputs (provisional → final).
///
/// **Purpose**: Update provisional PHI with actual inputs after all
/// predecessors are known. This is the "Populate" step.
///
/// **Contract**:
/// - Calls builder.update_phi_instruction(block, dst, inputs)
/// - Fail-fast if PHI not found or block not found
/// - Inputs are sorted by block ID before patching (SSA invariant)
///
/// **Errors**:
/// - Returns `[freeze:contract][lowerer/phi_patch_missing]` if:
///   - PHI instruction not found in block
///   - Block not found
///   - No current function
///
/// # Arguments
/// * `builder` - MirBuilder (for current_function access)
/// * `block` - Block containing the PHI instruction
/// * `dst` - PHI dst ValueId to patch
/// * `inputs` - Vec of (predecessor_block, value) pairs
/// * `tag` - Debug context string for error messages
///
/// # Example
/// ```ignore
/// // Step 4: Patch PHI inputs (Populate)
/// let inputs = vec![(preheader_bb, i_init), (step_bb, i_next)];
/// phi_lifecycle::patch_phi_inputs(
///     builder,
///     header_bb,
///     i_current,
///     inputs,
///     "loop_lowerer:step4",
/// )?;
/// // i_current PHI now has complete inputs
/// ```
pub(in crate::mir::builder) fn patch_phi_inputs(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    mut inputs: Vec<(BasicBlockId, ValueId)>,
    tag: &str,
) -> Result<(), String> {
    // Sort inputs by block ID (SSA invariant)
    inputs.sort_by_key(|(bb, _)| bb.0);

    if crate::config::env::joinir_dev::debug_enabled() {
        let mut detail = format!(" inputs={}", inputs.len());
        for (i, (pred, incoming)) in inputs.iter().take(2).enumerate() {
            detail.push_str(&format!(
                " phi{}_pred={:?} phi{}_in=%{}",
                i, pred, i, incoming.0
            ));
        }
        let fn_name = builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.as_str())
            .unwrap_or("<unknown>");
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phi_lifecycle/patch] fn={} bb={:?} dst=%{}{} tag={}",
                fn_name, block, dst.0, detail, tag
            ));
        }
    }

    builder
        .update_phi_instruction(block, dst, inputs)
        .map_err(|e| {
            format!("{e} op=patch_phi_inputs tag={tag}")
        })
}

/// Rollback a provisional PHI (empty inputs) if it still exists.
///
/// Purpose: enforce "patch or rollback" to avoid leaving empty-input PHIs in the function
/// when an error happens before patching.
///
/// Notes:
/// - Only removes `Phi { dst, inputs=[] }` for the given dst in the given block.
/// - Intended for strict/dev + planner_required error paths (contract enforcement).
#[track_caller]
pub(in crate::mir::builder) fn rollback_provisional_phi(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    tag: &str,
) -> Result<bool, String> {
    let func = builder
        .scope_ctx
        .current_function
        .as_mut()
        .ok_or_else(|| {
            format!(
                "[freeze:contract][phi_lifecycle/rollback_no_function] tag={} No current function",
                tag
            )
        })?;

    let Some(bb) = func.get_block_mut(block) else {
        return Err(format!(
            "[freeze:contract][phi_lifecycle/rollback_missing_block] fn={} bb={:?} dst=%{} tag={}",
            func.signature.name, block, dst.0, tag
        ));
    };

    let mut removed = false;
    let mut idx = 0usize;
    while idx < bb.instructions.len() {
        let is_target = matches!(
            &bb.instructions[idx],
            crate::mir::MirInstruction::Phi { dst: d, inputs, .. } if *d == dst && inputs.is_empty()
        );
        if is_target {
            bb.instructions.remove(idx);
            bb.instruction_spans.remove(idx);
            removed = true;
            break;
        }
        idx += 1;
    }

    if removed && crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phi_lifecycle/rollback] fn={} bb={:?} dst=%{} tag={}",
            func.signature.name, block, dst.0, tag
        ));
    }

    Ok(removed)
}
