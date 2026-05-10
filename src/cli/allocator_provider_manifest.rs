use super::CliConfig;
use crate::runtime::allocator_provider_manifest::{
    parse_allocator_provider_manifest_text,
    validate_allocator_provider_combined_dry_run_from_manifest_texts,
    AllocatorProviderCombinedDryRunStatus, AllocatorProviderManifestStatus,
};

const CLI_MISSING_PLAN_ARG: &str = "[allocator-provider/combined-cli-missing-plan-arg]";
const CLI_MISSING_PROOF_ARG: &str = "[allocator-provider/combined-cli-missing-proof-arg]";
const CLI_READ_ERROR: &str = "[allocator-provider/cli-read-error]";

pub fn maybe_run_allocator_provider_combined_dry_run(config: &CliConfig) -> Option<i32> {
    let manifest_path = config.allocator_provider_manifest.as_deref()?;
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

    match run_allocator_provider_combined_dry_run_files(manifest_path, plan_path, proof_path) {
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

pub fn maybe_run_allocator_provider_manifest_diagnostic(config: &CliConfig) -> Option<i32> {
    let manifest_path = config.allocator_provider_manifest.as_deref()?;

    match run_allocator_provider_manifest_file(manifest_path) {
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

fn run_allocator_provider_manifest_file(manifest_path: &str) -> Result<(String, i32), String> {
    let manifest_toml = std::fs::read_to_string(manifest_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: manifest={manifest_path}: {err}"))?;
    Ok(build_allocator_provider_manifest_output(&manifest_toml))
}

fn run_allocator_provider_combined_dry_run_files(
    manifest_path: &str,
    plan_path: &str,
    proof_path: &str,
) -> Result<(String, i32), String> {
    let manifest_toml = std::fs::read_to_string(manifest_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: manifest={manifest_path}: {err}"))?;
    let plan_toml = std::fs::read_to_string(plan_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: plan={plan_path}: {err}"))?;
    let proof_toml = std::fs::read_to_string(proof_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: proof={proof_path}: {err}"))?;
    Ok(build_allocator_provider_combined_dry_run_output(
        &manifest_toml,
        &plan_toml,
        &proof_toml,
    ))
}

pub fn build_allocator_provider_manifest_output(manifest_toml: &str) -> (String, i32) {
    let report = parse_allocator_provider_manifest_text(manifest_toml);
    let ready = report.status == AllocatorProviderManifestStatus::ReadyDiagnostic;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         manifest_status={}\n\
         provider_ids={}\n\
         missing_facts={}\n\
         would_select_provider={}\n",
        report.diagnostic,
        provider_manifest_status_name(report.status),
        report.provider_ids.join(","),
        report.missing_facts.join(","),
        report.would_select_provider
    );

    (output, exit_code)
}

pub fn build_allocator_provider_combined_dry_run_output(
    manifest_toml: &str,
    plan_toml: &str,
    proof_toml: &str,
) -> (String, i32) {
    let report = validate_allocator_provider_combined_dry_run_from_manifest_texts(
        manifest_toml,
        plan_toml,
        proof_toml,
    );
    let ready = report.status == AllocatorProviderCombinedDryRunStatus::ReadyDiagnostic;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         combined_status={}\n\
         hook_id={}\n\
         hook_dry_run_diagnostic={}\n\
         activation_proof_diagnostic={}\n\
         activation_preflight_diagnostic={}\n\
         provider_manifest_diagnostic={}\n\
         provider_readiness_diagnostic={}\n\
         provider_ids={}\n\
         missing_facts={}\n\
         would_install={}\n\
         would_select_provider={}\n\
         would_activate={}\n",
        report.diagnostic,
        provider_combined_dry_run_status_name(report.status),
        report.hook_id,
        report.hook_dry_run_diagnostic,
        report.activation_proof_diagnostic,
        report.activation_preflight_diagnostic,
        report.provider_manifest_diagnostic,
        report.provider_readiness_diagnostic,
        report.provider_ids.join(","),
        report.missing_facts.join(","),
        report.would_install,
        report.would_select_provider,
        report.would_activate
    );

    (output, exit_code)
}

fn provider_manifest_status_name(status: AllocatorProviderManifestStatus) -> &'static str {
    match status {
        AllocatorProviderManifestStatus::MissingOrInvalid => "missing_or_invalid",
        AllocatorProviderManifestStatus::ReadyDiagnostic => "ready",
    }
}

fn provider_combined_dry_run_status_name(
    status: AllocatorProviderCombinedDryRunStatus,
) -> &'static str {
    match status {
        AllocatorProviderCombinedDryRunStatus::MissingFacts => "missing_facts",
        AllocatorProviderCombinedDryRunStatus::ReadyDiagnostic => "ready",
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
    fn allocator_provider_manifest_cli_output_reports_ready_without_selection() {
        let (output, exit_code) =
            build_allocator_provider_manifest_output(PROVIDER_MANIFEST_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/manifest-ready]"));
        assert!(output.contains("manifest_status=ready"));
        assert!(output.contains("provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("would_select_provider=false"));
    }

    #[test]
    fn allocator_provider_manifest_cli_output_reports_missing_manifest() {
        let (output, exit_code) = build_allocator_provider_manifest_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/manifest-missing]"));
        assert!(output.contains("manifest_status=missing_or_invalid"));
        assert!(output.contains("missing_facts=schema_version"));
        assert!(output.contains("would_select_provider=false"));
    }

    #[test]
    fn allocator_provider_combined_dry_run_cli_output_reports_ready_without_activation() {
        let (output, exit_code) = build_allocator_provider_combined_dry_run_output(
            PROVIDER_MANIFEST_FIXTURE,
            HOOK_PLAN_FIXTURE,
            ACTIVATION_PROOF_FIXTURE,
        );

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/combined-dry-run-ready]"));
        assert!(output.contains("combined_status=ready"));
        assert!(output.contains("hook_id=hako_alloc.production.v0"));
        assert!(output.contains("hook_dry_run_diagnostic=[allocator-hook/dry-run-ready]"));
        assert!(
            output.contains("activation_proof_diagnostic=[allocator-hook/activation-proof-ready]")
        );
        assert!(output.contains(
            "activation_preflight_diagnostic=[allocator-hook/activation-preflight-ready]"
        ));
        assert!(output.contains("provider_manifest_diagnostic=[allocator-provider/manifest-ready]"));
        assert!(output.contains(
            "provider_readiness_diagnostic=[allocator-provider/readiness-preflight-ready]"
        ));
        assert!(output.contains("would_install=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_combined_dry_run_cli_output_reports_missing_provider() {
        let (output, exit_code) = build_allocator_provider_combined_dry_run_output(
            "",
            HOOK_PLAN_FIXTURE,
            ACTIVATION_PROOF_FIXTURE,
        );

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/combined-dry-run-missing]"));
        assert!(output.contains("combined_status=missing_facts"));
        assert!(
            output.contains("provider_manifest_diagnostic=[allocator-provider/manifest-missing]")
        );
        assert!(output
            .contains("missing_facts=provider_manifest_ready,provider_readiness_preflight_ready"));
        assert!(output.contains("would_install=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_activate=false"));
    }
}
