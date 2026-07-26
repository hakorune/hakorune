use super::*;
use crate::mir::normal_source_plan::{
    NormalSourcePlanErrorV1, NormalSourcePlanStageV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn request(path: PathBuf) -> super::super::NormalFileRequestV1 {
    super::super::NormalFileVmFrontDoorV1::file_no_import_request(path)
}

fn canonical_core_request(path: PathBuf) -> super::super::NormalFileRequestV1 {
    super::super::NormalFileVmFrontDoorV1::file_canonical_core_request(path)
}

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write source-plan fixture");
    path
}

fn classify(
    dir: &Path,
    name: &str,
    source: &str,
) -> Result<ClassifiedNormalFileSourcePlanV1, RejectedNormalFileSourcePlanningV1> {
    request(write_source(dir, name, source))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
}

fn classify_canonical_core(
    dir: &Path,
    name: &str,
    source: &str,
) -> ClassifiedNormalFileSourcePlanV1 {
    canonical_core_request(write_source(dir, name, source))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
        .expect("canonical-core source plan")
}

#[test]
fn parsed_empty_and_scalar_sources_become_script_plans_once() {
    let dir = tempdir().expect("tempdir");
    for (name, source) in [("empty.hako", ""), ("scalar.hako", "42")] {
        let classified = classify(dir.path(), name, source).expect("Script source plan");
        assert!(matches!(
            classified.plan(),
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
        ));
        assert_eq!(classified.receipt_counts(), (1, 1));
        assert!(classified.retained_source_identity().ends_with(name));
    }
}

#[test]
fn canonical_core_profile_reaches_the_same_one_read_one_parse_source_plan_boundary() {
    let dir = tempdir().expect("tempdir");
    let classified = canonical_core_request(write_source(dir.path(), "core-script.hako", "42"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
        .expect("Script source plan");
    assert!(matches!(
        classified.plan(),
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
    ));
    assert_eq!(classified.receipt_counts(), (1, 1));
    assert!(classified.is_canonical_core_profile_for_test());
}

#[test]
fn canonical_core_dispatch_script_handoff_moves_only_the_sealed_plan_and_receipt() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(dir.path(), "handoff.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical-core handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("Script candidate through the canonical physical entry");
    assert!(candidate.is_script());
    assert_eq!(candidate.receipt_counts(), (1, 1));
}

#[test]
fn canonical_core_script_candidate_retains_sealed_publication_evidence() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(dir.path(), "evidence.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical-core Script handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("unpublished Script candidate");
    let evidence = candidate
        .script_candidate_evidence_for_test()
        .expect("Script evidence");

    assert!(evidence.target_is_main);
    assert_eq!(evidence.target_symbol, "main");
    assert_eq!(evidence.target_arity, 0);
    assert!(evidence.source_identity.ends_with("evidence.hako"));
    assert_eq!(evidence.schema_row_count, 1);
    assert_eq!(evidence.result_kind, "integer");
    assert_eq!(evidence.verification_function_count, 1);
    assert_eq!(evidence.module_function_count, 1);
}

#[test]
fn canonical_core_publication_projects_main_and_script_without_reobservation() {
    let dir = tempdir().expect("tempdir");
    let script = classify_canonical_core(dir.path(), "published-script.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical Script handoff");
    let main = classify_canonical_core(
        dir.path(),
        "published-main.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical Main handoff");
    let mut compiler = crate::mir::MirCompiler::new();

    let script = compiler
        .compile_canonical_core_source_plan(script)
        .expect("Script candidate")
        .canonical_publication_summary_for_test()
        .expect("Script publication pairing");
    assert_eq!(script.target_symbol, "main");
    assert_eq!(script.target_arity, 0);
    assert_eq!(script.result_kind, "integer");
    assert_eq!(script.family, "script");

    let main = compiler
        .compile_canonical_core_source_plan(main)
        .expect("Main candidate")
        .canonical_publication_summary_for_test()
        .expect("Main publication pairing");
    assert_eq!(main.target_symbol, "main");
    assert_eq!(main.target_arity, 0);
    assert_eq!(main.result_kind, "unit");
    assert_eq!(main.family, "main");
}

#[test]
fn canonical_core_dispatch_script_candidate_preserves_compiler_reuse_for_main() {
    let dir = tempdir().expect("tempdir");
    let script = classify_canonical_core(dir.path(), "script-reuse.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical Script handoff");
    let main = classify_canonical_core(
        dir.path(),
        "main-reuse.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical Main handoff");

    let mut compiler = crate::mir::MirCompiler::new();
    let script_candidate = compiler
        .compile_canonical_core_source_plan(script)
        .expect("unpublished Script candidate");
    assert!(script_candidate.is_script());
    let main_candidate = compiler
        .compile_canonical_core_source_plan(main)
        .expect("unpublished Main candidate after Script");
    assert!(main_candidate.is_main());
}

#[test]
fn narrow_profile_cannot_enter_the_canonical_core_dispatch_handoff() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(dir.path(), "narrow-handoff.hako", "42")
        .expect("narrow source plan")
        .into_canonical_core_compile_request()
        .expect_err("narrow profile is not canonical-core");
    assert_eq!(
        rejected.error(),
        CanonicalCoreSourcePlanHandoffErrorV1::ProfileExcludesCanonicalCore
    );
    rejected.discard();
}

#[test]
fn canonical_core_dispatch_builds_only_main0_candidate_in_s0() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(
        dir.path(),
        "main-dispatch.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("unpublished Main candidate");
    assert!(candidate.is_main());
    assert_eq!(candidate.receipt_counts(), (1, 1));
}

#[test]
fn canonical_core_dispatch_rejects_callable_before_builder_effects() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(
        dir.path(),
        "callable-dispatch.hako",
        "function helper() {}\nstatic box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let rejected = compiler
        .compile_canonical_core_source_plan(request)
        .expect_err("Callable candidate remains pending in Main-only S0");
    assert_eq!(
        rejected.stage(),
        crate::mir::CanonicalCoreDispatchStageV1::FamilyCapability
    );
    assert!(matches!(
        rejected.cause(),
        crate::mir::CanonicalCoreDispatchErrorV1::FamilyCapabilityPending(
            crate::mir::CanonicalCorePendingFamilyV1::CallableModule
        )
    ));
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();
}

#[test]
fn parsed_main_zero_becomes_scalar_main_plan_once() {
    let dir = tempdir().expect("tempdir");
    let classified = classify(dir.path(), "main.hako", "static box Main { main() {} }")
        .expect("Main0 source plan");
    assert!(matches!(
        classified.plan(),
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(_))
    ));
    assert_eq!(classified.receipt_counts(), (1, 1));
}

