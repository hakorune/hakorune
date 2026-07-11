#!/usr/bin/env python3
"""Validate the closed contracts of the Failure/Outcome inventories."""

from __future__ import annotations

import argparse
import copy
import json
import re
from pathlib import Path
from typing import Any

from failure_outcome_semantic_site_graph import (
    ALLOWED_LAYERS,
    ALLOWED_OPERATIONS,
    ALLOWED_OUTCOME_BRANCHES,
    ALLOWED_OWNER_DOMAINS,
    SEMANTIC_CLASSES,
)


ROOT = Path(__file__).resolve().parents[2]
GRAPH = ROOT / "tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json"
BINDING = ROOT / "tools/checks/manifests/failure_outcome_projection_binding_v0.json"
RUNTIME = ROOT / "tools/checks/manifests/failure_outcome_runtime_provider_inventory_v0.json"
CONTROL = ROOT / "tools/checks/manifests/failure_outcome_control_flow_inventory_v0.json"
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_exhaustiveness_v0.json"
IDENTIFIER = re.compile(r"^[a-z][a-z0-9_]*$")
CLASSIFIED = "classified"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def read(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def site_id_error(site_id: str) -> str | None:
    parts = site_id.split(".") if isinstance(site_id, str) else []
    if len(parts) != 4:
        return "site_id must have four segments"
    if any(not IDENTIFIER.fullmatch(part) for part in parts):
        return "site_id segment is not lower_snake_case"
    vocabularies = (
        ALLOWED_LAYERS,
        ALLOWED_OWNER_DOMAINS,
        ALLOWED_OPERATIONS,
        ALLOWED_OUTCOME_BRANCHES,
    )
    if any(part not in vocabulary for part, vocabulary in zip(parts, vocabularies)):
        return "site_id contains unknown vocabulary"
    return None


def duplicate(values: list[Any]) -> bool:
    return len(values) != len(set(values))


def validate(
    graph: dict[str, Any],
    binding: dict[str, Any],
    runtime: dict[str, Any],
    control: dict[str, Any],
) -> list[str]:
    errors: list[str] = []
    manifests = (graph, binding, runtime, control)
    if any(manifest.get("semantic_activation") != 0 for manifest in manifests):
        errors.append("semantic activation must remain 0")

    sites = graph.get("semantic_sites", [])
    evidence_ids = [row.get("evidence_id") for row in graph.get("evidence_occurrences", [])]
    if duplicate(evidence_ids):
        errors.append("duplicate semantic evidence id")
    site_ids = [site.get("site_id") for site in sites]
    if duplicate(site_ids):
        errors.append("duplicate semantic site id")
    site_index = {site.get("site_id"): site for site in sites}
    for site in sites:
        site_id = site.get("site_id")
        if (error := site_id_error(site_id)):
            errors.append(f"{site_id}: {error}")
        semantic_class = site.get("semantic_class", "")
        if semantic_class not in SEMANTIC_CLASSES:
            errors.append(f"{site_id}: unknown semantic class")
        if semantic_class == "compatibility_only" and not site.get("profile"):
            errors.append(f"{site_id}: compatibility_only requires profile")
        if site.get("review_status") == CLASSIFIED and any(
            not site.get(field) for field in ("semantic_class", "owner", "target_carrier")
        ):
            errors.append(f"{site_id}: classified site is incomplete")
        if semantic_class == "foreign_null" and not site.get("backend_policy"):
            errors.append(f"{site_id}: foreign_null policy missing")
        if semantic_class == "optional_absence" and site.get("target_carrier") == "Unit":
            errors.append(f"{site_id}: Unit/absence conflation")
        if semantic_class == "successful_no_result" and site.get("target_carrier") in {
            "Option::None",
            "Result::Err",
            "Fault",
        }:
            errors.append(f"{site_id}: successful result conflates outcome")
        if site.get("site_kind") == "boundary_projection":
            source = site.get("projects_site")
            if not source or source not in site_index:
                errors.append(f"{site_id}: projection source missing")
            elif site_index[source].get("site_kind") == "boundary_projection":
                errors.append(f"{site_id}: projection chain forbidden")

    binding_sites = binding.get("operation_outcome_sites", [])
    binding_site_ids = [site.get("site_id") for site in binding_sites]
    if duplicate(binding_site_ids):
        errors.append("duplicate binding semantic site id")
    for site in binding_sites:
        site_id = site.get("site_id")
        if (error := site_id_error(site_id)):
            errors.append(f"{site_id}: {error}")
        semantic_class = site.get("semantic_class", "")
        if semantic_class not in SEMANTIC_CLASSES:
            errors.append(f"{site_id}: unknown semantic class")
        if semantic_class == "compatibility_only" and not site.get("profile"):
            errors.append(f"{site_id}: compatibility_only requires profile")
        if site.get("review_status") == CLASSIFIED and any(
            not site.get(field) for field in ("semantic_class", "semantic_owner", "target_carrier")
        ):
            errors.append(f"{site_id}: classified binding site is incomplete")
        if semantic_class == "foreign_null" and not site.get("backend_policy"):
            errors.append(f"{site_id}: foreign_null policy missing")
        if semantic_class == "optional_absence" and site.get("target_carrier") == "Unit":
            errors.append(f"{site_id}: Unit/absence conflation")
    bindings = binding.get("projection_bindings", [])
    binding_ids = [row.get("projection_id") for row in bindings]
    if duplicate(binding_ids):
        errors.append("duplicate projection binding id")
    for row in bindings:
        source = row.get("projects_site")
        if source not in set(binding_site_ids):
            errors.append(f"{row.get('projection_id')}: projection source missing")
        if row.get("resolution") != "BoundInventoryOnly":
            errors.append(f"{row.get('projection_id')}: unsupported projection resolution")

    runtime_rows = runtime.get("runtime_provider_evidence", [])
    runtime_ids = [row.get("runtime_evidence_id") for row in runtime_rows]
    if duplicate(runtime_ids):
        errors.append("duplicate runtime evidence id")
    control_rows = control.get("control_flow_evidence", [])
    control_ids = [row.get("control_flow_evidence_id") for row in control_rows]
    if duplicate(control_ids):
        errors.append("duplicate control-flow evidence id")
    for row in runtime_rows + control_rows:
        if row.get("resolution") == "Pending" and row.get("site_ref") is not None:
            errors.append(f"{row.get('runtime_evidence_id', row.get('control_flow_evidence_id'))}: pending site reference")

    pending = graph.get("pending_counts", {}).get("missing_argument_zero")
    baseline = graph.get("pending_baseline_counts", {}).get("missing_argument_zero")
    previous = graph.get("previous_pending_counts", {}).get("missing_argument_zero")
    if not all(isinstance(value, int) for value in (pending, baseline, previous)):
        errors.append("missing_argument_zero baseline is incomplete")
    else:
        if pending > baseline:
            errors.append("missing_argument_zero pending count increased from baseline")
        if pending > previous:
            errors.append("missing_argument_zero pending count increased from previous")
    return errors


def build_report() -> dict[str, Any]:
    graph, binding, runtime, control = (read(path) for path in (GRAPH, BINDING, RUNTIME, CONTROL))
    errors = validate(graph, binding, runtime, control)
    return {
        "schema_version": 0,
        "status": "failure_outcome_exhaustiveness",
        "semantic_activation": 0,
        "source_manifests": [path.relative_to(ROOT).as_posix() for path in (GRAPH, BINDING, RUNTIME, CONTROL)],
        "missing_argument_zero": {
            "pending": graph.get("pending_counts", {}).get("missing_argument_zero"),
            "baseline": graph.get("pending_baseline_counts", {}).get("missing_argument_zero"),
            "previous": graph.get("previous_pending_counts", {}).get("missing_argument_zero"),
        },
        "result": "pass" if not errors else "fail",
        "errors": errors,
    }


def main() -> int:
    args = parse_args()
    report = build_report()
    expected = json.dumps(report, ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-exhaustiveness] wrote {args.output}")
        return 0 if not report["errors"] else 1
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-exhaustiveness] drift detected")
        return 1
    if report["errors"]:
        for error in report["errors"]:
            print(f"[failure-outcome-exhaustiveness] {error}")
        return 1
    print("[failure-outcome-exhaustiveness] result=pass missing_argument_zero=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
