#!/usr/bin/env python3
"""Decompose the selected GenericLoopPlan projection-policy cluster."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-plan-subcluster-decomposition-v0.json"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.generic_loop_plan::"
    "FixtureMapped::GenericLoopPlanCluster"
)

FN_RE = re.compile(
    r"^\s*(?:(?:pub(?:\([^)]*\))?)\s+)?fn\s+"
    r"(?P<symbol>[A-Za-z_][A-Za-z0-9_]*)\s*\((?P<params>[^)]*)\)"
    r"\s*(?:->\s*(?P<return>[^\{]+))?",
    re.MULTILINE,
)

SUBCLUSTERS: dict[str, dict[str, Any]] = {
    "BodyCheckExprMatchers": {
        "path_contains": "/body_check/expr_matchers/",
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001",
        "reason_token": "ExpressionMatchersAreNarrowPurePredicates",
        "priority": 0,
    },
    "BodyCheckStepValidation": {
        "path_contains": "/body_check/step_validation.rs",
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001",
        "reason_token": "StepValidationRequiresSeparatePolicy",
        "priority": 1,
    },
    "BodyCheckExtractors": {
        "path_contains_any": [
            "/body_check_extractors.rs",
            "/facts/extract/collection.rs",
        ],
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXTRACTORS-PROJECTION-POLICY-001",
        "reason_token": "BodyCheckExtractorsRequireSeparatePolicy",
        "priority": 2,
    },
    "BodyCheckShapeDetectors": {
        "path_contains": "/body_check_shape_detectors/",
        "path_excludes": ["/body_check_shape_detectors/utils.rs"],
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-SHAPE-DETECTORS-PROJECTION-POLICY-001",
        "reason_token": "ShapeDetectorsRequireSeparatePolicy",
        "priority": 3,
    },
    "BodyCheckShapeDetectorUtils": {
        "path_contains": "/body_check_shape_detectors/utils.rs",
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-SHAPE-DETECTOR-UTILS-PROJECTION-POLICY-001",
        "reason_token": "ShapeDetectorUtilsAreSharedPredicateHelpers",
        "priority": 4,
    },
    "StatementClassifierPredicates": {
        "path_contains": "/facts/stmt_classifier/",
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-STMT-CLASSIFIER-PROJECTION-POLICY-001",
        "reason_token": "StatementClassifierPredicatesRequireSeparatePolicy",
        "priority": 5,
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def line_for_offset(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def classify_path(source_path: str) -> str:
    normalized = "/" + source_path
    matches: list[str] = []
    for name, definition in SUBCLUSTERS.items():
        if any(excluded in normalized for excluded in definition.get("path_excludes", [])):
            continue
        if "path_contains" in definition and definition["path_contains"] in normalized:
            matches.append(name)
        for marker in definition.get("path_contains_any", []):
            if marker in normalized:
                matches.append(name)
    if len(matches) != 1:
        raise SystemExit(f"unclassified or ambiguous GenericLoopPlan source path: {source_path} -> {matches}")
    return matches[0]


def scan_functions(source_path: str) -> list[dict[str, Any]]:
    text = (ROOT / source_path).read_text(encoding="utf-8")
    subcluster = classify_path(source_path)
    functions = []
    for match in FN_RE.finditer(text):
        line = line_for_offset(text, match.start())
        symbol = match.group("symbol")
        functions.append({
            "source_id": f"{source_path}::{symbol}:L{line}",
            "symbol": symbol,
            "source_path": source_path,
            "line": line,
            "params": (match.group("params") or "").strip(),
            "return_type": (match.group("return") or "").strip(),
            "subcluster_id": subcluster,
        })
    return functions


def selected_cluster(cluster_resolution: dict[str, Any]) -> dict[str, Any]:
    for cluster in cluster_resolution["clusters"]:
        if cluster["cluster_id"] == SELECTED_CLUSTER_ID:
            return cluster
    raise SystemExit(f"selected cluster not found: {SELECTED_CLUSTER_ID}")


def priority_has_cluster_evidence(priority: dict[str, Any]) -> bool:
    if priority["decision"].get("selected_cluster_id") == SELECTED_CLUSTER_ID:
        return True
    return any(
        cluster.get("cluster_id") == SELECTED_CLUSTER_ID
        for cluster in priority.get("excluded_existing_decision_clusters", [])
    )


def build_decomposition() -> dict[str, Any]:
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    cluster = selected_cluster(cluster_resolution)
    if not priority_has_cluster_evidence(priority):
        raise SystemExit("priority resolver has no GenericLoopPlanCluster evidence")

    source_surfaces = []
    for source_path in cluster["source_modules"]:
        source_surfaces.extend(scan_functions(source_path))
    if not source_surfaces:
        raise SystemExit("GenericLoopPlan source scan produced no functions")

    counts = Counter(surface["subcluster_id"] for surface in source_surfaces)
    subclusters = []
    for name, definition in sorted(SUBCLUSTERS.items(), key=lambda item: item[1]["priority"]):
        members = [
            surface for surface in source_surfaces
            if surface["subcluster_id"] == name
        ]
        subclusters.append({
            "subcluster_id": name,
            "source_count": len(members),
            "module_count": len({member["source_path"] for member in members}),
            "symbols": [member["symbol"] for member in members],
            "classification": "GenericLoopPlanSubcluster",
            "next_owner_kind": definition["next_owner_kind"],
            "next_card": definition["next_card"],
            "reason_token": definition["reason_token"],
            "selection_eligible": name == "BodyCheckExprMatchers",
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopPlanSubclusterDecompositionV1",
        "token": "MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001",
        "input_state": {
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "source_cluster_id": SELECTED_CLUSTER_ID,
            "source_cluster": "GenericLoopPlanCluster",
            "input_candidate_count": cluster["candidate_count"],
            "source_module_count": len(cluster["source_modules"]),
            "scanned_function_count": len(source_surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_plan",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_modules": cluster["source_modules"],
        "source_surfaces": source_surfaces,
        "subclusters": subclusters,
        "subcluster_counts": dict(sorted(counts.items())),
        "decomposition_policy": {
            "whole_cluster_projection_policy_selected": False,
            "whole_cluster_keep_parent_owner_selected": False,
            "path_role_decomposition": True,
            "candidate_count_as_proof": 0,
            "reason_token": "GenericLoopPlanClusterContainsMixedPathRoles",
        },
        "decision": {
            "kind": "SelectSubclusterProjectionPolicy",
            "selected_subcluster_id": "BodyCheckExprMatchers",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001",
            "reason_token": "SelectNarrowExprMatchersBeforeValidationAndShapeDetectors",
        },
        "claims": {
            "manual_family_selection": 0,
            "whole_cluster_projection_policy": 0,
            "whole_cluster_keep_parent_owner": 0,
            "candidate_count_as_proof": 0,
            "runtime_or_projection_policy_by_name": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
        "provenance": {
            "tool_role": "FactsAdapterGuardOrchestrator",
            "semantic_projection_inference": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in decomposition fixture.")
    args = parser.parse_args()

    output = stable_json(build_decomposition())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-generic-loop-plan-subcluster-decomposition unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
