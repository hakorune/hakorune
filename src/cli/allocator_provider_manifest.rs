use super::CliConfig;
use crate::runtime::allocator_provider_manifest::{
    parse_allocator_provider_manifest_text, AllocatorProviderManifestStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/cli-read-error]";

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

fn provider_manifest_status_name(status: AllocatorProviderManifestStatus) -> &'static str {
    match status {
        AllocatorProviderManifestStatus::MissingOrInvalid => "missing_or_invalid",
        AllocatorProviderManifestStatus::ReadyDiagnostic => "ready",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDER_MANIFEST_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-manifest-v0.toml"
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
}
