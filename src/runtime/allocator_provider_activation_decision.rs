//! Diagnostic-only allocator provider activation decision reports.
//!
//! This module parses caller-provided TOML text and reports the reserved
//! activation decision surface. It does not select a provider, consume proofs,
//! prepare rollback, open an activation gate, install hooks, or replace the
//! process allocator.

use super::allocator_provider_diagnostic_inactive::DIAGNOSTIC_INACTIVE_ACTIONS;

pub const DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED: &str =
    "[allocator-provider/activation-decision-reserved]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_SAFETY_GATE_MISSING: &str =
    "[allocator-provider/activation-decision-safety-gate-missing]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_REGISTRY_MISSING: &str =
    "[allocator-provider/activation-decision-registry-missing]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_SELECTION_MISSING: &str =
    "[allocator-provider/activation-decision-selection-missing]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_PROOF_BUNDLE_MISSING: &str =
    "[allocator-provider/activation-decision-proof-bundle-missing]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_ROLLBACK_MISSING: &str =
    "[allocator-provider/activation-decision-rollback-missing]";
pub const DIAG_PROVIDER_ACTIVATION_DECISION_BLOCKED: &str =
    "[allocator-provider/activation-decision-blocked]";

const DECISION_SURFACE_STATUS_RESERVED: &str = "reserved_fixture";
const OWNER_PATH: &str = "src/runtime/allocator_provider_activation_decision.rs";

const REQUIRED_ACTIVATION_DECISION_FACTS: &[&str] = &[
    "activation_decision_bundle_caller_provided",
    "operator_intent_diagnose_only",
    "requested_provider_id_explicit",
    "activation_safety_gate_report_path_explicit",
    "registry_snapshot_path_explicit",
    "selection_decision_path_explicit",
    "proof_bundle_report_path_explicit",
    "rollback_preflight_report_path_explicit",
    "activation_decision_allowed_false",
    "activation_gate_closed",
    "fail_fast_activation_decision_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_proof_discovery",
    "no_implicit_report_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "diagnostic_runtime_parser_report_only",
    "no_implicit_cli_route",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_activation_gate_opening",
    "no_hook_activation_implementation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_process_allocator_replacement",
    "no_route_widening",
];

