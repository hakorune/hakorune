use super::CliConfig;
use crate::runtime::allocator_hook_dry_run::{
    validate_allocator_hook_activation_proof_text,
    validate_allocator_hook_dry_run_from_manifest_texts, AllocatorHookActivationProofStatus,
    AllocatorHookDryRunStatus,
};

const CLI_MISSING_PLAN_ARG: &str = "[allocator-hook/cli-missing-plan-arg]";
const CLI_MISSING_PROOF_ARG: &str = "[allocator-hook/cli-missing-proof-arg]";
const CLI_READ_ERROR: &str = "[allocator-hook/cli-read-error]";

pub fn maybe_run_allocator_hook_dry_run(config: &CliConfig) -> Option<i32> {
    if !config.allocator_hook_dry_run
        && config.allocator_hook_dry_run_plan.is_none()
        && config.allocator_hook_dry_run_proof.is_none()
    {
        return None;
    }

    let Some(plan_path) = config.allocator_hook_dry_run_plan.as_deref() else {
        eprintln!("{CLI_MISSING_PLAN_ARG}");
        return Some(2);
    };
    let Some(proof_path) = config.allocator_hook_dry_run_proof.as_deref() else {
        eprintln!("{CLI_MISSING_PROOF_ARG}");
        return Some(2);
    };

    match run_allocator_hook_dry_run_files(plan_path, proof_path) {
        Ok((output, exit_code)) => {
            print!("{output}");
            Some(exit_code)
        }
        Err(message) => {
            eprintln!("{message}");
            Some(2)
        }
    }
}

fn run_allocator_hook_dry_run_files(
    plan_path: &str,
    proof_path: &str,
) -> Result<(String, i32), String> {
    let plan_toml = std::fs::read_to_string(plan_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: plan={plan_path}: {err}"))?;
    let proof_toml = std::fs::read_to_string(proof_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: proof={proof_path}: {err}"))?;
    Ok(build_allocator_hook_dry_run_output(&plan_toml, &proof_toml))
}

pub fn build_allocator_hook_dry_run_output(plan_toml: &str, proof_toml: &str) -> (String, i32) {
    let dry_run = validate_allocator_hook_dry_run_from_manifest_texts(plan_toml, proof_toml);
    let activation_proof =
        validate_allocator_hook_activation_proof_text(proof_toml, &dry_run.hook_id);
    let ready = dry_run.status == AllocatorHookDryRunStatus::ReadyDiagnostic
        && activation_proof.status == AllocatorHookActivationProofStatus::ReadyDiagnostic;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         hook_id={}\n\
         dry_run_status={}\n\
         would_install={}\n\
         activation_proof_diagnostic={}\n\
         activation_proof_status={}\n\
         would_activate={}\n",
        dry_run.diagnostic,
        dry_run.hook_id,
        dry_run_status_name(dry_run.status),
        dry_run.would_install,
        activation_proof.diagnostic,
        activation_proof_status_name(activation_proof.status),
        activation_proof.would_activate
    );

    (output, exit_code)
}

fn dry_run_status_name(status: AllocatorHookDryRunStatus) -> &'static str {
    match status {
        AllocatorHookDryRunStatus::MissingPlan => "missing_plan",
        AllocatorHookDryRunStatus::MissingActivationProof => "missing_activation_proof",
        AllocatorHookDryRunStatus::ReadyDiagnostic => "ready",
    }
}

fn activation_proof_status_name(status: AllocatorHookActivationProofStatus) -> &'static str {
    match status {
        AllocatorHookActivationProofStatus::MissingOrInvalid => "missing_or_invalid",
        AllocatorHookActivationProofStatus::ReadyDiagnostic => "ready",
    }
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
    fn allocator_hook_dry_run_cli_output_reports_ready_without_activating() {
        let (output, exit_code) = build_allocator_hook_dry_run_output(PLAN_FIXTURE, PROOF_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-hook/dry-run-ready]"));
        assert!(output.contains("dry_run_status=ready"));
        assert!(output.contains("would_install=false"));
        assert!(
            output.contains("activation_proof_diagnostic=[allocator-hook/activation-proof-ready]")
        );
        assert!(output.contains("activation_proof_status=ready"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_hook_dry_run_cli_output_reports_missing_plan() {
        let (output, exit_code) = build_allocator_hook_dry_run_output("", PROOF_FIXTURE);

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-hook/dry-run-missing-plan]"));
        assert!(output.contains("dry_run_status=missing_plan"));
        assert!(output.contains("would_install=false"));
        assert!(output.contains("would_activate=false"));
    }
}
