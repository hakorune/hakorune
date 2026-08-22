#!/usr/bin/env python3
"""Guard the G3 llvmlite keep-lane inventory without changing the route."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = ROOT / (
    "docs/development/current/main/investigations/"
    "llvmlite-keep0-ret0-inventory-v0.json"
)
SOURCE_MANIFESTS = (
    ("llvmlite-production-ingress-census-v0.json", "g0"),
    ("llvmlite-default-independence-census-v0.json", "g2"),
    ("llvmlite-shared-smoke-caller-census-v0.json", "smoke"),
)
ALLOWED_CLASSIFICATIONS = {
    "retain_keep",
    "convert_to_fixture",
    "archive_candidate",
    "reference_only",
    "blocked",
}
TAG = "llvm-llvmlite-keep-inventory-guard"


def fail(message: str) -> None:
    print(f"[{TAG}] FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_json(path: Path) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing manifest: {path}")
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"manifest root must be an object: {path}")
    return value


def git_paths(*pathspecs: str) -> set[str]:
    result = subprocess.run(
        ["git", "ls-files", *pathspecs],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        fail(f"git ls-files failed: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line}


def validate_classification(row: dict[str, Any], label: str) -> None:
    classification = row.get("classification")
    if classification not in ALLOWED_CLASSIFICATIONS:
        fail(f"{label}: invalid classification {classification!r}")


def validate_root_rows(
    rows: Any,
    expected: set[str],
    label: str,
    required_kind: str,
) -> None:
    if not isinstance(rows, list) or not rows:
        fail(f"{label}: rows are missing")
    seen: set[str] = set()
    for row in rows:
        if not isinstance(row, dict):
            fail(f"{label}: row is not an object")
        path = row.get("path")
        if not isinstance(path, str) or not path:
            fail(f"{label}: path is missing")
        if path in seen:
            fail(f"{label}: duplicate path {path}")
        if path not in expected:
            fail(f"{label}: unexpected path {path}")
        if row.get("kind") != required_kind:
            fail(f"{label}:{path}: kind drifted")
        validate_classification(row, f"{label}:{path}")
        if row.get("archive_status") not in {"not_started", "archived", "blocked"}:
            fail(f"{label}:{path}: archive_status drifted")
        seen.add(path)
    if seen != expected:
        fail(f"{label}: exact path set drifted: {sorted(expected ^ seen)}")


def expected_consumer_rows() -> dict[str, dict[str, Any]]:
    expected: dict[str, dict[str, Any]] = {}
    for filename, prefix in SOURCE_MANIFESTS:
        source = read_json(
            ROOT / "docs/development/current/main/investigations" / filename
        )
        rows = source.get("rows")
        if not isinstance(rows, list) or not rows:
            fail(f"{filename}: source rows are missing")
        for row in rows:
            if not isinstance(row, dict) or not isinstance(row.get("id"), str):
                fail(f"{filename}: invalid source row")
            row_id = f"{prefix}:{row['id']}"
            if row_id in expected:
                fail(f"source consumer row duplicated: {row_id}")
            expected[row_id] = {
                "owner": row.get("owner", row.get("path")),
                "class": row.get("class"),
                "source_manifest": filename,
            }
    return expected


def validate_consumers(rows: Any, expected: dict[str, dict[str, Any]]) -> None:
    if not isinstance(rows, list) or not rows:
        fail("consumer_rows are missing")
    seen: set[str] = set()
    for row in rows:
        if not isinstance(row, dict):
            fail("consumer row is not an object")
        row_id = row.get("row_id")
        if not isinstance(row_id, str) or not row_id:
            fail("consumer row_id is missing")
        if row_id in seen:
            fail(f"duplicate consumer row_id: {row_id}")
        if row_id not in expected:
            fail(f"unexpected consumer row_id: {row_id}")
        source = expected[row_id]
        if row.get("source_manifest") != source["source_manifest"]:
            fail(f"{row_id}: source manifest drifted")
        if row.get("owner") != source["owner"]:
            fail(f"{row_id}: owner drifted")
        if row.get("class") != source["class"]:
            fail(f"{row_id}: class drifted")
        if not isinstance(row.get("owner"), str) or not row["owner"]:
            fail(f"{row_id}: owner is missing")
        if not (ROOT / row["owner"]).exists():
            fail(f"{row_id}: owner path is missing: {row['owner']}")
        validate_classification(row, row_id)
        seen.add(row_id)
    if seen != set(expected):
        fail(f"consumer row universe drifted: {sorted(set(expected) ^ seen)}")


def validate_fixtures(rows: Any, allowed_paths: set[str]) -> None:
    if not isinstance(rows, list) or not rows:
        fail("fixture_golden_candidates are missing")
    seen: set[str] = set()
    required = {
        "path",
        "category",
        "classification",
        "independent_oracle_status",
        "expected_output_exit_evidence",
    }
    for row in rows:
        if not isinstance(row, dict) or not required <= row.keys():
            fail("fixture candidate is missing required fields")
        path = row["path"]
        if not isinstance(path, str) or path in seen:
            fail(f"fixture candidate path is missing or duplicated: {path!r}")
        if path not in allowed_paths or not (ROOT / path).exists():
            fail(f"fixture candidate path is not tracked: {path}")
        validate_classification(row, f"fixture:{path}")
        if not isinstance(row["category"], str) or not row["category"]:
            fail(f"fixture:{path}: category is empty")
        if not isinstance(row["independent_oracle_status"], str):
            fail(f"fixture:{path}: oracle status is invalid")
        seen.add(path)


def validate_artifacts(rows: Any) -> None:
    expected_platforms = {"linux", "windows", "macos", "ios"}
    if not isinstance(rows, list) or {row.get("platform") for row in rows} != expected_platforms:
        fail("artifact platform matrix drifted")
    for row in rows:
        required = {"platform", "target", "artifact_path", "checksum", "provenance", "status"}
        if not isinstance(row, dict) or not required <= row.keys():
            fail("artifact row is missing required fields")
        if row["artifact_path"] is None:
            if row["checksum"] != "unavailable" or row["provenance"] != "unavailable":
                fail(f"{row['platform']}: unavailable artifact lacks explicit gap state")
            if row["status"] != "blocked":
                fail(f"{row['platform']}: unavailable artifact is not blocked")


def validate_restore(rows: Any) -> None:
    if not isinstance(rows, list) or not rows:
        fail("restore_entries are missing")
    ids: set[str] = set()
    for row in rows:
        if not isinstance(row, dict) or not {"id", "path", "command", "status"} <= row.keys():
            fail("restore row is missing required fields")
        if row["id"] in ids or not isinstance(row["command"], str) or not row["command"]:
            fail(f"restore row id/command is invalid: {row.get('id')!r}")
        if not (ROOT / row["path"]).exists():
            fail(f"restore path is missing: {row['path']}")
        ids.add(row["id"])


def validate_archive_fields(fields: Any) -> None:
    required = {
        "archive_owner",
        "archive_uri",
        "archive_tag",
        "source_tree_or_commit",
        "artifact_checksums",
        "restore_command",
        "deletion_approval",
    }
    if not isinstance(fields, dict) or not required <= fields.keys():
        fail("archive_decision_fields are incomplete")
    uri = fields["archive_uri"]
    if uri is not None and not all(fields.get(key) for key in ("archive_owner", "archive_tag")):
        fail("archive URI requires an owner and tag")
    commit = fields["source_tree_or_commit"]
    if not isinstance(commit, str) or not commit:
        fail("source_tree_or_commit is missing")
    result = subprocess.run(
        ["git", "cat-file", "-e", f"{commit}^{{commit}}"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        fail(f"source_tree_or_commit is not a repository commit: {commit}")


def validate(path: Path) -> None:
    data = read_json(path)
    if data.get("schema") != "llvmlite-keep0-ret0-inventory-v0":
        fail("inventory schema drifted")
    if data.get("status") not in {"inventory-baseline-blocked", "inventory-complete"}:
        fail("inventory status is not an accepted state")
    if data.get("source_deletion") or data.get("production_switch") or data.get("fallback"):
        fail("inventory may not claim deletion, switch, or fallback")

    expected_sources = git_paths("src/llvm_py", "tools/llvmlite_harness.py")
    expected_support = git_paths(
        "tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep",
        "tools/historical/pyvm/pyvm_vs_llvmlite.sh",
    )
    validate_root_rows(data.get("source_roots"), expected_sources, "source_roots", "tracked_keep_root")
    validate_root_rows(data.get("support_roots"), expected_support, "support_roots", "restore_or_monitor_support")
    expected_consumers = expected_consumer_rows()
    validate_consumers(data.get("consumer_rows"), expected_consumers)
    validate_fixtures(data.get("fixture_golden_candidates"), expected_sources | expected_support)
    validate_artifacts(data.get("artifact_matrix"))
    validate_restore(data.get("restore_entries"))
    validate_archive_fields(data.get("archive_decision_fields"))

    counts = data.get("counts")
    expected_counts = {
        "tracked_keep_root_paths": len(expected_sources),
        "support_restore_paths": len(expected_support),
        "consumer_rows": len(expected_consumers),
        "consumer_row_id_duplicates": 0,
        "fixture_golden_candidates": len(data["fixture_golden_candidates"]),
    }
    if counts != expected_counts:
        fail(f"inventory counts drifted: expected {expected_counts}, got {counts}")
    print(
        f"[{TAG}] ok (roots={len(expected_sources)}, support={len(expected_support)}, "
        f"consumers={len(expected_consumers)}, fixtures={expected_counts['fixture_golden_candidates']}, "
        "artifacts=4, restore=3)"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    args = parser.parse_args()
    manifest = args.manifest if args.manifest.is_absolute() else ROOT / args.manifest
    validate(manifest)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
