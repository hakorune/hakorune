#!/usr/bin/env python3
"""Normalize carrier/type transport evidence after policy inventory rerun 003."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import type_transport_axis
from mirbuilder_carrier_type_transport_policy_inventory_rerun_003 import labels_for_return_type, lane_for


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_UNCLASSIFIED = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-002"
NEXT_PRIORITY = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-RERUN-003"
INPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    source = read_json(INPUT)
    report = read_json(REPORT)
    items = [
        item for item in report.get("items") or []
        if item.get("classification") == "MissingProjectionPolicy"
        and type_transport_axis(item) == "Missing"
    ]

    lane_counts: Counter[str] = Counter()
    label_counts: Counter[str] = Counter()
    source_counts: Counter[str] = Counter()
    rows_sample: list[dict[str, Any]] = []
    complete_count = 0

    for item in items:
        return_type = item.get("return_type") or "<unit>"
        labels = labels_for_return_type(return_type)
        lane = lane_for(labels)
        complete = lane != "CarrierTypeTransportEvidenceInventoryRequired"
        if complete:
            complete_count += 1
        lane_counts[lane] += 1
        for label in labels:
            label_counts[label] += 1
        for evidence_source in ["source_return_type", "shape_signature", "owner_edge_confidence"]:
            source_counts[evidence_source] += 1
        if len(rows_sample) < 60:
            rows_sample.append(
                {
                    "source_id": item["source_id"],
                    "known_owner_edge": item.get("known_owner_edge"),
                    "return_type": return_type,
                    "evidence_sources": ["source_return_type", "shape_signature", "owner_edge_confidence"],
                    "normalized_evidence_labels": labels,
                    "normalized_policy_lane_candidate": lane,
                    "evidence_inventory_complete": complete,
                }
            )

    unclassified = lane_counts.get("CarrierTypeTransportEvidenceInventoryRequired", 0)
    if unclassified:
        decision = {
            "kind": "SelectCarrierTypeTransportUnclassifiedEvidenceResolution",
            "reason_token": "UnclassifiedCarrierTypeTransportEvidenceRemainsAfterRerun003",
            "selected_next_card": NEXT_UNCLASSIFIED,
        }
    else:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyLanePriorityResolutionRerun003",
            "reason_token": "MultipleCarrierTypeTransportPolicyLanesRequirePriorityResolutionAfterRerun003",
            "selected_next_card": NEXT_PRIORITY,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportEvidenceInventoryRerunV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "carrier_type_transport_policy_inventory_rerun_003": rel(INPUT),
            "unconverted_surface_report": rel(REPORT),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(INPUT),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "input_decision": source.get("decision"),
        "evidence_rows_sample": rows_sample,
        "summary": {
            "input_candidate_count": len(items),
            "evidence_inventory_complete_count": complete_count,
            "unclassified_evidence_count": unclassified,
            "policy_lane_candidate_counts": dict(sorted(lane_counts.items())),
            "evidence_label_counts": dict(sorted(label_counts.items())),
            "evidence_source_counts": dict(sorted(source_counts.items())),
        },
        "selection_rule": {
            "source_return_type_is_evidence_not_policy": True,
            "return_type_count_as_proof": False,
            "manual_carrier_selection": False,
            "cluster_size_as_proof": False,
            "unclassified_evidence_blocks_policy_priority": True,
            "worker_inventory_required_or_waived": True,
        },
        "decision": decision,
        "claims": {
            "carrier_type_transport_policy_inventory_rerun_003_consumed": 1,
            "transport_evidence_inventory_ready": 1,
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
        print("mirbuilder-carrier-type-transport-evidence-inventory-rerun-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
