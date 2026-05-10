use super::CliConfig;
use crate::runtime::allocator_provider_registry::{
    validate_allocator_provider_activation_safety_gate_from_text,
    AllocatorProviderActivationSafetyStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/activation-safety-cli-read-error]";

pub fn maybe_run_allocator_provider_activation_safety_diagnostic(
    config: &CliConfig,
) -> Option<i32> {
    let safety_gate_path = config
        .allocator_provider_activation_safety_gate
        .as_deref()?;

    match run_allocator_provider_activation_safety_file(safety_gate_path) {
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

fn run_allocator_provider_activation_safety_file(
    safety_gate_path: &str,
) -> Result<(String, i32), String> {
    let safety_gate_toml = std::fs::read_to_string(safety_gate_path)
        .map_err(|err| format!("{CLI_READ_ERROR}: safety_gate={safety_gate_path}: {err}"))?;
    Ok(build_allocator_provider_activation_safety_output(
        &safety_gate_toml,
    ))
}

pub fn build_allocator_provider_activation_safety_output(safety_gate_toml: &str) -> (String, i32) {
    let report = validate_allocator_provider_activation_safety_gate_from_text(safety_gate_toml);
    let ready = report.status == AllocatorProviderActivationSafetyStatus::ReadyGateClosed;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         activation_safety_status={}\n\
         parse_error={}\n\
         missing_facts={}\n\
         missing_diagnostics={}\n\
         rollback_target_provider_id={}\n\
         activation_target_provider_id={}\n\
         safety_status={}\n\
         activation_gate_open={}\n\
         would_open_activation_gate={}\n\
         would_activate_hook={}\n\
         would_activate={}\n",
        report.diagnostic,
        activation_safety_status_name(report.status),
        one_line_option_text(report.parse_error.as_deref()),
        report.missing_facts.join(","),
        report.missing_diagnostics.join(","),
        one_line_option_text(report.rollback_target_provider_id.as_deref()),
        one_line_option_text(report.activation_target_provider_id.as_deref()),
        report.safety_status,
        report.activation_gate_open,
        report.would_open_activation_gate,
        report.would_activate_hook,
        report.would_activate
    );

    (output, exit_code)
}

fn activation_safety_status_name(status: AllocatorProviderActivationSafetyStatus) -> &'static str {
    match status {
        AllocatorProviderActivationSafetyStatus::MissingFacts => "missing_facts",
        AllocatorProviderActivationSafetyStatus::ReadyGateClosed => "ready_gate_closed",
    }
}

fn one_line_option_text(value: Option<&str>) -> String {
    value.unwrap_or("").replace(['\r', '\n'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVATION_SAFETY_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
    );

    #[test]
    fn allocator_provider_activation_safety_cli_output_reports_gate_closed_without_activation() {
        let (output, exit_code) =
            build_allocator_provider_activation_safety_output(ACTIVATION_SAFETY_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/activation-safety-blocked]"));
        assert!(output.contains("activation_safety_status=ready_gate_closed"));
        assert!(output.contains("parse_error=\n"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("missing_diagnostics=\n"));
        assert!(output.contains("rollback_target_provider_id=native_mimalloc"));
        assert!(output.contains("activation_target_provider_id=native_mimalloc"));
        assert!(output.contains("safety_status=reserved_gate_closed"));
        assert!(output.contains("activation_gate_open=false"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_activate_hook=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_activation_safety_cli_output_reports_missing_gate() {
        let (output, exit_code) = build_allocator_provider_activation_safety_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/activation-safety-gate-missing]"));
        assert!(output.contains("activation_safety_status=missing_facts"));
        assert!(output.contains("missing_facts=schema_version"));
        assert!(output.contains("[allocator-provider/activation-safety-target-missing]"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_activate_hook=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_activation_safety_cli_output_reports_parse_error_one_line() {
        let (output, exit_code) = build_allocator_provider_activation_safety_output("[");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/activation-safety-gate-missing]"));
        assert!(output.contains("activation_safety_status=missing_facts"));
        assert!(output.contains("missing_facts=parse_toml"));
        assert!(output.contains("parse_error="));
        assert!(!output.contains("\nline"));
        assert!(output.contains("would_activate=false"));
    }
}
