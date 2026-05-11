//! Diagnostic-only allocator provider registry facade.
//!
//! This module preserves the historical runtime API path while each diagnostic
//! report implementation lives in a narrower sibling module. The facade does
//! not select a provider, open an activation gate, install a hook, consume a
//! proof bundle, or replace the process allocator.

pub use super::allocator_provider_activation_safety::{
    validate_allocator_provider_activation_safety_gate,
    validate_allocator_provider_activation_safety_gate_from_text,
    AllocatorProviderActivationSafetyFacts, AllocatorProviderActivationSafetyReport,
    AllocatorProviderActivationSafetyStatus, DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED,
    DIAG_PROVIDER_ACTIVATION_SAFETY_COMBINED_DRY_RUN_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_ENTRY_MISSING, DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_HOOK_PLAN_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_PREFLIGHT_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_BUNDLE_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_READINESS_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_REGISTRY_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_ROLLBACK_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_SELECTION_MISSING,
    DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING,
};
pub use super::allocator_provider_proof_bundle_consumption::{
    validate_allocator_provider_proof_bundle_consumption,
    validate_allocator_provider_proof_bundle_consumption_from_text,
    AllocatorProviderProofBundleConsumptionFacts, AllocatorProviderProofBundleConsumptionReport,
    AllocatorProviderProofBundleConsumptionStatus, DIAG_PROVIDER_PROOF_BUNDLE_ACTIVATION_BLOCKED,
    DIAG_PROVIDER_PROOF_BUNDLE_CAPABILITY_MISSING, DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_INACTIVE,
    DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING, DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_MISMATCH,
    DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_PROOF_MISSING, DIAG_PROVIDER_PROOF_BUNDLE_REGISTRY_MISSING,
    DIAG_PROVIDER_PROOF_BUNDLE_SELECTION_MISSING,
};
pub use super::allocator_provider_registry_snapshot::{
    validate_allocator_provider_registry_snapshot,
    validate_allocator_provider_registry_snapshot_from_text,
    AllocatorProviderRegistrySnapshotFacts, AllocatorProviderRegistrySnapshotReport,
    AllocatorProviderRegistrySnapshotStatus, DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING,
    DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING, DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE,
    DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING,
};
pub use super::allocator_provider_selection_decision::{
    validate_allocator_provider_selection_decision,
    validate_allocator_provider_selection_decision_from_text,
    AllocatorProviderSelectionDecisionFacts, AllocatorProviderSelectionDecisionReport,
    AllocatorProviderSelectionDecisionStatus, DIAG_PROVIDER_SELECTION_AMBIGUOUS,
    DIAG_PROVIDER_SELECTION_CAPABILITY_MISSING, DIAG_PROVIDER_SELECTION_DECISION_INACTIVE,
    DIAG_PROVIDER_SELECTION_DECISION_MISSING, DIAG_PROVIDER_SELECTION_REGISTRY_MISSING,
    DIAG_PROVIDER_SELECTION_REQUEST_MISSING, DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER,
};
