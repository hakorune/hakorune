#!/usr/bin/env python3
"""Partition remaining JoinIRRouteVerifyCluster MissingProjectionPolicy rows."""

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
PREVIOUS_POLICY = FIXTURES / "mirbuilder-join-i-r-route-verify-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-joinir-route-verify-cluster-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(*parts: str) -> str:
    joined = "::".join(parts)
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", joined)


def source_module(path: str) -> str:
    if not path:
        return "unknown"
    source = Path(path)
    if len(source.parts) > 1 and source.suffix == ".rs":
        return str(source.parent)
    return path


def route_verify_role(item: dict[str, Any]) -> str:
    path = item.get("source_path") or ""
    symbol = item.get("symbol") or ""
    if item.get("cfg_test_surface"):
        return "test_only_surface"
    if "trace" in path or "debug" in path or "debug" in symbol:
        return "diagnostic_or_debug_helper"
    if "/edgecfg/" in path:
        return "edgecfg_compose_or_verify"
    if "/facts/" in path:
        return "facts_or_recognizer"
    if "/recipes/" in path:
        return "recipe_index_or_ref"
    if "/joinir/merge/rewriter/" in path:
        return "joinir_merge_rewriter"
    if "/joinir/merge/contract_checks/" in path:
        return "joinir_merge_contract"
    if "/joinir/merge/coordinator/" in path:
        return "joinir_merge_coordinator"
    if "/joinir/merge/" in path:
        return "joinir_merge_helper"
    if "/joinir/" in path:
        return "joinir_routing_or_trace"
    if "/verify/observability/" in path:
        return "verify_observability"
    if "/verify/diagnostics/" in path:
        return "verify_diagnostic"
    if "/verify/verifier/" in path or "/verify/" in path:
        return "verify_predicate_or_guard"
    return "route_verify_other"


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    receiver = item.get("receiver") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if "&mut" in params or receiver == "&mut self":
        return "NoReturnedBorrowMutableReceiver"
    if "&self" in params or receiver == "&self":
        return "NoReturnedBorrowSharedReceiver"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret in {"", "bool", "usize", "i64", "String"}:
        return "Known"
    if ret.startswith("Option<") and "&" not in ret:
        return "KnownOptionCarrier"
    if ret.startswith("Result<") and "&" not in ret:
        return "ResultCarrierNeedsVerifier"
    if ret == "Self":
        return "ConstructorCarrier"
    if "&" in ret:
        return "MissingBorrowTransport"
    return "MissingTypeTransport"


