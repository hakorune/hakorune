//! Diagnostic-only allocator provider manifest parser.
//!
//! This module parses caller-provided TOML text and reports reserved provider
//! facts. It does not discover files, select a provider, or activate allocator
//! replacement.

use crate::runtime::allocator_hook_dry_run::{
    validate_allocator_hook_activation_preflight_from_manifest_texts,
    AllocatorHookActivationPreflightReport, AllocatorHookActivationPreflightStatus,
};

pub const DIAG_PROVIDER_MANIFEST_MISSING: &str = "[allocator-provider/manifest-missing]";
pub const DIAG_PROVIDER_MANIFEST_READY: &str = "[allocator-provider/manifest-ready]";
pub const DIAG_PROVIDER_READINESS_PREFLIGHT_MISSING: &str =
    "[allocator-provider/readiness-preflight-missing]";
pub const DIAG_PROVIDER_READINESS_PREFLIGHT_READY: &str =
    "[allocator-provider/readiness-preflight-ready]";

const EXPECTED_PROVIDER_IDS: &[&str] = &[
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderManifestStatus {
    MissingOrInvalid,
    ReadyDiagnostic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorProviderReadinessPreflightStatus {
    MissingFacts,
    ReadyDiagnostic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderManifestReport {
    pub status: AllocatorProviderManifestStatus,
    pub diagnostic: &'static str,
    pub provider_ids: Vec<String>,
    pub missing_facts: Vec<&'static str>,
    pub would_select_provider: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderReadinessPreflightFacts {
    pub provider_manifest_ready: bool,
    pub activation_preflight_ready: bool,
    pub provider_ids_reserved_set: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorProviderReadinessPreflightReport {
    pub status: AllocatorProviderReadinessPreflightStatus,
    pub diagnostic: &'static str,
    pub provider_manifest_diagnostic: &'static str,
    pub activation_preflight_diagnostic: &'static str,
    pub provider_ids: Vec<String>,
    pub missing_facts: Vec<&'static str>,
    pub would_select_provider: bool,
    pub would_activate: bool,
}

pub fn parse_allocator_provider_manifest_text(
    manifest_toml: &str,
) -> AllocatorProviderManifestReport {
    let Ok(value) = toml::from_str::<toml::Value>(manifest_toml) else {
        return missing_report(vec!["parse_toml"]);
    };

    let mut missing_facts = Vec::new();
    let mut provider_ids = Vec::new();

    if value.get("schema_version").and_then(toml::Value::as_str)
        != Some("allocator_provider_manifest_v0")
    {
        missing_facts.push("schema_version");
    }
    if value.get("status").and_then(toml::Value::as_str) != Some("reserved") {
        missing_facts.push("status_reserved");
    }
    if value.get("active").and_then(toml::Value::as_bool) != Some(false) {
        missing_facts.push("active_false");
    }
    if value
        .get("provider_selection")
        .and_then(toml::Value::as_str)
        != Some("inactive")
    {
        missing_facts.push("provider_selection_inactive");
    }
    if value.get("activation").and_then(toml::Value::as_str) != Some("future_row_required") {
        missing_facts.push("activation_future_row_required");
    }

    let Some(providers) = value.get("providers").and_then(toml::Value::as_array) else {
        missing_facts.push("providers_array");
        return finalize_report(provider_ids, missing_facts);
    };
    if providers.is_empty() {
        missing_facts.push("providers_nonempty");
    }

    for provider in providers {
        let Some(provider_table) = provider.as_table() else {
            missing_facts.push("provider_rows_table");
            continue;
        };

        if let Some(provider_id) = provider_table
            .get("provider_id")
            .and_then(toml::Value::as_str)
        {
            provider_ids.push(provider_id.to_string());
        } else {
            missing_facts.push("provider_id");
        }

        if provider_table
            .get("provider_kind")
            .and_then(toml::Value::as_str)
            .is_none()
        {
            missing_facts.push("provider_kind");
        }
        if provider_table
            .get("role")
            .and_then(toml::Value::as_str)
            .is_none()
        {
            missing_facts.push("role");
        }
        if provider_table.get("state").and_then(toml::Value::as_str) != Some("reserved") {
            missing_facts.push("provider_state_reserved");
        }
        if provider_table
            .get("activation")
            .and_then(toml::Value::as_str)
            != Some("future_row_required")
        {
            missing_facts.push("provider_activation_future_row_required");
        }
        if provider_table
            .get("operations")
            .and_then(toml::Value::as_array)
            .map(|operations| !operations.is_empty())
            != Some(true)
        {
            missing_facts.push("provider_operations_nonempty");
        }
    }

    if !provider_ids_match_reserved_set(&provider_ids) {
        missing_facts.push("provider_ids_reserved_set");
    }

    finalize_report(provider_ids, missing_facts)
}

pub fn validate_allocator_provider_readiness_preflight(
    provider_manifest: &AllocatorProviderManifestReport,
    activation_preflight: &AllocatorHookActivationPreflightReport,
) -> AllocatorProviderReadinessPreflightReport {
    let facts = AllocatorProviderReadinessPreflightFacts {
        provider_manifest_ready: provider_manifest.status
            == AllocatorProviderManifestStatus::ReadyDiagnostic,
        activation_preflight_ready: activation_preflight.status
            == AllocatorHookActivationPreflightStatus::ReadyDiagnostic,
        provider_ids_reserved_set: provider_ids_match_reserved_set(&provider_manifest.provider_ids),
    };
    let missing_facts = collect_missing_readiness_preflight_facts(&facts);
    let (status, diagnostic) = if missing_facts.is_empty() {
        (
            AllocatorProviderReadinessPreflightStatus::ReadyDiagnostic,
            DIAG_PROVIDER_READINESS_PREFLIGHT_READY,
        )
    } else {
        (
            AllocatorProviderReadinessPreflightStatus::MissingFacts,
            DIAG_PROVIDER_READINESS_PREFLIGHT_MISSING,
        )
    };

    AllocatorProviderReadinessPreflightReport {
        status,
        diagnostic,
        provider_manifest_diagnostic: provider_manifest.diagnostic,
        activation_preflight_diagnostic: activation_preflight.diagnostic,
        provider_ids: provider_manifest.provider_ids.clone(),
        missing_facts,
        would_select_provider: false,
        would_activate: false,
    }
}

pub fn validate_allocator_provider_readiness_preflight_from_manifest_texts(
    provider_manifest_toml: &str,
    hook_plan_toml: &str,
    activation_proof_toml: &str,
) -> AllocatorProviderReadinessPreflightReport {
    let provider_manifest = parse_allocator_provider_manifest_text(provider_manifest_toml);
    let activation_preflight = validate_allocator_hook_activation_preflight_from_manifest_texts(
        hook_plan_toml,
        activation_proof_toml,
    );
    validate_allocator_provider_readiness_preflight(&provider_manifest, &activation_preflight)
}

#[cfg(test)]
pub(crate) fn parse_allocator_provider_manifest_reserved_fixture_for_test(
) -> AllocatorProviderManifestReport {
    parse_allocator_provider_manifest_text(include_str!(
        "../../docs/development/current/main/design/allocator-provider-manifest-v0.toml"
    ))
}

fn provider_ids_match_reserved_set(provider_ids: &[String]) -> bool {
    if provider_ids.len() != EXPECTED_PROVIDER_IDS.len() {
        return false;
    }
    EXPECTED_PROVIDER_IDS.iter().all(|expected| {
        provider_ids
            .iter()
            .any(|provider_id| provider_id == expected)
    })
}

fn collect_missing_readiness_preflight_facts(
    facts: &AllocatorProviderReadinessPreflightFacts,
) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if !facts.provider_manifest_ready {
        missing.push("provider_manifest_ready");
    }
    if !facts.activation_preflight_ready {
        missing.push("activation_preflight_ready");
    }
    if !facts.provider_ids_reserved_set {
        missing.push("provider_ids_reserved_set");
    }
    missing
}

fn missing_report(missing_facts: Vec<&'static str>) -> AllocatorProviderManifestReport {
    AllocatorProviderManifestReport {
        status: AllocatorProviderManifestStatus::MissingOrInvalid,
        diagnostic: DIAG_PROVIDER_MANIFEST_MISSING,
        provider_ids: Vec::new(),
        missing_facts,
        would_select_provider: false,
    }
}

fn finalize_report(
    provider_ids: Vec<String>,
    missing_facts: Vec<&'static str>,
) -> AllocatorProviderManifestReport {
    if missing_facts.is_empty() {
        AllocatorProviderManifestReport {
            status: AllocatorProviderManifestStatus::ReadyDiagnostic,
            diagnostic: DIAG_PROVIDER_MANIFEST_READY,
            provider_ids,
            missing_facts,
            would_select_provider: false,
        }
    } else {
        AllocatorProviderManifestReport {
            status: AllocatorProviderManifestStatus::MissingOrInvalid,
            diagnostic: DIAG_PROVIDER_MANIFEST_MISSING,
            provider_ids,
            missing_facts,
            would_select_provider: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDER_MANIFEST_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-manifest-v0.toml"
    );
    const HOOK_PLAN_FIXTURE: &str =
        include_str!("../../docs/development/current/main/design/allocator-hook-plan-v0.toml");
    const ACTIVATION_PROOF_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
    );

    #[test]
    fn provider_manifest_reserved_fixture_reports_ready_without_selection() {
        let report = parse_allocator_provider_manifest_reserved_fixture_for_test();

        assert_eq!(
            report.status,
            AllocatorProviderManifestStatus::ReadyDiagnostic
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_MANIFEST_READY);
        assert_eq!(report.provider_ids.len(), 4);
        assert!(report.missing_facts.is_empty());
        assert!(!report.would_select_provider);
    }

    #[test]
    fn provider_manifest_empty_text_reports_missing_without_selection() {
        let report = parse_allocator_provider_manifest_text("");

        assert_eq!(
            report.status,
            AllocatorProviderManifestStatus::MissingOrInvalid
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_MANIFEST_MISSING);
        assert!(report.missing_facts.contains(&"schema_version"));
        assert!(!report.would_select_provider);
    }

    #[test]
    fn provider_manifest_wrong_schema_reports_missing_without_selection() {
        let report = parse_allocator_provider_manifest_text(
            r#"
schema_version = "allocator_provider_manifest_v1"
status = "reserved"
active = false
provider_selection = "inactive"
activation = "future_row_required"
"#,
        );

        assert_eq!(
            report.status,
            AllocatorProviderManifestStatus::MissingOrInvalid
        );
        assert!(report.missing_facts.contains(&"schema_version"));
        assert!(!report.would_select_provider);
    }

    #[test]
    fn provider_manifest_missing_provider_row_fields_reports_missing() {
        let report = parse_allocator_provider_manifest_text(
            r#"
schema_version = "allocator_provider_manifest_v0"
status = "reserved"
active = false
provider_selection = "inactive"
activation = "future_row_required"

[[providers]]
provider_id = "native_system_malloc"
"#,
        );

        assert_eq!(
            report.status,
            AllocatorProviderManifestStatus::MissingOrInvalid
        );
        assert!(report.missing_facts.contains(&"provider_kind"));
        assert!(report
            .missing_facts
            .contains(&"provider_operations_nonempty"));
        assert!(report.missing_facts.contains(&"provider_ids_reserved_set"));
        assert!(!report.would_select_provider);
    }

    #[test]
    fn provider_manifest_full_fixture_reports_expected_provider_ids() {
        let report = parse_allocator_provider_manifest_text(PROVIDER_MANIFEST_FIXTURE);

        assert_eq!(
            report.status,
            AllocatorProviderManifestStatus::ReadyDiagnostic
        );
        for provider_id in EXPECTED_PROVIDER_IDS {
            assert!(report.provider_ids.iter().any(|id| id == provider_id));
        }
        assert!(!report.would_select_provider);
    }

    #[test]
    fn provider_readiness_preflight_fixtures_report_ready_without_activation() {
        let report = validate_allocator_provider_readiness_preflight_from_manifest_texts(
            PROVIDER_MANIFEST_FIXTURE,
            HOOK_PLAN_FIXTURE,
            ACTIVATION_PROOF_FIXTURE,
        );

        assert_eq!(
            report.status,
            AllocatorProviderReadinessPreflightStatus::ReadyDiagnostic
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_READINESS_PREFLIGHT_READY);
        assert_eq!(
            report.provider_manifest_diagnostic,
            DIAG_PROVIDER_MANIFEST_READY
        );
        assert_eq!(
            report.activation_preflight_diagnostic,
            crate::runtime::allocator_hook_dry_run::DIAG_ACTIVATION_PREFLIGHT_READY
        );
        assert_eq!(report.provider_ids.len(), EXPECTED_PROVIDER_IDS.len());
        assert!(report.missing_facts.is_empty());
        assert!(!report.would_select_provider);
        assert!(!report.would_activate);
    }

    #[test]
    fn provider_readiness_preflight_missing_provider_manifest_reports_missing() {
        let report = validate_allocator_provider_readiness_preflight_from_manifest_texts(
            "",
            HOOK_PLAN_FIXTURE,
            ACTIVATION_PROOF_FIXTURE,
        );

        assert_eq!(
            report.status,
            AllocatorProviderReadinessPreflightStatus::MissingFacts
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_READINESS_PREFLIGHT_MISSING);
        assert_eq!(
            report.provider_manifest_diagnostic,
            DIAG_PROVIDER_MANIFEST_MISSING
        );
        assert_eq!(
            report.activation_preflight_diagnostic,
            crate::runtime::allocator_hook_dry_run::DIAG_ACTIVATION_PREFLIGHT_READY
        );
        assert!(report.missing_facts.contains(&"provider_manifest_ready"));
        assert!(report.missing_facts.contains(&"provider_ids_reserved_set"));
        assert!(!report.would_select_provider);
        assert!(!report.would_activate);
    }

    #[test]
    fn provider_readiness_preflight_missing_activation_preflight_reports_missing() {
        let report = validate_allocator_provider_readiness_preflight_from_manifest_texts(
            PROVIDER_MANIFEST_FIXTURE,
            "",
            ACTIVATION_PROOF_FIXTURE,
        );

        assert_eq!(
            report.status,
            AllocatorProviderReadinessPreflightStatus::MissingFacts
        );
        assert_eq!(report.diagnostic, DIAG_PROVIDER_READINESS_PREFLIGHT_MISSING);
        assert_eq!(
            report.provider_manifest_diagnostic,
            DIAG_PROVIDER_MANIFEST_READY
        );
        assert_eq!(
            report.activation_preflight_diagnostic,
            crate::runtime::allocator_hook_dry_run::DIAG_ACTIVATION_PREFLIGHT_MISSING
        );
        assert!(report.missing_facts.contains(&"activation_preflight_ready"));
        assert!(!report.missing_facts.contains(&"provider_manifest_ready"));
        assert!(!report.would_select_provider);
        assert!(!report.would_activate);
    }
}
