//! HEADERPORT0-REENTRANT-TERM0-I0-STATE0-S0: invocation state seam.
//!
//! The state is a disconnected ownership product.  It keeps the invocation's
//! shell and draft collector together without exposing a module function map
//! or adding a second Builder/fact store.  Production roots remain unchanged
//! until the later STATE0-I0 cutover.

use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_lowering_shell::ModuleLoweringShellV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootCompletionStateV1 {
    MainPending,
    MainCaptured,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootCompletionTransitionErrorV1 {
    ExpectedMainPending,
    ExpectedMainCaptured,
}

impl std::fmt::Display for RootCompletionTransitionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][root_transition] {self:?}")
    }
}

impl std::error::Error for RootCompletionTransitionErrorV1 {}

/// Typed success handoff after the root body has been captured and completed.
/// The state is still the same shell+collector pair; only its lifecycle phase
/// has advanced.
#[derive(Debug)]
pub(in crate::mir::builder) struct CompleteInvocationV1 {
    state: ModuleLoweringInvocationStateV1,
    _seal: CompleteInvocationSealV1,
}

#[derive(Debug)]
struct CompleteInvocationSealV1;

/// One invocation owns one shell, one collector, and one root-completion
/// marker.  Function-local lowering state is deliberately not stored here.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringInvocationStateV1 {
    shell: ModuleLoweringShellV1,
    collector: ModuleDraftCollectorV1,
    root: RootCompletionStateV1,
    _seal: ModuleLoweringInvocationStateSealV1,
}

#[derive(Debug)]
struct ModuleLoweringInvocationStateSealV1;

impl ModuleLoweringInvocationStateV1 {
    pub(in crate::mir::builder) fn new(
        shell: ModuleLoweringShellV1,
        collector: ModuleDraftCollectorV1,
    ) -> Self {
        Self {
            shell,
            collector,
            root: RootCompletionStateV1::MainPending,
            _seal: ModuleLoweringInvocationStateSealV1,
        }
    }

    pub(in crate::mir::builder) fn shell(&self) -> &ModuleLoweringShellV1 {
        &self.shell
    }

    pub(in crate::mir::builder) fn shell_mut(&mut self) -> &mut ModuleLoweringShellV1 {
        &mut self.shell
    }

    pub(in crate::mir::builder) fn collector(&self) -> &ModuleDraftCollectorV1 {
        &self.collector
    }

    pub(in crate::mir::builder) fn collector_mut(&mut self) -> &mut ModuleDraftCollectorV1 {
        &mut self.collector
    }

    pub(in crate::mir::builder) fn split_shell_collector_mut(
        &mut self,
    ) -> (&mut ModuleLoweringShellV1, &mut ModuleDraftCollectorV1) {
        (&mut self.shell, &mut self.collector)
    }

    pub(in crate::mir::builder) fn root(&self) -> RootCompletionStateV1 {
        self.root
    }

    pub(in crate::mir::builder) fn capture_main(
        &mut self,
    ) -> Result<(), RootCompletionTransitionErrorV1> {
        if self.root != RootCompletionStateV1::MainPending {
            return Err(RootCompletionTransitionErrorV1::ExpectedMainPending);
        }
        self.root = RootCompletionStateV1::MainCaptured;
        Ok(())
    }

    pub(in crate::mir::builder) fn complete_root(
        &mut self,
    ) -> Result<(), RootCompletionTransitionErrorV1> {
        if self.root != RootCompletionStateV1::MainCaptured {
            return Err(RootCompletionTransitionErrorV1::ExpectedMainCaptured);
        }
        self.root = RootCompletionStateV1::Complete;
        Ok(())
    }

    /// Consume the complete invocation state only at the one drain boundary.
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ModuleLoweringShellV1,
        ModuleDraftCollectorV1,
        RootCompletionStateV1,
    ) {
        (self.shell, self.collector, self.root)
    }
}

impl CompleteInvocationV1 {
    pub(in crate::mir::builder) fn from_state(state: ModuleLoweringInvocationStateV1) -> Self {
        Self {
            state,
            _seal: CompleteInvocationSealV1,
        }
    }

    pub(in crate::mir::builder) fn root(&self) -> RootCompletionStateV1 {
        self.state.root()
    }

    pub(in crate::mir::builder) fn into_state(self) -> ModuleLoweringInvocationStateV1 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::builder::module_lowering_shell::ModuleLoweringShellV1;
    use crate::mir::MirModule;

    #[test]
    fn state_owns_empty_shell_and_collector_without_exposing_function_map() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("state".into())).unwrap();
        let state = ModuleLoweringInvocationStateV1::new(shell, ModuleDraftCollectorV1::default());
        assert_eq!(state.root(), RootCompletionStateV1::MainPending);
        assert!(!state.shell().has_published_functions());
        assert_eq!(state.collector().symbol_count(), 0);
    }

    #[test]
    fn state_parts_are_consumed_together_at_the_drain_boundary() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("state".into())).unwrap();
        let state = ModuleLoweringInvocationStateV1::new(shell, ModuleDraftCollectorV1::default());
        let (_shell, _collector, root) = state.into_parts();
        assert_eq!(root, RootCompletionStateV1::MainPending);
    }

    #[test]
    fn root_completion_requires_ordered_typed_transitions() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("state".into())).unwrap();
        let mut state =
            ModuleLoweringInvocationStateV1::new(shell, ModuleDraftCollectorV1::default());
        assert_eq!(
            state.complete_root().unwrap_err(),
            RootCompletionTransitionErrorV1::ExpectedMainCaptured
        );
        state.capture_main().unwrap();
        state.complete_root().unwrap();
        assert_eq!(state.root(), RootCompletionStateV1::Complete);
    }
}
