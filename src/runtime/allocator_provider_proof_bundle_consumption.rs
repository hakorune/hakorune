//! Diagnostic-only allocator provider proof bundle consumption report.

use super::allocator_provider_diagnostic_inactive::REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
use super::allocator_provider_registry_common::{
    string_list, string_list_matches, EXPECTED_PROVIDER_IDS, OWNER_PATH,
};
use super::allocator_provider_toml_helpers::{
    bool_field_false, nonempty_text_field, string_list_contains_all, text_field_matches,
    DiagnosticFactCheck,
};

pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING: &str =
    "[allocator-provider/proof-bundle-consumption-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_REGISTRY_MISSING: &str =
    "[allocator-provider/proof-bundle-registry-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_SELECTION_MISSING: &str =
    "[allocator-provider/proof-bundle-selection-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_PROOF_MISSING: &str =
    "[allocator-provider/proof-bundle-provider-proof-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_MISMATCH: &str =
    "[allocator-provider/proof-bundle-provider-mismatch]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CAPABILITY_MISSING: &str =
    "[allocator-provider/proof-bundle-capability-missing]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_ACTIVATION_BLOCKED: &str =
    "[allocator-provider/proof-bundle-activation-blocked]";
pub const DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_INACTIVE: &str =
    "[allocator-provider/proof-bundle-consumption-inactive]";

