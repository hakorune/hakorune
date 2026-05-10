//! Diagnostic-only allocator hook dry-run validation.
//!
//! This module does not install or replace the process allocator. It only
//! reports whether the future hook facts are present.

pub const DIAG_DRY_RUN_MISSING_PLAN: &str = "[allocator-hook/dry-run-missing-plan]";
pub const DIAG_ACTIVATION_PROOF_MISSING: &str = "[allocator-hook/activation-proof-missing]";
pub const DIAG_ACTIVATION_PROOF_READY: &str = "[allocator-hook/activation-proof-ready]";
pub const DIAG_DRY_RUN_READY: &str = "[allocator-hook/dry-run-ready]";

const REQUIRED_ACTIVATION_PROOFS: &[&str] = &[
    "explicit_hook_plan_fact",
    "runtime_dry_run_validated",
    "default_inactive",
    "no_app_or_facade_name_matching",
    "no_hidden_environment_toggle",
    "no_process_allocator_replacement_without_activation_row",
    "reentrancy_guard_named",
    "bootstrap_allocation_path_named",
    "no_alloc_no_safepoint_contract_named",
    "rollback_condition_named",
    "fail_fast_diagnostic_named",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorHookDryRunStatus {
    MissingPlan,
    MissingActivationProof,
    ReadyDiagnostic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorHookActivationProofStatus {
    MissingOrInvalid,
    ReadyDiagnostic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorHookDryRunRequest<'a> {
    pub hook_id: &'a str,
    pub hook_plan_present: bool,
    pub activation_proof_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorHookDryRunReport {
    pub hook_id: String,
    pub status: AllocatorHookDryRunStatus,
    pub diagnostic: &'static str,
    pub would_install: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorHookActivationProofReport {
    pub hook_id: String,
    pub status: AllocatorHookActivationProofStatus,
    pub diagnostic: &'static str,
    pub would_activate: bool,
}

pub fn validate_allocator_hook_dry_run(
    request: &AllocatorHookDryRunRequest<'_>,
) -> AllocatorHookDryRunReport {
    let (status, diagnostic) = if !request.hook_plan_present {
        (
            AllocatorHookDryRunStatus::MissingPlan,
            DIAG_DRY_RUN_MISSING_PLAN,
        )
    } else if !request.activation_proof_present {
        (
            AllocatorHookDryRunStatus::MissingActivationProof,
            DIAG_ACTIVATION_PROOF_MISSING,
        )
    } else {
        (
            AllocatorHookDryRunStatus::ReadyDiagnostic,
            DIAG_DRY_RUN_READY,
        )
    };

    AllocatorHookDryRunReport {
        hook_id: request.hook_id.to_string(),
        status,
        diagnostic,
        would_install: false,
    }
}

pub fn validate_allocator_hook_dry_run_from_manifest_texts(
    plan_toml: &str,
    activation_proof_toml: &str,
) -> AllocatorHookDryRunReport {
    let hook_id = read_reserved_plan_hook_id(plan_toml)
        .unwrap_or_else(|| "hako_alloc.production.v0".to_string());
    validate_allocator_hook_dry_run(&AllocatorHookDryRunRequest {
        hook_id: &hook_id,
        hook_plan_present: has_reserved_plan(plan_toml, &hook_id),
        activation_proof_present: has_reserved_activation_proof(activation_proof_toml, &hook_id),
    })
}

pub fn validate_allocator_hook_activation_proof_text(
    activation_proof_toml: &str,
    hook_id: &str,
) -> AllocatorHookActivationProofReport {
    let (status, diagnostic) = if has_reserved_activation_proof(activation_proof_toml, hook_id) {
        (
            AllocatorHookActivationProofStatus::ReadyDiagnostic,
            DIAG_ACTIVATION_PROOF_READY,
        )
    } else {
        (
            AllocatorHookActivationProofStatus::MissingOrInvalid,
            DIAG_ACTIVATION_PROOF_MISSING,
        )
    };

    AllocatorHookActivationProofReport {
        hook_id: hook_id.to_string(),
        status,
        diagnostic,
        would_activate: false,
    }
}

fn read_reserved_plan_hook_id(plan_toml: &str) -> Option<String> {
    let value = toml::from_str::<toml::Value>(plan_toml).ok()?;
    if value.get("schema_version")?.as_str()? != "allocator_hook_plan_v0" {
        return None;
    }
    if value.get("status")?.as_str()? != "reserved" {
        return None;
    }
    if value.get("active")?.as_bool()? {
        return None;
    }
    value
        .get("plans")?
        .as_array()?
        .iter()
        .find(|plan| {
            plan.get("state").and_then(toml::Value::as_str) == Some("reserved")
                && plan.get("activation").and_then(toml::Value::as_str)
                    == Some("future_row_required")
        })?
        .get("hook_id")?
        .as_str()
        .map(str::to_string)
}

fn has_reserved_plan(plan_toml: &str, hook_id: &str) -> bool {
    read_reserved_plan_hook_id(plan_toml).as_deref() == Some(hook_id)
}

fn has_reserved_activation_proof(activation_proof_toml: &str, hook_id: &str) -> bool {
    let Ok(value) = toml::from_str::<toml::Value>(activation_proof_toml) else {
        return false;
    };

    value.get("schema_version").and_then(toml::Value::as_str)
        == Some("allocator_hook_activation_proof_v0")
        && value.get("status").and_then(toml::Value::as_str) == Some("reserved")
        && value.get("active").and_then(toml::Value::as_bool) == Some(false)
        && value.get("hook_id").and_then(toml::Value::as_str) == Some(hook_id)
        && value.get("activation").and_then(toml::Value::as_str) == Some("future_row_required")
        && has_required_activation_proofs(&value)
}

fn has_required_activation_proofs(value: &toml::Value) -> bool {
    let Some(proofs) = value.get("required_proofs").and_then(toml::Value::as_array) else {
        return false;
    };
    REQUIRED_ACTIVATION_PROOFS.iter().all(|required| {
        proofs
            .iter()
            .filter_map(toml::Value::as_str)
            .any(|proof| proof == *required)
    })
}

#[cfg(test)]
pub(crate) fn validate_allocator_hook_dry_run_reserved_fixtures_for_test(
) -> AllocatorHookDryRunReport {
    validate_allocator_hook_dry_run_from_manifest_texts(
        include_str!("../../docs/development/current/main/design/allocator-hook-plan-v0.toml"),
        include_str!(
            "../../docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAN_FIXTURE: &str =
        include_str!("../../docs/development/current/main/design/allocator-hook-plan-v0.toml");
    const PROOF_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
    );

    #[test]
    fn dry_run_reports_missing_plan_without_installing() {
        let report = validate_allocator_hook_dry_run(&AllocatorHookDryRunRequest {
            hook_id: "hako_alloc.production.v0",
            hook_plan_present: false,
            activation_proof_present: false,
        });

        assert_eq!(report.status, AllocatorHookDryRunStatus::MissingPlan);
        assert_eq!(report.diagnostic, DIAG_DRY_RUN_MISSING_PLAN);
        assert!(!report.would_install);
    }

    #[test]
    fn dry_run_reports_missing_activation_proof_without_installing() {
        let report = validate_allocator_hook_dry_run(&AllocatorHookDryRunRequest {
            hook_id: "hako_alloc.production.v0",
            hook_plan_present: true,
            activation_proof_present: false,
        });

        assert_eq!(
            report.status,
            AllocatorHookDryRunStatus::MissingActivationProof
        );
        assert_eq!(report.diagnostic, DIAG_ACTIVATION_PROOF_MISSING);
        assert!(!report.would_install);
    }

    #[test]
    fn dry_run_ready_is_still_diagnostic_only() {
        let report = validate_allocator_hook_dry_run(&AllocatorHookDryRunRequest {
            hook_id: "hako_alloc.production.v0",
            hook_plan_present: true,
            activation_proof_present: true,
        });

        assert_eq!(report.status, AllocatorHookDryRunStatus::ReadyDiagnostic);
        assert_eq!(report.diagnostic, DIAG_DRY_RUN_READY);
        assert!(!report.would_install);
    }

    #[test]
    fn manifest_callsite_reports_ready_diagnostic_without_installing() {
        let report = validate_allocator_hook_dry_run_reserved_fixtures_for_test();

        assert_eq!(report.hook_id, "hako_alloc.production.v0");
        assert_eq!(report.status, AllocatorHookDryRunStatus::ReadyDiagnostic);
        assert_eq!(report.diagnostic, DIAG_DRY_RUN_READY);
        assert!(!report.would_install);
    }

    #[test]
    fn manifest_callsite_reports_missing_plan_without_installing() {
        let report = validate_allocator_hook_dry_run_from_manifest_texts("", PROOF_FIXTURE);

        assert_eq!(report.status, AllocatorHookDryRunStatus::MissingPlan);
        assert_eq!(report.diagnostic, DIAG_DRY_RUN_MISSING_PLAN);
        assert!(!report.would_install);
    }

    #[test]
    fn manifest_callsite_reports_missing_activation_proof_without_installing() {
        let report = validate_allocator_hook_dry_run_from_manifest_texts(PLAN_FIXTURE, "");

        assert_eq!(
            report.status,
            AllocatorHookDryRunStatus::MissingActivationProof
        );
        assert_eq!(report.diagnostic, DIAG_ACTIVATION_PROOF_MISSING);
        assert!(!report.would_install);
    }

    #[test]
    fn activation_proof_fixture_reports_ready_without_activating() {
        let report = validate_allocator_hook_activation_proof_text(
            PROOF_FIXTURE,
            "hako_alloc.production.v0",
        );

        assert_eq!(report.hook_id, "hako_alloc.production.v0");
        assert_eq!(
            report.status,
            AllocatorHookActivationProofStatus::ReadyDiagnostic
        );
        assert_eq!(report.diagnostic, DIAG_ACTIVATION_PROOF_READY);
        assert!(!report.would_activate);
    }

    #[test]
    fn activation_proof_empty_text_reports_missing_without_activating() {
        let report = validate_allocator_hook_activation_proof_text("", "hako_alloc.production.v0");

        assert_eq!(
            report.status,
            AllocatorHookActivationProofStatus::MissingOrInvalid
        );
        assert_eq!(report.diagnostic, DIAG_ACTIVATION_PROOF_MISSING);
        assert!(!report.would_activate);
    }

    #[test]
    fn activation_proof_wrong_hook_reports_missing_without_activating() {
        let report =
            validate_allocator_hook_activation_proof_text(PROOF_FIXTURE, "hako_alloc.other.v0");

        assert_eq!(
            report.status,
            AllocatorHookActivationProofStatus::MissingOrInvalid
        );
        assert_eq!(report.diagnostic, DIAG_ACTIVATION_PROOF_MISSING);
        assert!(!report.would_activate);
    }
}
