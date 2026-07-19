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

use crate::ast::Span;
use crate::mir::builder::MirBuilder;
use crate::mir::ssot::cf_common::{
    insert_phi_at_head_spanned, insert_phi_at_head_spanned_with_type_hint,
};
use crate::mir::{BasicBlockId, MirType, ValueId};

mod batch_type_publication;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiToken {
    block: BasicBlockId,
    dst: ValueId,
}

impl PhiToken {
    pub(in crate::mir::builder) const fn dst(self) -> ValueId {
        self.dst
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiRollbackFailureV1 {
    token: PhiToken,
    error: String,
}

#[cfg(test)]
impl PhiRollbackFailureV1 {
    pub(in crate::mir::builder) fn block(&self) -> BasicBlockId {
        self.token.block
    }

    pub(in crate::mir::builder) fn dst(&self) -> ValueId {
        self.token.dst
    }

    pub(in crate::mir::builder) fn error(&self) -> &str {
        &self.error
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiTxnAbortErrorV1 {
    tag: String,
    pending_count: usize,
    primary: String,
    cleanup_failures: Box<[PhiRollbackFailureV1]>,
}

#[cfg(test)]
impl PhiTxnAbortErrorV1 {
    pub(in crate::mir::builder) fn primary(&self) -> &str {
        &self.primary
    }

    pub(in crate::mir::builder) fn pending_count(&self) -> usize {
        self.pending_count
    }

    pub(in crate::mir::builder) fn cleanup_failures(&self) -> &[PhiRollbackFailureV1] {
        &self.cleanup_failures
    }
}

impl std::fmt::Display for PhiTxnAbortErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cleanup = self
            .cleanup_failures
            .iter()
            .map(|failure| {
                format!(
                    "bb={:?}:dst=%{}:<{}>",
                    failure.token.block, failure.token.dst.0, failure.error
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        write!(
            f,
            "[freeze:contract][phi_lifecycle/txn_abort] tag={} pending_count={} source_error=<{}> cleanup_failure_count={} cleanup_failures=[{}]",
            self.tag,
            self.pending_count,
            self.primary,
            self.cleanup_failures.len(),
            cleanup
        )
    }
}

impl std::error::Error for PhiTxnAbortErrorV1 {}

/// Transaction wrapper for provisional PHI lifecycle operations.
///
/// This is a structural guard over the existing low-level lifecycle helpers.
/// It does not change release routing or accepted source shapes; it only
/// centralizes "define provisional, patch or rollback, then commit" ordering.
#[derive(Debug)]
pub(in crate::mir::builder) struct PhiTxn {
    tag: String,
    pending: Vec<PhiToken>,
}

impl PhiTxn {
    pub(in crate::mir::builder) fn begin(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            pending: Vec::new(),
        }
    }

    pub(in crate::mir::builder) fn define_provisional_phi(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        dst: ValueId,
        tag: &str,
    ) -> Result<PhiToken, String> {
        define_provisional_phi(builder, block, dst, tag)?;
        let token = PhiToken { block, dst };
        self.pending.push(token);
        Ok(token)
    }

    pub(in crate::mir::builder) fn patch_phi_inputs(
        &mut self,
        builder: &mut MirBuilder,
        token: PhiToken,
        inputs: Vec<(BasicBlockId, ValueId)>,
        tag: &str,
    ) -> Result<(), String> {
        patch_phi_inputs(builder, token.block, token.dst, inputs, tag)?;
        self.pending
            .retain(|pending| pending.block != token.block || pending.dst != token.dst);
        Ok(())
    }

    /// Roll back one still-pending provisional PHI owned by this transaction.
    ///
    /// A patched token is no longer pending and cannot be rolled back through
    /// this entry. Failed rollback keeps the token pending so the enclosing
    /// abort path can retain and retry every cleanup obligation.
    pub(in crate::mir::builder) fn rollback_pending_phi(
        &mut self,
        builder: &mut MirBuilder,
        token: PhiToken,
        tag: &str,
    ) -> Result<(), String> {
        if !self.pending.contains(&token) {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/rollback_not_pending] tag={} bb={:?} dst=%{}",
                tag, token.block, token.dst.0
            ));
        }
        match rollback_provisional_phi(builder, token.block, token.dst, tag)? {
            true => {
                self.pending.retain(|pending| *pending != token);
                Ok(())
            }
            false => Err(format!(
                "[freeze:contract][phi_lifecycle/rollback_missing_phi] tag={} bb={:?} dst=%{}",
                tag, token.block, token.dst.0
            )),
        }
    }

    pub(in crate::mir::builder) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<(), PhiTxnAbortErrorV1> {
        if self.pending.is_empty() {
            return Ok(());
        }

        let err = format!(
            "[freeze:contract][phi_lifecycle/provisional_left_unpatched] tag={} pending_count={} pending={}",
            self.tag,
            self.pending.len(),
            self.pending
                .iter()
                .map(|token| format!("bb={:?}:dst=%{}", token.block, token.dst.0))
                .collect::<Vec<_>>()
                .join(",")
        );
        Err(self.abort_on_err(builder, err))
    }

    pub(in crate::mir::builder) fn abort_on_err(
        self,
        builder: &mut MirBuilder,
        err: String,
    ) -> PhiTxnAbortErrorV1 {
        let mut cleanup_failures = Vec::new();
        for token in &self.pending {
            let rollback = rollback_provisional_phi(
                builder,
                token.block,
                token.dst,
                &format!("{}:abort", self.tag),
            );
            match rollback {
                Ok(true) => {}
                Ok(false) => cleanup_failures.push(PhiRollbackFailureV1 {
                    token: *token,
                    error: "provisional PHI was not found during rollback".to_string(),
                }),
                Err(error) => cleanup_failures.push(PhiRollbackFailureV1 {
                    token: *token,
                    error,
                }),
            }
        }
        PhiTxnAbortErrorV1 {
            tag: self.tag,
            pending_count: self.pending.len(),
            primary: err,
            cleanup_failures: cleanup_failures.into_boxed_slice(),
        }
    }
}

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
    let span = builder.metadata_ctx.current_span();
    let origin_caller = if crate::config::env::joinir_dev::debug_enabled() {
        builder.record_value_origin_caller(dst, std::panic::Location::caller());
        builder.value_origin_caller(dst).map(str::to_owned)
    } else {
        None
    };
    let func = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| {
            format!(
                "[freeze:contract][phi_lifecycle/define_no_function] tag={} No current function",
                tag
            )
        })?;

    if let Some(loc) = origin_caller {
        func.metadata.value_origin_callers.insert(dst, loc);
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
    insert_phi_at_head_spanned(func, block, dst, vec![], span)
        .map_err(|e| format!("{e} op=define_provisional_phi tag={tag}"))?;

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
    inputs: Vec<(BasicBlockId, ValueId)>,
    tag: &str,
) -> Result<(), String> {
    define_phi_final_with_type_hint(builder, block, dst, inputs, None, tag)
}

