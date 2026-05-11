//! Diagnostic-only allocator provider activation safety report.

use super::allocator_provider_diagnostic_inactive::SAFETY_GATE_INACTIVE_ACTIONS;
use super::allocator_provider_registry_common::{
    string_list_matches, EXPECTED_PROVIDER_IDS, OWNER_PATH,
};
use super::allocator_provider_toml_helpers::{
    nonempty_text_field, string_list_contains_all, text_field_matches, DiagnosticFactCheck,
};

pub const DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING: &str =
    "[allocator-provider/activation-safety-gate-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_ENTRY_MISSING: &str =
    "[allocator-provider/activation-safety-entry-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_READINESS_MISSING: &str =
    "[allocator-provider/activation-safety-readiness-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_COMBINED_DRY_RUN_MISSING: &str =
    "[allocator-provider/activation-safety-combined-dry-run-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_REGISTRY_MISSING: &str =
    "[allocator-provider/activation-safety-registry-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_SELECTION_MISSING: &str =
    "[allocator-provider/activation-safety-selection-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_BUNDLE_MISSING: &str =
    "[allocator-provider/activation-safety-proof-bundle-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_ROLLBACK_MISSING: &str =
    "[allocator-provider/activation-safety-rollback-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_HOOK_PLAN_MISSING: &str =
    "[allocator-provider/activation-safety-hook-plan-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_PREFLIGHT_MISSING: &str =
    "[allocator-provider/activation-safety-preflight-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_MISSING: &str =
    "[allocator-provider/activation-safety-proof-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING: &str =
    "[allocator-provider/activation-safety-target-missing]";
pub const DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED: &str =
    "[allocator-provider/activation-safety-blocked]";

pub(crate) const SAFETY_STATUS_GATE_CLOSED: &str = "reserved_gate_closed";

const REQUIRED_ACTIVATION_SAFETY_FACTS: &[&str] = &[
    "activation_entry_contract_ready",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "registry_snapshot_ready",
    "selection_decision_ready",
    "selected_provider_id_absent",
    "proof_bundle_ready",
    "rollback_preflight_ready",
    "hook_plan_ready",
    "hook_activation_preflight_ready",
    "activation_proof_ready",
    "rollback_target_explicit",
    "activation_target_provider_id_explicit",
    "safety_gate_policy_named",
    "activation_gate_closed",
    "fail_fast_activation_safety_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_proof_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_hook_activation_implementation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_process_allocator_replacement",
    "no_route_widening",
];

