//! Allocator provider activation orchestration entry points.
//!
//! M101 creates the fail-fast proof-bundle consumption attempt entry. M102 adds
//! a caller-provided selected-provider precondition. M103 validates the
//! selected provider's proof facts without producing a consumption token. These
//! entries do not select providers, consume proofs, prepare rollback, open
//! gates, install hooks, activate a native allocator, or replace the process
//! allocator.

use super::allocator_provider_diagnostic_inactive::REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
use super::allocator_provider_proof_bundle_consumption::{
    AllocatorProviderProofBundleConsumptionReport, AllocatorProviderProofBundleConsumptionStatus,
};
use super::allocator_provider_proof_validation::{
    evaluate_selected_provider_precondition, validate_selected_provider_proof,
};

pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_REPORT_MISSING: &str =
    "[allocator-provider/proof-bundle-consumption-report-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISMATCH: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-mismatch]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_READY: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-ready]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_MISSING: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-proof-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_INCOMPLETE: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-proof-incomplete]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_READY: &str =
    "[allocator-provider/proof-bundle-consumption-selected-provider-proof-ready]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderProofBundleConsumptionAttemptStatus {
    BlockedMissingSelectedProvider,
    BlockedMissingProofBundleReport,
    BlockedSelectedProviderMismatch,
    BlockedSelectedProviderProofMissing,
    BlockedSelectedProviderProofIncomplete,
    ReadySelectedProviderPrecondition,
    ReadySelectedProviderProofValidated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderProofBundleConsumptionAttemptReport {
    pub status: AllocatorProviderProofBundleConsumptionAttemptStatus,
    pub diagnostic: &'static str,
    pub requested_provider_id: Option<String>,
    pub selected_provider_id: Option<String>,
    pub selected_provider_required: bool,
    pub selected_provider_id_absent: bool,
    pub proof_bundle_report_status: String,
    pub proof_bundle_valid_for_requested_provider: bool,
    pub selected_provider_proof_validated: bool,
    pub selected_provider_proof_operations_cover_request: bool,
    pub proof_bundle_consumed: bool,
    pub active_registry_built: bool,
    pub would_build_registry: bool,
    pub would_select_provider: bool,
    pub would_consume_proof_bundle: bool,
    pub would_prepare_rollback: bool,
    pub would_open_activation_gate: bool,
    pub would_install_hook: bool,
    pub would_replace_process_allocator: bool,
    pub would_activate: bool,
}

pub fn allocator_provider_proof_bundle_consumption_attempt(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
) -> AllocatorProviderProofBundleConsumptionAttemptReport {
    build_proof_bundle_consumption_attempt(
        proof_bundle_report,
        proof_bundle_report.selected_provider_id.as_deref(),
        true,
    )
}

pub fn allocator_provider_selected_provider_precondition_attempt(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
    selected_provider_id: Option<&str>,
) -> AllocatorProviderProofBundleConsumptionAttemptReport {
    build_proof_bundle_consumption_attempt(proof_bundle_report, selected_provider_id, false)
}

pub fn allocator_provider_selected_provider_proof_validation_attempt(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
    selected_provider_id: Option<&str>,
) -> AllocatorProviderProofBundleConsumptionAttemptReport {
    let mut attempt = allocator_provider_selected_provider_precondition_attempt(
        proof_bundle_report,
        selected_provider_id,
    );
    if attempt.status
        != AllocatorProviderProofBundleConsumptionAttemptStatus::ReadySelectedProviderPrecondition
    {
        return attempt;
    }

    let precondition =
        evaluate_selected_provider_precondition(proof_bundle_report, selected_provider_id, false);
    let validation = validate_selected_provider_proof(proof_bundle_report, &precondition);
    if validation.selected_provider_proof_missing {
        attempt.status =
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedSelectedProviderProofMissing;
        attempt.diagnostic = DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_MISSING;
    } else if !validation.selected_provider_proof_operations_cover_request {
        attempt.status =
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedSelectedProviderProofIncomplete;
        attempt.diagnostic =
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_INCOMPLETE;
    } else {
        attempt.status =
            AllocatorProviderProofBundleConsumptionAttemptStatus::ReadySelectedProviderProofValidated;
        attempt.diagnostic = DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_READY;
    }

    attempt.proof_bundle_valid_for_requested_provider =
        validation.selected_provider_proof_validated;
    attempt.selected_provider_proof_operations_cover_request =
        validation.selected_provider_proof_operations_cover_request;
    attempt.selected_provider_proof_validated = validation.selected_provider_proof_validated;
    attempt
}