/// Define a final PHI with all inputs and an explicit type hint.
#[track_caller]
pub(in crate::mir::builder) fn define_phi_final_with_type_hint(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    inputs: Vec<(BasicBlockId, ValueId)>,
    type_hint: Option<crate::mir::MirType>,
    tag: &str,
) -> Result<(), String> {
    let prepared_completion = crate::mir::builder::phi_completion::prepare_for_builder(
        builder, block, dst, &inputs, type_hint,
    )?;

    define_final_from_prepared_completion(builder, prepared_completion, tag)
}

/// Execute the one shared final-PHI physical commit after a route-specific or
/// generic completion owner has already prepared logical input/type facts.
///
/// This is not a fifth preparation API: it cannot construct a completion or
/// publish a type fact before the existing PHI insertion succeeds.
pub(in crate::mir::builder) fn define_final_from_prepared_completion(
    builder: &mut MirBuilder,
    prepared_completion: crate::mir::builder::phi_completion::PreparedPhiCompletionV1,
    tag: &str,
) -> Result<(), String> {
    let block = prepared_completion.draft().block();
    let dst = prepared_completion.draft().dst();
    let type_hint = prepared_completion.draft().type_hint().cloned();
    let mut inputs = prepared_completion.logical_inputs().to_vec();

    let span = builder.metadata_ctx.current_span();
    let origin_caller = if crate::config::env::joinir_dev::debug_enabled() {
        builder.record_value_origin_caller(dst, std::panic::Location::caller());
        builder.value_origin_caller(dst).map(str::to_owned)
    } else {
        None
    };
    {
        let func = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                format!(
                "[freeze:contract][phi_lifecycle/define_no_function] tag={} No current function",
                tag
            )
            })?;

        for (pred, incoming) in &mut inputs {
            *incoming = crate::mir::builder::ssa::phi_input_materializer::for_pred(
                func, *pred, *incoming, tag, "phi",
            )?;
        }

        if let Some(loc) = origin_caller {
            func.metadata.value_origin_callers.insert(dst, loc);
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
        insert_phi_at_head_spanned_with_type_hint(func, block, dst, inputs, type_hint, span)
            .map_err(|e| format!("{e} op=define_phi_final tag={tag}"))?;
    }

    crate::mir::builder::phi_completion::commit_for_builder(
        builder,
        prepared_completion.after_instruction_commit(),
    );
    Ok(())
}

