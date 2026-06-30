#!/usr/bin/env python3
"""Partition remaining JoinIRPlanCluster MissingProjectionPolicy rows."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-joinir-plan-cluster-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-PLAN-CLUSTER-001"


BUCKET_BY_SUBCLUSTER = {
    "GenericLoopPlanCluster": "loop_plan_extractor",
    "LoopBreakPlanCluster": "loop_plan_extractor",
    "LoopCondPlanCluster": "loop_plan_extractor",
    "NestedLoopPlanCluster": "loop_plan_extractor",
    "PlanPartsAssemblyCluster": "route_local_plan_descriptor",
    "PlanFeatureMaterializerCluster": "plan_feature_helper",
    "RecipeTreeMatcherCluster": "recipe_tree_matcher",
    "PlanFactsCluster": "plan_feature_helper",
    "PlanNormalizerCluster": "plan_feature_helper",
    "PlanLowererCluster": "joinir_plan_lowering_surface",
    "PlannerPolicyCluster": "joinir_plan_lowering_surface",
    "PlanComposerCluster": "joinir_plan_lowering_surface",
    "OtherJoinIRPlanCluster": "unknown_or_needs_owner_edge_repair",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_module(path: str) -> str:
    if not path:
        return "unknown"
    source = Path(path)
    if len(source.parts) > 1 and source.suffix == ".rs":
        return str(source.parent)
    return path


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if "&mut" in params or "&self" in params:
        return "NoReturnedBorrow"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if "unsafe" in (item.get("source_path") or ""):
        return "UnsafeOrFFI"
    if ret in {"", "bool", "usize", "i64", "String"}:
        return "Known"
    if "&" in ret:
        return "Missing"
    return "Missing"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    if visibility.startswith("pub"):
        return "public"
    return "private"


def high_level_bucket(item: dict[str, Any]) -> str:
    if item.get("cfg_test_surface"):
        return "test_only_surface"
    path = item.get("source_path") or ""
    symbol = item.get("symbol") or ""
    if "trace" in path or "debug" in path or "debug" in symbol:
        return "diagnostic_or_debug_helper"
    subcluster = item.get("joinir_plan_subcluster") or "OtherJoinIRPlanCluster"
    return BUCKET_BY_SUBCLUSTER.get(subcluster, "unknown_or_needs_owner_edge_repair")


def stable_id(*parts: str) -> str:
    joined = "::".join(parts)
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", joined)


def joinir_plan_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    items = [
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and (
            item.get("likely_owner_cluster") == "JoinIRPlanCluster"
            or bool(item.get("joinir_plan_subcluster"))
        )
    ]
    return sorted(items, key=lambda item: item["source_id"])


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    items = joinir_plan_items(report)

    grouped: dict[tuple[str, str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            high_level_bucket(item),
            item.get("joinir_plan_subcluster") or "OtherJoinIRPlanCluster",
            item.get("shape_signature") or "unknown_shape",
            source_module(item.get("source_path") or ""),
            borrow_axis(item),
            type_transport_axis(item),
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
            "cfg_test" if item.get("cfg_test_surface") else "non_test",
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, bucket_items in grouped.items():
        (
            bucket,
            joinir_subcluster,
            shape,
            module,
            borrow,
            type_axis,
            verifier,
            visibility,
            test_axis,
        ) = key
        blocked_by: list[str] = []
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {"Missing", "UnsafeOrFFI"}:
            blocked_by.append(f"TypeTransport{type_axis}")
        if bucket == "unknown_or_needs_owner_edge_repair":
            blocked_by.append("NeedsOwnerEdgeRepair")
        subclusters.append({
            "subcluster_id": stable_id(
                "joinir_plan",
                bucket,
                joinir_subcluster,
                shape,
                module,
                borrow,
                type_axis,
                verifier,
                visibility,
                test_axis,
            ),
            "bucket": bucket,
            "joinir_plan_subcluster": joinir_subcluster,
            "shape_signature": shape,
            "source_module": module,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "cfg_test_surface": test_axis == "cfg_test",
            "candidate_count": len(bucket_items),
            "source_ids": [item["source_id"] for item in bucket_items[:20]],
            "source_id_count": len(bucket_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "JoinIRPlanClusterPartitionedForFollowUp",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    eligible_subclusters = [
        item for item in subclusters
        if not item["blocked_by"]
        and item["public_or_private_surface"] == "public"
        and not item["cfg_test_surface"]
    ]
    bucket_counts = Counter(item["bucket"] for item in subclusters)
    joinir_subcluster_counts = Counter(item.get("joinir_plan_subcluster") or "OtherJoinIRPlanCluster" for item in items)
    shape_counts = Counter(item.get("shape_signature") or "unknown_shape" for item in items)

    if len(eligible_subclusters) == 1:
        selected = eligible_subclusters[0]
        decision = {
            "kind": "SelectProjectionPolicySubcluster",
            "selected_subcluster_id": selected["subcluster_id"],
            "selected_next_card": f"MIRBUILDER-{selected['shape_signature'].removeprefix('shape.').upper().replace('_', '-')}-PROJECTION-POLICY-001",
            "reason_token": "ExactlyOneJoinIRPlanProjectionSubcluster",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_subcluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": (
                "AmbiguousJoinIRPlanProjectionSubclusters"
                if eligible_subclusters else
                "NoEligibleJoinIRPlanProjectionSubcluster"
            ),
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyJoinIRPlanClusterV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "priority_decision": priority.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_joinir_plan_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY),
        },
        "subcluster_axes": [
            "source_module",
            "plan_feature_subcluster",
            "joinir_plan_subcluster",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "verifier_or_oracle_state",
            "public_or_private_surface",
            "cfg_test_surface",
        ],
        "high_level_buckets": [
            "route_local_plan_descriptor",
            "recipe_tree_matcher",
            "plan_feature_helper",
            "loop_plan_extractor",
            "joinir_plan_lowering_surface",
            "diagnostic_or_debug_helper",
            "test_only_surface",
            "unknown_or_needs_owner_edge_repair",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_joinir_plan_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "bucket_counts": dict(sorted(bucket_counts.items())),
            "joinir_plan_subcluster_counts": dict(sorted(joinir_subcluster_counts.items())),
            "shape_signature_counts": dict(sorted(shape_counts.items())),
            "selection_eligible_subcluster_count": len(eligible_subclusters),
            "selected_subcluster_id": (
                eligible_subclusters[0]["subcluster_id"]
                if len(eligible_subclusters) == 1
                else None
            ),
        },
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "projection_priority_consumed": 1,
            "input_joinir_plan_cluster_count": len(items),
            "all_joinir_plan_items_partitioned_exactly_once": 1,
            "subcluster_ids_are_stable": 1,
            "subcluster_reason_tokens_are_stable": 1,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in JoinIRPlan cluster fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-joinir-plan-cluster unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