def return_family(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret == "":
        return "unit"
    if ret == "bool":
        return "bool"
    if ret in {"usize", "i64"}:
        return "scalar"
    if ret == "String":
        return "string"
    if ret == "Self":
        return "constructor_self"
    if ret.startswith("Option<"):
        return "option"
    if ret.startswith("Result<"):
        return "result"
    if "&" in ret:
        return "borrow_return"
    return "custom_carrier"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    if visibility.startswith("pub"):
        return "public"
    return "private"


def route_verify_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    items = [
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("likely_owner_cluster") == "JoinIRRouteVerifyCluster"
    ]
    return sorted(items, key=lambda item: item["source_id"])


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    previous_policy = read_json(PREVIOUS_POLICY)
    items = route_verify_items(report)

    grouped: dict[tuple[str, str, str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            route_verify_role(item),
            source_module(item.get("source_path") or ""),
            item.get("shape_signature") or "unknown_shape",
            borrow_axis(item),
            type_transport_axis(item),
            return_family(item),
            item.get("receiver") or "no_receiver",
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
            "cfg_test" if item.get("cfg_test_surface") else "non_test",
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, bucket_items in grouped.items():
        (
            role,
            module,
            shape,
            borrow,
            type_axis,
            ret_family,
            receiver,
            verifier,
            visibility,
            test_axis,
        ) = key
        blocked_by: list[str] = []
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {"MissingBorrowTransport", "MissingTypeTransport", "ResultCarrierNeedsVerifier"}:
            blocked_by.append(type_axis)
        if borrow in {"BorrowPolicyNeeded", "ReturnedMutableAliasUnknown"}:
            blocked_by.append(borrow)
        if role in {"diagnostic_or_debug_helper", "test_only_surface"}:
            blocked_by.append("NonSemanticHelper")
        subclusters.append({
            "subcluster_id": stable_id(
                "joinir_route_verify",
                role,
                module,
                shape,
                borrow,
                type_axis,
                ret_family,
                receiver,
                verifier,
                visibility,
                test_axis,
            ),
            "route_verify_role": role,
            "source_module": module,
            "shape_signature": shape,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "return_family": ret_family,
            "receiver_axis": receiver,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "cfg_test_surface": test_axis == "cfg_test",
            "candidate_count": len(bucket_items),
            "source_ids": [item["source_id"] for item in bucket_items[:20]],
            "source_id_count": len(bucket_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "JoinIRRouteVerifyClusterPartitionedForFollowUp",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    eligible_subclusters = [
        item for item in subclusters
        if not item["blocked_by"]
        and item["public_or_private_surface"] == "public"
        and not item["cfg_test_surface"]
    ]
    role_counts = Counter(item["route_verify_role"] for item in subclusters)
    module_counts = Counter(source_module(item.get("source_path") or "") for item in items)
    return_family_counts = Counter(return_family(item) for item in items)
    type_axis_counts = Counter(type_transport_axis(item) for item in items)

    if len(eligible_subclusters) == 1:
        selected = eligible_subclusters[0]
        decision = {
            "kind": "SelectProjectionPolicySubcluster",
            "selected_subcluster_id": selected["subcluster_id"],
            "selected_next_card": (
                "MIRBUILDER-JOINIR-ROUTE-VERIFY-"
                f"{selected['route_verify_role'].upper().replace('_', '-')}-PROJECTION-POLICY-001"
            ),
            "reason_token": "ExactlyOneJoinIRRouteVerifyProjectionSubcluster",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_subcluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": (
                "AmbiguousJoinIRRouteVerifyProjectionSubclusters"
                if eligible_subclusters else
                "NoEligibleJoinIRRouteVerifyProjectionSubcluster"
            ),
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyJoinIRRouteVerifyClusterV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "previous_parent_owned_policy": rel(PREVIOUS_POLICY),
            "previous_policy_decision": previous_policy.get("decision", {}),
            "priority_decision": priority.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_joinir_route_verify_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY),
            "previous_parent_owned_policy_hash": sha256_file(PREVIOUS_POLICY),
        },
        "subcluster_axes": [
            "route_verify_role",
            "source_module",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "return_family",
            "receiver_axis",
            "verifier_or_oracle_state",
            "public_or_private_surface",
            "cfg_test_surface",
        ],
        "route_verify_roles": [
            "edgecfg_compose_or_verify",
            "facts_or_recognizer",
            "recipe_index_or_ref",
            "joinir_routing_or_trace",
            "joinir_merge_coordinator",
            "joinir_merge_contract",
            "joinir_merge_rewriter",
            "joinir_merge_helper",
            "verify_diagnostic",
            "verify_observability",
            "verify_predicate_or_guard",
            "diagnostic_or_debug_helper",
            "test_only_surface",
            "route_verify_other",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_joinir_route_verify_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "role_counts": dict(sorted(role_counts.items())),
            "source_module_counts": dict(sorted(module_counts.items())),
            "return_family_counts": dict(sorted(return_family_counts.items())),
            "type_transport_axis_counts": dict(sorted(type_axis_counts.items())),
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
            "previous_parent_owned_policy_consumed": 1,
            "input_joinir_route_verify_cluster_count": len(items),
            "all_joinir_route_verify_items_partitioned_exactly_once": 1,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in JoinIRRouteVerify cluster fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-joinir-route-verify-cluster unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