/// One item in a lifecycle-owned PHI batch prepend operation.
#[derive(Debug)]
pub(in crate::mir::builder) struct PhiBatchItem {
    pub dst: ValueId,
    pub inputs: Vec<(BasicBlockId, ValueId)>,
    pub type_hint: Option<MirType>,
    pub span: Span,
    pub item_tag: String,
}

/// Define an ordered batch of PHIs before the existing block body.
///
/// This shape is used by loop-header PHIs. The API materializes and validates
/// all inputs before mutating the target block, then prepends instructions and
/// spans in one low-level operation.
#[track_caller]
pub(in crate::mir::builder) fn define_phi_batch_prepend(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    items: Vec<PhiBatchItem>,
    tag: &str,
) -> Result<(), String> {
    batch_type_publication::define_phi_batch_prepend(builder, block, items, tag)
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
    inputs: Vec<(BasicBlockId, ValueId)>,
    span: crate::ast::Span,
) -> Result<(), String> {
    define_phi_final_fn_with_type_hint_and_tag(
        function,
        block,
        dst,
        inputs,
        None,
        span,
        "edgecfg_block_params",
    )
}

/// Define a final PHI with all inputs (function-level API) and a type hint.
#[track_caller]
pub(in crate::mir) fn define_phi_final_fn_with_type_hint_and_tag(
    function: &mut crate::mir::MirFunction,
    block: BasicBlockId,
    dst: ValueId,
    mut inputs: Vec<(BasicBlockId, ValueId)>,
    type_hint: Option<MirType>,
    span: crate::ast::Span,
    tag: &str,
) -> Result<(), String> {
    for (pred, incoming) in &mut inputs {
        *incoming = crate::mir::builder::ssa::phi_input_materializer::for_pred(
            function, *pred, *incoming, tag, "phi",
        )?;
    }

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
            "[phi_lifecycle/define] fn={} bb={:?} dst=%{} tag={}",
            function.signature.name, block, dst.0, tag
        ));
    }

    // Insert PHI with complete inputs (function-level)
    insert_phi_at_head_spanned_with_type_hint(function, block, dst, inputs, type_hint, span)
        .map_err(|e| format!("{e} op=define_phi_final_fn tag={tag}"))?;

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
    let type_hint = phi_type_hint_for_patch(builder, block, dst, tag)?;
    let prepared_completion = crate::mir::builder::phi_completion::prepare_for_builder(
        builder, block, dst, &inputs, type_hint,
    )?;
    inputs = prepared_completion.logical_inputs().to_vec();

    if crate::config::env::joinir_dev::debug_enabled() {
        let mut detail = format!(" inputs={}", inputs.len());
        for (i, (pred, incoming)) in inputs.iter().take(2).enumerate() {
            detail.push_str(&format!(
                " phi{}_pred={:?} phi{}_in=%{}",
                i, pred, i, incoming.0
            ));
        }
        let fn_name = builder
            .function_state
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
        .map_err(|e| format!("{e} op=patch_phi_inputs tag={tag}"))?;
    crate::mir::builder::phi_completion::commit_for_builder(
        builder,
        prepared_completion.after_instruction_commit(),
    );
    Ok(())
}

fn phi_type_hint_for_patch(
    builder: &MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    tag: &str,
) -> Result<Option<MirType>, String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| format!("[freeze:contract][phi_lifecycle/patch_no_function] tag={tag}"))?;
    let block_data = function.get_block(block).ok_or_else(|| {
        format!("[freeze:contract][phi_lifecycle/patch_missing_block] bb={block} tag={tag}")
    })?;
    block_data
        .instructions
        .iter()
        .find_map(|instruction| match instruction {
            crate::mir::MirInstruction::Phi {
                dst: phi_dst,
                type_hint,
                ..
            } if *phi_dst == dst => Some(type_hint.clone()),
            _ => None,
        })
        .ok_or_else(|| {
            format!(
                "[freeze:contract][phi_lifecycle/patch_missing_phi] bb={block} dst=%{} tag={tag}",
                dst.0
            )
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
        .function_state
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
