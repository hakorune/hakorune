use super::allocator_provider_registry::*;

const ACTIVATION_SAFETY_FIXTURE: &str = include_str!(
    "../../docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
);
const REGISTRY_SNAPSHOT_FIXTURE: &str = include_str!(
    "../../docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
);
const SELECTION_DECISION_FIXTURE: &str = include_str!(
    "../../docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
);
const PROOF_BUNDLE_CONSUMPTION_FIXTURE: &str = include_str!(
    "../../docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
);

#[test]
fn registry_snapshot_fixture_reports_inactive_without_building_registry() {
    let report = validate_allocator_provider_registry_snapshot_from_text(REGISTRY_SNAPSHOT_FIXTURE);

    assert_eq!(
        report.status,
        AllocatorProviderRegistrySnapshotStatus::ReadyInactive
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE);
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.is_empty());
    assert!(report.missing_diagnostics.is_empty());
    assert_eq!(report.provider_count, 4);
    assert_eq!(
        report.provider_ids,
        vec![
            "native_system_malloc".to_string(),
            "native_mimalloc".to_string(),
            "hako_model_allocator".to_string(),
            "debug_guarded_allocator".to_string(),
        ]
    );
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof);
    assert!(!report.would_prepare_rollback);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_install_hook);
    assert!(!report.would_replace_process_allocator);
    assert!(!report.would_activate);
}

#[test]
fn registry_snapshot_empty_text_reports_missing_without_building_registry() {
    let report = validate_allocator_provider_registry_snapshot_from_text("");

    assert_eq!(
        report.status,
        AllocatorProviderRegistrySnapshotStatus::MissingFacts
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING);
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.contains(&"schema_version"));
    assert!(report.missing_facts.contains(&"provider_entries_nonempty"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING));
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_activate);
}

#[test]
fn registry_snapshot_missing_operations_reports_capability_diagnostic() {
    let text = REGISTRY_SNAPSHOT_FIXTURE.replace(
        "operations = [\"alloc\", \"realloc\", \"free\", \"page_reserve\", \"page_commit\", \"page_decommit\"]",
        "operations = []",
    );
    let report = validate_allocator_provider_registry_snapshot_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderRegistrySnapshotStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"provider_operations_nonempty"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING));
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
}

#[test]
fn registry_snapshot_malformed_text_reports_parse_error_without_building_registry() {
    let report = validate_allocator_provider_registry_snapshot_from_text("[");

    assert_eq!(
        report.status,
        AllocatorProviderRegistrySnapshotStatus::MissingFacts
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING);
    assert!(report.parse_error.is_some());
    assert_eq!(report.missing_facts, vec!["parse_toml"]);
    assert_eq!(
        report.missing_diagnostics,
        vec![DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING]
    );
    assert_eq!(report.provider_count, 0);
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_activate);
}

#[test]
fn selection_decision_fixture_reports_inactive_without_selecting_provider() {
    let report =
        validate_allocator_provider_selection_decision_from_text(SELECTION_DECISION_FIXTURE);

    assert_eq!(
        report.status,
        AllocatorProviderSelectionDecisionStatus::ReadyInactive
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_SELECTION_DECISION_INACTIVE);
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.is_empty());
    assert!(report.missing_diagnostics.is_empty());
    assert_eq!(
        report.requested_provider_id.as_deref(),
        Some("native_mimalloc")
    );
    assert_eq!(
        report.required_operations,
        vec![
            "alloc".to_string(),
            "realloc".to_string(),
            "free".to_string()
        ]
    );
    assert_eq!(
        report.candidate_provider_ids,
        vec![
            "native_system_malloc".to_string(),
            "native_mimalloc".to_string(),
            "hako_model_allocator".to_string(),
            "debug_guarded_allocator".to_string(),
        ]
    );
    assert_eq!(
        report.selected_provider_id.as_deref(),
        Some("none_reserved")
    );
    assert!(report.selected_provider_id_absent);
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof);
    assert!(!report.would_prepare_rollback);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_install_hook);
    assert!(!report.would_replace_process_allocator);
    assert!(!report.would_activate);
}

#[test]
fn selection_decision_empty_text_reports_missing_without_selecting_provider() {
    let report = validate_allocator_provider_selection_decision_from_text("");

    assert_eq!(
        report.status,
        AllocatorProviderSelectionDecisionStatus::MissingFacts
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_SELECTION_DECISION_MISSING);
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.contains(&"schema_version"));
    assert!(report.missing_facts.contains(&"registry_snapshot_ready"));
    assert!(report
        .missing_facts
        .contains(&"selection_request_caller_provided"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_SELECTION_DECISION_MISSING));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_SELECTION_REGISTRY_MISSING));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_SELECTION_REQUEST_MISSING));
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_activate);
}