const EXPECTED_SAFETY_INPUTS: &[(&str, &str)] = &[
    (
        "activation_entry",
        DIAG_PROVIDER_ACTIVATION_SAFETY_ENTRY_MISSING,
    ),
    (
        "provider_readiness",
        DIAG_PROVIDER_ACTIVATION_SAFETY_READINESS_MISSING,
    ),
    (
        "combined_dry_run",
        DIAG_PROVIDER_ACTIVATION_SAFETY_COMBINED_DRY_RUN_MISSING,
    ),
    (
        "registry_snapshot",
        DIAG_PROVIDER_ACTIVATION_SAFETY_REGISTRY_MISSING,
    ),
    (
        "selection_decision",
        DIAG_PROVIDER_ACTIVATION_SAFETY_SELECTION_MISSING,
    ),
    (
        "proof_bundle",
        DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_BUNDLE_MISSING,
    ),
    (
        "rollback_preflight",
        DIAG_PROVIDER_ACTIVATION_SAFETY_ROLLBACK_MISSING,
    ),
    (
        "hook_plan",
        DIAG_PROVIDER_ACTIVATION_SAFETY_HOOK_PLAN_MISSING,
    ),
    (
        "hook_activation_preflight",
        DIAG_PROVIDER_ACTIVATION_SAFETY_PREFLIGHT_MISSING,
    ),
    (
        "activation_proof",
        DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_MISSING,
    ),
    (
        "activation_target",
        DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING,
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderActivationSafetyStatus {
    MissingFacts,
    ReadyGateClosed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderActivationSafetyFacts<'a> {
    pub schema_ready: bool,
    pub status_reserved: bool,
    pub active_false: bool,
    pub owner_named: bool,
    pub activation_entry_contract_ready: bool,
    pub provider_readiness_preflight_ready: bool,
    pub combined_dry_run_ready: bool,
    pub registry_snapshot_ready: bool,
    pub selection_decision_ready: bool,
    pub selected_provider_id_absent: bool,
    pub proof_bundle_ready: bool,
    pub rollback_preflight_ready: bool,
    pub hook_plan_ready: bool,
    pub hook_activation_preflight_ready: bool,
    pub activation_proof_ready: bool,
    pub rollback_target_provider_id: Option<&'a str>,
    pub activation_target_provider_id: Option<&'a str>,
    pub safety_gate_policy_named: bool,
    pub activation_gate_closed: bool,
    pub activation_blocked_diagnostic_named: bool,
    pub required_operations_named: bool,
    pub candidate_provider_ids_reserved_set: bool,
    pub required_fact_list_complete: bool,
    pub safety_inputs_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderActivationSafetyReport {
    pub status: AllocatorProviderActivationSafetyStatus,
    pub diagnostic: &'static str,
    pub parse_error: Option<String>,
    pub missing_facts: Vec<&'static str>,
    pub missing_diagnostics: Vec<&'static str>,
    pub rollback_target_provider_id: Option<String>,
    pub activation_target_provider_id: Option<String>,
    pub safety_status: &'static str,
    pub activation_gate_open: bool,
    pub would_open_activation_gate: bool,
    pub would_activate_hook: bool,
    pub would_activate: bool,
}

pub fn validate_allocator_provider_activation_safety_gate(
    facts: &AllocatorProviderActivationSafetyFacts<'_>,
) -> AllocatorProviderActivationSafetyReport {
    let inactive = SAFETY_GATE_INACTIVE_ACTIONS;
    let missing_facts = collect_missing_activation_safety_facts(facts);
    let missing_diagnostics = collect_activation_safety_missing_diagnostics(facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderActivationSafetyStatus::ReadyGateClosed,
            DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED,
        )
    } else {
        (
            AllocatorProviderActivationSafetyStatus::MissingFacts,
            DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING,
        )
    };

    AllocatorProviderActivationSafetyReport {
        status,
        diagnostic,
        parse_error: None,
        missing_facts,
        missing_diagnostics,
        rollback_target_provider_id: facts.rollback_target_provider_id.map(str::to_string),
        activation_target_provider_id: facts.activation_target_provider_id.map(str::to_string),
        safety_status: SAFETY_STATUS_GATE_CLOSED,
        activation_gate_open: inactive.activation_gate_open,
        would_open_activation_gate: inactive.would_open_activation_gate,
        would_activate_hook: inactive.would_activate_hook,
        would_activate: inactive.would_activate,
    }
}

pub fn validate_allocator_provider_activation_safety_gate_from_text(
    safety_gate_toml: &str,
) -> AllocatorProviderActivationSafetyReport {
    let value = match toml::from_str::<toml::Value>(safety_gate_toml) {
        Ok(value) => value,
        Err(err) => {
            let inactive = SAFETY_GATE_INACTIVE_ACTIONS;
            return AllocatorProviderActivationSafetyReport {
                status: AllocatorProviderActivationSafetyStatus::MissingFacts,
                diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING,
                parse_error: Some(err.to_string()),
                missing_facts: vec!["parse_toml"],
                missing_diagnostics: vec![DIAG_PROVIDER_ACTIVATION_SAFETY_GATE_MISSING],
                rollback_target_provider_id: None,
                activation_target_provider_id: None,
                safety_status: SAFETY_STATUS_GATE_CLOSED,
                activation_gate_open: inactive.activation_gate_open,
                would_open_activation_gate: inactive.would_open_activation_gate,
                would_activate_hook: inactive.would_activate_hook,
                would_activate: inactive.would_activate,
            };
        }
    };

    let facts = read_activation_safety_facts(&value);
    validate_allocator_provider_activation_safety_gate(&facts)
}

struct ActivationSafetyDiagnosticCheck {
    present: bool,
    diagnostic: &'static str,
}

fn activation_safety_fact_checks(
    facts: &AllocatorProviderActivationSafetyFacts<'_>,
) -> [DiagnosticFactCheck; 24] {
    [
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
            name: "safety_gate_owner_named",
        },
        DiagnosticFactCheck {
            present: facts.activation_entry_contract_ready,
            name: "activation_entry_contract_ready",
        },
        DiagnosticFactCheck {
            present: facts.provider_readiness_preflight_ready,
            name: "provider_readiness_preflight_ready",
        },
        DiagnosticFactCheck {
            present: facts.combined_dry_run_ready,
            name: "combined_dry_run_ready",
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
            present: facts.selected_provider_id_absent,
            name: "selected_provider_id_absent",
        },
        DiagnosticFactCheck {
            present: facts.proof_bundle_ready,
            name: "proof_bundle_ready",
        },
        DiagnosticFactCheck {
            present: facts.rollback_preflight_ready,
            name: "rollback_preflight_ready",
        },
        DiagnosticFactCheck {
            present: facts.hook_plan_ready,
            name: "hook_plan_ready",
        },
        DiagnosticFactCheck {
            present: facts.hook_activation_preflight_ready,
            name: "hook_activation_preflight_ready",
        },
        DiagnosticFactCheck {
            present: facts.activation_proof_ready,
            name: "activation_proof_ready",
        },
        DiagnosticFactCheck {
            present: facts.rollback_target_provider_id.is_some(),
            name: "rollback_target_explicit",
        },
        DiagnosticFactCheck {
            present: facts.activation_target_provider_id.is_some(),
            name: "activation_target_provider_id_explicit",
        },
        DiagnosticFactCheck {
            present: facts.safety_gate_policy_named,
            name: "safety_gate_policy_named",
        },
        DiagnosticFactCheck {
            present: facts.activation_gate_closed,
            name: "activation_gate_closed",
        },
        DiagnosticFactCheck {
            present: facts.activation_blocked_diagnostic_named,
            name: "fail_fast_activation_safety_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.required_operations_named,
            name: "required_operations_named",
        },
        DiagnosticFactCheck {
            present: facts.candidate_provider_ids_reserved_set,
            name: "candidate_provider_ids_reserved_set",
        },
        DiagnosticFactCheck {
            present: facts.required_fact_list_complete,
            name: "reserved_activation_safety_facts_complete",
        },
        DiagnosticFactCheck {
            present: facts.safety_inputs_complete,
            name: "safety_inputs_complete",
        },
    ]
}

fn activation_safety_diagnostic_checks(
    facts: &AllocatorProviderActivationSafetyFacts<'_>,
) -> [ActivationSafetyDiagnosticCheck; 11] {
    [
        ActivationSafetyDiagnosticCheck {
            present: facts.activation_entry_contract_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_ENTRY_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.provider_readiness_preflight_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_READINESS_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.combined_dry_run_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_COMBINED_DRY_RUN_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.registry_snapshot_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_REGISTRY_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.selection_decision_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_SELECTION_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.proof_bundle_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_BUNDLE_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.rollback_preflight_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_ROLLBACK_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.hook_plan_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_HOOK_PLAN_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.hook_activation_preflight_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_PREFLIGHT_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.activation_proof_ready,
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_PROOF_MISSING,
        },
        ActivationSafetyDiagnosticCheck {
            present: facts.activation_target_provider_id.is_some(),
            diagnostic: DIAG_PROVIDER_ACTIVATION_SAFETY_TARGET_MISSING,
        },
    ]
}

fn read_activation_safety_facts(value: &toml::Value) -> AllocatorProviderActivationSafetyFacts<'_> {
    AllocatorProviderActivationSafetyFacts {
        schema_ready: value.get("schema_version").and_then(toml::Value::as_str)
            == Some("allocator_provider_activation_safety_gate_v0"),
        status_reserved: value.get("status").and_then(toml::Value::as_str) == Some("reserved"),
        active_false: value.get("active").and_then(toml::Value::as_bool) == Some(false),
        owner_named: value.get("safety_gate_owner").and_then(toml::Value::as_str)
            == Some(OWNER_PATH),
        activation_entry_contract_ready: text_field_matches(
            value,
            "activation_entry_input",
            "allocator_provider_activation_entry_contract",
        ),
        provider_readiness_preflight_ready: text_field_matches(
            value,
            "provider_readiness_input",
            "allocator_provider_readiness_preflight_report",
        ),
        combined_dry_run_ready: text_field_matches(
            value,
            "combined_dry_run_input",
            "allocator_provider_combined_dry_run_report",
        ),
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
        selected_provider_id_absent: value
            .get("selected_provider_id")
            .and_then(toml::Value::as_str)
            == Some("none_reserved"),
        proof_bundle_ready: text_field_matches(
            value,
            "proof_bundle_input",
            "allocator_provider_proof_bundle_consumption_report",
        ),
        rollback_preflight_ready: text_field_matches(
            value,
            "rollback_preflight_input",
            "allocator_provider_rollback_preflight_report",
        ),
        hook_plan_ready: text_field_matches(value, "hook_plan_input", "allocator_hook_plan_report"),
        hook_activation_preflight_ready: text_field_matches(
            value,
            "hook_activation_preflight_input",
            "allocator_hook_activation_preflight_report",
        ),
        activation_proof_ready: text_field_matches(
            value,
            "activation_proof_input",
            "allocator_hook_activation_proof_v0",
        ),
        rollback_target_provider_id: nonempty_text_field(value, "rollback_target_provider_id"),
        activation_target_provider_id: nonempty_text_field(value, "activation_target_provider_id"),
        safety_gate_policy_named: text_field_matches(
            value,
            "safety_gate_policy",
            "explicit_activation_evidence_bundle_required_reserved",
        ),
        activation_gate_closed: activation_gate_is_closed(value),
        activation_blocked_diagnostic_named: text_field_matches(
            value,
            "activation_blocked_diagnostic",
            DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED,
        ),
        required_operations_named: string_list_matches(
            value.get("required_operations"),
            &["alloc", "realloc", "free"],
        ) && string_list_matches(
            value.get("activation_target_operations"),
            &["alloc", "realloc", "free"],
        ),
        candidate_provider_ids_reserved_set: string_list_matches(
            value.get("candidate_provider_ids"),
            EXPECTED_PROVIDER_IDS,
        ),
        required_fact_list_complete: string_list_contains_all(
            value.get("reserved_activation_safety_facts"),
            REQUIRED_ACTIVATION_SAFETY_FACTS,
        ),
        safety_inputs_complete: safety_inputs_complete(value),
    }
}

