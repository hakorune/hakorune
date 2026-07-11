#!/usr/bin/env python3
"""Build and validate the S5 Failure/Outcome conflict ledger."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
GRAPH = ROOT / "tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json"
RUNTIME = ROOT / "tools/checks/manifests/failure_outcome_runtime_provider_inventory_v0.json"
CONTROL = ROOT / "tools/checks/manifests/failure_outcome_control_flow_inventory_v0.json"
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_conflict_ledger_v0.json"

CONFLICTS = (
    ("null_vs_void", "carrier_alias", "activation design must choose distinct semantic owners"),
    ("local_default_null", "control_flow_owner", "activation design must choose local default contract"),
    ("weak_upgrade_to_void", "boundary_mapping", "activation design must choose optional absence mapping"),
    ("env_missing_or_error_to_void", "provider_contract", "activation design must split provider outcomes"),
    ("clock_failure_to_zero", "projection_collision", "activation design must prove zero collision policy"),
    ("missing_box_void_compatibility", "compatibility_profile", "activation design must name Compat2025 profile"),
    ("canonical_literal_null", "language_profile", "activation design must decide canonical null profile"),
    ("postfix_catch_vs_fault", "catchability", "activation design must close catchable Fault set"),
)
STATUS = "pending_consultation"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def read(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def first_ref(manifest: dict[str, Any], collection: str, needle: str) -> str:
    for row in manifest.get(collection, []):
        text = f"{row.get('evidence', '')} {row.get('token', '')} {row.get('source_path', '')}"
        if needle.lower() in text.lower():
            return row.get("evidence_id") or row.get("runtime_evidence_id") or row.get("control_flow_evidence_id")
    raise RuntimeError(f"missing conflict evidence: {collection}:{needle}")


def direct_ref(path: str, needle: str, token: str) -> str:
    source = ROOT / path
    for line_number, line in enumerate(source.read_text(encoding="utf-8").splitlines(), start=1):
        if needle in line:
            return f"{path}:{line_number}:{token}"
    raise RuntimeError(f"missing conflict anchor: {path}:{needle}")


def evidence_refs() -> dict[str, list[str]]:
    graph, runtime, control = read(GRAPH), read(RUNTIME), read(CONTROL)
    return {
        "null_vs_void": [direct_ref("docs/reference/language/failure-outcome-relations.md", "neither is an alias", "relation_contract")],
        "local_default_null": [first_ref(control, "control_flow_evidence", "uninitialized")],
        "weak_upgrade_to_void": [first_ref(runtime, "runtime_provider_evidence", "weak_to_strong")],
        "env_missing_or_error_to_void": [first_ref(runtime, "runtime_provider_evidence", "provider_missing_fallback")],
        "clock_failure_to_zero": [first_ref(graph, "evidence_occurrences", "env.now_ms")],
        "missing_box_void_compatibility": [first_ref(graph, "evidence_occurrences", "MissingBox")],
        "canonical_literal_null": [direct_ref("docs/development/current/main/workstreams/language-v1-convergence-current.md", "literal_null", "language_profile")],
        "postfix_catch_vs_fault": [first_ref(graph, "evidence_occurrences", "postfix_catch")],
    }


def build_manifest() -> dict[str, Any]:
    refs = evidence_refs()
    rows = [
        {
            "conflict_id": conflict_id,
            "conflict_kind": kind,
            "status": STATUS,
            "evidence_refs": refs[conflict_id],
            "current_observation": "inventory evidence is present; semantic owner is unresolved",
            "next_decision": next_decision,
            "semantic_activation": 0,
        }
        for conflict_id, kind, next_decision in CONFLICTS
    ]
    return {
        "schema_version": 0,
        "status": "failure_outcome_conflict_ledger",
        "semantic_activation": 0,
        "allowed_statuses": [STATUS],
        "conflicts": rows,
    }


def validate(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    rows = manifest.get("conflicts", [])
    ids = [row.get("conflict_id") for row in rows]
    if len(ids) != len(set(ids)):
        errors.append("duplicate conflict id")
    if tuple(ids) != tuple(conflict_id for conflict_id, _, _ in CONFLICTS):
        errors.append("conflict vocabulary or order drift")
    for row in rows:
        conflict_id = row.get("conflict_id")
        if row.get("status") not in set(manifest.get("allowed_statuses", [])):
            errors.append(f"unknown conflict status: {conflict_id}")
        if not row.get("evidence_refs"):
            errors.append(f"conflict evidence missing: {conflict_id}")
        if row.get("semantic_activation") != 0:
            errors.append(f"conflict activation is nonzero: {conflict_id}")
        if row.get("current_observation", "").find("unresolved") < 0:
            errors.append(f"conflict claims resolved owner: {conflict_id}")
        if not row.get("next_decision"):
            errors.append(f"conflict next decision missing: {conflict_id}")
    if len(rows) != len(CONFLICTS):
        errors.append(f"expected eight conflicts, got {len(rows)}")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_manifest(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-conflict-ledger] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-conflict-ledger] drift detected")
        return 1
    errors = validate(json.loads(actual))
    if errors:
        for error in errors:
            print(f"[failure-outcome-conflict-ledger] {error}")
        return 1
    print("[failure-outcome-conflict-ledger] conflicts=8 status=pending_consultation activation=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
