#!/usr/bin/env python3
"""Validate the Generic legacy corpus universe P0 manifest.

This checker owns inventory shape only.  It deliberately does not inspect or
select a runtime route, and it does not turn fixture names into semantic facts.
"""

from __future__ import annotations

import csv
import json
import pathlib
import re
import sys
from dataclasses import dataclass


HEADER = (
    "record_kind",
    "id",
    "canonical_fixture",
    "corpus",
    "mode",
    "profile",
    "alias_of",
    "observation_state",
    "current_acceptance",
    "observed_route",
    "nested_bypass",
    "source_surface",
    "disposition",
    "target_owner",
    "decision",
    "parity_gate",
    "retention_row",
    "symbol",
    "current_role",
    "production_callers",
    "test_callers",
    "first_effect",
    "cutover_action",
    "retire_row",
    "replacement_owner",
)

SENTINEL = "-"
OBSERVATIONS = {"unobserved", "accepted", "rejected", "failed-before-loop", "timeout"}
DISPOSITIONS = {"portable-owner", "accepted-typed-reject", "nonproduction-future-evidence"}
ACCEPTANCE = {"unknown", "accepted", "rejected"}
CASE_CORPORA = {"phase29bq", "selfhost", "generic-fixture", "generic-smoke"}
CASE_MODES = {"fast-gate", "selfhost-subset", "fixture-inventory", "release-adopt", "strict-shadow", "compat-alias"}
CASE_DECISION = "P0-INVENTORY-ONLY"
CASE_RETENTION = "GENERIC-LEGACY-CORPUS-UNIVERSE-P0"
UNKNOWN = "unknown"
FRONT_STATES = {"loop-reached", "failed-before-loop", "timeout", "spawn-error"}
FRONT_CLAIMS = {"loop-not-reached", "route-unobserved", "disposition-unclassified", "production-unchanged"}


class ManifestError(ValueError):
    """A deterministic manifest contract failure."""


@dataclass(frozen=True)
class Record:
    values: dict[str, str]
    line: int


def _fail(path: pathlib.Path, line: int, message: str) -> ManifestError:
    return ManifestError(f"{path}:{line}: {message}")


def _require(value: str, field: str, path: pathlib.Path, line: int) -> None:
    if not value:
        raise _fail(path, line, f"{field} must not be empty; use {SENTINEL!r}")


def _check_case(record: Record, root: pathlib.Path, ids: set[str]) -> None:
    value = record.values
    path = pathlib.Path(value["canonical_fixture"])
    if value["corpus"] not in CASE_CORPORA:
        raise _fail(root, record.line, f"unknown case corpus {value['corpus']!r}")
    if value["mode"] not in CASE_MODES:
        raise _fail(root, record.line, f"unknown case mode {value['mode']!r}")
    fixture_exists = path.is_file() if path.is_absolute() else (root / path).is_file()
    if not fixture_exists:
        raise _fail(root, record.line, f"canonical fixture is missing: {path}")
    if value["observation_state"] not in OBSERVATIONS:
        raise _fail(root, record.line, f"invalid observation state {value['observation_state']!r}")
    if value["current_acceptance"] not in ACCEPTANCE:
        raise _fail(root, record.line, f"invalid current acceptance {value['current_acceptance']!r}")
    if value["disposition"] not in DISPOSITIONS:
        raise _fail(root, record.line, f"invalid disposition {value['disposition']!r}")
    if value["decision"] != CASE_DECISION:
        raise _fail(root, record.line, "P0 case decision must remain inventory-only")
    if value["retention_row"] != CASE_RETENTION:
        raise _fail(root, record.line, "P0 case retention row drift")
    if value["current_acceptance"] == "accepted":
        raise _fail(root, record.line, "P0 must not claim an accepted case")
    if value["disposition"] != "nonproduction-future-evidence":
        raise _fail(root, record.line, "P0 cases must retain future evidence only")
    if value["nested_bypass"] not in {SENTINEL, UNKNOWN}:
        raise _fail(root, record.line, "P0 nested-bypass state must be unknown or sentinel")
    if value["parity_gate"] != "not-run":
        raise _fail(root, record.line, "P0 parity gate must remain not-run")
    for field in (
        "observed_route",
        "target_owner",
        "symbol",
        "current_role",
        "production_callers",
        "test_callers",
        "first_effect",
        "cutover_action",
        "retire_row",
        "replacement_owner",
    ):
        if value[field] != SENTINEL:
            raise _fail(root, record.line, f"P0 case field {field} must be {SENTINEL!r}")
    alias = value["alias_of"]
    if alias != SENTINEL and alias not in ids:
        raise _fail(root, record.line, f"alias target is not a canonical case: {alias}")