const REQUIRED_PROOF_BUNDLE_FACTS: &[&str] = &[
    "registry_snapshot_ready",
    "selection_decision_ready",
    "proof_bundle_caller_provided",
    "requested_provider_id_explicit",
    "selected_provider_id_absent",
    "provider_proof_entries_nonempty",
    "provider_proof_ids_reserved_set",
    "provider_proof_operations_cover_request",
    "proof_bundle_policy_named",
    "fail_fast_proof_bundle_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_runtime_hook_activation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_activation_without_later_row",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderProofBundleConsumptionStatus {
    MissingFacts,
    ReadyInactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderProofBundleConsumptionFacts<'a> {
    pub schema_ready: bool,
    pub status_reserved: bool,
    pub active_false: bool,
    pub owner_named: bool,
    pub registry_snapshot_ready: bool,
    pub selection_decision_ready: bool,
    pub proof_bundle_caller_provided: bool,
    pub requested_provider_id: Option<&'a str>,
    pub selected_provider_id: Option<&'a str>,
    pub selected_provider_id_absent: bool,
    pub requested_provider_has_proof: bool,
    pub requested_operations_nonempty: bool,
    pub candidate_provider_ids_reserved_set: bool,
    pub provider_proof_entries_nonempty: bool,
    pub provider_proof_ids_reserved_set: bool,
    pub provider_proof_operations_cover_request: bool,
    pub proof_bundle_policy_named: bool,
    pub consumption_status_reserved: bool,
    pub proof_bundle_consumption_inactive: bool,
    pub provider_selection_inactive: bool,
    pub decision_status_reserved: bool,
    pub selection_status_reserved: bool,
    pub proof_bundle_consumed_false: bool,
    pub would_build_registry_false: bool,
    pub would_select_provider_false: bool,
    pub would_consume_proof_bundle_false: bool,
    pub would_activate_false: bool,
    pub activation_future_row_required: bool,
    pub diagnostic_named: bool,
    pub missing_registry_diagnostic_named: bool,
    pub missing_selection_diagnostic_named: bool,
    pub missing_provider_proof_diagnostic_named: bool,
    pub provider_mismatch_diagnostic_named: bool,
    pub missing_capability_diagnostic_named: bool,
    pub activation_blocked_diagnostic_named: bool,
    pub required_fact_list_complete: bool,
    pub requested_operations: Vec<&'a str>,
    pub candidate_provider_ids: Vec<&'a str>,
    pub provider_proof_ids: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderProofBundleConsumptionReport {
    pub status: AllocatorProviderProofBundleConsumptionStatus,
    pub diagnostic: &'static str,
    pub parse_error: Option<String>,
    pub missing_facts: Vec<&'static str>,
    pub missing_diagnostics: Vec<&'static str>,
    pub requested_provider_id: Option<String>,
    pub selected_provider_id: Option<String>,
    pub selected_provider_id_absent: bool,
    pub requested_operations: Vec<String>,
    pub candidate_provider_ids: Vec<String>,
    pub provider_proof_ids: Vec<String>,
    pub provider_proof_count: usize,
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

pub fn validate_allocator_provider_proof_bundle_consumption(
    facts: &AllocatorProviderProofBundleConsumptionFacts<'_>,
) -> AllocatorProviderProofBundleConsumptionReport {
    let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
    let diagnostic_actions = inactive.diagnostic_actions;
    let missing_facts = collect_missing_proof_bundle_consumption_facts(facts);
    let missing_diagnostics = collect_proof_bundle_consumption_missing_diagnostics(facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderProofBundleConsumptionStatus::ReadyInactive,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_INACTIVE,
        )
    } else {
        (
            AllocatorProviderProofBundleConsumptionStatus::MissingFacts,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING,
        )
    };

    AllocatorProviderProofBundleConsumptionReport {
        status,
        diagnostic,
        parse_error: None,
        missing_facts,
        missing_diagnostics,
        requested_provider_id: facts.requested_provider_id.map(str::to_string),
        selected_provider_id: facts.selected_provider_id.map(str::to_string),
        selected_provider_id_absent: facts.selected_provider_id_absent,
        requested_operations: facts
            .requested_operations
            .iter()
            .map(|operation| (*operation).to_string())
            .collect(),
        candidate_provider_ids: facts
            .candidate_provider_ids
            .iter()
            .map(|provider_id| (*provider_id).to_string())
            .collect(),
        provider_proof_ids: facts
            .provider_proof_ids
            .iter()
            .map(|provider_id| (*provider_id).to_string())
            .collect(),
        provider_proof_count: facts.provider_proof_ids.len(),
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

pub fn validate_allocator_provider_proof_bundle_consumption_from_text(
    proof_bundle_consumption_toml: &str,
) -> AllocatorProviderProofBundleConsumptionReport {
    let value = match toml::from_str::<toml::Value>(proof_bundle_consumption_toml) {
        Ok(value) => value,
        Err(err) => {
            let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
            let diagnostic_actions = inactive.diagnostic_actions;
            return AllocatorProviderProofBundleConsumptionReport {
                status: AllocatorProviderProofBundleConsumptionStatus::MissingFacts,
                diagnostic: DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING,
                parse_error: Some(err.to_string()),
                missing_facts: vec!["parse_toml"],
                missing_diagnostics: vec![DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING],
                requested_provider_id: None,
                selected_provider_id: None,
                selected_provider_id_absent: false,
                requested_operations: Vec::new(),
                candidate_provider_ids: Vec::new(),
                provider_proof_ids: Vec::new(),
                provider_proof_count: 0,
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
            };
        }
    };

    let facts = read_proof_bundle_consumption_facts(&value);
    validate_allocator_provider_proof_bundle_consumption(&facts)
}

fn read_proof_bundle_consumption_facts(
    value: &toml::Value,
) -> AllocatorProviderProofBundleConsumptionFacts<'_> {
    let requested_provider_id = nonempty_text_field(value, "requested_provider_id");
    let selected_provider_id = nonempty_text_field(value, "selected_provider_id");
    let requested_operations = string_list(value.get("requested_operations"));
    let candidate_provider_ids = string_list(value.get("candidate_provider_ids"));
    let provider_proof_ids = proof_bundle_provider_proof_ids(value);

    AllocatorProviderProofBundleConsumptionFacts {
        schema_ready: text_field_matches(
            value,
            "schema_version",
            "allocator_provider_proof_bundle_consumption_v0",
        ),
        status_reserved: text_field_matches(value, "status", "reserved"),
        active_false: bool_field_false(value, "active"),
        owner_named: text_field_matches(value, "consumption_owner", OWNER_PATH),
        registry_snapshot_ready: text_field_matches(
            value,
            "registry_snapshot_input",
            "allocator_provider_registry_snapshot_report",
        ),
        selection_decision_ready: text_field_matches(
            value,
            "selection_decision_input",
            "allocator_provider_selection_decision_report",
        ),
        proof_bundle_caller_provided: text_field_matches(
            value,
            "proof_bundle_source",
            "caller_provided_diagnostic_bundle",
        ),
        requested_provider_id,
        selected_provider_id,
        selected_provider_id_absent: selected_provider_id == Some("none_reserved"),
        requested_provider_has_proof: requested_provider_id
            .map(|requested| provider_proof_ids.iter().any(|id| *id == requested))
            .unwrap_or(false),
        requested_operations_nonempty: !requested_operations.is_empty(),
        candidate_provider_ids_reserved_set: string_list_matches(
            value.get("candidate_provider_ids"),
            EXPECTED_PROVIDER_IDS,
        ),
        provider_proof_entries_nonempty: proof_bundle_provider_proofs(value)
            .map_or(false, |proofs| !proofs.is_empty()),
        provider_proof_ids_reserved_set: proof_bundle_provider_proof_ids_reserved_set(value),
        provider_proof_operations_cover_request: proof_bundle_provider_proof_operations_cover(
            value,
            requested_provider_id,
            &requested_operations,
        ),
        proof_bundle_policy_named: text_field_matches(
            value,
            "proof_bundle_policy",
            "explicit_provider_proof_bundle_required_reserved",
        ),
        consumption_status_reserved: text_field_matches(
            value,
            "consumption_status",
            "reserved_no_consumption",
        ),
        proof_bundle_consumption_inactive: text_field_matches(
            value,
            "proof_bundle_consumption",
            "inactive",
        ),
        provider_selection_inactive: text_field_matches(value, "provider_selection", "inactive"),
        decision_status_reserved: text_field_matches(value, "decision_status", "reserved"),
        selection_status_reserved: text_field_matches(
            value,
            "selection_status",
            "reserved_no_selection",
        ),
        proof_bundle_consumed_false: bool_field_false(value, "proof_bundle_consumed"),
        would_build_registry_false: bool_field_false(value, "would_build_registry"),
        would_select_provider_false: bool_field_false(value, "would_select_provider"),
        would_consume_proof_bundle_false: bool_field_false(value, "would_consume_proof_bundle"),
        would_activate_false: bool_field_false(value, "would_activate"),
        activation_future_row_required: text_field_matches(
            value,
            "activation",
            "future_row_required",
        ),
        diagnostic_named: text_field_matches(
            value,
            "diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING,
        ),
        missing_registry_diagnostic_named: text_field_matches(
            value,
            "missing_registry_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_REGISTRY_MISSING,
        ),
        missing_selection_diagnostic_named: text_field_matches(
            value,
            "missing_selection_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_SELECTION_MISSING,
        ),
        missing_provider_proof_diagnostic_named: text_field_matches(
            value,
            "missing_provider_proof_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_PROOF_MISSING,
        ),
        provider_mismatch_diagnostic_named: text_field_matches(
            value,
            "provider_mismatch_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_MISMATCH,
        ),
        missing_capability_diagnostic_named: text_field_matches(
            value,
            "missing_capability_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_CAPABILITY_MISSING,
        ),
        activation_blocked_diagnostic_named: text_field_matches(
            value,
            "activation_blocked_diagnostic",
            DIAG_PROVIDER_PROOF_BUNDLE_ACTIVATION_BLOCKED,
        ),
        required_fact_list_complete: string_list_contains_all(
            value.get("required_proof_bundle_consumption_facts"),
            REQUIRED_PROOF_BUNDLE_FACTS,
        ),
        requested_operations,
        candidate_provider_ids,
        provider_proof_ids,
    }
}

fn proof_bundle_consumption_fact_checks(
    facts: &AllocatorProviderProofBundleConsumptionFacts<'_>,
) -> Vec<DiagnosticFactCheck> {
    vec![
        DiagnosticFactCheck {
            present: facts.schema_ready,
            name: "schema_version",
        },
        DiagnosticFactCheck {
            present: facts.status_reserved,
            name: "status_reserved",
        },
        DiagnosticFactCheck {
            present: facts.active_false,
            name: "active_false",
        },
        DiagnosticFactCheck {
            present: facts.owner_named,
            name: "consumption_owner_named",
        },
        DiagnosticFactCheck {
            present: facts.registry_snapshot_ready,
            name: "registry_snapshot_ready",
        },
        DiagnosticFactCheck {
            present: facts.selection_decision_ready,
            name: "selection_decision_ready",
        },
        DiagnosticFactCheck {
            present: facts.proof_bundle_caller_provided,
            name: "proof_bundle_caller_provided",
        },
        DiagnosticFactCheck {
            present: facts.requested_provider_id.is_some(),
            name: "requested_provider_id_explicit",
        },
        DiagnosticFactCheck {
            present: facts.selected_provider_id_absent,
            name: "selected_provider_id_absent",
        },
        DiagnosticFactCheck {
            present: facts.requested_provider_has_proof,
            name: "requested_provider_has_proof",
        },
        DiagnosticFactCheck {
            present: facts.requested_operations_nonempty,
            name: "requested_operations_nonempty",
        },
        DiagnosticFactCheck {
            present: facts.candidate_provider_ids_reserved_set,
            name: "candidate_provider_ids_reserved_set",
        },
        DiagnosticFactCheck {
            present: facts.provider_proof_entries_nonempty,
            name: "provider_proof_entries_nonempty",
        },
        DiagnosticFactCheck {
            present: facts.provider_proof_ids_reserved_set,
            name: "provider_proof_ids_reserved_set",
        },
        DiagnosticFactCheck {
            present: facts.provider_proof_operations_cover_request,
            name: "provider_proof_operations_cover_request",
        },
        DiagnosticFactCheck {
            present: facts.proof_bundle_policy_named,
            name: "proof_bundle_policy_named",
        },
        DiagnosticFactCheck {
            present: facts.consumption_status_reserved,
            name: "consumption_status_reserved",
        },
        DiagnosticFactCheck {
            present: facts.proof_bundle_consumption_inactive,
            name: "proof_bundle_consumption_inactive",
        },
        DiagnosticFactCheck {
            present: facts.provider_selection_inactive,
            name: "provider_selection_inactive",
        },
        DiagnosticFactCheck {
            present: facts.decision_status_reserved,
            name: "decision_status_reserved",
        },
        DiagnosticFactCheck {
            present: facts.selection_status_reserved,
            name: "selection_status_reserved",
        },
        DiagnosticFactCheck {
            present: facts.proof_bundle_consumed_false,
            name: "proof_bundle_consumed_false",
        },
        DiagnosticFactCheck {
            present: facts.would_build_registry_false,
            name: "would_build_registry_false",
        },
        DiagnosticFactCheck {
            present: facts.would_select_provider_false,
            name: "would_select_provider_false",
        },
        DiagnosticFactCheck {
            present: facts.would_consume_proof_bundle_false,
            name: "would_consume_proof_bundle_false",
        },
        DiagnosticFactCheck {
            present: facts.would_activate_false,
            name: "would_activate_false",
        },
        DiagnosticFactCheck {
            present: facts.activation_future_row_required,
            name: "activation_future_row_required",
        },
        DiagnosticFactCheck {
            present: facts.diagnostic_named,
            name: "proof_bundle_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_registry_diagnostic_named,
            name: "missing_registry_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_selection_diagnostic_named,
            name: "missing_selection_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_provider_proof_diagnostic_named,
            name: "missing_provider_proof_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.provider_mismatch_diagnostic_named,
            name: "provider_mismatch_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_capability_diagnostic_named,
            name: "missing_capability_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.activation_blocked_diagnostic_named,
            name: "fail_fast_proof_bundle_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.required_fact_list_complete,
            name: "required_proof_bundle_consumption_facts_complete",
        },
    ]
}

fn collect_missing_proof_bundle_consumption_facts(
    facts: &AllocatorProviderProofBundleConsumptionFacts<'_>,
) -> Vec<&'static str> {
    proof_bundle_consumption_fact_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.name))
        .collect()
}

