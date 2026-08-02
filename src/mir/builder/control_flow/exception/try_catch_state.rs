//! Narrow transient-state owner for one raw TryCatch region.
//!
//! This transaction deliberately owns only the total protected-region state
//! touched by TryCatch. It restores it on successful completion only:
//! typed failures retain the exact partially-mutated state of the historical
//! lowering path, while the outer candidate session remains responsible for
//! discarding unpublished Builder effects.

use crate::mir::builder::function_lowering_state::{
    FunctionLoweringStateV1, ProtectedRegionTransientStateV1,
};
use crate::mir::{BasicBlockId, ValueId};

pub(super) struct ActiveRawTryCatchFunctionStateV1 {
    caller: ProtectedRegionTransientStateV1,
}

impl ActiveRawTryCatchFunctionStateV1 {
    pub(super) fn begin(
        state: &mut FunctionLoweringStateV1,
        return_slot: ValueId,
        return_target: BasicBlockId,
    ) -> Self {
        let caller = state.protected_region;
        state
            .protected_region
            .return_defer
            .activate(return_slot, return_target);
        Self { caller }
    }

    pub(super) fn enter_cleanup(
        &self,
        state: &mut FunctionLoweringStateV1,
        allow_return: bool,
        allow_throw: bool,
    ) {
        state.protected_region.cleanup.active = true;
        state.protected_region.cleanup.allow_return = allow_return;
        state.protected_region.cleanup.allow_throw = allow_throw;
        state.protected_region.return_defer.deactivate_for_cleanup();
    }

    pub(super) fn leave_cleanup(&self, state: &mut FunctionLoweringStateV1) {
        state.protected_region.cleanup.active = false;
    }

    pub(super) fn complete_success(
        self,
        state: &mut FunctionLoweringStateV1,
        value: ValueId,
    ) -> CompletedRawTryCatchV1 {
        state.protected_region = self.caller;
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
        state.protected_region.return_defer =
            crate::mir::builder::function_lowering_state::ReturnDeferTransientStateV1::inactive_with_retained_destination_for_test(
                ValueId(41),
                BasicBlockId(42),
                true,
            );
        state.protected_region.cleanup.active = true;
        state.protected_region.cleanup.allow_return = true;
        state.protected_region.cleanup.allow_throw = false;
        state
    }

    #[test]
    fn success_restores_exact_protected_region_caller_state() {
        let mut state = seeded_state();
        let caller = state.protected_region;
        let transaction =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(7), BasicBlockId(8));
        transaction.enter_cleanup(&mut state, false, true);
        transaction.leave_cleanup(&mut state);

        assert_eq!(
            transaction
                .complete_success(&mut state, ValueId(9))
                .into_value(),
            ValueId(9)
        );
        assert_eq!(state.protected_region, caller);
    }

    #[test]
    fn rejection_preserves_inner_state_without_restoring_caller() {
        let mut state = seeded_state();
        let transaction =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(7), BasicBlockId(8));
        transaction.enter_cleanup(&mut state, false, true);
        let rejected = transaction.reject("primary".to_string());
        assert_eq!(rejected.error(), "primary");
        rejected.discard();

        assert!(!state.protected_region.return_defer.is_active());
        assert_eq!(
            state.protected_region.return_defer.retained_slot(),
            Some(ValueId(7))
        );
        assert_eq!(
            state.protected_region.return_defer.retained_target(),
            Some(BasicBlockId(8))
        );
        assert!(!state.protected_region.return_defer.emitted());
        assert!(state.protected_region.cleanup.active);
        assert!(!state.protected_region.cleanup.allow_return);
        assert!(state.protected_region.cleanup.allow_throw);
    }

    #[test]
    fn nested_success_restores_outer_state_before_caller_state() {
        let mut state = seeded_state();
        let caller = state.protected_region;
        let outer =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(10), BasicBlockId(11));
        outer.enter_cleanup(&mut state, false, true);
        let outer_installed = state.protected_region;

        let inner =
            ActiveRawTryCatchFunctionStateV1::begin(&mut state, ValueId(12), BasicBlockId(13));
        inner.complete_success(&mut state, ValueId(14)).into_value();
        assert_eq!(state.protected_region, outer_installed);

        outer.complete_success(&mut state, ValueId(15)).into_value();
        assert_eq!(state.protected_region, caller);
    }
}