const EXPECTED_ACTIVATION_DECISION_INPUTS: &[(&str, &str, &str)] = &[
    (
        "activation_safety_gate",
        "activation_safety_gate_report_path",
        DIAG_PROVIDER_ACTIVATION_DECISION_SAFETY_GATE_MISSING,
    ),
    (
        "registry_snapshot",
        "registry_snapshot_path",
        DIAG_PROVIDER_ACTIVATION_DECISION_REGISTRY_MISSING,
    ),
    (
        "selection_decision",
        "selection_decision_path",
        DIAG_PROVIDER_ACTIVATION_DECISION_SELECTION_MISSING,
    ),
    (
        "proof_bundle",
        "proof_bundle_report_path",
        DIAG_PROVIDER_ACTIVATION_DECISION_PROOF_BUNDLE_MISSING,
    ),
    (
        "rollback_preflight",
        "rollback_preflight_report_path",
        DIAG_PROVIDER_ACTIVATION_DECISION_ROLLBACK_MISSING,
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderActivationDecisionStatus {
    MissingFacts,
    ReadyBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderActivationDecisionFacts<'a> {
    pub surface_version_ready: bool,
    pub status_reserved: bool,
    pub active_false: bool,
    pub owner_named: bool,
    pub input_source_caller_provided: bool,
    pub operator_intent_diagnose: bool,
    pub requested_provider_id: Option<&'a str>,
    pub activation_safety_gate_report_path: Option<&'a str>,
    pub registry_snapshot_path: Option<&'a str>,
    pub selection_decision_path: Option<&'a str>,
    pub proof_bundle_report_path: Option<&'a str>,
    pub rollback_preflight_report_path: Option<&'a str>,
    pub decision_surface_status_reserved: bool,
    pub activation_decision_allowed_false: bool,
    pub provider_selection_inactive: bool,
    pub proof_bundle_consumption_inactive: bool,
    pub rollback_preparation_inactive: bool,
    pub activation_gate_closed: bool,
    pub hook_activation_inactive: bool,
    pub process_allocator_replacement_inactive: bool,
    pub would_select_provider_false: bool,
    pub would_consume_proof_false: bool,
    pub would_prepare_rollback_false: bool,
    pub would_open_activation_gate_false: bool,
    pub would_install_hook_false: bool,
    pub would_replace_process_allocator_false: bool,
    pub would_activate_false: bool,
    pub activation_future_row_required: bool,
    pub diagnostic_reserved_named: bool,
    pub activation_blocked_diagnostic_named: bool,
    pub required_fact_list_complete: bool,
    pub activation_decision_inputs_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderActivationDecisionReport {
    pub status: AllocatorProviderActivationDecisionStatus,
    pub diagnostic: &'static str,
    pub parse_error: Option<String>,
    pub missing_facts: Vec<&'static str>,
    pub missing_diagnostics: Vec<&'static str>,
    pub operator_intent: Option<String>,
    pub requested_provider_id: Option<String>,
    pub activation_safety_gate_report_path: Option<String>,
    pub registry_snapshot_path: Option<String>,
    pub selection_decision_path: Option<String>,
    pub proof_bundle_report_path: Option<String>,
    pub rollback_preflight_report_path: Option<String>,
    pub activation_decision_surface_status: &'static str,
    pub activation_decision_allowed: bool,
    pub would_select_provider: bool,
    pub would_consume_proof: bool,
    pub would_prepare_rollback: bool,
    pub would_open_activation_gate: bool,
    pub would_install_hook: bool,
    pub would_replace_process_allocator: bool,
    pub would_activate: bool,
}

pub fn validate_allocator_provider_activation_decision(
    facts: &AllocatorProviderActivationDecisionFacts<'_>,
) -> AllocatorProviderActivationDecisionReport {
    let inactive = DIAGNOSTIC_INACTIVE_ACTIONS;
    let missing_facts = collect_missing_activation_decision_facts(facts);
    let missing_diagnostics = collect_activation_decision_missing_diagnostics(facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderActivationDecisionStatus::ReadyBlocked,
            DIAG_PROVIDER_ACTIVATION_DECISION_BLOCKED,
        )
    } else {
        (
            AllocatorProviderActivationDecisionStatus::MissingFacts,
            DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED,
        )
    };

    AllocatorProviderActivationDecisionReport {
        status,
        diagnostic,
        parse_error: None,
        missing_facts,
        missing_diagnostics,
        operator_intent: facts
            .operator_intent_diagnose
            .then_some("diagnose".to_string()),
        requested_provider_id: facts.requested_provider_id.map(str::to_string),
        activation_safety_gate_report_path: facts
            .activation_safety_gate_report_path
            .map(str::to_string),
        registry_snapshot_path: facts.registry_snapshot_path.map(str::to_string),
        selection_decision_path: facts.selection_decision_path.map(str::to_string),
        proof_bundle_report_path: facts.proof_bundle_report_path.map(str::to_string),
        rollback_preflight_report_path: facts.rollback_preflight_report_path.map(str::to_string),
        activation_decision_surface_status: DECISION_SURFACE_STATUS_RESERVED,
        activation_decision_allowed: false,
        would_select_provider: inactive.would_select_provider,
        would_consume_proof: inactive.would_consume_proof,
        would_prepare_rollback: inactive.would_prepare_rollback,
        would_open_activation_gate: inactive.would_open_activation_gate,
        would_install_hook: inactive.would_install_hook,
        would_replace_process_allocator: inactive.would_replace_process_allocator,
        would_activate: inactive.would_activate,
    }
}

pub fn validate_allocator_provider_activation_decision_from_text(
    activation_decision_toml: &str,
) -> AllocatorProviderActivationDecisionReport {
    let value = match toml::from_str::<toml::Value>(activation_decision_toml) {
        Ok(value) => value,
        Err(err) => {
            let inactive = DIAGNOSTIC_INACTIVE_ACTIONS;
            return AllocatorProviderActivationDecisionReport {
                status: AllocatorProviderActivationDecisionStatus::MissingFacts,
                diagnostic: DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED,
                parse_error: Some(err.to_string()),
                missing_facts: vec!["parse_toml"],
                missing_diagnostics: vec![DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED],
                operator_intent: None,
                requested_provider_id: None,
                activation_safety_gate_report_path: None,
                registry_snapshot_path: None,
                selection_decision_path: None,
                proof_bundle_report_path: None,
                rollback_preflight_report_path: None,
                activation_decision_surface_status: DECISION_SURFACE_STATUS_RESERVED,
                activation_decision_allowed: false,
                would_select_provider: inactive.would_select_provider,
                would_consume_proof: inactive.would_consume_proof,
                would_prepare_rollback: inactive.would_prepare_rollback,
                would_open_activation_gate: inactive.would_open_activation_gate,
                would_install_hook: inactive.would_install_hook,
                would_replace_process_allocator: inactive.would_replace_process_allocator,
                would_activate: inactive.would_activate,
            };
        }
    };

    let facts = read_activation_decision_facts(&value);
    validate_allocator_provider_activation_decision(&facts)
}

struct ActivationDecisionFactCheck {
    present: bool,
    name: &'static str,
}

fn read_activation_decision_facts(
    value: &toml::Value,
) -> AllocatorProviderActivationDecisionFacts<'_> {
    AllocatorProviderActivationDecisionFacts {
        surface_version_ready: value.get("surface_version").and_then(toml::Value::as_str)
            == Some("allocator_provider_activation_decision_v0"),
        status_reserved: text_field_matches(value, "status", "reserved"),
        active_false: bool_field_false(value, "active"),
        owner_named: text_field_matches(value, "decision_surface_owner", OWNER_PATH),
        input_source_caller_provided: text_field_matches(
            value,
            "input_source",
            "caller_provided_activation_decision_bundle",
        ),
        operator_intent_diagnose: text_field_matches(value, "operator_intent", "diagnose"),
        requested_provider_id: nonempty_text_field(value, "requested_provider_id"),
        activation_safety_gate_report_path: nonempty_text_field(
            value,
            "activation_safety_gate_report_path",
        ),
        registry_snapshot_path: nonempty_text_field(value, "registry_snapshot_path"),
        selection_decision_path: nonempty_text_field(value, "selection_decision_path"),
        proof_bundle_report_path: nonempty_text_field(value, "proof_bundle_report_path"),
        rollback_preflight_report_path: nonempty_text_field(
            value,
            "rollback_preflight_report_path",
        ),
        decision_surface_status_reserved: text_field_matches(
            value,
            "activation_decision_surface_status",
            DECISION_SURFACE_STATUS_RESERVED,
        ),
        activation_decision_allowed_false: bool_field_false(value, "activation_decision_allowed"),
        provider_selection_inactive: text_field_matches(value, "provider_selection", "inactive"),
        proof_bundle_consumption_inactive: text_field_matches(
            value,
            "proof_bundle_consumption",
            "inactive",
        ),
        rollback_preparation_inactive: text_field_matches(
            value,
            "rollback_preparation",
            "inactive",
        ),
        activation_gate_closed: text_field_matches(value, "activation_gate", "closed_reserved"),
        hook_activation_inactive: text_field_matches(value, "hook_activation", "inactive"),
        process_allocator_replacement_inactive: text_field_matches(
            value,
            "process_allocator_replacement",
            "inactive",
        ),
        would_select_provider_false: bool_field_false(value, "would_select_provider"),
        would_consume_proof_false: bool_field_false(value, "would_consume_proof"),
        would_prepare_rollback_false: bool_field_false(value, "would_prepare_rollback"),
        would_open_activation_gate_false: bool_field_false(value, "would_open_activation_gate"),
        would_install_hook_false: bool_field_false(value, "would_install_hook"),
        would_replace_process_allocator_false: bool_field_false(
            value,
            "would_replace_process_allocator",
        ),
        would_activate_false: bool_field_false(value, "would_activate"),
        activation_future_row_required: text_field_matches(
            value,
            "activation",
            "future_row_required",
        ),
        diagnostic_reserved_named: text_field_matches(
            value,
            "diagnostic",
            DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED,
        ),
        activation_blocked_diagnostic_named: text_field_matches(
            value,
            "activation_blocked_diagnostic",
            DIAG_PROVIDER_ACTIVATION_DECISION_BLOCKED,
        ),
        required_fact_list_complete: string_list_contains_all(
            value.get("reserved_activation_decision_facts"),
            REQUIRED_ACTIVATION_DECISION_FACTS,
        ),
        activation_decision_inputs_complete: activation_decision_inputs_complete(value),
    }
}

fn activation_decision_fact_checks(
    facts: &AllocatorProviderActivationDecisionFacts<'_>,
) -> Vec<ActivationDecisionFactCheck> {
    vec![
        ActivationDecisionFactCheck {
            present: facts.surface_version_ready,
            name: "surface_version",
        },
        ActivationDecisionFactCheck {
            present: facts.status_reserved,
            name: "status_reserved",
        },
        ActivationDecisionFactCheck {
            present: facts.active_false,
            name: "active_false",
        },
        ActivationDecisionFactCheck {
            present: facts.owner_named,
            name: "decision_surface_owner_named",
        },
        ActivationDecisionFactCheck {
            present: facts.input_source_caller_provided,
            name: "input_source_caller_provided",
        },
        ActivationDecisionFactCheck {
            present: facts.operator_intent_diagnose,
            name: "operator_intent_diagnose",
        },
        ActivationDecisionFactCheck {
            present: facts.requested_provider_id.is_some(),
            name: "requested_provider_id_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_safety_gate_report_path.is_some(),
            name: "activation_safety_gate_report_path_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.registry_snapshot_path.is_some(),
            name: "registry_snapshot_path_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.selection_decision_path.is_some(),
            name: "selection_decision_path_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.proof_bundle_report_path.is_some(),
            name: "proof_bundle_report_path_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.rollback_preflight_report_path.is_some(),
            name: "rollback_preflight_report_path_explicit",
        },
        ActivationDecisionFactCheck {
            present: facts.decision_surface_status_reserved,
            name: "activation_decision_surface_status_reserved",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_decision_allowed_false,
            name: "activation_decision_allowed_false",
        },
        ActivationDecisionFactCheck {
            present: facts.provider_selection_inactive,
            name: "provider_selection_inactive",
        },
        ActivationDecisionFactCheck {
            present: facts.proof_bundle_consumption_inactive,
            name: "proof_bundle_consumption_inactive",
        },
        ActivationDecisionFactCheck {
            present: facts.rollback_preparation_inactive,
            name: "rollback_preparation_inactive",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_gate_closed,
            name: "activation_gate_closed",
        },
        ActivationDecisionFactCheck {
            present: facts.hook_activation_inactive,
            name: "hook_activation_inactive",
        },
        ActivationDecisionFactCheck {
            present: facts.process_allocator_replacement_inactive,
            name: "process_allocator_replacement_inactive",
        },
        ActivationDecisionFactCheck {
            present: facts.would_select_provider_false,
            name: "would_select_provider_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_consume_proof_false,
            name: "would_consume_proof_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_prepare_rollback_false,
            name: "would_prepare_rollback_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_open_activation_gate_false,
            name: "would_open_activation_gate_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_install_hook_false,
            name: "would_install_hook_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_replace_process_allocator_false,
            name: "would_replace_process_allocator_false",
        },
        ActivationDecisionFactCheck {
            present: facts.would_activate_false,
            name: "would_activate_false",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_future_row_required,
            name: "activation_future_row_required",
        },
        ActivationDecisionFactCheck {
            present: facts.diagnostic_reserved_named,
            name: "diagnostic_reserved_named",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_blocked_diagnostic_named,
            name: "fail_fast_activation_decision_diagnostic_named",
        },
        ActivationDecisionFactCheck {
            present: facts.required_fact_list_complete,
            name: "reserved_activation_decision_facts_complete",
        },
        ActivationDecisionFactCheck {
            present: facts.activation_decision_inputs_complete,
            name: "activation_decision_inputs_complete",
        },
    ]
}

fn collect_missing_activation_decision_facts(
    facts: &AllocatorProviderActivationDecisionFacts<'_>,
) -> Vec<&'static str> {
    activation_decision_fact_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.name))
        .collect()
}

