use super::*;
use tempfile::tempdir;

fn request(path: PathBuf) -> NormalFileRequestV1 {
    NormalFileVmFrontDoorV1::file_no_import_request(path)
}

fn canonical_core_request(path: PathBuf) -> NormalFileRequestV1 {
    NormalFileVmFrontDoorV1::file_canonical_core_request(path)
}

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write fixture source");
    path
}

#[cfg(feature = "vm-reference")]
fn run_file_source(
    compiler: &mut crate::mir::MirCompiler,
    dir: &Path,
    name: &str,
    source: &str,
) -> crate::mir::RawVmReferenceRunReportV1 {
    run_file_source_result(compiler, dir, name, source)
        .expect("existing Raw VM-reference terminal should execute handoff")
}

#[cfg(feature = "vm-reference")]
fn run_file_source_result(
    compiler: &mut crate::mir::MirCompiler,
    dir: &Path,
    name: &str,
    source: &str,
) -> Result<crate::mir::RawVmReferenceRunReportV1, String> {
    let path = write_source(dir, name, source);
    let invocation = request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect("parse")
        .prepare_raw_vm_handoff()
        .expect("frozen Raw profile must prepare Raw handoff")
        .into_raw_vm_reference_invocation();
    compiler.run_raw_vm_reference_v1(invocation)
}

#[test]
fn empty_path_rejects_before_file_read() {
    let rejected = request(PathBuf::new())
        .prepare()
        .expect_err("empty path rejects");
    assert_eq!(rejected.stage(), NormalFileSourceStageV1::Profile);
    assert!(matches!(
        rejected.error(),
        NormalFileSourceErrorV1::Profile(NormalFileProfileErrorV1::EmptySourcePath)
    ));
}

#[test]
fn reads_and_parses_one_canonical_file_once() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "scalar.hako", "42");
    let prepared = request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect("parse");
    assert_eq!(prepared.receipt().read_count, 1);
    assert_eq!(prepared.receipt().parse_count, 1);
    assert_eq!(prepared.receipt().utf8_len, 2);
    prepared.discard_at_named_terminal();
}

#[test]
fn parse_rejection_retains_one_read_receipt() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "invalid.hako", "@");
    let rejected = request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect_err("parse rejects");
    assert_eq!(rejected.stage(), NormalFileSourceStageV1::Parse);
    let RejectedNormalFileSourceV1::Parse { loaded, .. } = rejected else {
        panic!("expected parse rejection");
    };
    assert_eq!(loaded.receipt.read_count, 1);
    assert_eq!(loaded.receipt.parse_count, 1);
}

#[test]
fn source_using_rejects_after_authorized_raw_extraction() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "using.hako", "using foo");
    let rejected = request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect("parse source once")
        .prepare_raw_vm_handoff()
        .expect_err("using rejects at the Raw profile boundary");
    assert_eq!(rejected.error(), NormalFileVmHandoffErrorV1::UsingStatement);
    rejected.discard();
}

#[test]
fn consuming_handoff_keeps_the_existing_raw_profile_paired() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "handoff.hako", "42");
    let handoff = request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect("parse")
        .prepare_raw_vm_handoff()
        .expect("frozen Raw profile must prepare Raw handoff");
    assert_eq!(handoff.source.read_count, 1);
    assert_eq!(handoff.source.parse_count, 1);
    let invocation = handoff.into_raw_vm_reference_invocation();
    assert_eq!(invocation.compile.module_name.as_ref(), "main");
    assert_eq!(
        invocation.compile.profile,
        crate::mir::RawPublishedCompileProfileV1::narrow_v1()
    );
}

#[test]
fn canonical_core_profile_rejects_raw_handoff_without_losing_source_owner() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "canonical-core.hako", "42");
    let rejected = canonical_core_request(path)
        .prepare()
        .expect("profile")
        .read_once()
        .expect("read")
        .parse_once()
        .expect("parse")
        .prepare_raw_vm_handoff()
        .expect_err("canonical-core must not construct a Raw invocation");
    assert_eq!(
        rejected.error(),
        NormalFileVmHandoffErrorV1::ProfileExcludesRawVmReference
    );
    let super::RejectedNormalFileVmHandoffOwnerV1::Prepared(source) = &rejected.owner else {
        panic!("profile rejection must retain the unconsumed prepared source")
    };
    assert!(source.profile_is_canonical_core());
    assert_eq!(source.receipt().read_count, 1);
    assert_eq!(source.receipt().parse_count, 1);
    rejected.discard();
}

#[cfg(feature = "vm-reference")]
#[test]
fn handoff_reuses_the_existing_raw_vm_reference_execution_terminal() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();

    for (name, source, expected_status) in [("first.hako", "42", 42), ("second.hako", "255", 255)] {
        let report = run_file_source(&mut compiler, dir.path(), name, source);
        assert_eq!(report.status_code(), expected_status);
        assert_eq!(report.diagnostic_tag(), None);
    }
}

#[cfg(feature = "vm-reference")]
#[test]
fn script_source_text_matrix_uses_the_front_door() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();
    let cases = [
        ("empty.hako", "", 0, None),
        ("void.hako", "void", 0, None),
        ("integer-zero.hako", "0", 0, None),
        ("integer-max.hako", "255", 255, None),
        (
            "bool.hako",
            "true",
            70,
            Some("[process/unsupported-result]"),
        ),
        (
            "float.hako",
            "1.5",
            70,
            Some("[process/unsupported-result]"),
        ),
        (
            "string.hako",
            "\"raw\"",
            70,
            Some("[process/unsupported-result]"),
        ),
        ("print.hako", "print(1)", 0, None),
        ("local.hako", "local x = 3", 0, None),
        ("assignment.hako", "local x = 1\nx = 3", 0, None),
        ("compound.hako", "local x = 1\nx += 2", 0, None),
        (
            "out-of-range.hako",
            "256",
            70,
            Some("[process/exit-code-out-of-range]"),
        ),
    ];

    for (name, source, expected_status, expected_diagnostic) in cases {
        let report = run_file_source(&mut compiler, dir.path(), name, source);
        assert_eq!(report.status_code(), expected_status, "{name}");
        assert_eq!(report.diagnostic_tag(), expected_diagnostic, "{name}");
    }
}

#[cfg(feature = "vm-reference")]
#[test]
fn function_and_main_shapes_are_observed_before_normal_admission() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();
    let cases = [
        (
            "main-explicit-return.hako",
            "static box Main { main() { return 1 } }",
        ),
        (
            "main-explicit-unit.hako",
            "static box Main { main() { return void } }",
        ),
        ("ordinary-function.hako", "function f() { return 1 }"),
        ("non-main-entry.hako", "static box Decoy { main() {} }"),
    ];
    for (name, source) in cases {
        let error = run_file_source_result(&mut compiler, dir.path(), name, source)
            .expect_err("current NarrowV1 must not admit this normal boundary");
        assert!(
            error.starts_with("[raw-public/eligibility/rejected]"),
            "{name}: {error}"
        );
    }

    let report = run_file_source(
        &mut compiler,
        dir.path(),
        "main-fallthrough.hako",
        "static box Main { main() { 1 } }",
    );
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);

    let helper = run_file_source(
        &mut compiler,
        dir.path(),
        "helper-main.hako",
        "static box Main { helper() {} main() {} }",
    );
    assert_eq!(helper.status_code(), 0);
    assert_eq!(helper.diagnostic_tag(), None);
}