#[test]
fn parsed_main_with_top_level_or_box_helper_becomes_callable_module() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        (
            "top-level-helper.hako",
            "function helper() {}\nstatic box Main { main() {} }",
        ),
        (
            "main-box-helper.hako",
            "static box Main { helper() {} main() {} }",
        ),
    ];
    for (name, source) in cases {
        let classified = classify(dir.path(), name, source).expect("callable source plan");
        assert!(matches!(
            classified.plan(),
            SealedNormalSourcePlanV1::CallableModule(_)
        ));
        assert_eq!(classified.receipt_counts(), (1, 1));
    }
}

#[test]
fn parsed_function_only_retains_missing_entry_rejection() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(dir.path(), "function-only.hako", "function helper() {}")
        .expect_err("function-only source has no entry");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::SourceEntry);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MissingSourceEntry
    );
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();
}

#[test]
fn parsed_script_plus_main_retains_mixed_family_rejection() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(
        dir.path(),
        "mixed.hako",
        "42\nstatic box Main { main() {} }",
    )
    .expect_err("mixed source families reject");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::FamilyClosure);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MixedSourceFamilies
    );
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();
}

#[test]
fn parse_and_using_rejections_never_issue_source_plan_requests() {
    let dir = tempdir().expect("tempdir");

    let parse_rejected = request(write_source(dir.path(), "invalid.hako", "@"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect_err("parse rejects before source-plan request");
    assert_eq!(
        parse_rejected.stage(),
        super::super::NormalFileSourceStageV1::Parse
    );

    let using_rejected = request(write_source(dir.path(), "using.hako", "using foo"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect_err("using rejects before source-plan request");
    assert_eq!(
        using_rejected.stage(),
        super::super::NormalFileSourceStageV1::SourceProfile
    );
}
