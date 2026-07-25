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
