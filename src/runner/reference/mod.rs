pub(crate) mod raw_vm_reference;
pub(crate) mod normal_file_vm;
pub(crate) mod normal_file_vm_request;
pub(crate) mod cli_profile;
#[allow(dead_code)]
pub(crate) mod normal_file_vm_frontdoor;
pub(crate) mod normal_file_canonical_core_request;
pub mod vm_hako;
// SUPPORT0 keeps the request vocabulary at the runner/MIR boundary while the
// supported opt-in lane remains separate from normal/default routing.
#[allow(dead_code)]
pub(crate) mod raw_vm_reference_request;
pub(crate) mod request;
pub(crate) mod terminal;

use crate::cli::CliConfig;
use request::{ExplicitReferenceRunnerRequestV1, ExplicitReferenceRunnerSelectionV1};
use terminal::ReferenceRunOutcomeV1;

/// Select one explicit reference request, or leave the default runner untouched.
///
/// This is the sole CLI selector for the Raw and NormalFile VM-reference lanes.
pub(crate) fn select_and_run(config: &CliConfig) -> Option<ReferenceRunOutcomeV1> {
    let selection = match request::select_from_cli(config) {
        Ok(selection) => selection,
        Err(error) => return Some(ReferenceRunOutcomeV1::Usage(error.into_usage_report())),
    };
    match selection {
        ExplicitReferenceRunnerSelectionV1::NotSelected => None,
        ExplicitReferenceRunnerSelectionV1::Selected(
            ExplicitReferenceRunnerRequestV1::RawVmReference(request),
        ) => Some(raw_vm_reference::run(request)),
        ExplicitReferenceRunnerSelectionV1::Selected(
            ExplicitReferenceRunnerRequestV1::NormalFileVmReference(request),
        ) => Some(normal_file_vm::run(request)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_backend_stays_unselected() {
        assert!(select_and_run(&CliConfig::default()).is_none());
    }

    #[test]
    fn normal_profile_rejection_is_usage_before_source_io() {
        let mut config = CliConfig::default();
        config.backend = "normal-file-vm-reference".to_owned();
        config.file = Some("unread.hako".to_owned());
        config.no_optimize = true;

        let Some(ReferenceRunOutcomeV1::Usage(report)) = select_and_run(&config) else {
            panic!("normal profile conflict must be a usage outcome");
        };
        assert!(report.line().contains("non-default-optimization-requested"));
    }

    #[cfg(feature = "vm-reference")]
    #[test]
    fn central_selector_dispatches_the_normal_request_once() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("normal.hako");
        std::fs::write(&path, "42").expect("write source");
        let mut config = CliConfig::default();
        config.backend = "normal-file-vm-reference".to_owned();
        config.file = Some(path.to_string_lossy().into_owned());

        let Some(ReferenceRunOutcomeV1::Program(report)) = select_and_run(&config) else {
            panic!("central selector must dispatch normal source to its program outcome");
        };
        assert_eq!(report.status_code(), 42);
    }
}
