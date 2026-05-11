//! Selected-provider proof validation facts for allocator provider activation.
//!
//! This module is intentionally inactive: it validates caller-provided proof
//! facts for a selected provider, but never consumes a proof bundle and never
//! opens any activation path.

use super::allocator_provider_proof_bundle_consumption::{
    AllocatorProviderProofBundleConsumptionReport, AllocatorProviderProofBundleConsumptionStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AllocatorProviderSelectedProviderPreconditionFacts {
    pub selected_provider_id_for_report: Option<String>,
    pub selected_provider_id_absent: bool,
    pub selected_provider_matches_request: bool,
    pub selected_provider_has_proof: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AllocatorProviderSelectedProviderProofValidationFacts {
    pub proof_bundle_report_ready: bool,
    pub requested_operations_nonempty: bool,
    pub selected_provider_proof_missing: bool,
    pub selected_provider_proof_operations_cover_request: bool,
    pub selected_provider_proof_validated: bool,
}

pub(crate) fn evaluate_selected_provider_precondition(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
    selected_provider_id: Option<&str>,
    honor_report_selected_provider_absent: bool,
) -> AllocatorProviderSelectedProviderPreconditionFacts {
    let selected_provider_id_for_report = match selected_provider_id {
        Some(id) => Some(id.trim().to_string()),
        None if honor_report_selected_provider_absent => {
            proof_bundle_report.selected_provider_id.clone()
        }
        None => None,
    };
    let normalized_selected_provider_id =
        normalize_selected_provider_id(selected_provider_id_for_report.as_deref());
    let selected_provider_id_absent = normalized_selected_provider_id.is_none();
    let selected_provider_matches_request = match (
        proof_bundle_report.requested_provider_id.as_deref(),
        normalized_selected_provider_id,
    ) {
        (Some(requested), Some(selected)) => requested == selected,
        _ => false,
    };
    let selected_provider_has_proof = normalized_selected_provider_id.is_some_and(|selected| {
        proof_bundle_report
            .provider_proof_ids
            .iter()
            .any(|provider_id| provider_id == selected)
    });

    AllocatorProviderSelectedProviderPreconditionFacts {
        selected_provider_id_for_report,
        selected_provider_id_absent,
        selected_provider_matches_request,
        selected_provider_has_proof,
    }
}

pub(crate) fn validate_selected_provider_proof(
    proof_bundle_report: &AllocatorProviderProofBundleConsumptionReport,
    precondition: &AllocatorProviderSelectedProviderPreconditionFacts,
) -> AllocatorProviderSelectedProviderProofValidationFacts {
    let proof_bundle_report_ready =
        proof_bundle_report.status == AllocatorProviderProofBundleConsumptionStatus::ReadyInactive;
    let requested_operations_nonempty = !proof_bundle_report.requested_operations.is_empty();
    let selected_provider_available = !precondition.selected_provider_id_absent
        && precondition.selected_provider_matches_request
        && precondition.selected_provider_has_proof;
    let selected_provider_proof_missing =
        !precondition.selected_provider_id_absent && !precondition.selected_provider_has_proof;
    let selected_provider_proof_operations_cover_request =
        proof_bundle_report_ready && requested_operations_nonempty && selected_provider_available;

    AllocatorProviderSelectedProviderProofValidationFacts {
        proof_bundle_report_ready,
        requested_operations_nonempty,
        selected_provider_proof_missing,
        selected_provider_proof_operations_cover_request,
        selected_provider_proof_validated: selected_provider_proof_operations_cover_request,
    }
}

fn normalize_selected_provider_id(selected_provider_id: Option<&str>) -> Option<&str> {
    selected_provider_id
        .map(str::trim)
        .filter(|id| !id.is_empty() && *id != "none_reserved")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::allocator_provider_proof_bundle_consumption::validate_allocator_provider_proof_bundle_consumption_from_text;

    const M98_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
    );

    #[test]
    fn selected_provider_proof_validation_accepts_ready_report_for_selected_provider() {
        let report = validate_allocator_provider_proof_bundle_consumption_from_text(M98_FIXTURE);
        let precondition =
            evaluate_selected_provider_precondition(&report, Some("native_mimalloc"), false);
        let validation = validate_selected_provider_proof(&report, &precondition);

        assert_eq!(
            report.status,
            AllocatorProviderProofBundleConsumptionStatus::ReadyInactive
        );
        assert!(precondition.selected_provider_matches_request);
        assert!(precondition.selected_provider_has_proof);
        assert!(validation.proof_bundle_report_ready);
        assert!(validation.requested_operations_nonempty);
        assert!(validation.selected_provider_proof_operations_cover_request);
        assert!(validation.selected_provider_proof_validated);
    }

    #[test]
    fn selected_provider_proof_validation_rejects_missing_requested_operations() {
        let mut report =
            validate_allocator_provider_proof_bundle_consumption_from_text(M98_FIXTURE);
        report.requested_operations.clear();
        let precondition =
            evaluate_selected_provider_precondition(&report, Some("native_mimalloc"), false);
        let validation = validate_selected_provider_proof(&report, &precondition);

        assert!(precondition.selected_provider_matches_request);
        assert!(precondition.selected_provider_has_proof);
        assert!(!validation.requested_operations_nonempty);
        assert!(!validation.selected_provider_proof_operations_cover_request);
        assert!(!validation.selected_provider_proof_validated);
    }

    #[test]
    fn selected_provider_proof_validation_rejects_unproven_selected_provider() {
        let report = validate_allocator_provider_proof_bundle_consumption_from_text(M98_FIXTURE);
        let precondition = evaluate_selected_provider_precondition(
            &report,
            Some("native_unknown_allocator"),
            false,
        );
        let validation = validate_selected_provider_proof(&report, &precondition);

        assert!(!precondition.selected_provider_matches_request);
        assert!(!precondition.selected_provider_has_proof);
        assert!(validation.selected_provider_proof_missing);
        assert!(!validation.selected_provider_proof_operations_cover_request);
        assert!(!validation.selected_provider_proof_validated);
    }
}
