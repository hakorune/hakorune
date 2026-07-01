#!/usr/bin/env python3
"""Normalize carrier/type transport evidence before policy lane selection."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-v0.json"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-001"
NEXT = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def evidence_for(verifier: dict[str, Any]) -> tuple[list[str], list[str]]:
    sources: list[str] = []
    labels: set[str] = set()
    notes = verifier.get("transport_notes") or {}
    checks = verifier.get("checks") or {}
    operations = verifier.get("verified_operations") or []

    if notes:
        sources.append("transport_notes")
        text = json.dumps(notes, ensure_ascii=False, sort_keys=True)
        if re.search(r"ResultBox", text):
            labels.add("ResultBoxCarrierEvidence")
        if re.search(r"Option|PresenceTagged|presence", text):
            labels.add("OptionPresenceEvidence")
        if re.search(r"Array|Vec", text):
            labels.add("ArrayOrVecEvidence")
        if re.search(r"MapBox|OrderedMap|BTreeMap|Table|map_transport", text):
            labels.add("MapOrTableCarrierEvidence")
        if re.search(r"Shell|Handle|Prepared|Contract", text):
            labels.add("CustomCarrierEvidence")
        if re.search(r"Bool|bool|i64_bool", text):
            labels.add("BoolScalarTransportEvidence")
        if re.search(r"ScalarI64|ValueIdAsI64|BasicBlockIdAsI64|i64", text):
            labels.add("ScalarI64TransportEvidence")

    if operations:
        sources.append("verified_operations")
        ops = " ".join(operations)
        if re.search(r"I64Array|ArrayPush", ops):
            labels.add("ArrayOrVecEvidence")
        if re.search(r"ExplicitPhiI64|LocalI64|ReturnI64|ReturnSource", ops):
            labels.add("ScalarI64TransportEvidence")
        if re.search(r"ExplicitPhi", ops):
            labels.add("PhiCarrierEvidence")

    if checks:
        sources.append("checks")
        check_text = json.dumps(checks, ensure_ascii=False, sort_keys=True)
        if re.search(r"i64|ValueId|BasicBlockId", check_text):
            labels.add("ScalarI64TransportEvidence")
        if re.search(r"carrier_count|single_scalar_carrier", check_text):
            labels.add("CarrierShapeEvidence")
        if re.search(r"loop_carried_state|phi_required", check_text):
            labels.add("ControlCarrierBoundaryEvidence")

    return sorted(labels) or ["TransportEvidenceUnclassified"], sorted(set(sources))


def lane_for(labels: list[str]) -> str:
    label_set = set(labels)
    if "ResultBoxCarrierEvidence" in label_set:
        return "ResultCarrierVerifierPolicyCandidate"
    if "OptionPresenceEvidence" in label_set:
        return "OptionCarrierPolicyCandidate"
    if "ArrayOrVecEvidence" in label_set:
        return "VecOrArrayCarrierPolicyCandidate"
    if label_set & {"MapOrTableCarrierEvidence", "CustomCarrierEvidence", "CarrierShapeEvidence"}:
        return "GenericCarrierPolicyCandidate"
    if label_set & {"ScalarI64TransportEvidence", "BoolScalarTransportEvidence", "PhiCarrierEvidence"}:
        return "KnownTypeTransportNoCarrierPolicy"
    return "CarrierTypeTransportEvidenceUnclassified"


def build_fixture() -> dict[str, Any]:
    source = read_json(INPUT)
    rows: list[dict[str, Any]] = []
    lane_counts: Counter[str] = Counter()
    label_counts: Counter[str] = Counter()
    source_counts: Counter[str] = Counter()

    for row in source.get("transport_rows", []):
        verifier_path = ROOT / row["verifier_result_fixture"]
        verifier = read_json(verifier_path)
        labels, evidence_sources = evidence_for(verifier)
        lane = lane_for(labels)
        lane_counts[lane] += 1
        for label in labels:
            label_counts[label] += 1
        for evidence_source in evidence_sources:
            source_counts[evidence_source] += 1
        rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "family_id": row["family_id"],
                "verifier_result_fixture": row["verifier_result_fixture"],
                "input_transport_notes_state": row["transport_notes_state"],
                "evidence_sources": evidence_sources,
                "normalized_evidence_labels": labels,
                "normalized_policy_lane_candidate": lane,
                "evidence_inventory_complete": lane != "CarrierTypeTransportEvidenceUnclassified",
            }
        )

    unclassified = lane_counts.get("CarrierTypeTransportEvidenceUnclassified", 0)
    decision = {
        "kind": "SelectCarrierTypeTransportPolicyLanePriorityResolution",
        "reason_token": "MultipleCarrierTypeTransportPolicyLanesRequirePriorityResolution",
        "selected_next_card": NEXT,
    }
    if unclassified:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "UnclassifiedCarrierTypeTransportEvidenceRemains",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportEvidenceInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "carrier_type_transport_policy_inventory_rerun": rel(INPUT),
        },
        "provenance": {
            "carrier_type_transport_policy_inventory_rerun_hash": sha256_file(INPUT),
        },
        "evidence_rows": rows,
        "summary": {
            "input_candidate_count": len(rows),
            "input_transport_notes_missing_count": sum(
                1 for row in rows if row["input_transport_notes_state"] == "Missing"
            ),
            "evidence_inventory_complete_count": sum(
                1 for row in rows if row["evidence_inventory_complete"]
            ),
            "unclassified_evidence_count": unclassified,
            "policy_lane_candidate_counts": dict(sorted(lane_counts.items())),
            "evidence_label_counts": dict(sorted(label_counts.items())),
            "evidence_source_counts": dict(sorted(source_counts.items())),
        },
        "selection_rule": {
            "transport_notes_are_not_required_if_verifier_operations_or_checks_prove_transport": True,
            "owner_name_as_transport_policy": False,
            "manual_carrier_selection": False,
            "cluster_size_as_proof": False,
        },
        "decision": decision,
        "claims": {
            "carrier_type_transport_policy_inventory_rerun_consumed": 1,
            "transport_evidence_inventory_ready": 1,
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
        print("mirbuilder-carrier-type-transport-evidence-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
