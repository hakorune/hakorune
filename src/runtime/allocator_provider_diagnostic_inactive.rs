//! Shared inactive output shapes for allocator provider diagnostics.
//!
//! These constants are diagnostic-only. They do not build a registry, select a
//! provider, consume proofs, prepare rollback, open the activation gate,
//! install hooks, or replace the process allocator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AllocatorProviderDiagnosticInactiveActions {
    pub(crate) would_select_provider: bool,
    pub(crate) would_consume_proof: bool,
    pub(crate) would_prepare_rollback: bool,
    pub(crate) would_open_activation_gate: bool,
    pub(crate) would_install_hook: bool,
    pub(crate) would_replace_process_allocator: bool,
    pub(crate) would_activate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AllocatorProviderRegistrySnapshotInactiveActions {
    pub(crate) active_registry_built: bool,
    pub(crate) would_build_registry: bool,
    pub(crate) diagnostic_actions: AllocatorProviderDiagnosticInactiveActions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AllocatorProviderSafetyGateInactiveActions {
    pub(crate) activation_gate_open: bool,
    pub(crate) would_open_activation_gate: bool,
    pub(crate) would_activate_hook: bool,
    pub(crate) would_activate: bool,
}

pub(crate) const DIAGNOSTIC_INACTIVE_ACTIONS: AllocatorProviderDiagnosticInactiveActions =
    AllocatorProviderDiagnosticInactiveActions {
        would_select_provider: false,
        would_consume_proof: false,
        would_prepare_rollback: false,
        would_open_activation_gate: false,
        would_install_hook: false,
        would_replace_process_allocator: false,
        would_activate: false,
    };

pub(crate) const REGISTRY_SNAPSHOT_INACTIVE_ACTIONS:
    AllocatorProviderRegistrySnapshotInactiveActions =
    AllocatorProviderRegistrySnapshotInactiveActions {
        active_registry_built: false,
        would_build_registry: false,
        diagnostic_actions: DIAGNOSTIC_INACTIVE_ACTIONS,
    };

pub(crate) const SAFETY_GATE_INACTIVE_ACTIONS: AllocatorProviderSafetyGateInactiveActions =
    AllocatorProviderSafetyGateInactiveActions {
        activation_gate_open: false,
        would_open_activation_gate: false,
        would_activate_hook: false,
        would_activate: false,
    };

#[cfg(test)]
mod tests {
    use super::{
        DIAGNOSTIC_INACTIVE_ACTIONS, REGISTRY_SNAPSHOT_INACTIVE_ACTIONS,
        SAFETY_GATE_INACTIVE_ACTIONS,
    };

    #[test]
    fn allocator_provider_inactive_actions_are_all_false() {
        let actions = DIAGNOSTIC_INACTIVE_ACTIONS;
        assert!(!actions.would_select_provider);
        assert!(!actions.would_consume_proof);
        assert!(!actions.would_prepare_rollback);
        assert!(!actions.would_open_activation_gate);
        assert!(!actions.would_install_hook);
        assert!(!actions.would_replace_process_allocator);
        assert!(!actions.would_activate);

        let registry = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
        assert!(!registry.active_registry_built);
        assert!(!registry.would_build_registry);
        assert_eq!(registry.diagnostic_actions, actions);

        let safety_gate = SAFETY_GATE_INACTIVE_ACTIONS;
        assert!(!safety_gate.activation_gate_open);
        assert!(!safety_gate.would_open_activation_gate);
        assert!(!safety_gate.would_activate_hook);
        assert!(!safety_gate.would_activate);
    }
}
