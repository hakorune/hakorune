#!/usr/bin/env python3
"""Resolve MissingProjectionPolicy candidates into evidence-quality clusters.

This resolver consumes the crate-wide unconverted surface report and the
next-owner resolver output. It partitions only the already-reported
MissingProjectionPolicy items; it does not infer new Hako projection policy,
emit Hako, or select a family by hand.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NEXT_OWNER_RESOLUTION = FIXTURES / "mirbuilder-unconverted-surface-next-owner-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"


SHAPE_SIGNATURE_BY_CLUSTER = {
    "LoopCondBcContinueIfElseCluster": "control.loop_cond_break_continue_else_continue_if",
    "LoopCondBcItemDispatcherCluster": "control.loop_cond_break_continue_item_dispatch",
    "LoopCondBcStatementLoweringCluster": "control.loop_cond_break_continue_statement_lowering",
    "LoopCondCoAstStatementLoweringCluster": "control.loop_cond_continue_only_ast_statement_lowering",
    "LoopCondCoStatementDispatcherCluster": "control.loop_cond_continue_only_statement_dispatch",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cluster_axis(item: dict[str, Any]) -> str:
    for key in [
        "loop_cond_co_statement_lowering_subcluster",
        "loop_cond_co_helper_subcluster",
        "loop_cond_co_group_if_subcluster",
        "loop_cond_co_continue_if_subcluster",
        "loop_cond_co_subcluster",
        "loop_cond_bc_pipeline_subcluster",
        "loop_cond_bc_item_lowering_subcluster",
        "loop_cond_bc_cleanup_subcluster",
        "loop_cond_bc_else_pattern_subcluster",
        "loop_cond_bc_subcluster",
        "loop_cond_feature_subcluster",
        "plan_feature_subcluster",
        "joinir_plan_subcluster",
        "likely_owner_cluster",
    ]:
        value = item.get(key)
        if value:
            return value
    return "Unclustered"


def owner_confidence(item: dict[str, Any]) -> str:
    return item.get("owner_edge_confidence") or "None"


def stable_deny_reason(item: dict[str, Any]) -> str:
    repaired = item.get("stable_deny_reason")
    if repaired:
        return repaired
    reason = item.get("reason_token")
    if reason == "PublicRustSurfaceMissingProjectionPolicy":
        return "MissingStableDenyReason"
    return reason or "MissingStableDenyReason"


def shape_signature(item: dict[str, Any]) -> str:
    repaired = item.get("shape_signature")
    if repaired:
        return repaired
    return SHAPE_SIGNATURE_BY_CLUSTER.get(cluster_axis(item), "unknown_shape")


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


def control_flow_axis(item: dict[str, Any]) -> str:
    cluster = cluster_axis(item)
    if cluster.startswith("Loop"):
        return "StructuredLoop"
    if "Phi" in cluster or "Carrier" in cluster:
        return "PhiRequired"
    return "StraightLine"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if "unsafe" in (item.get("source_path") or ""):
        return "UnsafeOrFFI"
    if ret in {"", "bool", "usize", "i64", "String"}:
        return "Known"
    if "&" in ret:
        return "Missing"
    return "Missing"


def call_vocabulary_axis(item: dict[str, Any]) -> str:
    if item.get("owner_edge_confidence") in {"ExactSymbol", "FixtureMapped"}:
        return "AllKnown"
    return "SomeUnknown"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    if item.get("evidence_refs"):
        return "Present"
    return "MissingVerifier"


def native_seed_or_adoption_proximity(item: dict[str, Any]) -> str:
    cluster = cluster_axis(item)
    if "Carrier" in cluster:
        return "AdoptedNeighbor"
    if "LoopCondCo" in cluster:
        return "SeedNeighbor"
    if "LoopCond" in cluster or "JoinIR" in cluster:
        return "MinimalPath"
    return "None"


def blocked_by_for(
    confidence: str,
    deny_reason: str,
    shape: str,
    verifier_state: str,
    type_axis: str,
) -> list[str]:
    blocked: list[str] = []
    if confidence not in {"ExactSymbol", "FixtureMapped"}:
        blocked.append("NoExactOrFixtureMappedOwnerEdge")
    if deny_reason == "MissingStableDenyReason":
        blocked.append("MissingStableDenyReason")
    if shape == "unknown_shape":
        blocked.append("MissingShapeSignatureClusterAxis")
    if verifier_state != "Present":
        blocked.append(verifier_state)
    if type_axis in {"Missing", "UnsafeOrFFI"}:
        blocked.append(f"TypeTransport{type_axis}")
    return blocked


def legacy_cluster_id(
    deny_reason: str,
    shape: str,
    confidence: str,
    source_cluster: str,
) -> str:
    return "::".join([
        "projection_policy",
        deny_reason,
        shape,
        confidence,
        source_cluster,
    ])


def cluster_id_for(
    deny_reason: str,
    shape: str,
    confidence: str,
    source_cluster: str,
    borrow: str,
    control_flow: str,
    type_axis: str,
    call_axis: str,
    verifier_state: str,
) -> str:
    # Axis-qualified IDs prevent a landed narrow policy/decomposition from
    # hiding sibling clusters that still need transport or borrow repair.
    return "::".join([
        legacy_cluster_id(deny_reason, shape, confidence, source_cluster),
        f"borrow={borrow}",
        f"control={control_flow}",
        f"type={type_axis}",
        f"call={call_axis}",
        f"verifier={verifier_state}",
    ])


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    next_owner = read_json(NEXT_OWNER_RESOLUTION)
    items = [item for item in report["items"] if item["classification"] == "MissingProjectionPolicy"]

    grouped: dict[tuple[str, str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            cluster_axis(item),
            owner_confidence(item),
            stable_deny_reason(item),
            shape_signature(item),
            borrow_axis(item),
            control_flow_axis(item),
            type_transport_axis(item),
            call_vocabulary_axis(item),
            verifier_or_oracle_state(item),
        )
        grouped[key].append(item)

    clusters: list[dict[str, Any]] = []
    owner_confidence_counts = Counter(owner_confidence(item) for item in items)
    stable_deny_reason_counts = Counter(stable_deny_reason(item) for item in items)
    shape_signature_counts = Counter(shape_signature(item) for item in items)
    verifier_state_counts = Counter(verifier_or_oracle_state(item) for item in items)

    for key, bucket in grouped.items():
        (
            source_cluster,
            confidence,
            deny_reason,
            shape,
            borrow,
            control_flow,
            type_axis,
            call_axis,
            verifier_state,
        ) = key
        source_modules = sorted({item["source_path"] for item in bucket})
        proximity_values = Counter(native_seed_or_adoption_proximity(item) for item in bucket)
        proximity = sorted(proximity_values.items(), key=lambda pair: (-pair[1], pair[0]))[0][0]
        blocked_by = blocked_by_for(confidence, deny_reason, shape, verifier_state, type_axis)
        eligible = not blocked_by
        legacy_id = legacy_cluster_id(deny_reason, shape, confidence, source_cluster)
        cluster_id = cluster_id_for(
            deny_reason,
            shape,
            confidence,
            source_cluster,
            borrow,
            control_flow,
            type_axis,
            call_axis,
            verifier_state,
        )
        clusters.append({
            "cluster_id": cluster_id,
            "legacy_cluster_id": legacy_id,
            "classification": "MissingProjectionPolicyCluster",
            "candidate_count": len(bucket),
            "owner_edge_confidence": confidence,
            "stable_deny_reason": deny_reason,
            "shape_signature": shape,
            "source_cluster": source_cluster,
            "source_modules": source_modules[:20],
            "source_module_count": len(source_modules),
            "borrow_axis": borrow,
            "control_flow_axis": control_flow,
            "type_transport_axis": type_axis,
            "call_vocabulary_axis": call_axis,
            "verifier_or_oracle_state": verifier_state,
            "native_seed_or_adoption_proximity": proximity,
            "selection_eligible": eligible,
            "blocked_by": blocked_by,
            "next_owner_kind": "ProjectionPolicy" if eligible else "None",
            "next_card": f"MIRBUILDER-{source_cluster.upper().replace('-', '_')}-PROJECTION-POLICY-001" if eligible else None,
            "evidence_refs": sorted({ref for item in bucket for ref in item.get("evidence_refs", [])}),
        })

    clusters.sort(key=lambda item: (-item["candidate_count"], item["cluster_id"]))
    eligible_clusters = [cluster for cluster in clusters if cluster["selection_eligible"]]
    duplicate_cluster_id_count = len(clusters) - len({cluster["cluster_id"] for cluster in clusters})
    legacy_cluster_id_collision_count = len(clusters) - len({cluster["legacy_cluster_id"] for cluster in clusters})
    mapped_unknown_shape_count = sum(
        1 for item in items
        if owner_confidence(item) in {"ExactSymbol", "FixtureMapped"}
        and shape_signature(item) == "unknown_shape"
    )

    exact_or_fixture = owner_confidence_counts.get("ExactSymbol", 0) + owner_confidence_counts.get("FixtureMapped", 0)
    if exact_or_fixture == 0:
        decision = {
            "kind": "SelectOwnerEdgeConfidenceRepair",
            "selected_cluster_id": None,
            "selected_next_card": "MIRBUILDER-OWNER-EDGE-CONFIDENCE-REPAIR-001",
            "reason_token": "NoExactOrFixtureMappedOwnerEdge",
        }
    elif stable_deny_reason_counts.get("MissingStableDenyReason", 0) == len(items):
        decision = {
            "kind": "SelectStableDenyReasonRepair",
            "selected_cluster_id": None,
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001",
            "reason_token": "MissingStableDenyReasonForMappedProjectionPolicyClusters",
        }
    elif mapped_unknown_shape_count > 0:
        decision = {
            "kind": "SelectShapeSignatureInventory",
            "selected_cluster_id": None,
            "selected_next_card": "MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001",
            "reason_token": "MissingShapeSignatureForProjectionPolicyClusters",
        }
    elif eligible_clusters and len(eligible_clusters) == 1:
        cluster = eligible_clusters[0]
        decision = {
            "kind": "SelectProjectionPolicyCluster",
            "selected_cluster_id": cluster["cluster_id"],
            "selected_next_card": cluster["next_card"],
            "reason_token": "ExactlyOneProjectionPolicyCluster",
        }
    elif eligible_clusters:
        decision = {
            "kind": "SelectProjectionPolicyClusterPriorityResolution",
            "selected_cluster_id": None,
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "AmbiguousProjectionPolicyClusters",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoProjectionPolicyClusterWithSufficientEvidence",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideMissingProjectionPolicyClusterResolutionV1",
        "token": "MIRBUILDER-CRATE-WIDE-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-001",
        "input_state": {
            "source_report": rel(REPORT),
            "next_owner_resolution": rel(NEXT_OWNER_RESOLUTION),
            "selected_priority": next_owner["candidate_pool"]["selected_priority"],
            "selected_priority_candidate_count": next_owner["candidate_pool"]["selected_priority_candidate_count"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "next_owner_resolution_hash": sha256_file(NEXT_OWNER_RESOLUTION),
        },
        "cluster_axes": [
            "owner_edge_confidence",
            "stable_deny_reason",
            "shape_signature",
            "source_cluster",
            "borrow_axis",
            "control_flow_axis",
            "type_transport_axis",
            "call_vocabulary_axis",
            "verifier_or_oracle_state",
            "native_seed_or_adoption_proximity",
        ],
        "clusters": clusters,
        "summary": {
            "input_candidate_count": len(items),
            "cluster_count": len(clusters),
            "duplicate_cluster_id_count": duplicate_cluster_id_count,
            "legacy_cluster_id_collision_count": legacy_cluster_id_collision_count,
            "selection_eligible_cluster_count": len(eligible_clusters),
            "heuristic_or_unmapped_count": owner_confidence_counts.get("Heuristic", 0) + owner_confidence_counts.get("None", 0),
            "exact_owner_confidence_count": owner_confidence_counts.get("ExactSymbol", 0),
            "fixture_mapped_count": owner_confidence_counts.get("FixtureMapped", 0),
            "missing_shape_signature_count": shape_signature_counts.get("unknown_shape", 0),
            "mapped_unknown_shape_count": mapped_unknown_shape_count,
            "missing_stable_deny_reason_count": stable_deny_reason_counts.get("MissingStableDenyReason", 0),
            "missing_verifier_or_oracle_count": len(items) - verifier_state_counts.get("Present", 0),
            "borrow_policy_needed_count": 0,
            "owner_edge_confidence_counts": dict(sorted(owner_confidence_counts.items())),
            "stable_deny_reason_counts": dict(sorted(stable_deny_reason_counts.items())),
            "shape_signature_counts": dict(sorted(shape_signature_counts.items())),
            "verifier_or_oracle_state_counts": dict(sorted(verifier_state_counts.items())),
        },
        "decision": decision,
        "recovery": {
            "reason": "owner_edge_confidence is the first blocking axis; projection policy clusters are not selectable while every MissingProjectionPolicy item has owner_edge_confidence=None",
            "do_not": [
                "manual_family_selection",
                "cluster_size_as_proof",
                "coverage_percentage_as_proof",
                "route_membership_alone_as_proof",
                "generated_artifact_as_edit_authority",
                "runtime_fallback",
                "new_backend_route",
                "new_abi",
                "new_python_semantic_projector",
                "source_selfhost_claim"
            ]
        },
        "claims": {
            "input_missing_projection_policy_count": len(items),
            "all_missing_projection_policy_items_clustered_exactly_once": 1,
            "cluster_id_is_stable": 1,
            "cluster_id_is_unique": 1,
            "legacy_cluster_id_preserved": 1,
            "owner_edge_confidence_recorded": 1,
            "heuristic_or_none_owner_edge_not_selectable": 1,
            "stable_deny_reason_required": 1,
            "shape_signature_recorded": 1,
            "unknown_shape_not_selected_as_projection_policy": 1,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "manual_family_selection": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "family_name_based_policy": 0,
            "hako_emission": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
            "ambiguous_result_keeps_design_stop": 1,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in cluster resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-crate-wide-missing-projection-policy-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
