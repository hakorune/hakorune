//! Private move-only vocabulary for the S0c function-state transaction.
//!
//! S0c-S0 deliberately has no production lifecycle consumer. The later I0
//! connection alone may capture and restore these products. In particular,
//! this module must not take or replace the whole `FunctionLoweringStateV1`:
//! C0 owns fresh child-session construction.

use std::collections::HashMap;

use hakorune_mir_builder::BindingContext;

use crate::mir::builder::control_flow::edgecfg::api::FragEmitSession;
use crate::mir::builder::function_lowering_state::{
    FunctionCompilationScratchV1, FunctionLoweringStateV1, FunctionScopeStateV1,
};
use crate::mir::builder::type_context::TypeContext;
use crate::mir::builder::variable_context::VariableContext;
use crate::mir::builder::vars::resolved_binding_state::ResolvedBindingLoweringStateV1;
use crate::mir::{BasicBlockId, MirFunction, ValueId};

/// Selects the exact pre-existing action for variable/type facts.
///
/// `LegacyRestore` moves all six TypeContext maps with the caller state.
/// `BoxCompilationPartialClear` keeps the existing three-clear/three-retain
/// behavior and therefore must not capture `VariableContext` or `TypeContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionStateCaptureModeV1 {
    LegacyRestore,
    BoxCompilationPartialClear,
}

/// The legacy-only function-local value state that moves with a caller.
///
/// This is intentionally absent in the BoxCompilationContext disposition.
#[derive(Debug)]
pub(super) struct LegacyFunctionValueStateV1 {
    pub(super) variable_ctx: VariableContext,
    pub(super) type_ctx: TypeContext,
}

/// The exact FunctionOwned subset restored to the caller after nested lowering.
///
/// `FunctionValueOriginFactsV1` is intentionally absent: METAISO owns its
/// no-snapshot/no-clear/no-restore behavior. LegacyCompatibility and
/// ObservationBorrow products are likewise separate from this payload.
#[derive(Debug)]
pub(super) struct CapturedFunctionOwnedStateV1 {
    pub(super) current_function: Option<MirFunction>,
    pub(super) current_block: Option<BasicBlockId>,
    pub(super) legacy_value_state: Option<LegacyFunctionValueStateV1>,
    pub(super) binding_ctx: BindingContext,
    pub(super) resolved_binding_state: ResolvedBindingLoweringStateV1,
    pub(super) scope: FunctionScopeStateV1,
    pub(super) compilation: FunctionCompilationScratchV1,
    pub(super) pending_phis: Vec<(BasicBlockId, ValueId, String)>,
    pub(super) local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>,
    pub(super) schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>,
    pub(super) pin_slot_names: HashMap<ValueId, String>,
    pub(super) frag_emit_session: FragEmitSession,
    pub(super) return_defer_active: bool,
    pub(super) return_defer_slot: Option<ValueId>,
    pub(super) return_defer_target: Option<BasicBlockId>,
    pub(super) return_deferred_emitted: bool,
    pub(super) in_cleanup_block: bool,
    pub(super) cleanup_allow_return: bool,
    pub(super) cleanup_allow_throw: bool,
    pub(super) suppress_pin_entry_copy_next: bool,
    pub(super) in_unified_boxcall_fallback: bool,
}

/// One-shot owner for the captured FunctionOwned caller subset.
///
/// The `Option` makes the eventual restore transition consuming. This product
/// has no `Clone`, `Copy`, `Deref`, whole-state replacement, or Builder field;
/// I0 will connect it to the existing canonical session close/drop owner.
#[derive(Debug)]
pub(super) struct FunctionOwnedStateTransactionV1 {
    mode: FunctionStateCaptureModeV1,
    caller: Option<CapturedFunctionOwnedStateV1>,
}

