#!/usr/bin/env python3
"""Rerun carrier/type transport policy inventory for BridgeBlocked gaps."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed
from mirbuilder_strict_converter_emission_native_seed_candidate_selection import build_fixture as build_selection


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001"
BRIDGE_GAP = FIXTURES / "mirbuilder-bridge-blocked-gap-cluster-resolution-v0.json"
RERUN_007 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json"
PRECEDENT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-v0.json"
NEXT_INVENTORY = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def labels_for(notes: dict[str, Any]) -> list[str]:
    if not notes:
        return ["TransportNotesMissing"]
    text = json.dumps(notes, ensure_ascii=False, sort_keys=True)
    labels: list[str] = []
    if re.search(r"ResultBox", text):
        labels.append("ResultBoxCarrierEvidence")
    if re.search(r"Option|PresenceTagged|presence", text):
        labels.append("OptionPresenceEvidence")
    if re.search(r"Array|Vec", text):
        labels.append("ArrayOrVecEvidence")
    if re.search(r"MapBox|OrderedMap|BTreeMap|Table|map_transport", text):
        labels.append("MapOrTableCarrierEvidence")
    if re.search(r"Shell|Handle|Prepared|Contract", text):
        labels.append("CustomCarrierEvidence")
    if re.search(r"Bool|bool|i64_bool", text):
        labels.append("BoolScalarTransportEvidence")
    if re.search(r"ScalarI64|ValueIdAsI64|BasicBlockIdAsI64|i64", text):
        labels.append("ScalarI64TransportEvidence")
    return sorted(set(labels)) or ["TransportNotesPresentUnclassified"]


def lane_for(labels: list[str]) -> str:
    label_set = set(labels)
    if "TransportNotesMissing" in label_set:
        return "TransportEvidenceInventoryRequired"
    if "ResultBoxCarrierEvidence" in label_set:
        return "ResultCarrierVerifierPolicyCandidate"
    if "OptionPresenceEvidence" in label_set:
        return "OptionCarrierPolicyCandidate"
    if "ArrayOrVecEvidence" in label_set:
        return "VecOrArrayCarrierPolicyCandidate"
    if label_set & {"MapOrTableCarrierEvidence", "CustomCarrierEvidence"}:
        return "GenericCarrierPolicyCandidate"
    if label_set <= {"BoolScalarTransportEvidence", "ScalarI64TransportEvidence"}:
        return "KnownTypeTransportNoCarrierPolicy"
    return "CarrierTypeTransportEvidenceInventoryRequired"


def carrier_only_candidates() -> list[dict[str, Any]]:
    selection = build_selection(
        cutoff_token="MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007"
    )
    return [
        item for item in selection.get("candidates", [])
        if item.get("blocked_by") == ["PolicyGapInDeniedBoundaries"]
        and item.get("carrier_type_transport_gap") is True
        and item.get("borrow_policy_gap") is False
    ]


def build_fixture() -> dict[str, Any]:
    bridge_gap = read_json(BRIDGE_GAP)
    rerun = read_json(RERUN_007)
    precedent = read_json(PRECEDENT)
    candidates = carrier_only_candidates()

    rows: list[dict[str, Any]] = []
    lane_counts: Counter[str] = Counter()
    label_counts: Counter[str] = Counter()
    missing_notes_count = 0

    for item in candidates:
        verifier_path = ROOT / item["verifier_result_fixture"]
        verifier = read_json(verifier_path)
        notes = verifier.get("transport_notes") or {}
        labels = labels_for(notes)
        lane = lane_for(labels)
        lane_counts[lane] += 1
        for label in labels:
            label_counts[label] += 1
        if not notes:
            missing_notes_count += 1
        rows.append(
            {
                "owner_edge_id": item["owner_edge_id"],
                "family_id": item["family_id"],
                "verifier_result_fixture": rel(verifier_path),
                "transport_notes_state": "Present" if notes else "Missing",
                "evidence_labels": labels,
                "policy_lane_candidate": lane,
                "selection_eligible": lane not in {
                    "TransportEvidenceInventoryRequired",
                    "CarrierTypeTransportEvidenceInventoryRequired",
                    "KnownTypeTransportNoCarrierPolicy",
                },
            }
        )

    eligible_lanes = sorted(
        lane for lane, count in lane_counts.items()
        if count > 0 and lane not in {
            "TransportEvidenceInventoryRequired",
            "CarrierTypeTransportEvidenceInventoryRequired",
            "KnownTypeTransportNoCarrierPolicy",
        }
    )
    if len(eligible_lanes) == 1 and missing_notes_count == 0:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyLane",
            "reason_token": "ExactlyOneCarrierTypeTransportPolicyLaneEligible",
            "selected_policy_lane": eligible_lanes[0],
            "selected_next_card": f"MIRBUILDER-{eligible_lanes[0].upper()}-001",
        }
    else:
        decision = {
            "kind": "SelectCarrierTypeTransportEvidenceInventory",
            "reason_token": "CarrierTypeTransportEvidenceRequiresInventoryBeforePolicyLane",
            "selected_policy_lane": None,
            "selected_next_card": NEXT_INVENTORY,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportPolicyInventoryRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "bridge_gap_cluster_resolution": rel(BRIDGE_GAP),
            "native_owner_seed_rerun_007": rel(RERUN_007),
            "precedent_inventory": rel(PRECEDENT),
            "selected_cluster_id": "bridge_gap::carrier_type_transport_only",
        },
        "provenance": {
            "bridge_gap_cluster_resolution_hash": sha256_file(BRIDGE_GAP),
            "native_owner_seed_rerun_007_hash": sha256_file(RERUN_007),
            "precedent_inventory_hash": sha256_file(PRECEDENT),
        },
        "input_decision": bridge_gap["decision"],
        "input_candidate_pool": rerun["candidate_pool"],
        "precedent_inventory_summary": precedent["summary"],
        "transport_rows": rows,
        "summary": {
            "carrier_type_transport_only_count": len(candidates),
            "mixed_borrow_carrier_type_transport_count": 1,
            "transport_notes_missing_count": missing_notes_count,
            "policy_lane_candidate_counts": dict(sorted(lane_counts.items())),
            "evidence_label_counts": dict(sorted(label_counts.items())),
            "eligible_policy_lane_count": len(eligible_lanes),
        },
        "selection_rule": {
            "consume_bridge_gap_carrier_type_transport_only": True,
            "defer_mixed_borrow_carrier_gap": True,
            "owner_name_as_transport_policy": False,
            "cluster_size_as_proof": False,
            "manual_carrier_selection": False,
        },
        "decision": decision,
        "claims": {
            "bridge_gap_cluster_resolution_consumed": 1,
            "carrier_type_transport_inventory_rerun_ready": 1,
            "mixed_gap_deferred": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "owner_name_as_transport_policy": 0,
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
        print("mirbuilder-carrier-type-transport-policy-inventory-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
