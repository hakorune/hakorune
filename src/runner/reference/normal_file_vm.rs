//! Unconnected production-shaped runner for the normal-file VM-reference lane.

use super::normal_file_vm_frontdoor::{
    NormalFileReadErrorV1, NormalFileSourceErrorV1, NormalFileSourceProfileErrorV1,
    NormalFileSourceStageV1, RejectedNormalFileSourceV1,
};
use super::normal_file_vm_request::NormalFileVmReferenceProductionRequestV1;
use super::terminal::{ReferenceInvocationReportV1, ReferenceRunOutcomeV1, ReferenceUsageReportV1};

/// Consume one sealed normal request through the existing front door and the
/// bounded MIR runner adapter. This owner has no CLI selection or process exit.
pub(crate) fn run(request: NormalFileVmReferenceProductionRequestV1) -> ReferenceRunOutcomeV1 {
    #[cfg(not(feature = "vm-reference"))]
    {
        let _ = request;
        return ReferenceRunOutcomeV1::Usage(ReferenceUsageReportV1::new(
            "[normal-file-vm-reference/feature-unavailable] build with --features vm-reference",
        ));
    }

    #[cfg(feature = "vm-reference")]
    {
        let prepared = match request.into_frontdoor_request().prepare() {
            Ok(prepared) => prepared,
            Err(rejected) => return usage_from_source_rejection(rejected),
        };
        let loaded = match prepared.read_once() {
            Ok(loaded) => loaded,
            Err(rejected) => return invocation_from_source_rejection(rejected),
        };
        let source = match loaded.parse_once() {
            Ok(source) => source,
            Err(rejected) => return invocation_from_source_rejection(rejected),
        };
        let invocation = source
            .prepare_raw_vm_handoff()
            .into_raw_vm_reference_invocation();
        let mut compiler = crate::mir::MirCompiler::new();
        match compiler.run_raw_vm_reference_for_runner_v1(invocation) {
            Ok(report) => ReferenceRunOutcomeV1::Program(report),
            Err(report) => {
                ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(format!(
                    "[normal-file-vm-reference/invocation/{}] stage={:?} {}",
                    report.code(),
                    report.stage(),
                    report.detail()
                )))
            }
        }
    }
}

fn usage_from_source_rejection(rejected: RejectedNormalFileSourceV1) -> ReferenceRunOutcomeV1 {
    let code = normal_source_error_code(rejected.stage(), rejected.error());
    rejected.discard();
    ReferenceRunOutcomeV1::Usage(ReferenceUsageReportV1::new(format!(
        "[normal-file-vm-reference/profile/rejected] {code}"
    )))
}

fn invocation_from_source_rejection(rejected: RejectedNormalFileSourceV1) -> ReferenceRunOutcomeV1 {
    let code = normal_source_error_code(rejected.stage(), rejected.error());
    rejected.discard();
    ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(format!(
        "[normal-file-vm-reference/source/rejected] {code}"
    )))
}

fn normal_source_error_code(
    _stage: NormalFileSourceStageV1,
    error: NormalFileSourceErrorV1<'_>,
) -> &'static str {
    match error {
        NormalFileSourceErrorV1::Profile(_) => "profile",
        NormalFileSourceErrorV1::Read(NormalFileReadErrorV1::NotFound) => "file-not-found",
        NormalFileSourceErrorV1::Read(NormalFileReadErrorV1::InvalidUtf8) => "invalid-utf8",
        NormalFileSourceErrorV1::Read(NormalFileReadErrorV1::Other(_)) => "file-read",
        NormalFileSourceErrorV1::Parse(_) => "parse",
        NormalFileSourceErrorV1::SourceProfile(NormalFileSourceProfileErrorV1::UsingStatement) => {
            "using-not-supported"
        }
        NormalFileSourceErrorV1::SourceProfile(NormalFileSourceProfileErrorV1::ImportStatement) => {
            "import-not-supported"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::CliConfig;
    use tempfile::tempdir;

    fn selected_request(path: &std::path::Path) -> NormalFileVmReferenceProductionRequestV1 {
        let mut config = CliConfig::default();
        config.backend = NormalFileVmReferenceProductionRequestV1::backend_name().to_owned();
        config.file = Some(path.to_string_lossy().into_owned());
        NormalFileVmReferenceProductionRequestV1::try_from_selected_cli(&config)
            .expect("test normal profile should seal")
    }

    #[cfg(not(feature = "vm-reference"))]
    #[test]
    fn feature_disabled_is_usage_before_file_read() {
        let outcome = run(selected_request(std::path::Path::new(
            "does-not-exist.hako",
        )));
        let ReferenceRunOutcomeV1::Usage(report) = outcome else {
            panic!("feature-disabled normal route must report usage");
        };
        assert!(report.line().contains("feature-unavailable"));
    }

    #[cfg(feature = "vm-reference")]
    #[test]
    fn executes_normal_source_through_the_existing_raw_terminal() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("scalar.hako");
        std::fs::write(&path, "42").expect("write source");

        let outcome = run(selected_request(&path));
        let ReferenceRunOutcomeV1::Program(report) = outcome else {
            panic!("scalar normal source must reach the Raw program terminal");
        };
        assert_eq!(report.status_code(), 42);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[cfg(feature = "vm-reference")]
    #[test]
    fn missing_file_is_an_invocation_failure_before_raw_handoff() {
        let dir = tempdir().expect("tempdir");
        let outcome = run(selected_request(&dir.path().join("missing.hako")));
        let ReferenceRunOutcomeV1::Invocation(report) = outcome else {
            panic!("missing normal source must be an invocation failure");
        };
        assert!(report.line().contains("file-not-found"));
    }
}