#[test]
fn selection_decision_unsupported_provider_reports_diagnostic_without_selection() {
    let text = SELECTION_DECISION_FIXTURE.replace(
        "requested_provider_id = \"native_mimalloc\"",
        "requested_provider_id = \"jemalloc\"",
    );
    let report = validate_allocator_provider_selection_decision_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderSelectionDecisionStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"requested_provider_supported"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER));
    assert_eq!(report.requested_provider_id.as_deref(), Some("jemalloc"));
    assert!(!report.would_select_provider);
    assert!(!report.would_activate);
}

#[test]
fn selection_decision_missing_operations_reports_capability_diagnostic() {
    let text = SELECTION_DECISION_FIXTURE.replace(
        "required_operations = [\"alloc\", \"realloc\", \"free\"]",
        "required_operations = []",
    );
    let report = validate_allocator_provider_selection_decision_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderSelectionDecisionStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"required_operations_nonempty"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_SELECTION_CAPABILITY_MISSING));
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof);
}

#[test]
fn selection_decision_malformed_text_reports_parse_error_without_selection() {
    let report = validate_allocator_provider_selection_decision_from_text("[");

    assert_eq!(
        report.status,
        AllocatorProviderSelectionDecisionStatus::MissingFacts
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_SELECTION_DECISION_MISSING);
    assert!(report.parse_error.is_some());
    assert_eq!(report.missing_facts, vec!["parse_toml"]);
    assert_eq!(
        report.missing_diagnostics,
        vec![DIAG_PROVIDER_SELECTION_DECISION_MISSING]
    );
    assert!(report.candidate_provider_ids.is_empty());
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof);
    assert!(!report.would_prepare_rollback);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_install_hook);
    assert!(!report.would_replace_process_allocator);
    assert!(!report.would_activate);
}

#[test]
fn proof_bundle_consumption_fixture_reports_inactive_without_consuming_proof() {
    let report = validate_allocator_provider_proof_bundle_consumption_from_text(
        PROOF_BUNDLE_CONSUMPTION_FIXTURE,
    );

    assert_eq!(
        report.status,
        AllocatorProviderProofBundleConsumptionStatus::ReadyInactive
    );
    assert_eq!(
        report.diagnostic,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_INACTIVE
    );
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.is_empty());
    assert!(report.missing_diagnostics.is_empty());
    assert_eq!(
        report.requested_provider_id.as_deref(),
        Some("native_mimalloc")
    );
    assert_eq!(
        report.selected_provider_id.as_deref(),
        Some("none_reserved")
    );
    assert!(report.selected_provider_id_absent);
    assert_eq!(
        report.requested_operations,
        vec![
            "alloc".to_string(),
            "realloc".to_string(),
            "free".to_string()
        ]
    );
    assert_eq!(
        report.candidate_provider_ids,
        vec![
            "native_system_malloc".to_string(),
            "native_mimalloc".to_string(),
            "hako_model_allocator".to_string(),
            "debug_guarded_allocator".to_string(),
        ]
    );
    assert_eq!(
        report.provider_proof_ids,
        vec![
            "native_system_malloc".to_string(),
            "native_mimalloc".to_string(),
            "hako_model_allocator".to_string(),
            "debug_guarded_allocator".to_string(),
        ]
    );
    assert_eq!(report.provider_proof_count, 4);
    assert!(!report.proof_bundle_consumed);
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof_bundle);
    assert!(!report.would_prepare_rollback);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_install_hook);
    assert!(!report.would_replace_process_allocator);
    assert!(!report.would_activate);
}

#[test]
fn proof_bundle_consumption_empty_text_reports_missing_without_consuming_proof() {
    let report = validate_allocator_provider_proof_bundle_consumption_from_text("");

    assert_eq!(
        report.status,
        AllocatorProviderProofBundleConsumptionStatus::MissingFacts
    );
    assert_eq!(
        report.diagnostic,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING
    );
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.contains(&"schema_version"));
    assert!(report.missing_facts.contains(&"registry_snapshot_ready"));
    assert!(report.missing_facts.contains(&"selection_decision_ready"));
    assert!(report
        .missing_facts
        .contains(&"provider_proof_entries_nonempty"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_PROOF_BUNDLE_REGISTRY_MISSING));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_PROOF_BUNDLE_SELECTION_MISSING));
    assert!(!report.proof_bundle_consumed);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof_bundle);
    assert!(!report.would_activate);
}

#[test]
fn proof_bundle_consumption_provider_mismatch_reports_diagnostic() {
    let text = PROOF_BUNDLE_CONSUMPTION_FIXTURE.replace(
        "requested_provider_id = \"native_mimalloc\"",
        "requested_provider_id = \"jemalloc\"",
    );
    let report = validate_allocator_provider_proof_bundle_consumption_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderProofBundleConsumptionStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"requested_provider_has_proof"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_MISMATCH));
    assert_eq!(report.requested_provider_id.as_deref(), Some("jemalloc"));
    assert!(!report.proof_bundle_consumed);
    assert!(!report.would_consume_proof_bundle);
    assert!(!report.would_activate);
}

