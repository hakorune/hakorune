use super::diagnostic_output::{finish_result, one_line_option_text, read_labeled_file};
use super::CliConfig;
use crate::runtime::allocator_provider_registry::{
    validate_allocator_provider_proof_bundle_consumption_from_text,
    AllocatorProviderProofBundleConsumptionStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/proof-bundle-consumption-cli-read-error]";

pub fn maybe_run_allocator_provider_proof_bundle_consumption_diagnostic(
    config: &CliConfig,
) -> Option<i32> {
    let proof_bundle_consumption_path = config
        .allocator_provider_proof_bundle_consumption
        .as_deref()?;

    finish_result(run_allocator_provider_proof_bundle_consumption_file(
        proof_bundle_consumption_path,
    ))
}

fn run_allocator_provider_proof_bundle_consumption_file(
    proof_bundle_consumption_path: &str,
) -> Result<(String, i32), String> {
    let proof_bundle_consumption_toml = read_labeled_file(
        CLI_READ_ERROR,
        "proof_bundle_consumption",
        proof_bundle_consumption_path,
    )?;
    Ok(build_allocator_provider_proof_bundle_consumption_output(
        &proof_bundle_consumption_toml,
    ))
}

pub fn build_allocator_provider_proof_bundle_consumption_output(
    proof_bundle_consumption_toml: &str,
) -> (String, i32) {
    let report = validate_allocator_provider_proof_bundle_consumption_from_text(
        proof_bundle_consumption_toml,
    );
    let ready = report.status == AllocatorProviderProofBundleConsumptionStatus::ReadyInactive;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         proof_bundle_consumption_status={}\n\
         parse_error={}\n\
         missing_facts={}\n\
         missing_diagnostics={}\n\
         requested_provider_id={}\n\
         selected_provider_id={}\n\
         selected_provider_id_absent={}\n\
         requested_operations={}\n\
         candidate_provider_ids={}\n\
         provider_proof_ids={}\n\
         provider_proof_count={}\n\
         proof_bundle_consumed={}\n\
         active_registry_built={}\n\
         would_build_registry={}\n\
         would_select_provider={}\n\
         would_consume_proof_bundle={}\n\
         would_prepare_rollback={}\n\
         would_open_activation_gate={}\n\
         would_install_hook={}\n\
         would_replace_process_allocator={}\n\
         would_activate={}\n",
        report.diagnostic,
        proof_bundle_consumption_status_name(report.status),
        one_line_option_text(report.parse_error.as_deref()),
        report.missing_facts.join(","),
        report.missing_diagnostics.join(","),
        one_line_option_text(report.requested_provider_id.as_deref()),
        one_line_option_text(report.selected_provider_id.as_deref()),
        report.selected_provider_id_absent,
        report.requested_operations.join(","),
        report.candidate_provider_ids.join(","),
        report.provider_proof_ids.join(","),
        report.provider_proof_count,
        report.proof_bundle_consumed,
        report.active_registry_built,
        report.would_build_registry,
        report.would_select_provider,
        report.would_consume_proof_bundle,
        report.would_prepare_rollback,
        report.would_open_activation_gate,
        report.would_install_hook,
        report.would_replace_process_allocator,
        report.would_activate
    );

    (output, exit_code)
}

fn proof_bundle_consumption_status_name(
    status: AllocatorProviderProofBundleConsumptionStatus,
) -> &'static str {
    match status {
        AllocatorProviderProofBundleConsumptionStatus::MissingFacts => "missing_facts",
        AllocatorProviderProofBundleConsumptionStatus::ReadyInactive => "ready_inactive",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF_BUNDLE_CONSUMPTION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
    );

    #[test]
    fn allocator_provider_proof_bundle_consumption_cli_output_reports_inactive_without_consuming_proof(
    ) {
        let (output, exit_code) = build_allocator_provider_proof_bundle_consumption_output(
            PROOF_BUNDLE_CONSUMPTION_FIXTURE,
        );

        assert_eq!(exit_code, 0);
        assert!(
            output.contains("diagnostic=[allocator-provider/proof-bundle-consumption-inactive]")
        );
        assert!(output.contains("proof_bundle_consumption_status=ready_inactive"));
        assert!(output.contains("parse_error=\n"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("missing_diagnostics=\n"));
        assert!(output.contains("requested_provider_id=native_mimalloc"));
        assert!(output.contains("selected_provider_id=none_reserved"));
        assert!(output.contains("selected_provider_id_absent=true"));
        assert!(output.contains("requested_operations=alloc,realloc,free"));
        assert!(output.contains("candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"));
        assert!(output.contains("provider_proof_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"));
        assert!(output.contains("provider_proof_count=4"));
        assert!(output.contains("proof_bundle_consumed=false"));
        assert!(output.contains("active_registry_built=false"));
        assert!(output.contains("would_build_registry=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_consume_proof_bundle=false"));
        assert!(output.contains("would_prepare_rollback=false"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_replace_process_allocator=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_proof_bundle_consumption_cli_output_reports_missing_facts() {
        let (output, exit_code) = build_allocator_provider_proof_bundle_consumption_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/proof-bundle-consumption-missing]"));
        assert!(output.contains("proof_bundle_consumption_status=missing_facts"));
        assert!(output.contains("missing_facts=schema_version"));
        assert!(output.contains("[allocator-provider/proof-bundle-registry-missing]"));
        assert!(output.contains("[allocator-provider/proof-bundle-selection-missing]"));
        assert!(output.contains("[allocator-provider/proof-bundle-provider-proof-missing]"));
        assert!(output.contains("proof_bundle_consumed=false"));
        assert!(output.contains("active_registry_built=false"));
        assert!(output.contains("would_build_registry=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_consume_proof_bundle=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_proof_bundle_consumption_cli_output_reports_parse_error_one_line() {
        let (output, exit_code) = build_allocator_provider_proof_bundle_consumption_output("[");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/proof-bundle-consumption-missing]"));
        assert!(output.contains("proof_bundle_consumption_status=missing_facts"));
        assert!(output.contains("missing_facts=parse_toml"));
        assert!(output.contains("parse_error="));
        assert!(!output.contains("\nline"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_activate=false"));
    }
}
