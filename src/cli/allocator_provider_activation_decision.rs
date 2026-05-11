use super::CliConfig;
use crate::runtime::allocator_provider_activation_decision::{
    validate_allocator_provider_activation_decision_from_text,
    AllocatorProviderActivationDecisionStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/activation-decision-cli-read-error]";

pub fn maybe_run_allocator_provider_activation_decision_diagnostic(
    config: &CliConfig,
) -> Option<i32> {
    let activation_decision_path = config.allocator_provider_activation_decision.as_deref()?;

    match run_allocator_provider_activation_decision_file(activation_decision_path) {
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

fn run_allocator_provider_activation_decision_file(
    activation_decision_path: &str,
) -> Result<(String, i32), String> {
    let activation_decision_toml =
        std::fs::read_to_string(activation_decision_path).map_err(|err| {
            format!("{CLI_READ_ERROR}: activation_decision={activation_decision_path}: {err}")
        })?;
    Ok(build_allocator_provider_activation_decision_output(
        &activation_decision_toml,
    ))
}

pub fn build_allocator_provider_activation_decision_output(
    activation_decision_toml: &str,
) -> (String, i32) {
    let report =
        validate_allocator_provider_activation_decision_from_text(activation_decision_toml);
    let ready = report.status == AllocatorProviderActivationDecisionStatus::ReadyBlocked;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         activation_decision_status={}\n\
         parse_error={}\n\
         missing_facts={}\n\
         missing_diagnostics={}\n\
         operator_intent={}\n\
         requested_provider_id={}\n\
         activation_safety_gate_report_path={}\n\
         registry_snapshot_path={}\n\
         selection_decision_path={}\n\
         proof_bundle_report_path={}\n\
         rollback_preflight_report_path={}\n\
         activation_decision_surface_status={}\n\
         activation_decision_allowed={}\n\
         would_select_provider={}\n\
         would_consume_proof={}\n\
         would_prepare_rollback={}\n\
         would_open_activation_gate={}\n\
         would_install_hook={}\n\
         would_replace_process_allocator={}\n\
         would_activate={}\n",
        report.diagnostic,
        activation_decision_status_name(report.status),
        one_line_option_text(report.parse_error.as_deref()),
        report.missing_facts.join(","),
        report.missing_diagnostics.join(","),
        one_line_option_text(report.operator_intent.as_deref()),
        one_line_option_text(report.requested_provider_id.as_deref()),
        one_line_option_text(report.activation_safety_gate_report_path.as_deref()),
        one_line_option_text(report.registry_snapshot_path.as_deref()),
        one_line_option_text(report.selection_decision_path.as_deref()),
        one_line_option_text(report.proof_bundle_report_path.as_deref()),
        one_line_option_text(report.rollback_preflight_report_path.as_deref()),
        report.activation_decision_surface_status,
        report.activation_decision_allowed,
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

fn activation_decision_status_name(
    status: AllocatorProviderActivationDecisionStatus,
) -> &'static str {
    match status {
        AllocatorProviderActivationDecisionStatus::MissingFacts => "missing_facts",
        AllocatorProviderActivationDecisionStatus::ReadyBlocked => "ready_blocked",
    }
}

fn one_line_option_text(value: Option<&str>) -> String {
    value.unwrap_or("").replace(['\r', '\n'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVATION_DECISION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
    );

    #[test]
    fn allocator_provider_activation_decision_cli_output_reports_blocked_without_activation() {
        let (output, exit_code) =
            build_allocator_provider_activation_decision_output(ACTIVATION_DECISION_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/activation-decision-blocked]"));
        assert!(output.contains("activation_decision_status=ready_blocked"));
        assert!(output.contains("parse_error=\n"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("missing_diagnostics=\n"));
        assert!(output.contains("operator_intent=diagnose"));
        assert!(output.contains("requested_provider_id=mimalloc"));
        assert!(
            output.contains("activation_safety_gate_report_path=activation-safety-gate-v0.toml")
        );
        assert!(output.contains("registry_snapshot_path=registry-snapshot-v0.toml"));
        assert!(output.contains("selection_decision_path=selection-decision-v0.toml"));
        assert!(output.contains("proof_bundle_report_path=proof-bundle-consumption-v0.toml"));
        assert!(output.contains("rollback_preflight_report_path=rollback-preflight-v0.toml"));
        assert!(output.contains("activation_decision_surface_status=reserved_fixture"));
        assert!(output.contains("activation_decision_allowed=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_consume_proof=false"));
        assert!(output.contains("would_prepare_rollback=false"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_replace_process_allocator=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_activation_decision_cli_output_reports_missing_facts() {
        let (output, exit_code) = build_allocator_provider_activation_decision_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/activation-decision-reserved]"));
        assert!(output.contains("activation_decision_status=missing_facts"));
        assert!(output.contains("missing_facts=surface_version"));
        assert!(output.contains("[allocator-provider/activation-decision-safety-gate-missing]"));
        assert!(output.contains("activation_decision_allowed=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_activation_decision_cli_output_reports_parse_error_one_line() {
        let (output, exit_code) = build_allocator_provider_activation_decision_output("[");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/activation-decision-reserved]"));
        assert!(output.contains("activation_decision_status=missing_facts"));
        assert!(output.contains("missing_facts=parse_toml"));
        assert!(output.contains("parse_error="));
        assert!(!output.contains("\nline"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_activate=false"));
    }
}
