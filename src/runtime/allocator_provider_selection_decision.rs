//! Diagnostic-only allocator provider selection decision report.

use super::allocator_provider_diagnostic_inactive::REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
use super::allocator_provider_registry_common::{
    string_list, string_list_matches, EXPECTED_PROVIDER_IDS, OWNER_PATH,
};
use super::allocator_provider_toml_helpers::{
    bool_field_false, nonempty_text_field, string_list_contains_all, text_field_matches,
    DiagnosticFactCheck,
};

pub const DIAG_PROVIDER_SELECTION_DECISION_MISSING: &str =
    "[allocator-provider/selection-decision-missing]";
pub const DIAG_PROVIDER_SELECTION_REGISTRY_MISSING: &str =
    "[allocator-provider/selection-registry-missing]";
pub const DIAG_PROVIDER_SELECTION_REQUEST_MISSING: &str =
    "[allocator-provider/selection-request-missing]";
pub const DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER: &str =
    "[allocator-provider/selection-unsupported-provider]";
pub const DIAG_PROVIDER_SELECTION_CAPABILITY_MISSING: &str =
    "[allocator-provider/selection-capability-missing]";
pub const DIAG_PROVIDER_SELECTION_AMBIGUOUS: &str = "[allocator-provider/selection-ambiguous]";
pub const DIAG_PROVIDER_SELECTION_DECISION_INACTIVE: &str =
    "[allocator-provider/selection-decision-inactive]";