fn collect_activation_decision_missing_diagnostics(
    facts: &AllocatorProviderActivationDecisionFacts<'_>,
) -> Vec<&'static str> {
    [
        (
            facts.activation_safety_gate_report_path.is_some(),
            DIAG_PROVIDER_ACTIVATION_DECISION_SAFETY_GATE_MISSING,
        ),
        (
            facts.registry_snapshot_path.is_some(),
            DIAG_PROVIDER_ACTIVATION_DECISION_REGISTRY_MISSING,
        ),
        (
            facts.selection_decision_path.is_some(),
            DIAG_PROVIDER_ACTIVATION_DECISION_SELECTION_MISSING,
        ),
        (
            facts.proof_bundle_report_path.is_some(),
            DIAG_PROVIDER_ACTIVATION_DECISION_PROOF_BUNDLE_MISSING,
        ),
        (
            facts.rollback_preflight_report_path.is_some(),
            DIAG_PROVIDER_ACTIVATION_DECISION_ROLLBACK_MISSING,
        ),
    ]
    .into_iter()
    .filter_map(|(present, diagnostic)| (!present).then_some(diagnostic))
    .collect()
}

fn activation_decision_inputs_complete(value: &toml::Value) -> bool {
    let Some(inputs) = value
        .get("activation_decision_inputs")
        .and_then(toml::Value::as_array)
    else {
        return false;
    };
    EXPECTED_ACTIVATION_DECISION_INPUTS
        .iter()
        .all(|(name, path_key, diagnostic)| {
            inputs.iter().any(|input| {
                input.get("name").and_then(toml::Value::as_str) == Some(*name)
                    && input.get("source").and_then(toml::Value::as_str)
                        == Some("caller_provided_path")
                    && input.get("path_key").and_then(toml::Value::as_str) == Some(*path_key)
                    && input.get("required").and_then(toml::Value::as_bool) == Some(true)
                    && input
                        .get("missing_diagnostic")
                        .and_then(toml::Value::as_str)
                        == Some(*diagnostic)
            })
        })
}

