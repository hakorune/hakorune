use super::diagnostic_output::{finish_result, one_line_option_text, read_labeled_file};
use super::CliConfig;
use crate::runtime::allocator_provider_registry::{
    validate_allocator_provider_registry_snapshot_from_text,
    AllocatorProviderRegistrySnapshotStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/registry-snapshot-cli-read-error]";

pub fn maybe_run_allocator_provider_registry_snapshot_diagnostic(
    config: &CliConfig,
) -> Option<i32> {
    let registry_snapshot_path = config.allocator_provider_registry_snapshot.as_deref()?;

    finish_result(run_allocator_provider_registry_snapshot_file(
        registry_snapshot_path,
    ))
}

fn run_allocator_provider_registry_snapshot_file(
    registry_snapshot_path: &str,
) -> Result<(String, i32), String> {
    let registry_snapshot_toml =
        read_labeled_file(CLI_READ_ERROR, "registry_snapshot", registry_snapshot_path)?;
    Ok(build_allocator_provider_registry_snapshot_output(
        &registry_snapshot_toml,
    ))
}

pub fn build_allocator_provider_registry_snapshot_output(
    registry_snapshot_toml: &str,
) -> (String, i32) {
    let report = validate_allocator_provider_registry_snapshot_from_text(registry_snapshot_toml);
    let ready = report.status == AllocatorProviderRegistrySnapshotStatus::ReadyInactive;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         registry_snapshot_status={}\n\
         parse_error={}\n\
         missing_facts={}\n\
         missing_diagnostics={}\n\
         provider_ids={}\n\
         provider_count={}\n\
         active_registry_built={}\n\
         would_build_registry={}\n\
         would_select_provider={}\n\
         would_consume_proof={}\n\
         would_prepare_rollback={}\n\
         would_open_activation_gate={}\n\
         would_install_hook={}\n\
         would_replace_process_allocator={}\n\
         would_activate={}\n",
        report.diagnostic,
        registry_snapshot_status_name(report.status),
        one_line_option_text(report.parse_error.as_deref()),
        report.missing_facts.join(","),
        report.missing_diagnostics.join(","),
        report.provider_ids.join(","),
        report.provider_count,
        report.active_registry_built,
        report.would_build_registry,
        report.would_select_provider,
        report.would_consume_proof,
        report.would_prepare_rollback,
        report.would_open_activation_gate,
        report.would_install_hook,
        report.would_replace_process_allocator,
        report.would_activate
    );

    (output, exit_code)
}

fn registry_snapshot_status_name(status: AllocatorProviderRegistrySnapshotStatus) -> &'static str {
    match status {
        AllocatorProviderRegistrySnapshotStatus::MissingFacts => "missing_facts",
        AllocatorProviderRegistrySnapshotStatus::ReadyInactive => "ready_inactive",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REGISTRY_SNAPSHOT_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
    );

    #[test]
    fn allocator_provider_registry_snapshot_cli_output_reports_inactive_without_registry_build() {
        let (output, exit_code) =
            build_allocator_provider_registry_snapshot_output(REGISTRY_SNAPSHOT_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/registry-snapshot-inactive]"));
        assert!(output.contains("registry_snapshot_status=ready_inactive"));
        assert!(output.contains("parse_error=\n"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("missing_diagnostics=\n"));
        assert!(output.contains("provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"));
        assert!(output.contains("provider_count=4"));
        assert!(output.contains("active_registry_built=false"));
        assert!(output.contains("would_build_registry=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_consume_proof=false"));
        assert!(output.contains("would_prepare_rollback=false"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_replace_process_allocator=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_registry_snapshot_cli_output_reports_missing_facts() {
        let (output, exit_code) = build_allocator_provider_registry_snapshot_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/registry-snapshot-missing]"));
        assert!(output.contains("registry_snapshot_status=missing_facts"));
        assert!(output.contains("missing_facts=schema_version"));
        assert!(output.contains("[allocator-provider/registry-provider-missing]"));
        assert!(output.contains("active_registry_built=false"));
        assert!(output.contains("would_build_registry=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_registry_snapshot_cli_output_reports_parse_error_one_line() {
        let (output, exit_code) = build_allocator_provider_registry_snapshot_output("[");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/registry-snapshot-missing]"));
        assert!(output.contains("registry_snapshot_status=missing_facts"));
        assert!(output.contains("missing_facts=parse_toml"));
        assert!(output.contains("parse_error="));
        assert!(!output.contains("\nline"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_activate=false"));
    }
}
