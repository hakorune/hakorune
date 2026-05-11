use super::diagnostic_output::{finish_result, one_line_option_text, read_labeled_file};
use super::CliConfig;
use crate::runtime::allocator_provider_registry::{
    validate_allocator_provider_selection_decision_from_text,
    AllocatorProviderSelectionDecisionStatus,
};

const CLI_READ_ERROR: &str = "[allocator-provider/selection-decision-cli-read-error]";

pub fn maybe_run_allocator_provider_selection_decision_diagnostic(
    config: &CliConfig,
) -> Option<i32> {
    let selection_decision_path = config.allocator_provider_selection_decision.as_deref()?;

    finish_result(run_allocator_provider_selection_decision_file(
        selection_decision_path,
    ))
}

fn run_allocator_provider_selection_decision_file(
    selection_decision_path: &str,
) -> Result<(String, i32), String> {
    let selection_decision_toml = read_labeled_file(
        CLI_READ_ERROR,
        "selection_decision",
        selection_decision_path,
    )?;
    Ok(build_allocator_provider_selection_decision_output(
        &selection_decision_toml,
    ))
}

pub fn build_allocator_provider_selection_decision_output(
    selection_decision_toml: &str,
) -> (String, i32) {
    let report = validate_allocator_provider_selection_decision_from_text(selection_decision_toml);
    let ready = report.status == AllocatorProviderSelectionDecisionStatus::ReadyInactive;
    let exit_code = if ready { 0 } else { 2 };

    let output = format!(
        "diagnostic={}\n\
         selection_decision_status={}\n\
         parse_error={}\n\
         missing_facts={}\n\
         missing_diagnostics={}\n\
         requested_provider_id={}\n\
         required_operations={}\n\
         candidate_provider_ids={}\n\
         selected_provider_id={}\n\
         selected_provider_id_absent={}\n\
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
        selection_decision_status_name(report.status),
        one_line_option_text(report.parse_error.as_deref()),
        report.missing_facts.join(","),
        report.missing_diagnostics.join(","),
        one_line_option_text(report.requested_provider_id.as_deref()),
        report.required_operations.join(","),
        report.candidate_provider_ids.join(","),
        one_line_option_text(report.selected_provider_id.as_deref()),
        report.selected_provider_id_absent,
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

fn selection_decision_status_name(
    status: AllocatorProviderSelectionDecisionStatus,
) -> &'static str {
    match status {
        AllocatorProviderSelectionDecisionStatus::MissingFacts => "missing_facts",
        AllocatorProviderSelectionDecisionStatus::ReadyInactive => "ready_inactive",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SELECTION_DECISION_FIXTURE: &str = include_str!(
        "../../docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
    );

    #[test]
    fn allocator_provider_selection_decision_cli_output_reports_inactive_without_selection() {
        let (output, exit_code) =
            build_allocator_provider_selection_decision_output(SELECTION_DECISION_FIXTURE);

        assert_eq!(exit_code, 0);
        assert!(output.contains("diagnostic=[allocator-provider/selection-decision-inactive]"));
        assert!(output.contains("selection_decision_status=ready_inactive"));
        assert!(output.contains("parse_error=\n"));
        assert!(output.contains("missing_facts=\n"));
        assert!(output.contains("missing_diagnostics=\n"));
        assert!(output.contains("requested_provider_id=native_mimalloc"));
        assert!(output.contains("required_operations=alloc,realloc,free"));
        assert!(output.contains("candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"));
        assert!(output.contains("selected_provider_id=none_reserved"));
        assert!(output.contains("selected_provider_id_absent=true"));
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
    fn allocator_provider_selection_decision_cli_output_reports_missing_facts() {
        let (output, exit_code) = build_allocator_provider_selection_decision_output("");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/selection-decision-missing]"));
        assert!(output.contains("selection_decision_status=missing_facts"));
        assert!(output.contains("missing_facts=schema_version"));
        assert!(output.contains("[allocator-provider/selection-registry-missing]"));
        assert!(output.contains("[allocator-provider/selection-request-missing]"));
        assert!(output.contains("active_registry_built=false"));
        assert!(output.contains("would_build_registry=false"));
        assert!(output.contains("would_select_provider=false"));
        assert!(output.contains("would_activate=false"));
    }

    #[test]
    fn allocator_provider_selection_decision_cli_output_reports_parse_error_one_line() {
        let (output, exit_code) = build_allocator_provider_selection_decision_output("[");

        assert_eq!(exit_code, 2);
        assert!(output.contains("diagnostic=[allocator-provider/selection-decision-missing]"));
        assert!(output.contains("selection_decision_status=missing_facts"));
        assert!(output.contains("missing_facts=parse_toml"));
        assert!(output.contains("parse_error="));
        assert!(!output.contains("\nline"));
        assert!(output.contains("would_open_activation_gate=false"));
        assert!(output.contains("would_install_hook=false"));
        assert!(output.contains("would_activate=false"));
    }
}
