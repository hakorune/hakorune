#!/usr/bin/env python3
"""Validate the observation-only FunctionMetadata consumer manifest.

The manifest is deliberately a static census.  It records owner-file evidence
and does not decide semantic authority, enable a backend, or rewrite MIR.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys
from typing import Any


TAG = "mir-metadata-consumer-manifest"
SCHEMA_VERSION = 1
MANIFEST_KIND = "MirFunctionMetadataConsumerManifestV1"
REVISION_RE = re.compile(r"^[0-9a-f]{40}$")
FIELD_RE = re.compile(
    r"    (?:(?:pub(?:\([^)]*\))?)\s+)?([A-Za-z_]\w*)\s*:"
)
CLASSES = {
    "SourceAttrs",
    "SemanticFacts",
    "Contracts",
    "LayoutPlans",
    "PlacementPlans",
    "LoweringRoutes",
    "DiagnosticsMetadata",
    "ExperimentalSeedRoutes",
}
BACKEND_ROLES = {
    "none",
    "selected_observed",
    "reference_only",
    "non_selected_only",
}
CALLER_STATES = {
    "live",
    "retained_authority",
    "producer_only",
    "caller_zero",
}
VERIFICATION_KINDS = {"static_owner_file", "focused_gate"}
REQUIRED_ROW_KEYS = {
    "field",
    "class",
    "producer",
    "production_consumer",
    "backend_consumer",
    "reference_consumer",
    "non_selected_consumer",
    "egress",
    "verification_kind",
    "caller_state",
    "retire_when",
}


def fail(message: str) -> None:
    print(f"[{TAG}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"cannot read manifest {path}: {exc}")
    if not isinstance(value, dict):
        fail("manifest root must be an object")
    return value


def source_fields(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8")
    try:
        start = text.index("pub struct FunctionMetadata {")
        end = text.index("\n}\n\nimpl FunctionMetadata", start)
    except ValueError as exc:
        fail(f"FunctionMetadata boundary missing in {path}: {exc}")
    fields: list[str] = []
    for line in text[start:end].splitlines():
        match = FIELD_RE.match(line)
        if match:
            fields.append(match.group(1))
    if len(fields) != len(set(fields)):
        fail("FunctionMetadata source contains duplicate field names")
    return fields


def anchor_path(anchor: str) -> tuple[Path, int, str]:
    if not isinstance(anchor, str) or not anchor:
        fail("anchors must be non-empty strings")
    parts = anchor.rsplit(":", 2)
    if len(parts) != 3 or not parts[0] or not parts[2]:
        fail(f"anchor must be path:line:token: {anchor!r}")
    try:
        line = int(parts[1])
    except ValueError:
        fail(f"anchor line must be an integer: {anchor!r}")
    if line < 1:
        fail(f"anchor line must be positive: {anchor!r}")
    return Path(parts[0]), line, parts[2]


def is_test_path(path: str) -> bool:
    name = Path(path).name
    return (
        "/tests/" in f"/{path}"
        or "/fixtures/" in f"/{path}"
        or path.startswith("src/tests/")
        or name in {"test.rs", "tests.rs"}
        or name.startswith("test_")
        or name.endswith("_test.rs")
        or name.endswith("_tests.rs")
    )


def validate_evidence(root: Path, evidence: Any, label: str, *, allow_zero: bool) -> int:
    if not isinstance(evidence, dict):
        fail(f"{label} must be an object")
    owners = evidence.get("owners")
    anchors = evidence.get("anchors")
    count = evidence.get("count")
    if not isinstance(owners, list) or not all(isinstance(v, str) and v for v in owners):
        fail(f"{label}.owners must be a non-empty string array")
    if not isinstance(anchors, list) or not all(isinstance(v, str) and v for v in anchors):
        fail(f"{label}.anchors must be a string array")
    if not isinstance(count, int) or isinstance(count, bool) or count < 0:
        fail(f"{label}.count must be a non-negative integer")
    if count != len(owners):
        fail(f"{label}.count must equal the number of owner files")
    if not allow_zero and count == 0:
        fail(f"{label} cannot be empty")
    if count == 0 and anchors:
        fail(f"{label} cannot have anchors when count is zero")
    if len(owners) != len(set(owners)):
        fail(f"{label}.owners contains duplicates")
    if any(is_test_path(owner) for owner in owners) and not label.endswith(".producer"):
        fail(f"{label} must not count test/fixture files as production evidence")
    if len(anchors) > count:
        fail(f"{label}.anchors cannot exceed owner count")
    seen_anchor_paths: set[str] = set()
    for anchor in anchors:
        rel_path, line, token = anchor_path(anchor)
        rel = rel_path.as_posix()
        if rel in seen_anchor_paths:
            fail(f"{label} repeats an anchor owner file: {rel}")
        seen_anchor_paths.add(rel)
        path = root / rel_path
        if not path.is_file():
            fail(f"{label} anchor file is missing: {rel}")
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        if line > len(lines) or token not in lines[line - 1]:
            fail(f"{label} anchor drift: {anchor}")
    return count


def validate_row(root: Path, row: Any, index: int, source_field_set: set[str]) -> dict[str, Any]:
    if not isinstance(row, dict):
        fail(f"row {index} must be an object")
    missing = REQUIRED_ROW_KEYS - set(row)
    if missing:
        fail(f"row {index} missing keys: {', '.join(sorted(missing))}")
    field = row["field"]
    if not isinstance(field, str) or not field:
        fail(f"row {index} field must be a non-empty string")
    if field not in source_field_set:
        fail(f"row {index} names a field absent from FunctionMetadata: {field}")
    if row["class"] not in CLASSES:
        fail(f"{field}: unknown metadata class {row['class']!r}")
    producer_count = validate_evidence(root, row["producer"], f"{field}.producer", allow_zero=False)
    production_count = validate_evidence(
        root, row["production_consumer"], f"{field}.production_consumer", allow_zero=True
    )
    backend = row["backend_consumer"]
    if not isinstance(backend, dict) or backend.get("role") not in BACKEND_ROLES:
        fail(f"{field}.backend_consumer.role is invalid")
    backend_count = validate_evidence(
        root,
        backend.get("evidence"),
        f"{field}.backend_consumer.evidence",
        allow_zero=True,
    )
    reference_count = validate_evidence(
        root, row["reference_consumer"], f"{field}.reference_consumer", allow_zero=True
    )
    non_selected_count = validate_evidence(
        root,
        row["non_selected_consumer"],
        f"{field}.non_selected_consumer",
        allow_zero=True,
    )
    egress_count = validate_evidence(root, row["egress"], f"{field}.egress", allow_zero=True)
    if row["verification_kind"] not in VERIFICATION_KINDS:
        fail(f"{field}: invalid verification_kind")
    if row["caller_state"] not in CALLER_STATES:
        fail(f"{field}: invalid caller_state")
    retire_when = row["retire_when"]
    if not isinstance(retire_when, str) or not retire_when.strip():
        fail(f"{field}: retire_when must be non-empty")
    if production_count == 0 and backend_count == 0 and reference_count == 0:
        if row["caller_state"] not in {"producer_only", "caller_zero", "retained_authority"}:
            fail(f"{field}: consumer-free row must declare producer_only/caller_zero/retained_authority")
        if "caller-zero" not in retire_when and "retain" not in retire_when:
            fail(f"{field}: consumer-free row must explain caller-zero or retention condition")
    if backend["role"] == "selected_observed" and backend_count == 0:
        fail(f"{field}: selected_observed backend role requires an evidence owner")
    if backend["role"] == "none" and backend_count != 0:
        fail(f"{field}: backend role none cannot carry backend evidence")
    return {
        "field": field,
        "producer_count": producer_count,
        "production_consumer_count": production_count,
        "backend_consumer_count": backend_count,
        "reference_consumer_count": reference_count,
        "non_selected_consumer_count": non_selected_count,
        "egress_count": egress_count,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    parser.add_argument(
        "--manifest",
        default="tools/checks/manifests/mir_function_metadata_consumer_manifest_v1.json",
    )
    args = parser.parse_args()
    root = Path(args.root).resolve()
    manifest_path = root / args.manifest
    data = load_json(manifest_path)
    if data.get("schema_version") != SCHEMA_VERSION or data.get("kind") != MANIFEST_KIND:
        fail("manifest schema_version/kind mismatch")
    revision = data.get("observed_revision")
    if not isinstance(revision, str) or not REVISION_RE.fullmatch(revision):
        fail("observed_revision must be a full 40-character git revision")
    source = data.get("source")
    if not isinstance(source, dict):
        fail("source must be an object")
    source_path = source.get("path")
    if source_path != "src/mir/function/metadata.rs":
        fail("source.path must point at FunctionMetadata metadata.rs")
    source_file = root / source_path
    if not source_file.is_file():
        fail(f"source file missing: {source_path}")
    fields = source_fields(source_file)
    if source.get("field_count") != len(fields):
        fail(f"source.field_count drift: manifest={source.get('field_count')} source={len(fields)}")
    rows = data.get("rows")
    if not isinstance(rows, list):
        fail("rows must be an array")
    if len(rows) != len(fields):
        fail(f"row count drift: manifest={len(rows)} source={len(fields)}")
    seen: set[str] = set()
    totals = {"producer_count": 0, "production_consumer_count": 0, "backend_consumer_count": 0}
    for index, row in enumerate(rows, start=1):
        result = validate_row(root, row, index, set(fields))
        field = result["field"]
        if field in seen:
            fail(f"duplicate family/field row: {field}")
        seen.add(field)
        for key in totals:
            totals[key] += result[key]
    missing = sorted(set(fields) - seen)
    if missing:
        fail(f"fields missing from manifest: {', '.join(missing)}")
    boundary = data.get("boundary")
    if not isinstance(boundary, dict):
        fail("boundary must be an object")
    for key in ("start", "end", "includes", "excludes"):
        if key not in boundary:
            fail(f"boundary missing {key}")
    if not isinstance(boundary["includes"], list) or not boundary["includes"]:
        fail("boundary.includes must be a non-empty array")
    if not isinstance(boundary["excludes"], list) or not boundary["excludes"]:
        fail("boundary.excludes must be a non-empty array")
    print(
        f"[{TAG}] ok fields={len(fields)} rows={len(rows)} "
        f"producer_owner_files={totals['producer_count']} "
        f"production_consumer_owner_files={totals['production_consumer_count']} "
        f"backend_consumer_owner_files={totals['backend_consumer_count']} "
        f"observed_revision={revision}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
