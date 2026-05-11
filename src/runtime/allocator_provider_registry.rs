//! Diagnostic-only allocator provider registry, selection, and activation safety reports.
//!
//! This module does not select a provider, open an activation gate, install a
//! hook, or replace the process allocator. It only owns the provider-registry
//! side diagnostics reserved by the allocator provider ladder.

use super::allocator_provider_diagnostic_inactive::{
    REGISTRY_SNAPSHOT_INACTIVE_ACTIONS, SAFETY_GATE_INACTIVE_ACTIONS,
};
use super::allocator_provider_toml_helpers::{
    bool_field_false, nonempty_text_field, string_list_contains_all, text_field_matches,
    DiagnosticFactCheck,
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
pub const DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING: &str =
    "[allocator-provider/registry-snapshot-missing]";
pub const DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING: &str =
    "[allocator-provider/registry-provider-missing]";
pub const DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING: &str =
    "[allocator-provider/registry-capability-missing]";
pub const DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE: &str =
    "[allocator-provider/registry-snapshot-inactive]";
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

const OWNER_PATH: &str = "src/runtime/allocator_provider_registry.rs";
const SAFETY_STATUS_GATE_CLOSED: &str = "reserved_gate_closed";

const EXPECTED_PROVIDER_IDS: &[&str] = &[
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
];

const REQUIRED_REGISTRY_SNAPSHOT_FACTS: &[&str] = &[
    "provider_manifest_ready",
    "provider_readiness_preflight_ready",
    "provider_entries_nonempty",
    "provider_ids_reserved_set",
    "provider_operations_nonempty",
    "registry_owner_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
];

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
pub enum AllocatorProviderRegistrySnapshotStatus {
    MissingFacts,
    ReadyInactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderRegistrySnapshotFacts<'a> {
    pub schema_ready: bool,
    pub status_reserved: bool,
    pub active_false: bool,
    pub owner_named: bool,
    pub provider_manifest_ready: bool,
    pub provider_readiness_preflight_ready: bool,
    pub provider_entries_nonempty: bool,
    pub provider_ids_reserved_set: bool,
    pub provider_operations_nonempty: bool,
    pub provider_selection_inactive: bool,
    pub would_build_registry_false: bool,
    pub would_select_provider_false: bool,
    pub would_activate_false: bool,
    pub activation_future_row_required: bool,
    pub diagnostic_named: bool,
    pub missing_provider_diagnostic_named: bool,
    pub missing_capability_diagnostic_named: bool,
    pub required_fact_list_complete: bool,
    pub provider_ids: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderRegistrySnapshotReport {
    pub status: AllocatorProviderRegistrySnapshotStatus,
    pub diagnostic: &'static str,
    pub parse_error: Option<String>,
    pub missing_facts: Vec<&'static str>,
    pub missing_diagnostics: Vec<&'static str>,
    pub provider_ids: Vec<String>,
    pub provider_count: usize,
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

pub fn validate_allocator_provider_registry_snapshot(
    facts: &AllocatorProviderRegistrySnapshotFacts<'_>,
) -> AllocatorProviderRegistrySnapshotReport {
    let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
    let diagnostic_actions = inactive.diagnostic_actions;
    let missing_facts = collect_missing_registry_snapshot_facts(facts);
    let missing_diagnostics = collect_registry_snapshot_missing_diagnostics(facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderRegistrySnapshotStatus::ReadyInactive,
            DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE,
        )
    } else {
        (
            AllocatorProviderRegistrySnapshotStatus::MissingFacts,
            DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING,
        )
    };

    AllocatorProviderRegistrySnapshotReport {
        status,
        diagnostic,
        parse_error: None,
        missing_facts,
        missing_diagnostics,
        provider_ids: facts
            .provider_ids
            .iter()
            .map(|id| (*id).to_string())
            .collect(),
        provider_count: facts.provider_ids.len(),
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

pub fn validate_allocator_provider_registry_snapshot_from_text(
    registry_snapshot_toml: &str,
) -> AllocatorProviderRegistrySnapshotReport {
    let value = match toml::from_str::<toml::Value>(registry_snapshot_toml) {
        Ok(value) => value,
        Err(err) => {
            let inactive = REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
            let diagnostic_actions = inactive.diagnostic_actions;
            return AllocatorProviderRegistrySnapshotReport {
                status: AllocatorProviderRegistrySnapshotStatus::MissingFacts,
                diagnostic: DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING,
                parse_error: Some(err.to_string()),
                missing_facts: vec!["parse_toml"],
                missing_diagnostics: vec![DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING],
                provider_ids: Vec::new(),
                provider_count: 0,
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

    let facts = read_registry_snapshot_facts(&value);
    validate_allocator_provider_registry_snapshot(&facts)
}

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

fn read_registry_snapshot_facts(value: &toml::Value) -> AllocatorProviderRegistrySnapshotFacts<'_> {
    let provider_ids = registry_snapshot_provider_ids(value);
    AllocatorProviderRegistrySnapshotFacts {
        schema_ready: text_field_matches(
            value,
            "schema_version",
            "allocator_provider_registry_snapshot_v0",
        ),
        status_reserved: text_field_matches(value, "status", "reserved"),
        active_false: bool_field_false(value, "active"),
        owner_named: text_field_matches(value, "registry_owner", OWNER_PATH),
        provider_manifest_ready: text_field_matches(
            value,
            "provider_manifest_input",
            "allocator_provider_manifest_report",
        ),
        provider_readiness_preflight_ready: text_field_matches(
            value,
            "provider_readiness_input",
            "allocator_provider_readiness_preflight_report",
        ),
        provider_entries_nonempty: registry_snapshot_entries(value)
            .map_or(false, |entries| !entries.is_empty()),
        provider_ids_reserved_set: registry_snapshot_provider_ids_reserved_set(value),
        provider_operations_nonempty: registry_snapshot_provider_operations_nonempty(value),
        provider_selection_inactive: text_field_matches(value, "provider_selection", "inactive"),
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
            DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING,
        ),
        missing_provider_diagnostic_named: text_field_matches(
            value,
            "missing_provider_diagnostic",
            DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING,
        ),
        missing_capability_diagnostic_named: text_field_matches(
            value,
            "missing_capability_diagnostic",
            DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING,
        ),
        required_fact_list_complete: string_list_contains_all(
            value.get("required_registry_snapshot_facts"),
            REQUIRED_REGISTRY_SNAPSHOT_FACTS,
        ),
        provider_ids,
    }
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

fn registry_snapshot_fact_checks(
    facts: &AllocatorProviderRegistrySnapshotFacts<'_>,
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
            name: "registry_owner_named",
        },
        DiagnosticFactCheck {
            present: facts.provider_manifest_ready,
            name: "provider_manifest_ready",
        },
        DiagnosticFactCheck {
            present: facts.provider_readiness_preflight_ready,
            name: "provider_readiness_preflight_ready",
        },
        DiagnosticFactCheck {
            present: facts.provider_entries_nonempty,
            name: "provider_entries_nonempty",
        },
        DiagnosticFactCheck {
            present: facts.provider_ids_reserved_set,
            name: "provider_ids_reserved_set",
        },
        DiagnosticFactCheck {
            present: facts.provider_operations_nonempty,
            name: "provider_operations_nonempty",
        },
        DiagnosticFactCheck {
            present: facts.provider_selection_inactive,
            name: "provider_selection_inactive",
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
            name: "registry_snapshot_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_provider_diagnostic_named,
            name: "missing_provider_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.missing_capability_diagnostic_named,
            name: "missing_capability_diagnostic_named",
        },
        DiagnosticFactCheck {
            present: facts.required_fact_list_complete,
            name: "required_registry_snapshot_facts_complete",
        },
    ]
}

fn collect_missing_registry_snapshot_facts(
    facts: &AllocatorProviderRegistrySnapshotFacts<'_>,
) -> Vec<&'static str> {
    registry_snapshot_fact_checks(facts)
        .into_iter()
        .filter_map(|check| (!check.present).then_some(check.name))
        .collect()
}

fn collect_registry_snapshot_missing_diagnostics(
    facts: &AllocatorProviderRegistrySnapshotFacts<'_>,
) -> Vec<&'static str> {
    [
        (
            facts.provider_manifest_ready && facts.provider_readiness_preflight_ready,
            DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING,
        ),
        (
            facts.provider_entries_nonempty && facts.provider_ids_reserved_set,
            DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING,
        ),
        (
            facts.provider_operations_nonempty,
            DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING,
        ),
    ]
    .into_iter()
    .filter_map(|(present, diagnostic)| (!present).then_some(diagnostic))
    .collect()
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

fn string_list_matches(value: Option<&toml::Value>, expected: &[&str]) -> bool {
    let Some(items) = value.and_then(toml::Value::as_array) else {
        return false;
    };
    let actual: Vec<&str> = items.iter().filter_map(toml::Value::as_str).collect();
    actual == expected
}

fn string_list(value: Option<&toml::Value>) -> Vec<&str> {
    value
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .filter(|item| !item.is_empty())
        .collect()
}

fn registry_snapshot_entries(value: &toml::Value) -> Option<&Vec<toml::Value>> {
    value.get("entries").and_then(toml::Value::as_array)
}

fn registry_snapshot_provider_ids<'a>(value: &'a toml::Value) -> Vec<&'a str> {
    registry_snapshot_entries(value)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("provider_id").and_then(toml::Value::as_str))
        .filter(|id| !id.is_empty())
        .collect()
}