fn collect_proof_bundle_consumption_missing_diagnostics(
    facts: &AllocatorProviderProofBundleConsumptionFacts<'_>,
) -> Vec<&'static str> {
    [
        (
            facts.schema_ready && facts.status_reserved && facts.active_false && facts.owner_named,
            DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_MISSING,
        ),
        (
            facts.registry_snapshot_ready,
            DIAG_PROVIDER_PROOF_BUNDLE_REGISTRY_MISSING,
        ),
        (
            facts.selection_decision_ready && facts.selected_provider_id_absent,
            DIAG_PROVIDER_PROOF_BUNDLE_SELECTION_MISSING,
        ),
        (
            facts.proof_bundle_caller_provided
                && facts.provider_proof_entries_nonempty
                && facts.provider_proof_ids_reserved_set,
            DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_PROOF_MISSING,
        ),
        (
            facts.requested_provider_id.is_none() || facts.requested_provider_has_proof,
            DIAG_PROVIDER_PROOF_BUNDLE_PROVIDER_MISMATCH,
        ),
        (
            facts.requested_operations_nonempty && facts.provider_proof_operations_cover_request,
            DIAG_PROVIDER_PROOF_BUNDLE_CAPABILITY_MISSING,
        ),
    ]
    .into_iter()
    .filter_map(|(present, diagnostic)| (!present).then_some(diagnostic))
    .collect()
}