fn build_proof_bundle_consumption_attempt(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
    selected_provider_id: Option<&str>,
    honor_report_selected_provider_absent: bool,
) -> AllocatorProviderProofBundleConsumptionAttemptReport {
    let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
    let diagnostic_actions = inactive.diagnostic_actions;
    let proof_bundle_ready =
        proof_bundle_report.status == AllocatorProviderProofBundleConsumptionStatus::ReadyInactive;
    let precondition = evaluate_selected_provider_precondition(
        proof_bundle_report,
        selected_provider_id,
        honor_report_selected_provider_absent,
    );
    let selected_provider_id_absent = precondition.selected_provider_id_absent
        || (honor_report_selected_provider_absent
            && proof_bundle_report.selected_provider_id_absent);

    let (status, diagnostic) = if !proof_bundle_ready {
        (
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedMissingProofBundleReport,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_REPORT_MISSING,
        )
    } else if selected_provider_id_absent {
        (
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedMissingSelectedProvider,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING,
        )
    } else if !precondition.selected_provider_matches_request
        || !precondition.selected_provider_has_proof
    {
        (
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedSelectedProviderMismatch,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISMATCH,
        )
    } else {
        (
            AllocatorProviderProofBundleConsumptionAttemptStatus::ReadySelectedProviderPrecondition,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_READY,
        )
    };

    AllocatorProviderProofBundleConsumptionAttemptReport {
        status,
        diagnostic,
        requested_provider_id: proof_bundle_report.requested_provider_id.clone(),
        selected_provider_id: precondition.selected_provider_id_for_report,
        selected_provider_required: true,
        selected_provider_id_absent,
        proof_bundle_report_status: format!("{:?}", proof_bundle_report.status),
        proof_bundle_valid_for_requested_provider: proof_bundle_ready,
        selected_provider_proof_validated: false,
        selected_provider_proof_operations_cover_request: false,
        proof_bundle_consumed: false,
        active_registry_built: inactive.active_registry_built,
        would_build_registry: inactive.would_build_registry,
        would_select_provider: diagnostic_actions.would_select_provider,
        would_consume_proof_bundle: diagnostic_actions.would_consume_proof,
        would_prepare_rollback: diagnostic_actions.would_prepare_rollback,
        would_open_activation_gate: diagnostic_actions.would_open_activation_gate,
        would_install_hook: diagnostic_actions.would_install_hook,
        would_replace_process_allocator: diagnostic_actions.would_replace_process_allocator,
        would_activate: diagnostic_actions.would_activate,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        allocator_provider_proof_bundle_consumption_attempt,
        allocator_provider_selected_provider_precondition_attempt,
        allocator_provider_selected_provider_proof_validation_attempt,
        AllocatorProviderProofBundleConsumptionAttemptStatus,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_REPORT_MISSING,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISMATCH,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_INCOMPLETE,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_READY,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_READY,
    };
    use crate::runtime::allocator_provider_proof_bundle_consumption::{
        validate_allocator_provider_proof_bundle_consumption_from_text,
        AllocatorProviderProofBundleConsumptionStatus,
    };

    const PROOF_BUNDLE_CONSUMPTION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
    );

    #[test]
    fn proof_bundle_consumption_attempt_blocks_when_selected_provider_is_absent() {
        let proof_bundle_report = validate_allocator_provider_proof_bundle_consumption_from_text(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );
        assert_eq!(
            proof_bundle_report.status,
            AllocatorProviderProofBundleConsumptionStatus::ReadyInactive
        );

        let attempt = allocator_provider_proof_bundle_consumption_attempt(&proof_bundle_report);

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedMissingSelectedProvider
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING
        );
        assert_eq!(
            attempt.requested_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert_eq!(
            attempt.selected_provider_id.as_deref(),
            Some("none_reserved")
        );
        assert!(attempt.selected_provider_required);
        assert!(attempt.selected_provider_id_absent);
        assert!(attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.selected_provider_proof_validated);
        assert!(!attempt.selected_provider_proof_operations_cover_request);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.active_registry_built);
        assert!(!attempt.would_build_registry);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_prepare_rollback);
        assert!(!attempt.would_open_activation_gate);
        assert!(!attempt.would_install_hook);
        assert!(!attempt.would_replace_process_allocator);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn proof_bundle_consumption_attempt_blocks_malformed_report_without_consuming() {
        let proof_bundle_report =
            validate_allocator_provider_proof_bundle_consumption_from_text("[");
        let attempt = allocator_provider_proof_bundle_consumption_attempt(&proof_bundle_report);

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedMissingProofBundleReport
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_REPORT_MISSING
        );
        assert!(!attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn selected_provider_precondition_accepts_matching_provider_without_consuming() {
        let proof_bundle_report = validate_allocator_provider_proof_bundle_consumption_from_text(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );
        let attempt = allocator_provider_selected_provider_precondition_attempt(
            &proof_bundle_report,
            Some("native_mimalloc"),
        );

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::ReadySelectedProviderPrecondition
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_READY
        );
        assert_eq!(
            attempt.requested_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert_eq!(
            attempt.selected_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert!(attempt.selected_provider_required);
        assert!(!attempt.selected_provider_id_absent);
        assert!(attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.selected_provider_proof_validated);
        assert!(!attempt.selected_provider_proof_operations_cover_request);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.active_registry_built);
        assert!(!attempt.would_build_registry);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_prepare_rollback);
        assert!(!attempt.would_open_activation_gate);
        assert!(!attempt.would_install_hook);
        assert!(!attempt.would_replace_process_allocator);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn selected_provider_precondition_blocks_absent_caller_provider_without_consuming() {
        let proof_bundle_report = validate_allocator_provider_proof_bundle_consumption_from_text(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );
        let attempt =
            allocator_provider_selected_provider_precondition_attempt(&proof_bundle_report, None);

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedMissingSelectedProvider
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING
        );
        assert_eq!(
            attempt.requested_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert_eq!(attempt.selected_provider_id, None);
        assert!(attempt.selected_provider_required);
        assert!(attempt.selected_provider_id_absent);
        assert!(attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn selected_provider_precondition_blocks_provider_mismatch_without_consuming() {
        let proof_bundle_report = validate_allocator_provider_proof_bundle_consumption_from_text(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );
        let attempt = allocator_provider_selected_provider_precondition_attempt(
            &proof_bundle_report,
            Some("native_system_malloc"),
        );

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedSelectedProviderMismatch
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISMATCH
        );
        assert_eq!(
            attempt.requested_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert_eq!(
            attempt.selected_provider_id.as_deref(),
            Some("native_system_malloc")
        );
        assert!(attempt.selected_provider_required);
        assert!(!attempt.selected_provider_id_absent);
        assert!(attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_prepare_rollback);
        assert!(!attempt.would_open_activation_gate);
        assert!(!attempt.would_install_hook);
        assert!(!attempt.would_replace_process_allocator);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn selected_provider_proof_validation_accepts_matching_provider_without_consuming() {
        let proof_bundle_report = validate_allocator_provider_proof_bundle_consumption_from_text(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );
        let attempt = allocator_provider_selected_provider_proof_validation_attempt(
            &proof_bundle_report,
            Some("native_mimalloc"),
        );

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::ReadySelectedProviderProofValidated
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_READY
        );
        assert_eq!(
            attempt.requested_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert_eq!(
            attempt.selected_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert!(attempt.selected_provider_required);
        assert!(!attempt.selected_provider_id_absent);
        assert!(attempt.proof_bundle_valid_for_requested_provider);
        assert!(attempt.selected_provider_proof_operations_cover_request);
        assert!(attempt.selected_provider_proof_validated);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.active_registry_built);
        assert!(!attempt.would_build_registry);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_prepare_rollback);
        assert!(!attempt.would_open_activation_gate);
        assert!(!attempt.would_install_hook);
        assert!(!attempt.would_replace_process_allocator);
        assert!(!attempt.would_activate);
    }

    #[test]
    fn selected_provider_proof_validation_blocks_incomplete_operations_without_consuming() {
        let mut proof_bundle_report =
            validate_allocator_provider_proof_bundle_consumption_from_text(
                PROOF_BUNDLE_CONSUMPTION_FIXTURE,
            );
        proof_bundle_report.requested_operations.clear();
        let attempt = allocator_provider_selected_provider_proof_validation_attempt(
            &proof_bundle_report,
            Some("native_mimalloc"),
        );

        assert_eq!(
            attempt.status,
            AllocatorProviderProofBundleConsumptionAttemptStatus::BlockedSelectedProviderProofIncomplete
        );
        assert_eq!(
            attempt.diagnostic,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_INCOMPLETE
        );
        assert_eq!(
            attempt.selected_provider_id.as_deref(),
            Some("native_mimalloc")
        );
        assert!(!attempt.selected_provider_id_absent);
        assert!(!attempt.proof_bundle_valid_for_requested_provider);
        assert!(!attempt.selected_provider_proof_operations_cover_request);
        assert!(!attempt.selected_provider_proof_validated);
        assert!(!attempt.proof_bundle_consumed);
        assert!(!attempt.would_select_provider);
        assert!(!attempt.would_consume_proof_bundle);
        assert!(!attempt.would_activate);
    }
}