const REQUIRED_SELECTION_DECISION_FACTS: &[&str] = &[
    "registry_snapshot_ready",
    "selection_request_caller_provided",
    "requested_provider_id_explicit",
    "required_operations_nonempty",
    "candidate_provider_ids_reserved_set",
    "deterministic_provider_order_named",
    "selection_policy_named",
    "fail_fast_selection_diagnostic_named",
    "no_selected_provider_id",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
    "no_runtime_registry_implementation",
    "no_runtime_hook_activation",
    "no_global_allocator_attribute",
    "no_activation_without_later_row",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderSelectionDecisionStatus {
    MissingFacts,
    ReadyInactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderSelectionDecisionFacts<'a> {
    pub schema_ready: bool,
    pub status_reserved: bool,
    pub active_false: bool,
    pub owner_named: bool,
    pub registry_snapshot_ready: bool,
    pub selection_request_caller_provided: bool,
    pub requested_provider_id: Option<&'a str>,
    pub requested_provider_supported: bool,
    pub required_operations_nonempty: bool,
    pub candidate_provider_ids_reserved_set: bool,
    pub deterministic_provider_order_named: bool,
    pub selection_policy_named: bool,
    pub selection_status_reserved: bool,
    pub provider_selection_inactive: bool,
    pub decision_status_reserved: bool,
    pub selected_provider_id: Option<&'a str>,
    pub selected_provider_id_absent: bool,
    pub would_build_registry_false: bool,
    pub would_select_provider_false: bool,
    pub would_activate_false: bool,
    pub activation_future_row_required: bool,
    pub diagnostic_named: bool,
    pub missing_registry_diagnostic_named: bool,
    pub missing_request_diagnostic_named: bool,
    pub unsupported_provider_diagnostic_named: bool,
    pub missing_capability_diagnostic_named: bool,
    pub ambiguous_provider_diagnostic_named: bool,
    pub required_fact_list_complete: bool,
    pub required_operations: Vec<&'a str>,
    pub candidate_provider_ids: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderSelectionDecisionReport {
    pub status: AllocatorProviderSelectionDecisionStatus,
    pub diagnostic: &'static str,
    pub parse_error: Option<String>,
    pub missing_facts: Vec<&'static str>,
    pub missing_diagnostics: Vec<&'static str>,
    pub requested_provider_id: Option<String>,
    pub required_operations: Vec<String>,
    pub candidate_provider_ids: Vec<String>,
    pub selected_provider_id: Option<String>,
    pub selected_provider_id_absent: bool,
    pub active_registry_built: bool,
    pub would_build_registry: bool,
    pub would_select_provider: bool,
    pub would_consume_proof: bool,
    pub would_prepare_rollback: bool,
    pub would_open_activation_gate: bool,
    pub would_install_hook: bool,
    pub would_replace_process_allocator: bool,
    pub would_activate: bool,
}

pub fn validate_allocator_provider_selection_decision(
    facts: &AllocatorProviderSelectionDecisionFacts<'_>,
) -> AllocatorProviderSelectionDecisionReport {
    let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
    let diagnostic_actions = inactive.diagnostic_actions;
    let missing_facts = collect_missing_selection_decision_facts(facts);
    let missing_diagnostics = collect_selection_decision_missing_diagnostics(facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderSelectionDecisionStatus::ReadyInactive,
            DIAG_PROVIDER_SELECTION_DECISION_INACTIVE,
        )
    } else {
        (
            AllocatorProviderSelectionDecisionStatus::MissingFacts,
            DIAG_PROVIDER_SELECTION_DECISION_MISSING,
        )
    };

    AllocatorProviderSelectionDecisionReport {
        status,
        diagnostic,
        parse_error: None,
        missing_facts,
        missing_diagnostics,
        requested_provider_id: facts.requested_provider_id.map(str::to_string),
        required_operations: facts
            .required_operations
            .iter()
            .map(|operation| (*operation).to_string())
            .collect(),
        candidate_provider_ids: facts
            .candidate_provider_ids
            .iter()
            .map(|provider_id| (*provider_id).to_string())
            .collect(),
        selected_provider_id: facts.selected_provider_id.map(str::to_string),
        selected_provider_id_absent: facts.selected_provider_id_absent,
        active_registry_built: inactive.active_registry_built,
        would_build_registry: inactive.would_build_registry,
        would_select_provider: diagnostic_actions.would_select_provider,
        would_consume_proof: diagnostic_actions.would_consume_proof,
        would_prepare_rollback: diagnostic_actions.would_prepare_rollback,
        would_open_activation_gate: diagnostic_actions.would_open_activation_gate,
        would_install_hook: diagnostic_actions.would_install_hook,
        would_replace_process_allocator: diagnostic_actions.would_replace_process_allocator,
        would_activate: diagnostic_actions.would_activate,
    }
}

pub fn validate_allocator_provider_selection_decision_from_text(
    selection_decision_toml: &str,
) -> AllocatorProviderSelectionDecisionReport {
    let value = match toml::from_str::<toml::Value>(selection_decision_toml) {
        Ok(value) => value,
        Err(err) => {
            let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
            let diagnostic_actions = inactive.diagnostic_actions;
            return AllocatorProviderSelectionDecisionReport {
                status: AllocatorProviderSelectionDecisionStatus::MissingFacts,
                diagnostic: DIAG_PROVIDER_SELECTION_DECISION_MISSING,
                parse_error: Some(err.to_string()),
                missing_facts: vec!["parse_toml"],
                missing_diagnostics: vec![DIAG_PROVIDER_SELECTION_DECISION_MISSING],
                requested_provider_id: None,
                required_operations: Vec::new(),
                candidate_provider_ids: Vec::new(),
                selected_provider_id: None,
                selected_provider_id_absent: false,
                active_registry_built: inactive.active_registry_built,
                would_build_registry: inactive.would_build_registry,
                would_select_provider: diagnostic_actions.would_select_provider,
                would_consume_proof: diagnostic_actions.would_consume_proof,
                would_prepare_rollback: diagnostic_actions.would_prepare_rollback,
                would_open_activation_gate: diagnostic_actions.would_open_activation_gate,
                would_install_hook: diagnostic_actions.would_install_hook,
                would_replace_process_allocator: diagnostic_actions.would_replace_process_allocator,
                would_activate: diagnostic_actions.would_activate,
            };
        }
    };

    let facts = read_selection_decision_facts(&value);
    validate_allocator_provider_selection_decision(&facts)
}

fn read_selection_decision_facts(
    value: &toml::Value,
) -> AllocatorProviderSelectionDecisionFacts<'_> {
    let requested_provider_id = nonempty_text_field(value, "requested_provider_id");
    let required_operations = string_list(value.get("required_operations"));
    let candidate_provider_ids = string_list(value.get("candidate_provider_ids"));
    let selected_provider_id = nonempty_text_field(value, "selected_provider_id");

    AllocatorProviderSelectionDecisionFacts {
        schema_ready: text_field_matches(
            value,
            "schema_version",
            "allocator_provider_selection_decision_v0",
        ),
        status_reserved: text_field_matches(value, "status", "reserved"),
        active_false: bool_field_false(value, "active"),
        owner_named: text_field_matches(value, "selection_owner", OWNER_PATH),
        registry_snapshot_ready: text_field_matches(
            value,
            "registry_snapshot_input",
            "allocator_provider_registry_snapshot_report",
        ),
        selection_request_caller_provided: text_field_matches(
            value,
            "selection_request_source",
            "caller_provided_diagnostic_request",
        ),
        requested_provider_id,
        requested_provider_supported: requested_provider_id
            .map(|requested| candidate_provider_ids.iter().any(|id| *id == requested))
            .unwrap_or(false),
        required_operations_nonempty: !required_operations.is_empty(),
        candidate_provider_ids_reserved_set: string_list_matches(
            value.get("candidate_provider_ids"),
            EXPECTED_PROVIDER_IDS,
        ),
        deterministic_provider_order_named: text_field_matches(
            value,
            "deterministic_provider_order",
            "registry_snapshot_order",
        ),
        selection_policy_named: text_field_matches(
            value,
            "selection_policy",
            "explicit_provider_id_required_reserved",
        ),
        selection_status_reserved: text_field_matches(
            value,
            "selection_status",
            "reserved_no_selection",
        ),
        provider_selection_inactive: text_field_matches(value, "provider_selection", "inactive"),
        decision_status_reserved: text_field_matches(value, "decision_status", "reserved"),
        selected_provider_id,
        selected_provider_id_absent: selected_provider_id == Some("none_reserved"),
        would_build_registry_false: bool_field_false(value, "would_build_registry"),
        would_select_provider_false: bool_field_false(value, "would_select_provider"),
        would_activate_false: bool_field_false(value, "would_activate"),
        activation_future_row_required: text_field_matches(
            value,
            "activation",
            "future_row_required",
        ),
        diagnostic_named: text_field_matches(
            value,
            "diagnostic",
            DIAG_PROVIDER_SELECTION_DECISION_MISSING,
        ),
        missing_registry_diagnostic_named: text_field_matches(
            value,
            "missing_registry_diagnostic",
            DIAG_PROVIDER_SELECTION_REGISTRY_MISSING,
        ),
        missing_request_diagnostic_named: text_field_matches(
            value,
            "missing_request_diagnostic",
            DIAG_PROVIDER_SELECTION_REQUEST_MISSING,
        ),
        unsupported_provider_diagnostic_named: text_field_matches(
            value,
            "unsupported_provider_diagnostic",
            DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER,
        ),
        missing_capability_diagnostic_named: text_field_matches(
            value,
            "missing_capability_diagnostic",
            DIAG_PROVIDER_SELECTION_CAPABILITY_MISSING,
        ),
        ambiguous_provider_diagnostic_named: text_field_matches(
            value,
            "ambiguous_provider_diagnostic",
            DIAG_PROVIDER_SELECTION_AMBIGUOUS,
        ),
        required_fact_list_complete: string_list_contains_all(
            value.get("required_selection_decision_facts"),
            REQUIRED_SELECTION_DECISION_FACTS,
        ),
        required_operations,
        candidate_provider_ids,
    }
}

