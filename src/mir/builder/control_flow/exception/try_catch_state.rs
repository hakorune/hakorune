//! Narrow transient-state owner for one raw TryCatch region.
//!
//! This transaction deliberately owns only the seven defer/cleanup fields
//! touched by TryCatch. It restores them on successful completion only:
//! typed failures retain the exact partially-mutated state of the historical
//! lowering path, while the outer candidate session remains responsible for
//! discarding unpublished Builder effects.

use crate::mir::builder::function_lowering_state::FunctionLoweringStateV1;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RawTryCatchCallerFunctionStateV1 {
    return_defer_active: bool,
    return_defer_slot: Option<ValueId>,
    return_defer_target: Option<BasicBlockId>,
    return_deferred_emitted: bool,
    in_cleanup_block: bool,
    cleanup_allow_return: bool,
    cleanup_allow_throw: bool,
}

impl RawTryCatchCallerFunctionStateV1 {
    fn capture(state: &FunctionLoweringStateV1) -> Self {
        Self {
            return_defer_active: state.return_defer_active,
            return_defer_slot: state.return_defer_slot,
            return_defer_target: state.return_defer_target,
            return_deferred_emitted: state.return_deferred_emitted,
            in_cleanup_block: state.in_cleanup_block,
            cleanup_allow_return: state.cleanup_allow_return,
            cleanup_allow_throw: state.cleanup_allow_throw,
        }
    }

    fn restore(self, state: &mut FunctionLoweringStateV1) {
        state.return_defer_active = self.return_defer_active;
        state.return_defer_slot = self.return_defer_slot;
        state.return_defer_target = self.return_defer_target;
        state.return_deferred_emitted = self.return_deferred_emitted;
        state.in_cleanup_block = self.in_cleanup_block;
        state.cleanup_allow_return = self.cleanup_allow_return;
        state.cleanup_allow_throw = self.cleanup_allow_throw;
    }
}

pub(super) struct ActiveRawTryCatchFunctionStateV1 {
    caller: RawTryCatchCallerFunctionStateV1,
}

impl ActiveRawTryCatchFunctionStateV1 {
    pub(super) fn begin(
        state: &mut FunctionLoweringStateV1,
        return_slot: ValueId,
        return_target: BasicBlockId,
    ) -> Self {
        let caller = RawTryCatchCallerFunctionStateV1::capture(state);
        state.return_defer_active = true;
        state.return_defer_slot = Some(return_slot);
        state.return_defer_target = Some(return_target);
        state.return_deferred_emitted = false;
        Self { caller }
    }

    pub(super) fn enter_cleanup(
        &self,
        state: &mut FunctionLoweringStateV1,
        allow_return: bool,
        allow_throw: bool,
    ) {
        state.in_cleanup_block = true;
        state.cleanup_allow_return = allow_return;
        state.cleanup_allow_throw = allow_throw;
        state.return_defer_active = false;
    }

    pub(super) fn leave_cleanup(&self, state: &mut FunctionLoweringStateV1) {
        state.in_cleanup_block = false;
    }

    pub(super) fn complete_success(
        self,
        state: &mut FunctionLoweringStateV1,
        value: ValueId,
    ) -> CompletedRawTryCatchV1 {
        self.caller.restore(state);
        CompletedRawTryCatchV1 { value }
    }

    pub(super) fn reject(self, error: String) -> RejectedRawTryCatchV1 {
        RejectedRawTryCatchV1 { error }
    }
}

pub(super) struct CompletedRawTryCatchV1 {
    value: ValueId,
}

impl CompletedRawTryCatchV1 {
    pub(super) fn into_value(self) -> ValueId {
        self.value
    }
}

pub(super) struct RejectedRawTryCatchV1 {
    error: String,
}

impl RejectedRawTryCatchV1 {
    pub(super) fn error(&self) -> &str {
        &self.error
    }

    pub(super) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_state() -> FunctionLoweringStateV1 {
        let mut state = FunctionLoweringStateV1::default();
        state.return_defer_active = false;
        state.return_defer_slot = Some(ValueId(41));
        state.return_defer_target = Some(BasicBlockId(42));
        state.return_deferred_emitted = true;
        state.in_cleanup_block = true;
        state.cleanup_allow_return = true;
        state.cleanup_allow_throw = false;
        state
    }

    #[test]
    fn success_restores_exact_seven_field_caller_state() {
        let mut state = seeded_state();
        let caller = RawTryCatchCallerFunctionStateV1::capture(&state);
        let transaction =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(7), BasicBlockId(8));
        transaction.enter_cleanup(&mut state, false, true);
        state.return_deferred_emitted = false;
        transaction.leave_cleanup(&mut state);

        assert_eq!(
            transaction
                .complete_success(&mut state, ValueId(9))
                .into_value(),
            ValueId(9)
        );
        assert_eq!(RawTryCatchCallerFunctionStateV1::capture(&state), caller);
    }

    #[test]
    fn rejection_preserves_inner_state_without_restoring_caller() {
        let mut state = seeded_state();
        let transaction =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(7), BasicBlockId(8));
        transaction.enter_cleanup(&mut state, false, true);
        state.return_deferred_emitted = false;

        let rejected = transaction.reject("primary".to_string());
        assert_eq!(rejected.error(), "primary");
        rejected.discard();

        assert!(!state.return_defer_active);
        assert_eq!(state.return_defer_slot, Some(ValueId(7)));
        assert_eq!(state.return_defer_target, Some(BasicBlockId(8)));
        assert!(!state.return_deferred_emitted);
        assert!(state.in_cleanup_block);
        assert!(!state.cleanup_allow_return);
        assert!(state.cleanup_allow_throw);
    }

    #[test]
    fn nested_success_restores_outer_state_before_caller_state() {
        let mut state = seeded_state();
        let caller = RawTryCatchCallerFunctionStateV1::capture(&state);
        let outer =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(10), BasicBlockId(11));
        outer.enter_cleanup(&mut state, false, true);
        let outer_installed = RawTryCatchCallerFunctionStateV1::capture(&state);

        let inner =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(12), BasicBlockId(13));
        inner.complete_success(&mut state, ValueId(14)).into_value();
        assert_eq!(
            RawTryCatchCallerFunctionStateV1::capture(&state),
            outer_installed
        );

        outer.complete_success(&mut state, ValueId(15)).into_value();
        assert_eq!(RawTryCatchCallerFunctionStateV1::capture(&state), caller);
    }
}