fn proof_bundle_provider_proofs(value: &toml::Value) -> Option<&Vec<toml::Value>> {
    value.get("provider_proofs").and_then(toml::Value::as_array)
}

fn proof_bundle_provider_proof_ids<'a>(value: &'a toml::Value) -> Vec<&'a str> {
    proof_bundle_provider_proofs(value)
        .into_iter()
        .flatten()
        .filter_map(|proof| proof.get("provider_id").and_then(toml::Value::as_str))
        .filter(|provider_id| !provider_id.is_empty())
        .collect()
}

fn proof_bundle_provider_proof_ids_reserved_set(value: &toml::Value) -> bool {
    let Some(proofs) = proof_bundle_provider_proofs(value) else {
        return false;
    };
    if proofs.len() != EXPECTED_PROVIDER_IDS.len() {
        return false;
    }
    proofs
        .iter()
        .zip(EXPECTED_PROVIDER_IDS.iter())
        .all(|(proof, expected_id)| {
            proof.get("provider_id").and_then(toml::Value::as_str) == Some(*expected_id)
                && proof.get("state").and_then(toml::Value::as_str) == Some("reserved")
                && proof.get("consumption").and_then(toml::Value::as_str)
                    == Some("future_row_required")
        })
}

fn proof_bundle_provider_proof_operations_cover(
    value: &toml::Value,
    requested_provider_id: Option<&str>,
    requested_operations: &[&str],
) -> bool {
    let Some(requested_provider_id) = requested_provider_id else {
        return false;
    };
    if requested_operations.is_empty() {
        return false;
    }
    let Some(proofs) = proof_bundle_provider_proofs(value) else {
        return false;
    };
    let Some(proof) = proofs.iter().find(|proof| {
        proof.get("provider_id").and_then(toml::Value::as_str) == Some(requested_provider_id)
    }) else {
        return false;
    };
    let Some(operations) = proof.get("operations").and_then(toml::Value::as_array) else {
        return false;
    };
    requested_operations.iter().all(|requested_operation| {
        operations
            .iter()
            .filter_map(toml::Value::as_str)
            .any(|operation| operation == *requested_operation)
    })
}
