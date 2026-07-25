//! RESULT-CARRIER-NORMAL-CAPABILITY0 S2 source-text evidence.
//!
//! This module observes the sealed front-door boundary.  It deliberately does
//! not classify AST results or construct source/process results itself.

use super::*;
use tempfile::tempdir;

fn request(path: PathBuf) -> NormalFileRequestV1 {
    NormalFileVmFrontDoorV1::file_no_import_request(path)
}

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write source fixture");
    path
}

#[cfg(feature = "vm-reference")]
fn run_source(
    compiler: &mut crate::mir::MirCompiler,
    dir: &Path,
    name: &str,
    source: &str,
) -> Result<crate::mir::RawVmReferenceRunReportV1, String> {
    let invocation = request(write_source(dir, name, source))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_raw_vm_handoff()
        .into_raw_vm_reference_invocation();
    compiler.run_raw_vm_reference_v1(invocation)
}

#[cfg(feature = "vm-reference")]
#[test]
fn void_and_null_are_observed_but_not_provenance_credited() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();

    for (name, source) in [("void.hako", "void"), ("null.hako", "null")] {
        let report = run_source(&mut compiler, dir.path(), name, source)
            .expect("current Scalar-and-Unit lane observes this source form");
        assert_eq!(report.status_code(), 0, "{name}");
        assert_eq!(report.diagnostic_tag(), None, "{name}");
    }
}

#[cfg(feature = "vm-reference")]
#[test]
fn annotations_and_callable_returns_remain_raw_capability_rejections() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();
    let cases = [
        ("main-i64.hako", "static box Main { main(): i64 {} }"),
        (
            "helper-i64.hako",
            "static box Main { helper(): i64 {} main() {} }",
        ),
        (
            "main-return.hako",
            "static box Main { main() { return 1 } }",
        ),
        ("ordinary.hako", "function f() { return 1 }"),
    ];

    for (name, source) in cases {
        let error = run_source(&mut compiler, dir.path(), name, source)
            .expect_err("S2 must not admit this function-exit capability");
        assert!(
            error.starts_with("[raw-public/eligibility/rejected]"),
            "{name}: {error}"
        );
    }
}

#[cfg(feature = "vm-reference")]
#[test]
fn owner_bearing_source_is_rejected_before_vm_result_decoding() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();
    let error = run_source(
        &mut compiler,
        dir.path(),
        "new-map.hako",
        "new MapBox()",
    )
    .expect_err("the first normal profile has no owner-bearing result carrier");

    assert!(
        error.starts_with("[raw-public/eligibility/rejected]"),
        "{error}"
    );
}

#[cfg(feature = "vm-reference")]
#[test]
fn front_door_rejections_leave_the_compiler_reusable() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();

    let profile_rejection = request(PathBuf::new())
        .prepare()
        .expect_err("empty path rejects before file read");
    profile_rejection.discard();
    assert_eq!(
        run_source(&mut compiler, dir.path(), "after-profile.hako", "42")
            .expect("profile rejection must not poison compiler")
            .status_code(),
        42
    );

    for (name, source) in [("parse.hako", "@"), ("using.hako", "using foo")] {
        let rejected = request(write_source(dir.path(), name, source))
            .prepare()
            .expect("profile")
            .read_once()
            .expect("one read")
            .parse_once()
            .expect_err("source must reject before Raw handoff");
        rejected.discard();
        assert_eq!(
            run_source(&mut compiler, dir.path(), "after-source.hako", "255")
                .expect("source rejection must not poison compiler")
                .status_code(),
            255
        );
    }

    run_source(
        &mut compiler,
        dir.path(),
        "rejected-main-return.hako",
        "static box Main { main() { return 1 } }",
    )
    .expect_err("Raw compile rejection remains a rejection");
    assert_eq!(
        run_source(&mut compiler, dir.path(), "after-compile.hako", "1")
            .expect("Raw rejection must not poison compiler")
            .status_code(),
        1
    );
}

#[cfg(feature = "vm-reference")]
#[test]
fn canonical_process_and_vm_faults_leave_the_compiler_reusable() {
    let dir = tempdir().expect("tempdir");
    let mut compiler = crate::mir::MirCompiler::new();

    let unsupported = run_source(&mut compiler, dir.path(), "bool.hako", "true")
        .expect("Bool is a source result and faults only at process projection");
    assert_eq!(unsupported.status_code(), 70);
    assert_eq!(
        unsupported.diagnostic_tag(),
        Some("[process/unsupported-result]")
    );
    assert_eq!(
        run_source(&mut compiler, dir.path(), "after-process-fault.hako", "42")
            .expect("canonical process fault must not poison compiler")
            .status_code(),
        42
    );

    let division = run_source(&mut compiler, dir.path(), "division.hako", "1 / 0")
        .expect("VM fault remains a normal process terminal");
    assert_eq!(division.status_code(), 70);
    assert_eq!(division.diagnostic_tag(), Some("[process/source-fault]"));
    assert_eq!(
        run_source(&mut compiler, dir.path(), "after-vm-fault.hako", "")
            .expect("VM execution fault must not poison compiler")
            .status_code(),
        0
    );
}