fn selection_decision_fact_checks(
    facts: &AllocatorProviderSelectionDecisionFacts<'_>,
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
            name: "selection_owner_named",
        },
        DiagnosticFactCheck {
            present: facts.registry_snapshot_ready,
            name: "registry_snapshot_ready",
        },
        DiagnosticFactCheck {
            present: facts.selection_request_caller_provided,
            name: "selection_request_caller_provided",
        },
        DiagnosticFactCheck {
            present: facts.requested_provider_id.is_some(),
            name: "requested_provider_id_explicit",
        },
        DiagnosticFactCheck {
            present: facts.requested_provider_supported,
            name: "requested_provider_supported",
        },
        DiagnosticFactCheck {
            present: facts.required_operations_nonempty,
            name: "required_operations_nonempty",
        },
        DiagnosticFactCheck {
            present: facts.candidate_provider_ids_reserved_set,
            name: "candidate_provider_ids_reserved_set",
        },
        DiagnosticFactCheck {
            present: facts.deterministic_provider_order_named,
            name: "deterministic_provider_order_named",
        },
        DiagnosticFactCheck {
            present: facts.selection_policy_named,
            name: "selection_policy_named",
        },
        DiagnosticFactCheck {
            present: facts.selection_status_reserved,
            name: "selection_status_reserved",
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
            present: facts.selected_provider_id_absent,
            name: "no_selected_provider_id",
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
            present: facts.would_activate_false,
            name: "would_activate_false",
        },
        DiagnosticFactCheck {
            present: facts.activation_future_row_required,
            name: "activation_future_row_required",
        },
        DiagnosticFactCheck {
            present: facts.diagnostic_named,
            name: "selection_decision_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_registry_diagnostic_named,
            name: "missing_registry_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_request_diagnostic_named,
            name: "missing_request_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.unsupported_provider_diagnostic_named,
            name: "unsupported_provider_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_capability_diagnostic_named,
            name: "missing_capability_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.ambiguous_provider_diagnostic_named,
            name: "ambiguous_provider_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.required_fact_list_complete,
            name: "required_selection_decision_facts_complete",
        },
    ]
}

fn collect_missing_selection_decision_facts(
    facts: &AllocatorProviderSelectionDecisionFacts<'_>,
) -> Vec<&'static str> {
    selection_decision_fact_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.name))
        .collect()
}

fn collect_selection_decision_missing_diagnostics(
    facts: &AllocatorProviderSelectionDecisionFacts<'_>,
) -> Vec<&'static str> {
    [
        (
            facts.schema_ready && facts.status_reserved && facts.active_false && facts.owner_named,
            DIAG_PROVIDER_SELECTION_DECISION_MISSING,
        ),
        (
            facts.registry_snapshot_ready,
            DIAG_PROVIDER_SELECTION_REGISTRY_MISSING,
        ),
        (
            facts.selection_request_caller_provided && facts.requested_provider_id.is_some(),
            DIAG_PROVIDER_SELECTION_REQUEST_MISSING,
        ),
        (
            facts.requested_provider_id.is_none() || facts.requested_provider_supported,
            DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER,
        ),
        (
            facts.required_operations_nonempty,
            DIAG_PROVIDER_SELECTION_CAPABILITY_MISSING,
        ),
        (
            facts.candidate_provider_ids_reserved_set && facts.deterministic_provider_order_named,
            DIAG_PROVIDER_SELECTION_AMBIGUOUS,
        ),
    ]
    .into_iter()
    .filter_map(|(present, diagnostic)| (!present).then_some(diagnostic))
    .collect()
}
