//! Diagnostic-only allocator provider registry snapshot report.

use super::allocator_provider_diagnostic_inactive::REGISTRY_SNAPSHOT_INACTIVE_ACTIONS;
use super::allocator_provider_registry_common::{EXPECTED_PROVIDER_IDS, OWNER_PATH};
use super::allocator_provider_toml_helpers::{
    bool_field_false, string_list_contains_all, text_field_matches, DiagnosticFactCheck,
};

pub const DIAG_PROVIDER_REGISTRY_SNAPSHOT_MISSING: &str =
    "[allocator-provider/registry-snapshot-missing]";
pub const DIAG_PROVIDER_REGISTRY_PROVIDER_MISSING: &str =
    "[allocator-provider/registry-provider-missing]";
pub const DIAG_PROVIDER_REGISTRY_CAPABILITY_MISSING: &str =
    "[allocator-provider/registry-capability-missing]";
pub const DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE: &str =
    "[allocator-provider/registry-snapshot-inactive]";

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