fn collect_missing_activation_safety_facts(
    facts: &AllocatorProviderActivationSafetyFacts<'_>,
) -> Vec<&'static str> {
    activation_safety_fact_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.name))
        .collect()
}

fn collect_activation_safety_missing_diagnostics(
    facts: &AllocatorProviderActivationSafetyFacts<'_>,
) -> Vec<&'static str> {
    let mut diagnostics: Vec<&'static str> = activation_safety_diagnostic_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.diagnostic))
        .collect();
    if diagnostics.is_empty() && !facts.activation_gate_closed {
        diagnostics.push(DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED);
    }
    diagnostics
}

fn activation_gate_is_closed(value: &toml::Value) -> bool {
    value
        .get("activation_safety_gate")
        .and_then(toml::Value::as_str)
        == Some("inactive")
        && value.get("safety_status").and_then(toml::Value::as_str)
            == Some(SAFETY_STATUS_GATE_CLOSED)
        && value
            .get("activation_gate_open")
            .and_then(toml::Value::as_bool)
            == Some(false)
        && value
            .get("would_open_activation_gate")
            .and_then(toml::Value::as_bool)
            == Some(false)
        && value
            .get("would_activate_hook")
            .and_then(toml::Value::as_bool)
            == Some(false)
        && value.get("would_activate").and_then(toml::Value::as_bool) == Some(false)
        && value.get("activation").and_then(toml::Value::as_str) == Some("future_row_required")
}

fn safety_inputs_complete(value: &toml::Value) -> bool {
    let Some(inputs) = value.get("safety_inputs").and_then(toml::Value::as_array) else {
        return false;
    };
    EXPECTED_SAFETY_INPUTS.iter().all(|(name, diagnostic)| {
        inputs.iter().any(|input| {
            input.get("name").and_then(toml::Value::as_str) == Some(*name)
                && input.get("required").and_then(toml::Value::as_bool) == Some(true)
                && input
                    .get("missing_diagnostic")
                    .and_then(toml::Value::as_str)
                    == Some(*diagnostic)
        })
    })
}
