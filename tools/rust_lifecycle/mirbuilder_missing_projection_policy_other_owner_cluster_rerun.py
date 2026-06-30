#!/usr/bin/env python3
"""Rerun Other owner-cluster partition after owner-edge confidence repair."""

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
REPAIR = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(*parts: str) -> str:
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", "::".join(parts))


def selected_items(report: dict[str, Any], repair: dict[str, Any]) -> list[dict[str, Any]]:
    repaired = {row["source_id"]: row for row in repair.get("repaired_rows", [])}
    rows = []
    for item in report.get("items", []):
        source_id = item.get("source_id")
        if source_id in repaired:
            merged = dict(item)
            merged["known_owner_edge"] = repaired[source_id]["repaired_known_owner_edge"]
            merged["owner_edge_confidence"] = repaired[source_id]["repaired_owner_edge_confidence"]
            rows.append(merged)
    return sorted(rows, key=lambda item: item["source_id"])


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    receiver = item.get("receiver") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if receiver == "&mut self":
        return "NoReturnedBorrowMutableReceiver"
    if receiver == "&self":
        return "NoReturnedBorrowSharedReceiver"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret in {"", "bool", "usize", "u32", "i64", "String"}:
        return "Known"
    if ret == "Self":
        return "ConstructorCarrier"
    if ret.startswith("Option<") and "&" not in ret:
        return "KnownOptionCarrier"
    if ret.startswith("Result<") and "&" not in ret:
        return "ResultCarrierNeedsVerifier"
    if ret.startswith("Vec<") and "&" not in ret:
        return "KnownVecCarrier"
    if "impl Iterator" in ret:
        return "ReturnedIteratorNeedsPolicy"
    if "&" in ret:
        return "MissingBorrowTransport"
    return "MissingTypeTransport"


def return_family(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret == "":
        return "unit"
    if ret == "bool":
        return "bool"
    if ret in {"usize", "u32", "i64"}:
        return "scalar"
    if ret == "String":
        return "string"
    if ret == "Self":
        return "constructor_self"
    if ret.startswith("Option<"):
        return "option"
    if ret.startswith("Result<"):
        return "result"
    if ret.startswith("Vec<"):
        return "vec"
    if "impl Iterator" in ret:
        return "iterator"
    if "&" in ret:
        return "borrow_return"
    return "custom_carrier"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    return "public" if visibility.startswith("pub") else "private"


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    repair = read_json(REPAIR)
    items = selected_items(report, repair)

    grouped: dict[tuple[str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            item.get("known_owner_edge") or "",
            item.get("shape_signature") or "unknown_shape",
            borrow_axis(item),
            type_transport_axis(item),
            return_family(item),
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, cluster_items in grouped.items():
        owner_edge, shape, borrow, type_axis, ret_family, verifier, visibility = key
        blocked_by = []
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {
            "MissingBorrowTransport",
            "MissingTypeTransport",
            "ResultCarrierNeedsVerifier",
            "ReturnedIteratorNeedsPolicy",
        }:
            blocked_by.append(type_axis)
        if borrow in {"BorrowPolicyNeeded", "ReturnedMutableAliasUnknown"}:
            blocked_by.append(borrow)
        subclusters.append({
            "subcluster_id": stable_id(
                "other_owner_rerun",
                owner_edge,
                shape,
                borrow,
                type_axis,
                ret_family,
                verifier,
                visibility,
            ),
            "known_owner_edge": owner_edge,
            "owner_edge_confidence": "FileScoped",
            "shape_signature": shape,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "return_family": ret_family,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "candidate_count": len(cluster_items),
            "source_ids": [item["source_id"] for item in cluster_items[:20]],
            "source_id_count": len(cluster_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "OtherOwnerClusterRerunRequiresShapeSignatureInventory",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    shape_counts = Counter(item.get("shape_signature") or "unknown_shape" for item in items)
    owner_confidence_counts = Counter(item.get("owner_edge_confidence") or "None" for item in items)
    owner_edge_counts = Counter(item.get("known_owner_edge") or "" for item in items)

    decision = {
        "kind": "SelectShapeSignatureInventory",
        "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001",
        "reason_token": "OtherOwnerClusterShapeSignatureMissing",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyOtherOwnerClusterRerunV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "owner_edge_confidence_repair": rel(REPAIR),
            "owner_edge_confidence_repair_decision": repair.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_other_owner_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "owner_edge_confidence_repair_hash": sha256_file(REPAIR),
        },
        "subcluster_axes": [
            "known_owner_edge",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "return_family",
            "verifier_or_oracle_state",
            "public_or_private_surface",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_other_owner_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "shape_signature_counts": dict(sorted(shape_counts.items())),
            "owner_edge_confidence_counts": dict(sorted(owner_confidence_counts.items())),
            "distinct_known_owner_edge_count": len(owner_edge_counts),
            "top_known_owner_edges": [
                {"owner_edge": edge, "count": count}
                for edge, count in owner_edge_counts.most_common(20)
            ],
            "selection_eligible_subcluster_count": 0,
            "selected_subcluster_id": None,
        },
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "owner_edge_confidence_repair_consumed": 1,
            "input_other_owner_cluster_count": len(items),
            "all_other_owner_cluster_items_partitioned_exactly_once": 1,
            "owner_edge_confidence_repair_applied": 1,
            "shape_signature_gap_selected": 1,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Other owner rerun fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-other-owner-cluster-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
