//! Private move-only vocabulary for the S0c function-state transaction.
//!
//! The canonical lifecycle is the sole production consumer of this product.
//! In particular,
//! this module must not take or replace the whole `FunctionLoweringStateV1`:
//! C0 owns fresh child-session construction.

use std::collections::HashMap;

use hakorune_mir_builder::BindingContext;

use crate::mir::builder::function_lowering_state::{
    FunctionCompilationScratchV1, FunctionLoweringStateV1, FunctionScopeStateV1,
    ProtectedRegionTransientStateV1,
};
use crate::mir::builder::type_context::TypeContext;
use crate::mir::builder::variable_context::VariableContext;
use crate::mir::builder::vars::resolved_binding_state::ResolvedBindingLoweringStateV1;
use crate::mir::builder::FragEmitSession;
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
    pub(super) protected_region: ProtectedRegionTransientStateV1,
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
            protected_region: state.protected_region,
            suppress_pin_entry_copy_next: state.suppress_pin_entry_copy_next,
            in_unified_boxcall_fallback: state.in_unified_boxcall_fallback,
        };

        state.enter_fresh_child_transient_control_v1();

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
        state.protected_region = caller.protected_region;
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
    use crate::mir::value_kind::MirValueKind;
    use crate::mir::{BindingId, MirType};

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

    #[test]
    fn legacy_mode_moves_and_restores_the_captured_function_owned_subset() {
        let mut state = FunctionLoweringStateV1::default();
        state.current_block = Some(BasicBlockId::new(10));
        state
            .variable_ctx
            .insert("outer_value".into(), ValueId::new(11));
        state
            .type_ctx
            .value_types
            .insert(ValueId::new(11), MirType::Integer);
        state
            .type_ctx
            .value_kinds
            .insert(ValueId::new(12), MirValueKind::Local(0));
        state
            .type_ctx
            .value_origin_newbox
            .insert(ValueId::new(13), "OuterBox".into());
        state
            .type_ctx
            .string_literals
            .insert(ValueId::new(14), "outer".into());
        state
            .type_ctx
            .map_value_types
            .insert(ValueId::new(15), MirType::Bool);
        state
            .type_ctx
            .map_literal_value_types
            .insert((ValueId::new(16), "key".into()), MirType::String);
        state
            .binding_ctx
            .insert("outer_binding".into(), BindingId::new(17));
        state.scope.loop_header_stack.push(BasicBlockId::new(18));
        state.compilation.reserve_value_id(ValueId::new(19));
        state
            .pending_phis
            .push((BasicBlockId::new(20), ValueId::new(21), "outer".into()));
        state.local_ssa_map.insert(
            (BasicBlockId::new(22), ValueId::new(23), 0),
            ValueId::new(24),
        );
        state.protected_region.return_defer.active = true;
        state.protected_region.return_defer.slot = Some(ValueId::new(25));
        state.protected_region.return_defer.target = Some(BasicBlockId::new(26));
        state.protected_region.return_defer.emitted = true;
        state.protected_region.cleanup.active = true;
        state.protected_region.cleanup.allow_return = true;
        state.protected_region.cleanup.allow_throw = true;
        state.suppress_pin_entry_copy_next = true;
        state.in_unified_boxcall_fallback = true;

        let transaction = FunctionOwnedStateTransactionV1::begin(
            &mut state,
            FunctionStateCaptureModeV1::LegacyRestore,
        );

        assert!(state.current_block.is_none());
        assert!(state.variable_ctx.lookup("outer_value").is_none());
        assert!(state.type_ctx.value_types.is_empty());
        assert!(state.type_ctx.value_kinds.is_empty());
        assert!(state.type_ctx.value_origin_newbox.is_empty());
        assert!(state.type_ctx.string_literals.is_empty());
        assert!(state.type_ctx.map_value_types.is_empty());
        assert!(state.type_ctx.map_literal_value_types.is_empty());
        assert!(state.binding_ctx.is_empty());
        assert!(state.scope.loop_header_stack.is_empty());
        assert!(!state.compilation.is_reserved_value_id(ValueId::new(19)));
        assert!(state.pending_phis.is_empty());
        assert!(state.local_ssa_map.is_empty());
        assert!(!state.protected_region.return_defer.active);
        assert!(state.protected_region.return_defer.slot.is_none());
        assert!(state.protected_region.return_defer.target.is_none());
        assert!(!state.protected_region.return_defer.emitted);
        assert!(!state.protected_region.cleanup.active);
        assert!(!state.protected_region.cleanup.allow_return);
        assert!(!state.protected_region.cleanup.allow_throw);
        assert!(!state.suppress_pin_entry_copy_next);
        assert!(!state.in_unified_boxcall_fallback);

        state
            .variable_ctx
            .insert("child_value".into(), ValueId::new(30));
        state
            .type_ctx
            .value_types
            .insert(ValueId::new(30), MirType::Float);
        state
            .pending_phis
            .push((BasicBlockId::new(31), ValueId::new(32), "child".into()));
        transaction.restore(&mut state);

        assert_eq!(state.current_block, Some(BasicBlockId::new(10)));
        assert_eq!(
            state.variable_ctx.lookup("outer_value"),
            Some(ValueId::new(11))
        );
        assert!(state.variable_ctx.lookup("child_value").is_none());
        assert_eq!(
            state.type_ctx.value_types[&ValueId::new(11)],
            MirType::Integer
        );
        assert!(!state.type_ctx.value_types.contains_key(&ValueId::new(30)));
        assert_eq!(
            state.type_ctx.value_kinds[&ValueId::new(12)],
            MirValueKind::Local(0)
        );
        assert_eq!(
            state.type_ctx.value_origin_newbox[&ValueId::new(13)],
            "OuterBox"
        );
        assert_eq!(state.type_ctx.string_literals[&ValueId::new(14)], "outer");
        assert_eq!(
            state.type_ctx.map_value_types[&ValueId::new(15)],
            MirType::Bool
        );
        assert_eq!(
            state.type_ctx.map_literal_value_types[&(ValueId::new(16), "key".into())],
            MirType::String
        );
        assert_eq!(
            state.binding_ctx.lookup("outer_binding"),
            Some(BindingId::new(17))
        );
        assert_eq!(state.scope.loop_header_stack, vec![BasicBlockId::new(18)]);
        assert!(state.compilation.is_reserved_value_id(ValueId::new(19)));
        assert_eq!(state.pending_phis.len(), 1);
        assert_eq!(state.local_ssa_map.len(), 1);
        assert!(state.protected_region.return_defer.active);
        assert_eq!(
            state.protected_region.return_defer.slot,
            Some(ValueId::new(25))
        );
        assert_eq!(
            state.protected_region.return_defer.target,
            Some(BasicBlockId::new(26))
        );
        assert!(state.protected_region.return_defer.emitted);
        assert!(state.protected_region.cleanup.active);
        assert!(state.protected_region.cleanup.allow_return);
        assert!(state.protected_region.cleanup.allow_throw);
        assert!(state.suppress_pin_entry_copy_next);
        assert!(state.in_unified_boxcall_fallback);
    }

    #[test]
    fn box_mode_keeps_the_existing_three_clear_three_retain_type_action() {
        let mut state = FunctionLoweringStateV1::default();
        state
            .variable_ctx
            .insert("outer_value".into(), ValueId::new(40));
        state
            .type_ctx
            .value_types
            .insert(ValueId::new(41), MirType::Integer);
        state
            .type_ctx
            .value_kinds
            .insert(ValueId::new(42), MirValueKind::Local(0));
        state
            .type_ctx
            .value_origin_newbox
            .insert(ValueId::new(43), "OuterBox".into());
        state
            .type_ctx
            .string_literals
            .insert(ValueId::new(44), "outer".into());
        state
            .type_ctx
            .map_value_types
            .insert(ValueId::new(45), MirType::Bool);
        state
            .type_ctx
            .map_literal_value_types
            .insert((ValueId::new(46), "key".into()), MirType::String);

        let transaction = FunctionOwnedStateTransactionV1::begin(
            &mut state,
            FunctionStateCaptureModeV1::BoxCompilationPartialClear,
        );

        assert!(state.variable_ctx.lookup("outer_value").is_none());
        assert!(state.type_ctx.value_types.is_empty());
        assert!(state.type_ctx.value_kinds.is_empty());
        assert!(state.type_ctx.value_origin_newbox.is_empty());
        assert_eq!(state.type_ctx.string_literals[&ValueId::new(44)], "outer");
        assert_eq!(
            state.type_ctx.map_value_types[&ValueId::new(45)],
            MirType::Bool
        );
        assert_eq!(
            state.type_ctx.map_literal_value_types[&(ValueId::new(46), "key".into())],
            MirType::String
        );

        state
            .variable_ctx
            .insert("child_value".into(), ValueId::new(50));
        state
            .type_ctx
            .value_types
            .insert(ValueId::new(51), MirType::Float);
        state
            .type_ctx
            .value_kinds
            .insert(ValueId::new(52), MirValueKind::Local(1));
        state
            .type_ctx
            .value_origin_newbox
            .insert(ValueId::new(53), "ChildBox".into());
        state
            .type_ctx
            .string_literals
            .insert(ValueId::new(54), "child".into());
        state
            .type_ctx
            .map_value_types
            .insert(ValueId::new(55), MirType::Float);
        state
            .type_ctx
            .map_literal_value_types
            .insert((ValueId::new(56), "child-key".into()), MirType::Bool);
        transaction.restore(&mut state);

        assert!(state.variable_ctx.lookup("outer_value").is_none());
        assert!(state.variable_ctx.lookup("child_value").is_none());
        assert!(state.type_ctx.value_types.is_empty());
        assert!(state.type_ctx.value_kinds.is_empty());
        assert!(state.type_ctx.value_origin_newbox.is_empty());
        assert_eq!(state.type_ctx.string_literals[&ValueId::new(44)], "outer");
        assert_eq!(state.type_ctx.string_literals[&ValueId::new(54)], "child");
        assert_eq!(
            state.type_ctx.map_value_types[&ValueId::new(45)],
            MirType::Bool
        );
        assert_eq!(
            state.type_ctx.map_value_types[&ValueId::new(55)],
            MirType::Float
        );
        assert_eq!(
            state.type_ctx.map_literal_value_types[&(ValueId::new(46), "key".into())],
            MirType::String
        );
        assert_eq!(
            state.type_ctx.map_literal_value_types[&(ValueId::new(56), "child-key".into())],
            MirType::Bool
        );
    }
}
