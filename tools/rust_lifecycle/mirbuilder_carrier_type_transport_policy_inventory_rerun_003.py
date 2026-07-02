#!/usr/bin/env python3
"""Inventory carrier/type transport gaps after MissingProjectionPolicy V4."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import build_resolution, type_transport_axis


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_EVIDENCE = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-003"
V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def labels_for_return_type(return_type: str) -> list[str]:
    text = return_type or "<unit>"
    labels: list[str] = []
    if "Result<" in text:
        labels.append("ResultCarrierEvidence")
    if "Option<" in text:
        labels.append("OptionCarrierEvidence")
    if "Vec<" in text or "BTreeSet<" in text or "BTreeMap<" in text:
        labels.append("CollectionCarrierEvidence")
    if text == "Self":
        labels.append("SelfConstructorTransportEvidence")
    if "&" in text:
        labels.append("BorrowInTypeTransportEvidence")
    if any(token in text for token in ["ValueId", "BasicBlockId", "MirType", "ASTNode", "Callee", "CarrierSets"]):
        labels.append("DomainObjectOrIdTransportEvidence")
    return sorted(set(labels)) or ["UnclassifiedTypeTransportEvidence"]


def lane_for(labels: list[str]) -> str:
    label_set = set(labels)
    if "ResultCarrierEvidence" in label_set:
        return "ResultCarrierPolicyCandidate"
    if "OptionCarrierEvidence" in label_set:
        return "OptionCarrierPolicyCandidate"
    if "CollectionCarrierEvidence" in label_set:
        return "CollectionCarrierPolicyCandidate"
    if "SelfConstructorTransportEvidence" in label_set:
        return "SelfConstructorTransportPolicyCandidate"
    return "CarrierTypeTransportEvidenceInventoryRequired"


def build_fixture() -> dict[str, Any]:
    v4 = read_json(V4)
    report = read_json(REPORT)
    resolution = build_resolution()
    type_missing_clusters = [
        cluster for cluster in resolution.get("clusters") or []
        if "TypeTransportMissing" in (cluster.get("blocked_by") or [])
    ]
    items = [
        item for item in report.get("items") or []
        if item.get("classification") == "MissingProjectionPolicy"
        and type_transport_axis(item) == "Missing"
    ]

    lane_counts: Counter[str] = Counter()
    label_counts: Counter[str] = Counter()
    return_type_counts: Counter[str] = Counter()
    rows: list[dict[str, Any]] = []
    for item in items:
        return_type = item.get("return_type") or "<unit>"
        labels = labels_for_return_type(return_type)
        lane = lane_for(labels)
        lane_counts[lane] += 1
        return_type_counts[return_type] += 1
        for label in labels:
            label_counts[label] += 1
        rows.append(
            {
                "source_id": item["source_id"],
                "known_owner_edge": item.get("known_owner_edge"),
                "return_type": return_type,
                "shape_signature": item.get("shape_signature"),
                "evidence_labels": labels,
                "policy_lane_candidate": lane,
                "selection_eligible": lane != "CarrierTypeTransportEvidenceInventoryRequired",
            }
        )

    eligible_lanes = sorted(lane for lane in lane_counts if lane != "CarrierTypeTransportEvidenceInventoryRequired")
    if len(eligible_lanes) == 1:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyLane",
            "reason_token": "ExactlyOneCarrierTypeTransportLaneEligible",
            "selected_policy_lane": eligible_lanes[0],
            "selected_next_card": f"MIRBUILDER-{eligible_lanes[0].upper()}-001",
        }
    else:
        decision = {
            "kind": "SelectCarrierTypeTransportEvidenceInventoryRerun003",
            "reason_token": "MultipleCarrierTypeTransportLanesRequireEvidenceInventory",
            "selected_policy_lane": None,
            "selected_next_card": NEXT_EVIDENCE,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportPolicyInventoryRerunV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "missing_projection_policy_cluster_resolution_v4": rel(V4),
            "unconverted_surface_report": rel(REPORT),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(V4),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "input_decision": v4.get("decision"),
        "transport_rows_sample": rows[:40],
        "summary": {
            "type_transport_missing_cluster_count": len(type_missing_clusters),
            "type_transport_missing_item_count": len(items),
            "policy_lane_candidate_counts": dict(sorted(lane_counts.items())),
            "evidence_label_counts": dict(sorted(label_counts.items())),
            "top_return_type_counts": dict(return_type_counts.most_common(20)),
            "eligible_policy_lane_count": len(eligible_lanes),
        },
        "selection_rule": {
            "consume_missing_projection_policy_v4_type_transport_gaps": True,
            "evidence_inventory_precedes_policy_when_multiple_lanes": True,
            "return_type_count_as_proof": False,
            "cluster_size_as_proof": False,
            "manual_carrier_selection": False,
            "worker_inventory_required_or_waived": True,
        },
        "decision": decision,
        "claims": {
            "missing_projection_policy_v4_consumed": 1,
            "carrier_type_transport_inventory_rerun_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "return_type_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-transport-policy-inventory-rerun-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
