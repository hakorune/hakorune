//! 🎯 箱理論: canonical function-session context lifecycle.
//!
//! `FunctionOwnedStateTransactionV1` owns the one move-only FunctionOwned
//! caller transition. Legacy compatibility and observation state stay in their
//! own small snapshots: neither is function-lowering storage.

use crate::ast::Span;
use crate::mir::builder::function_state_transaction::{
    FunctionOwnedStateTransactionV1, FunctionStateCaptureModeV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RegionId;

/// Compatibility context that remains outside FunctionOwned lowering state.
#[derive(Debug)]
struct LegacyCompatibilitySnapshotV1 {
    saved_static_ctx: Option<String>,
}

/// Read-only observation state borrowed around one child lowering session.
#[derive(Debug)]
struct ObservationBorrowSnapshotV1 {
    saved_slot_registry: Option<FunctionSlotRegistry>,
    saved_debug_scope_stack: Vec<String>,
    saved_current_span: Span,
    saved_region_stack: Vec<RegionId>,
    saved_recursion_depth: usize,
}

/// One canonical function-lowering context.
///
/// Its FunctionOwned part is deliberately opaque and move-only. The remaining
/// snapshots are not FunctionOwned and therefore stay outside that transaction.
pub(super) struct LoweringContext {
    function_state_transaction: FunctionOwnedStateTransactionV1,
    legacy: LegacyCompatibilitySnapshotV1,
    observation: ObservationBorrowSnapshotV1,
}

impl LoweringContext {
    pub(super) fn saved_region_stack(&self) -> &[RegionId] {
        &self.observation.saved_region_stack
    }
}

impl MirBuilder {
    /// Establish the existing child lowering state through the single
    /// FunctionOwned transition. No source, type, route, or MIR decision is
    /// made here.
    pub(super) fn prepare_lowering_context(&mut self, func_name: &str) -> LoweringContext {
        let legacy = LegacyCompatibilitySnapshotV1 {
            saved_static_ctx: self.comp_ctx.current_static_box.clone(),
        };
        if let Some(pos) = func_name.find('.') {
            let box_name = &func_name[..pos];
            if !box_name.is_empty() {
                self.comp_ctx.current_static_box = Some(box_name.to_string());
            }
        }

        let mode = if self.comp_ctx.compilation_context.is_some() {
            FunctionStateCaptureModeV1::BoxCompilationPartialClear
        } else {
            FunctionStateCaptureModeV1::LegacyRestore
        };
        let function_state_transaction =
            FunctionOwnedStateTransactionV1::begin(&mut self.function_state, mode);

        let observation = ObservationBorrowSnapshotV1 {
            saved_slot_registry: self.comp_ctx.current_slot_registry.take(),
            saved_debug_scope_stack: std::mem::take(&mut self.scope_ctx.debug_scope_stack),
            saved_current_span: self.metadata_ctx.current_span(),
            saved_region_stack: self.metadata_ctx.current_region_stack().to_vec(),
            saved_recursion_depth: self.recursion_depth,
        };
        // Keep the observation entry helper as the sole debug-scope clear
        // owner. The transaction has already moved FunctionOwned scope state.
        self.scope_ctx.clear_debug_scope_for_function_entry();
        self.recursion_depth = 0;

        LoweringContext {
            function_state_transaction,
            legacy,
            observation,
        }
    }

    /// Consume the FunctionOwned transition exactly once, then restore the
    /// independent compatibility and observation snapshots.
    pub(super) fn restore_lowering_context(&mut self, ctx: LoweringContext) {
        ctx.function_state_transaction
            .restore(&mut self.function_state);

        self.comp_ctx.current_static_box = ctx.legacy.saved_static_ctx;
        self.comp_ctx.current_slot_registry = ctx.observation.saved_slot_registry;
        self.scope_ctx.debug_scope_stack = ctx.observation.saved_debug_scope_stack;
        self.metadata_ctx
            .set_current_span(ctx.observation.saved_current_span);
        while self.metadata_ctx.pop_region().is_some() {}
        for region in ctx.observation.saved_region_stack {
            self.metadata_ctx.push_region(region);
        }
        self.recursion_depth = ctx.observation.saved_recursion_depth;
    }
}