impl FunctionOwnedStateTransactionV1 {
    /// Move the legacy captured caller subset out and establish the existing
    /// in-place child entry state. No whole FunctionLoweringState is moved.
    pub(super) fn begin(
        state: &mut FunctionLoweringStateV1,
        mode: FunctionStateCaptureModeV1,
    ) -> Self {
        let legacy_value_state = match mode {
            FunctionStateCaptureModeV1::LegacyRestore => Some(LegacyFunctionValueStateV1 {
                variable_ctx: std::mem::take(&mut state.variable_ctx),
                type_ctx: std::mem::take(&mut state.type_ctx),
            }),
            FunctionStateCaptureModeV1::BoxCompilationPartialClear => None,
        };
        let caller = CapturedFunctionOwnedStateV1 {
            current_function: state.current_function.take(),
            current_block: state.current_block.take(),
            legacy_value_state,
            binding_ctx: std::mem::take(&mut state.binding_ctx),
            resolved_binding_state: std::mem::take(&mut state.resolved_binding_state),
            scope: std::mem::take(&mut state.scope),
            compilation: std::mem::take(&mut state.compilation),
            pending_phis: std::mem::take(&mut state.pending_phis),
            local_ssa_map: std::mem::take(&mut state.local_ssa_map),
            schedule_mat_map: std::mem::take(&mut state.schedule_mat_map),
            pin_slot_names: std::mem::take(&mut state.pin_slot_names),
            frag_emit_session: std::mem::take(&mut state.frag_emit_session),
            return_defer_active: state.return_defer_active,
            return_defer_slot: state.return_defer_slot,
            return_defer_target: state.return_defer_target,
            return_deferred_emitted: state.return_deferred_emitted,
            in_cleanup_block: state.in_cleanup_block,
            cleanup_allow_return: state.cleanup_allow_return,
            cleanup_allow_throw: state.cleanup_allow_throw,
            suppress_pin_entry_copy_next: state.suppress_pin_entry_copy_next,
            in_unified_boxcall_fallback: state.in_unified_boxcall_fallback,
        };

        state.return_defer_active = false;
        state.return_defer_slot = None;
        state.return_defer_target = None;
        state.return_deferred_emitted = false;
        state.in_cleanup_block = false;
        state.cleanup_allow_return = false;
        state.cleanup_allow_throw = false;
        state.suppress_pin_entry_copy_next = false;
        state.in_unified_boxcall_fallback = false;

        if mode == FunctionStateCaptureModeV1::BoxCompilationPartialClear {
            state.variable_ctx = VariableContext::new();
            Self::clear_box_compilation_type_facts(state);
        }

        Self {
            mode,
            caller: Some(caller),
        }
    }

    /// Consume this transaction and restore the caller state exactly once.
    pub(super) fn restore(mut self, state: &mut FunctionLoweringStateV1) {
        let caller = self
            .caller
            .take()
            .expect("function-owned transaction is restored once");

        state.current_function = caller.current_function;
        state.current_block = caller.current_block;
        if let Some(legacy) = caller.legacy_value_state {
            state.variable_ctx = legacy.variable_ctx;
            state.type_ctx = legacy.type_ctx;
        } else {
            debug_assert_eq!(
                self.mode,
                FunctionStateCaptureModeV1::BoxCompilationPartialClear
            );
            state.variable_ctx.variable_map.clear();
            Self::clear_box_compilation_type_facts(state);
        }
        state.binding_ctx = caller.binding_ctx;
        state.resolved_binding_state = caller.resolved_binding_state;
        state.scope = caller.scope;
        state.compilation = caller.compilation;
        state.pending_phis = caller.pending_phis;
        state.local_ssa_map = caller.local_ssa_map;
        state.schedule_mat_map = caller.schedule_mat_map;
        state.pin_slot_names = caller.pin_slot_names;
        state.frag_emit_session = caller.frag_emit_session;
        state.return_defer_active = caller.return_defer_active;
        state.return_defer_slot = caller.return_defer_slot;
        state.return_defer_target = caller.return_defer_target;
        state.return_deferred_emitted = caller.return_deferred_emitted;
        state.in_cleanup_block = caller.in_cleanup_block;
        state.cleanup_allow_return = caller.cleanup_allow_return;
        state.cleanup_allow_throw = caller.cleanup_allow_throw;
        state.suppress_pin_entry_copy_next = caller.suppress_pin_entry_copy_next;
        state.in_unified_boxcall_fallback = caller.in_unified_boxcall_fallback;
    }

    fn clear_box_compilation_type_facts(state: &mut FunctionLoweringStateV1) {
        state.type_ctx.value_types.clear();
        state.type_ctx.value_kinds.clear();
        state.type_ctx.value_origin_newbox.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_modes_are_disjoint_and_transaction_has_one_caller_slot() {
        assert_ne!(
            FunctionStateCaptureModeV1::LegacyRestore,
            FunctionStateCaptureModeV1::BoxCompilationPartialClear
        );
        let mut state = FunctionLoweringStateV1::default();
        let transaction = FunctionOwnedStateTransactionV1::begin(
            &mut state,
            FunctionStateCaptureModeV1::LegacyRestore,
        );
        transaction.restore(&mut state);
    }
}