def _check_edge(record: Record, root: pathlib.Path) -> None:
    value = record.values
    if value["id"] == SENTINEL or value["canonical_fixture"] == SENTINEL:
        raise _fail(root, record.line, "edge id and path are required")
    for field in ("corpus", "mode", "profile", "alias_of", "observation_state", "current_acceptance", "observed_route", "nested_bypass", "source_surface", "disposition", "target_owner", "decision", "parity_gate", "retention_row"):
        if value[field] != SENTINEL:
            raise _fail(root, record.line, f"edge case field {field} must use the documented sentinel")
    for field in ("symbol", "current_role", "production_callers", "test_callers", "first_effect", "cutover_action", "retire_row", "replacement_owner"):
        _require(value[field], field, root, record.line)


def _source_rows(path: pathlib.Path) -> list[tuple[int, str, str, str, str]]:
    result: list[tuple[int, str, str, str, str]] = []
    with path.open(newline="") as stream:
        for line, raw in enumerate(stream, start=1):
            if not raw.strip() or raw.lstrip().startswith("#"):
                continue
            row = next(csv.reader([raw], delimiter="\t"))
            if path.name == "phase29bq_fast_gate_cases.tsv":
                if len(row) == 6:
                    fixture = row[0]
                    expected, allowed, planner, case_id, reason = row[1:]
                    corpus = "selfhost" if "selfhost" in f"{fixture} {case_id} {reason}".lower() else ("generic-fixture" if "generic" in f"{fixture} {case_id} {reason}".lower() else "phase29bq")
                elif len(row) == 7 and row[5] == "selfhost":
                    fixture = row[0]
                    expected, allowed, planner, case_id, _, reason = row[1:]
                    corpus = "selfhost"
                else:
                    raise ManifestError(f"{path}:{line}: unexpected legacy row width {len(row)}")
                mode = "fast-gate"
                profile = f"expected={expected};allowed_rc={allowed};planner={planner}"
            else:
                if len(row) not in {5, 6}:
                    raise ManifestError(f"{path}:{line}: unexpected selfhost row width {len(row)}")
                fixture = row[0]
                expected, allowed, planner, reason = row[1:5]
                filter_alias = row[5] if len(row) == 6 else ""
                profile = f"expected={expected};allow_rc={allowed};planner={planner}"
                if filter_alias:
                    profile += f";filter_alias={filter_alias}"
                corpus = "selfhost"
                mode = "selfhost-subset"
            result.append((line, fixture, corpus, mode, profile))
    return result


def _check_source_inventory(records: list[Record], repo_root: pathlib.Path, manifest: pathlib.Path) -> None:
    cases = [record for record in records if record.values["record_kind"] == "case"]
    by_source = {record.values["source_surface"]: record for record in cases}
    if len(by_source) != len(cases):
        raise ManifestError(f"{manifest}: duplicate source provenance")
    for relative in (
        "tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv",
        "tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv",
    ):
        source_path = repo_root / relative
        expected = _source_rows(source_path)
        prefix = "phase29bq_fast_gate_cases.tsv:" if source_path.name == "phase29bq_fast_gate_cases.tsv" else "selfhost/planner_required_selfhost_subset.tsv:"
        selected = []
        for line, fixture, corpus, mode, profile in expected:
            key = f"{prefix}{line}"
            record = by_source.get(key)
            if record is None:
                raise ManifestError(f"{manifest}: missing source row {relative}:{line}")
            if record.values["canonical_fixture"] != fixture:
                raise ManifestError(f"{manifest}: source fixture drift at {key}")
            if (record.values["corpus"], record.values["mode"], record.values["profile"]) != (corpus, mode, profile):
                raise ManifestError(f"{manifest}: source profile drift at {key}")
            selected.append(key)
        actual = [key for key in by_source if key.startswith(prefix)]
        if sorted(actual) != sorted(selected):
            raise ManifestError(f"{manifest}: source row universe drift for {relative}")
    generic_paths = {str(path.relative_to(repo_root)) for path in (repo_root / "apps/tests").glob("*generic*loop*.hako")}
    generic_paths.add("apps/tests/selfhost_trim_generic_loop_min.hako")
    manifest_paths = {record.values["canonical_fixture"] for record in cases}
    missing = sorted(generic_paths - manifest_paths)
    if missing:
        raise ManifestError(f"{manifest}: Generic-named fixture missing from universe: {missing[0]}")
    required_ids = {
        "generic_loop_continue_release_adopt_vm",
        "generic_loop_continue_strict_shadow_vm",
        "generic_loop_in_body_step_release_adopt_vm",
        "generic_loop_in_body_step_strict_shadow_vm",
        "phase29ca_generic_loop_continue_release_adopt_vm",
        "phase29ca_generic_loop_continue_strict_shadow_vm",
        "phase29cb_generic_loop_in_body_step_release_adopt_vm",
        "phase29cb_generic_loop_in_body_step_strict_shadow_vm",
    }
    actual_ids = {record.values["id"] for record in cases}
    if required_ids - actual_ids:
        missing = sorted(required_ids - actual_ids)[0]
        raise ManifestError(f"{manifest}: required Generic smoke record missing: {missing}")