#[test]
fn proof_bundle_consumption_missing_operations_reports_capability_diagnostic() {
    let text = PROOF_BUNDLE_CONSUMPTION_FIXTURE.replace(
        "requested_operations = [\"alloc\", \"realloc\", \"free\"]",
        "requested_operations = [\"alloc\", \"calloc\"]",
    );
    let report = validate_allocator_provider_proof_bundle_consumption_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderProofBundleConsumptionStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"provider_proof_operations_cover_request"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_PROOF_BUNDLE_CAPABILITY_MISSING));
    assert!(!report.proof_bundle_consumed);
    assert!(!report.would_consume_proof_bundle);
}

#[test]
fn proof_bundle_consumption_malformed_text_reports_parse_error_without_consuming_proof() {
    let report = validate_allocator_provider_proof_bundle_consumption_from_text("[");

    assert_eq!(
        report.status,
        AllocatorProviderProofBundleConsumptionStatus::MissingFacts
    );
    assert_eq!(
        report.diagnostic,
        DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING
    );
    assert!(report.parse_error.is_some());
    assert_eq!(report.missing_facts, vec!["parse_toml"]);
    assert_eq!(
        report.missing_diagnostics,
        vec![DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING]
    );
    assert_eq!(report.provider_proof_count, 0);
    assert!(!report.proof_bundle_consumed);
    assert!(!report.active_registry_built);
    assert!(!report.would_build_registry);
    assert!(!report.would_select_provider);
    assert!(!report.would_consume_proof_bundle);
    assert!(!report.would_prepare_rollback);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_install_hook);
    assert!(!report.would_replace_process_allocator);
    assert!(!report.would_activate);
}

#[test]
fn activation_safety_fixture_reports_gate_closed_without_activation() {
    let report =
        validate_allocator_provider_activation_safety_gate_from_text(ACTIVATION_SAFETY_FIXTURE);

    assert_eq!(
        report.status,
        AllocatorProviderActivationSafetyStatus::ReadyGateClosed
    );
    assert_eq!(report.diagnostic, DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED);
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.is_empty());
    assert!(report.missing_diagnostics.is_empty());
    assert_eq!(
        report.activation_target_provider_id.as_deref(),
        Some("native_mimalloc")
    );
    assert_eq!(
        report.rollback_target_provider_id.as_deref(),
        Some("native_mimalloc")
    );
    assert_eq!(
        report.safety_status,
        crate::runtime::allocator_provider_activation_safety::SAFETY_STATUS_GATE_CLOSED
    );
    assert!(!report.activation_gate_open);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_activate_hook);
    assert!(!report.would_activate);
}

#[test]
fn activation_safety_empty_text_reports_missing_without_activation() {
    let report = validate_allocator_provider_activation_safety_gate_from_text("");

    assert_eq!(
        report.status,
        AllocatorProviderActivationSafetyStatus::MissingFacts
    );
    assert_eq!(
        report.diagnostic,
        DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING
    );
    assert_eq!(report.parse_error, None);
    assert!(report.missing_facts.contains(&"schema_version"));
    assert!(report
        .missing_facts
        .contains(&"activation_target_provider_id_explicit"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING));
    assert!(!report.activation_gate_open);
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_activate_hook);
    assert!(!report.would_activate);
}

#[test]
fn activation_safety_missing_target_reports_target_diagnostic() {
    let text = ACTIVATION_SAFETY_FIXTURE.replace(
        "activation_target_provider_id = \"native_mimalloc\"",
        "activation_target_provider_id = \"\"",
    );
    let report = validate_allocator_provider_activation_safety_gate_from_text(&text);

    assert_eq!(
        report.status,
        AllocatorProviderActivationSafetyStatus::MissingFacts
    );
    assert!(report
        .missing_facts
        .contains(&"activation_target_provider_id_explicit"));
    assert!(report
        .missing_diagnostics
        .contains(&DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING));
    assert!(!report.would_activate);
}

#[test]
fn activation_safety_malformed_text_reports_parse_error_without_activation() {
    let report = validate_allocator_provider_activation_safety_gate_from_text("[");

    assert_eq!(
        report.status,
        AllocatorProviderActivationSafetyStatus::MissingFacts
    );
    assert_eq!(
        report.diagnostic,
        DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING
    );
    assert!(report.parse_error.is_some());
    assert_eq!(report.missing_facts, vec!["parse_toml"]);
    assert_eq!(
        report.missing_diagnostics,
        vec![DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING]
    );
    assert!(!report.would_open_activation_gate);
    assert!(!report.would_activate_hook);
    assert!(!report.would_activate);
}
