use super::*;
use crate::parser::ParserNormalRawVmSourceKindV1;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write atomic root fixture");
    path
}

fn parse_raw(path: PathBuf) -> PreparedNormalFileSourceV1 {
    NormalFileVmFrontDoorV1::file_no_import_request(path)
        .prepare()
        .expect("raw profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one parser product")
}

fn parse_canonical(path: PathBuf) -> PreparedNormalFileSourceV1 {
    NormalFileVmFrontDoorV1::file_canonical_core_request(path)
        .prepare()
        .expect("canonical profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one parser product")
}

fn into_selected_raw_source(
    parsed: PreparedNormalFileSourceV1,
) -> raw_source_handoff::PreparedCanonicalParserRawSourceV1 {
    let PreparedNormalFileSourceV1 { route, _seal } = parsed;
    drop(_seal);
    match route {
        PreparedNormalFileParsedRouteV1::Raw {
            source_file,
            source,
        } => {
            drop(source_file);
            source
        }
        PreparedNormalFileParsedRouteV1::Canonical {
            source_file,
            source,
        } => {
            drop(source_file);
            source.discard_at_wrong_route_terminal();
            panic!("Raw profile selected the canonical route")
        }
    }
}

#[test]
fn raw_route_closes_parser_rows_without_canonical_co_seal() {
    let dir = tempdir().expect("tempdir");
    let parsed = parse_raw(write_source(dir.path(), "raw.hako", "42"));
    let extracted = into_selected_raw_source(parsed)
        .extract_once()
        .expect("source-backed Raw extraction closes the Script sibling");
    assert_eq!(
        extracted.kind(),
        ParserNormalRawVmSourceKindV1::SourceBacked
    );
    extracted.discard();
}

#[test]
fn raw_compatibility_route_extracts_once_only_after_typed_absence() {
    let dir = tempdir().expect("tempdir");
    let parsed = parse_raw(write_source(
        dir.path(),
        "raw-compat.hako",
        "interface box Api { run() }",
    ));
    let extracted = into_selected_raw_source(parsed)
        .extract_once()
        .expect("explicit compatibility absence authorizes one Raw extraction");
    assert_eq!(
        extracted.kind(),
        ParserNormalRawVmSourceKindV1::Compatibility
    );
    extracted.discard();
}

#[test]
fn raw_source_failures_never_become_compatibility_extractions() {
    let dir = tempdir().expect("tempdir");
    for (name, source, expected) in [
        (
            "raw-unavailable.hako",
            "gate Build.release { box Enabled { run() {} } } else { box Disabled { run() {} } }",
            NormalFileVmHandoffErrorV1::SourceAuthorityUnavailable,
        ),
        (
            "raw-incomplete.hako",
            "static box Main { helper() {} }",
            NormalFileVmHandoffErrorV1::SourceIncomplete,
        ),
        (
            "raw-invalid.hako",
            "static box Main { main() {} }\nstatic box Main { main() {} }",
            NormalFileVmHandoffErrorV1::SourceIntegrityInvalid,
        ),
    ] {
        let rejected = parse_raw(write_source(dir.path(), name, source))
            .prepare_raw_vm_handoff()
            .expect_err("typed source failure cannot enter compatibility extraction");
        assert_eq!(rejected.error(), expected);
        rejected.discard();
    }
}

#[test]
fn raw_route_is_a_typed_reject_at_the_canonical_policy_boundary() {
    let dir = tempdir().expect("tempdir");
    let rejected = parse_raw(write_source(dir.path(), "raw-policy.hako", "42"))
        .prepare_source_plan_request()
        .expect_err("Raw must not construct canonical source-plan input");
    assert_eq!(
        rejected.error(),
        NormalFileSourcePlanRouteErrorV1::ProfileExcludesCanonicalCore
    );
    rejected.discard();
}

#[test]
fn canonical_route_is_a_typed_reject_at_the_raw_boundary() {
    let dir = tempdir().expect("tempdir");
    let rejected = parse_canonical(write_source(dir.path(), "canonical-raw.hako", "42"))
        .prepare_raw_vm_handoff()
        .expect_err("canonical source must not retry through Raw");
    assert_eq!(
        rejected.error(),
        NormalFileVmHandoffErrorV1::ProfileExcludesRawVmReference
    );
    rejected.discard();
}