def validate_manifest(manifest: pathlib.Path, repo_root: pathlib.Path) -> list[Record]:
    with manifest.open(newline="") as stream:
        rows = list(csv.reader(stream, delimiter="\t"))
    if not rows or tuple(rows[0]) != HEADER:
        raise ManifestError(f"{manifest}: header drift")
    records: list[Record] = []
    keys: set[tuple[str, str, str]] = set()
    ids: set[str] = set()
    for line, row in enumerate(rows[1:], start=2):
        if not row or all(cell.startswith("#") for cell in row):
            continue
        if len(row) != len(HEADER):
            raise _fail(manifest, line, f"expected {len(HEADER)} fields, got {len(row)}")
        value = dict(zip(HEADER, row))
        for field, cell in value.items():
            _require(cell, field, manifest, line)
        kind = value["record_kind"]
        if kind not in {"case", "edge"}:
            raise _fail(manifest, line, f"unknown record kind {kind!r}")
        if kind == "case":
            key = (value["id"], value["mode"], value["profile"])
            if key in keys:
                raise _fail(manifest, line, f"duplicate case/mode/profile key {key}")
            keys.add(key)
            if value["id"] in ids:
                raise _fail(manifest, line, f"duplicate case id {value['id']}")
            ids.add(value["id"])
        records.append(Record(value, line))
    if not records:
        raise ManifestError(f"{manifest}: empty manifest")
    for record in records:
        if record.values["record_kind"] == "case":
            _check_case(record, repo_root, ids)
        else:
            _check_edge(record, manifest)
    _check_source_inventory(records, repo_root, manifest)
    canonical = {record.values["id"] for record in records if record.values["record_kind"] == "case" and record.values["alias_of"] == SENTINEL}
    aliases = [record for record in records if record.values["record_kind"] == "case" and record.values["alias_of"] != SENTINEL]
    if not canonical:
        raise ManifestError(f"{manifest}: no canonical case records")
    for record in aliases:
        target = next(item for item in records if item.values["id"] == record.values["alias_of"])
        if target.values["record_kind"] != "case" or target.values["alias_of"] != SENTINEL:
            raise _fail(manifest, record.line, "compatibility alias must target a canonical case")
        if record.values["canonical_fixture"] != target.values["canonical_fixture"]:
            raise _fail(manifest, record.line, "compatibility alias fixture differs from canonical case")
    return records


