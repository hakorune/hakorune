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
