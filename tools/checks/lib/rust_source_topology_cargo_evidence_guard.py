#!/usr/bin/env python3
"""Freeze CARGO0's disconnected Cargo/rustc evidence boundary."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "rust-source-topology-cargo-evidence"
TOOL = "tools/checks/rust_source_topology"
COMMAND = f"{TOOL}/src/project/cargo/command.rs"
ADAPTER = f"{TOOL}/src/project/cargo/adapter.rs"
ORCHESTRATION = f"{TOOL}/src/project/cargo/orchestration.rs"
PROCESS_MODEL = f"{TOOL}/src/project/cargo/process_model.rs"
RUSTC_CFG = f"{TOOL}/src/project/rustc_cfg.rs"
FINGERPRINT = f"{TOOL}/src/project/fingerprint.rs"
MAIN = f"{TOOL}/src/main.rs"
TESTS = {
    f"{TOOL}/tests/cargo_declared_unit.rs": 7,
    f"{TOOL}/tests/cargo_process.rs": 3,
    f"{TOOL}/tests/cargo_profiles.rs": 2,
}
SELF = "tools/checks/lib/rust_source_topology_cargo_evidence_guard.py"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(source: str, needle: str, expected: int, label: str) -> None:
    actual = source.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def require_absent(source: str, needle: str, label: str) -> None:
    if needle in source:
        fail(f"{label}: forbidden token present: {needle}")


def struct_body(source: str, declaration: str) -> str:
    start = source.find(declaration)
    if start < 0:
        fail(f"missing declaration: {declaration}")
    body_start = source.find("{", start)
    body_end = source.find("}\n", body_start)
    if body_start < 0 or body_end < 0:
        fail(f"malformed declaration: {declaration}")
    return source[body_start + 1 : body_end]


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    command = read(root, COMMAND)
    adapter = read(root, ADAPTER)
    orchestration = read(root, ORCHESTRATION)
    process_model = read(root, PROCESS_MODEL)
    rustc_cfg = read(root, RUSTC_CFG)
    fingerprint = read(root, FINGERPRINT)
    cli = read(root, MAIN)

    require_count(
        adapter,
        "pub fn seal_declared_cargo_unit_v1(",
        1,
        "declared-unit seal owner",
    )
    require_count(
        command,
        "pub fn collect_cargo_metadata_process_evidence_v1(",
        1,
        "cargo metadata process owner",
    )
    require_count(command, "MetadataCommand::new()", 1, "cargo metadata command")
    require_count(command, '.cargo_path("cargo")', 1, "literal cargo executable")
    for option in ("--locked", "--offline", "--filter-platform"):
        require_count(command, f'"{option}"', 1, f"cargo option {option}")
    require_count(
        command,
        'format!("{}/{}", profile.package_name, feature)',
        1,
        "package-qualified feature projection",
    )
    require_absent(command, "no_deps", "dependency-pruned Cargo metadata")

    require_count(
        rustc_cfg,
        "pub fn collect_rustc_cfg_probe_v1(",
        1,
        "rustc cfg probe owner",
    )
    require_count(rustc_cfg, 'Command::new("rustc")', 1, "literal rustc executable")
    require_count(
        fingerprint,
        "pub fn collect_workspace_input_fingerprints_v1(",
        1,
        "workspace fingerprint owner",
    )
    require_count(
        orchestration,
        "pub fn collect_declared_cargo_unit_process_evidence_v1(",
        1,
        "complete process-evidence owner",
    )

    durable = struct_body(
        orchestration,
        "pub struct CargoDeclaredUnitProcessEvidenceV1",
    )
    for field_type in (
        "CargoDeclaredCompileUnitEvidenceV1",
        "CargoMetadataInvocationEvidenceV1",
        "RustcCfgProbeEvidenceV1",
        "WorkspaceInputFingerprintsV1",
    ):
        require_count(durable, field_type, 1, f"durable field {field_type}")
    for forbidden in (
        "CargoMetadataSnapshotV1",
        "CargoMetadataProcessEvidenceV1",
        "PackageId",
        "PathBuf",
        "selected_manifest_path_observation",
    ):
        require_absent(durable, forbidden, "durable process evidence boundary")

    cargo_sources = command + adapter + orchestration + process_model
    for forbidden in ("FINALIZE0", "MirBuilder", "include!(", "mod traversal"):
        require_absent(cargo_sources, forbidden, "Cargo evidence semantic boundary")
    for forbidden in (
        "collect_declared_cargo_unit_process_evidence_v1",
        "collect_cargo_metadata_process_evidence_v1",
        "collect_rustc_cfg_probe_v1",
    ):
        require_absent(cli, forbidden, "project CLI connection before S0b-G0")

    for test_file, expected in TESTS.items():
        require_count(read(root, test_file), "#[test]", expected, f"{test_file} test inventory")

    guarded = [
        COMMAND,
        ADAPTER,
        ORCHESTRATION,
        PROCESS_MODEL,
        RUSTC_CFG,
        FINGERPRINT,
        MAIN,
        *TESTS,
        SELF,
    ]
    oversized = [
        relative
        for relative in guarded
        if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok declared_unit=1 cargo_process=1 rustc_probe=1 "
        "workspace_fingerprint=1 durable_raw_snapshot=0 cli_consumers=0 "
        "tests=12"
    )


if __name__ == "__main__":
    main()