def validate_front_receipt(receipt: pathlib.Path, manifest: pathlib.Path, repo_root: pathlib.Path) -> None:
    try:
        value = json.loads(receipt.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ManifestError(f"{receipt}: invalid JSON: {error}") from error
    required = {
        "schema_version", "kind", "row", "case_id", "canonical_case_id",
        "canonical_fixture", "invocation_fixture", "mode", "profile", "invocation_profile_id", "working_directory",
        "command_argv", "timeout_seconds", "front_state", "exit_code",
        "first_front_evidence", "pre_loop_owner", "failure_phase",
        "stdout_digest", "stderr_digest", "claims", "next_repair_row",
    }
    if set(value) != required:
        raise ManifestError(f"{receipt}: receipt field set drift")
    if value["schema_version"] != "generic-legacy-observation-front-v1" or value["kind"] != "GenericLegacyObservationFrontReceiptV1":
        raise ManifestError(f"{receipt}: receipt identity drift")
    if value["row"] != "GENERIC-LEGACY-OBSERVATION-FRONT-G0":
        raise ManifestError(f"{receipt}: wrong observation row")
    records = validate_manifest(manifest, repo_root)
    case = next((record for record in records if record.values["record_kind"] == "case" and record.values["id"] == value["case_id"]), None)
    if case is None or case.values["alias_of"] != SENTINEL:
        raise ManifestError(f"{receipt}: case_id must resolve to a canonical manifest case")
    for field in ("canonical_fixture", "mode", "profile"):
        if value[field] != case.values[field]:
            raise ManifestError(f"{receipt}: {field} does not match the manifest case")
    if value["canonical_case_id"] != value["case_id"]:
        raise ManifestError(f"{receipt}: canonical_case_id must equal the non-alias case")
    if value["invocation_profile_id"] != "vm-strict-planner-direct-v1":
        raise ManifestError(f"{receipt}: invocation profile is not the fixed direct VM profile")
    argv = value["command_argv"]
    if not isinstance(argv, list) or not argv or any(not isinstance(item, str) or not item for item in argv):
        raise ManifestError(f"{receipt}: command_argv must be a non-empty string list")
    invocation_fixture = pathlib.Path(value["invocation_fixture"])
    if any(item.endswith(".sh") for item in argv) or value["invocation_fixture"] not in argv:
        raise ManifestError(f"{receipt}: command must directly invoke the recorded fixture, not a smoke wrapper")
    if not (repo_root / invocation_fixture).is_file():
        raise ManifestError(f"{receipt}: invocation fixture is missing")
    executable = next((item for item in argv if item.startswith("target/") and item.endswith("/hakorune")), None)
    if executable is None or not (repo_root / executable).is_file():
        raise ManifestError(f"{receipt}: direct Hakorune executable is missing")
    if value["working_directory"] != "." or value["timeout_seconds"] != 10:
        raise ManifestError(f"{receipt}: front invocation profile drift")
    state = value["front_state"]
    if state not in FRONT_STATES:
        raise ManifestError(f"{receipt}: unknown front state {state!r}")
    evidence = value["first_front_evidence"]
    if not isinstance(evidence, dict) or set(evidence) != {"kind", "token", "source"}:
        raise ManifestError(f"{receipt}: first front evidence shape drift")
    if state == "failed-before-loop":
        if value["exit_code"] != 1 or value["pre_loop_owner"] != "src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1(BinaryOp)":
            raise ManifestError(f"{receipt}: failed-before-loop receipt must retain the observed owner and exit")
        if evidence["token"] != "[freeze:contract][raw-structured/unconsumed-demands]":
            raise ManifestError(f"{receipt}: diagnostic token drift")
        if evidence["source"] != "src/mir/builder/raw_structured_child_scope.rs:108":
            raise ManifestError(f"{receipt}: diagnostic source drift")
        if value["next_repair_row"] != "GENERIC-RAW-STRUCTURED-DEMANDS-REPAIR-S0-D0":
            raise ManifestError(f"{receipt}: missing actual-owner repair row")
    elif state == "loop-reached":
        if value["exit_code"] != 0 or value["pre_loop_owner"] != SENTINEL:
            raise ManifestError(f"{receipt}: loop-reached receipt cannot retain a pre-loop owner")
    elif state == "timeout":
        if value["exit_code"] != 124:
            raise ManifestError(f"{receipt}: timeout receipt must use exit 124")
    else:
        if value["pre_loop_owner"] == SENTINEL:
            raise ManifestError(f"{receipt}: spawn-error must retain an owner")
    if set(value["claims"]) != FRONT_CLAIMS:
        raise ManifestError(f"{receipt}: claim set must stay observation-only")
    digest_pattern = re.compile(r"^[0-9a-f]{64}$")
    for field in ("stdout_digest", "stderr_digest"):
        if not isinstance(value[field], str) or not digest_pattern.fullmatch(value[field]):
            raise ManifestError(f"{receipt}: {field} must be a sha256 digest")


def main(argv: list[str]) -> int:
    if len(argv) not in {3, 4}:
        print(f"usage: {argv[0]} MANIFEST REPO_ROOT [OBSERVATION_RECEIPT]", file=sys.stderr)
        return 2
    try:
        records = validate_manifest(pathlib.Path(argv[1]), pathlib.Path(argv[2]))
        if len(argv) == 4:
            validate_front_receipt(pathlib.Path(argv[3]), pathlib.Path(argv[1]), pathlib.Path(argv[2]))
    except (OSError, ManifestError) as error:
        print(f"[generic-legacy-corpus] FAIL: {error}", file=sys.stderr)
        return 1
    cases = sum(record.values["record_kind"] == "case" for record in records)
    edges = sum(record.values["record_kind"] == "edge" for record in records)
    aliases = sum(record.values["record_kind"] == "case" and record.values["alias_of"] != SENTINEL for record in records)
    print(f"[generic-legacy-corpus] OK cases={cases} aliases={aliases} edges={edges}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
