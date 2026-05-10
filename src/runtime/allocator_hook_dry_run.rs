//! Diagnostic-only allocator hook dry-run validation.
//!
//! This module does not install or replace the process allocator. It only
//! reports whether the future hook facts are present.

pub const DIAG_DRY_RUN_MISSING_PLAN: &str = "[allocator-hook/dry-run-missing-plan]";
pub const DIAG_ACTIVATION_PROOF_MISSING: &str = "[allocator-hook/activation-proof-missing]";
pub const DIAG_DRY_RUN_READY: &str = "[allocator-hook/dry-run-ready]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorHookDryRunStatus {
    MissingPlan,
    MissingActivationProof,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
