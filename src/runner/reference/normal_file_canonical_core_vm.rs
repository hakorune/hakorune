//! Runner-neutral terminal for the explicit canonical-core reference request.
//!
//! This module consumes the sealed request once. It owns neither CLI selection
//! nor process exit, and delegates source-family selection, publication, VM
//! execution, and process-status projection to their existing owners.

use super::normal_file_canonical_core_request::NormalFileCanonicalCoreVmReferenceProductionRequestV1;
use super::normal_file_vm_frontdoor::{
    NormalFileReadErrorV1, NormalFileSourceErrorV1, NormalFileSourceProfileErrorV1,
    NormalFileSourceStageV1, RejectedNormalFileSourceV1,
};
use super::terminal::{ReferenceInvocationReportV1, ReferenceRunOutcomeV1, ReferenceUsageReportV1};

pub(crate) fn run(
    request: NormalFileCanonicalCoreVmReferenceProductionRequestV1,
) -> ReferenceRunOutcomeV1 {
    #[cfg(not(feature = "vm-reference"))]
    {
        let _ = request;
        return ReferenceRunOutcomeV1::Usage(ReferenceUsageReportV1::new(
            "[normal-file-canonical-core-vm-reference/feature-unavailable] build with --features vm-reference",
        ));
    }

    #[cfg(feature = "vm-reference")]
    {
        let prepared = match request.into_frontdoor_request().prepare() {
            Ok(prepared) => prepared,
            Err(rejected) => return invocation_from_source_rejection(rejected),
        };
        let loaded = match prepared.read_once() {
            Ok(loaded) => loaded,
            Err(rejected) => return invocation_from_source_rejection(rejected),
        };
        let source = match loaded.parse_once() {
            Ok(source) => source,
            Err(rejected) => return invocation_from_source_rejection(rejected),
        };
        let classified = match source.prepare_source_plan_request().classify() {
            Ok(classified) => classified,
            Err(rejected) => {
                let detail = format!("stage={:?} error={:?}", rejected.stage(), rejected.error());
                rejected.discard();
                return invocation("source-plan-rejected", detail);
            }
        };
        let request = match classified.into_canonical_core_compile_request() {
            Ok(request) => request,
            Err(rejected) => {
                let detail = format!("error={:?}", rejected.error());
                rejected.discard();
                return invocation("source-plan-handoff-rejected", detail);
            }
        };
        let mut compiler = crate::mir::MirCompiler::new();
        match compiler.run_canonical_core_source_plan_for_runner_v1(request) {
            Ok(report) => ReferenceRunOutcomeV1::Program(report),
            Err(report) => invocation(
                report.code(),
                format!("stage={:?} {}", report.stage(), report.detail()),
            ),
        }
    }
}

fn invocation_from_source_rejection(rejected: RejectedNormalFileSourceV1) -> ReferenceRunOutcomeV1 {
    let code = normal_source_error_code(rejected.stage(), rejected.error());
    rejected.discard();
    invocation("source-rejected", code)
}

fn invocation(code: &str, detail: impl std::fmt::Display) -> ReferenceRunOutcomeV1 {
    ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(format!(
        "[normal-file-canonical-core-vm-reference/{code}] {detail}"
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

#[cfg(all(test, feature = "vm-reference"))]
mod tests {
    use super::*;
    use crate::cli::CliConfig;
    use tempfile::tempdir;

    fn request(path: &std::path::Path) -> NormalFileCanonicalCoreVmReferenceProductionRequestV1 {
        let mut config = CliConfig::default();
        config.backend = NormalFileCanonicalCoreVmReferenceProductionRequestV1::backend_name()
            .to_owned();
        config.file = Some(path.to_string_lossy().into_owned());
        NormalFileCanonicalCoreVmReferenceProductionRequestV1::try_from_selected_cli(&config)
            .expect("canonical-core request")
    }

    #[test]
    fn consumes_script_and_main_through_the_shared_program_terminal() {
        let dir = tempdir().expect("tempdir");
        let script = dir.path().join("script.hako");
        let main = dir.path().join("main.hako");
        std::fs::write(&script, "42").expect("write script");
        std::fs::write(&main, "static box Main { main() {} }").expect("write main");

        let ReferenceRunOutcomeV1::Program(script) = run(request(&script)) else {
            panic!("Script must reach the program terminal");
        };
        assert_eq!(script.status_code(), 42);
        let ReferenceRunOutcomeV1::Program(main) = run(request(&main)) else {
            panic!("Main must reach the program terminal");
        };
        assert_eq!(main.status_code(), 0);
    }

    #[test]
    fn classifies_parse_and_canonical_dispatch_rejections_as_invocation_failures() {
        let dir = tempdir().expect("tempdir");
        let parse = dir.path().join("parse.hako");
        let direct_call = dir.path().join("direct-call.hako");
        std::fs::write(&parse, "static box {").expect("write parse failure");
        std::fs::write(
            &direct_call,
            "static function helper(x: i64): i64 { return x }\nstatic box Main { main() { helper(42) } }",
        )
        .expect("write direct call");

        let ReferenceRunOutcomeV1::Invocation(parse) = run(request(&parse)) else {
            panic!("parse failure must be invocation failure");
        };
        assert!(parse.line().contains("parse"));
        let ReferenceRunOutcomeV1::Invocation(dispatch) = run(request(&direct_call)) else {
            panic!("dispatch rejection must be invocation failure");
        };
        assert!(dispatch.line().contains("canonical-core-dispatch-rejected"));
    }
}