fn registry_snapshot_provider_ids_reserved_set(value: &toml::Value) -> bool {
    let Some(entries) = registry_snapshot_entries(value) else {
        return false;
    };
    if entries.len() != EXPECTED_PROVIDER_IDS.len() {
        return false;
    }
    entries
        .iter()
        .zip(EXPECTED_PROVIDER_IDS.iter())
        .all(|(entry, expected_id)| {
            entry.get("provider_id").and_then(toml::Value::as_str) == Some(*expected_id)
                && entry.get("state").and_then(toml::Value::as_str) == Some("reserved")
                && entry.get("activation").and_then(toml::Value::as_str)
                    == Some("future_row_required")
        })
}

fn registry_snapshot_provider_operations_nonempty(value: &toml::Value) -> bool {
    let Some(entries) = registry_snapshot_entries(value) else {
        return false;
    };
    entries.iter().all(|entry| {
        entry
            .get("operations")
            .and_then(toml::Value::as_array)
            .map_or(false, |operations| !operations.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVATION_SAFETY_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
    );
    const REGISTRY_SNAPSHOT_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
    );
    const SELECTION_DECISION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
    );

    #[test]
    fn registry_snapshot_fixture_reports_inactive_without_building_registry() {
        let report =
            validate_allocator_provider_registry_snapshot_from_text(REGISTRY_SNAPSHOT_FIXTURE);

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
        assert_eq!(report.safety_status, SAFETY_STATUS_GATE_CLOSED);
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
}