fn text_field_matches(value: &toml::Value, key: &str, expected: &str) -> bool {
    value.get(key).and_then(toml::Value::as_str) == Some(expected)
}

fn bool_field_false(value: &toml::Value, key: &str) -> bool {
    value.get(key).and_then(toml::Value::as_bool) == Some(false)
}

fn nonempty_text_field<'a>(value: &'a toml::Value, key: &str) -> Option<&'a str> {
    let text = value.get(key)?.as_str()?;
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn string_list_contains_all(value: Option<&toml::Value>, required: &[&str]) -> bool {
    let Some(items) = value.and_then(toml::Value::as_array) else {
        return false;
    };
    required.iter().all(|required| {
        items
            .iter()
            .filter_map(toml::Value::as_str)
            .any(|item| item == *required)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVATION_DECISION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
    );

    #[test]
    fn activation_decision_fixture_reports_blocked_without_activation() {
        let report =
            validate_allocator_provider_activation_decision_from_text(ACTIVATION_DECISION_FIXTURE);

        assert_eq!(
            report.status,
            AllocatorProviderActivationDecisionStatus::ReadyBlocked
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_ACTIVATION_DECISION_BLOCKED);
        assert_eq!(report.parse_error, None);
        assert!(report.missing_facts.is_empty());
        assert!(report.missing_diagnostics.is_empty());
        assert_eq!(report.operator_intent.as_deref(), Some("diagnose"));
        assert_eq!(report.requested_provider_id.as_deref(), Some("mimalloc"));
        assert_eq!(
            report.activation_safety_gate_report_path.as_deref(),
            Some("activation-safety-gate-v0.toml")
        );
        assert_eq!(
            report.registry_snapshot_path.as_deref(),
            Some("registry-snapshot-v0.toml")
        );
        assert_eq!(
            report.selection_decision_path.as_deref(),
            Some("selection-decision-v0.toml")
        );
        assert_eq!(
            report.proof_bundle_report_path.as_deref(),
            Some("proof-bundle-consumption-v0.toml")
        );
        assert_eq!(
            report.rollback_preflight_report_path.as_deref(),
            Some("rollback-preflight-v0.toml")
        );
        assert_eq!(
            report.activation_decision_surface_status,
            DECISION_SURFACE_STATUS_RESERVED
        );
        assert!(!report.activation_decision_allowed);
        assert!(!report.would_select_provider);
        assert!(!report.would_consume_proof);
        assert!(!report.would_prepare_rollback);
        assert!(!report.would_open_activation_gate);
        assert!(!report.would_install_hook);
        assert!(!report.would_replace_process_allocator);
        assert!(!report.would_activate);
    }

    #[test]
    fn activation_decision_empty_text_reports_missing_without_activation() {
        let report = validate_allocator_provider_activation_decision_from_text("");

        assert_eq!(
            report.status,
            AllocatorProviderActivationDecisionStatus::MissingFacts
        );
        assert_eq!(
            report.diagnostic,
            DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED
        );
        assert_eq!(report.parse_error, None);
        assert!(report.missing_facts.contains(&"surface_version"));
        assert!(report
            .missing_facts
            .contains(&"activation_safety_gate_report_path_explicit"));
        assert!(report
            .missing_diagnostics
            .contains(&DIAG_PROVIDER_ACTIVATION_DECISION_SAFETY_GATE_MISSING));
        assert!(!report.activation_decision_allowed);
        assert!(!report.would_select_provider);
        assert!(!report.would_prepare_rollback);
        assert!(!report.would_activate);
    }

    #[test]
    fn activation_decision_missing_path_reports_input_diagnostic() {
        let text = ACTIVATION_DECISION_FIXTURE.replace(
            "activation_safety_gate_report_path = \"activation-safety-gate-v0.toml\"",
            "activation_safety_gate_report_path = \"\"",
        );
        let report = validate_allocator_provider_activation_decision_from_text(&text);

        assert_eq!(
            report.status,
            AllocatorProviderActivationDecisionStatus::MissingFacts
        );
        assert!(report
            .missing_facts
            .contains(&"activation_safety_gate_report_path_explicit"));
        assert!(report
            .missing_diagnostics
            .contains(&DIAG_PROVIDER_ACTIVATION_DECISION_SAFETY_GATE_MISSING));
        assert!(!report.would_activate);
    }

    #[test]
    fn activation_decision_malformed_text_reports_parse_error_without_activation() {
        let report = validate_allocator_provider_activation_decision_from_text("[");

        assert_eq!(
            report.status,
            AllocatorProviderActivationDecisionStatus::MissingFacts
        );
        assert_eq!(
            report.diagnostic,
            DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED
        );
        assert!(report.parse_error.is_some());
        assert_eq!(report.missing_facts, vec!["parse_toml"]);
        assert_eq!(
            report.missing_diagnostics,
            vec![DIAG_PROVIDER_ACTIVATION_DECISION_RESERVED]
        );
        assert!(!report.would_open_activation_gate);
        assert!(!report.would_install_hook);
        assert!(!report.would_replace_process_allocator);
        assert!(!report.would_activate);
    }
}
