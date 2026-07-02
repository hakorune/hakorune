#!/usr/bin/env python3
"""Resolve unclassified carrier/type evidence after evidence rerun 003."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import type_transport_axis
from mirbuilder_carrier_type_transport_policy_inventory_rerun_003 import labels_for_return_type, lane_for
from mirbuilder_carrier_type_transport_unclassified_evidence_resolution import unclassified_axis


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_DOMAIN = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002"
EVIDENCE = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    evidence = read_json(EVIDENCE)
    report = read_json(REPORT)
    items = [
        item for item in report.get("items") or []
        if item.get("classification") == "MissingProjectionPolicy"
        and type_transport_axis(item) == "Missing"
        and lane_for(labels_for_return_type(item.get("return_type") or "<unit>"))
        == "CarrierTypeTransportEvidenceInventoryRequired"
    ]

    axis_counts: Counter[str] = Counter()
    return_type_counts: Counter[str] = Counter()
    rows_sample: list[dict[str, Any]] = []
    for item in items:
        return_type = item.get("return_type") or "<unit>"
        axis = unclassified_axis(return_type)
        axis_counts[axis] += 1
        return_type_counts[return_type] += 1
        if len(rows_sample) < 60:
            rows_sample.append(
                {
                    "source_id": item["source_id"],
                    "known_owner_edge": item.get("known_owner_edge"),
                    "return_type": return_type,
                    "unclassified_axis": axis,
                    "shape_signature": item.get("shape_signature"),
                }
            )

    if axis_counts.get("DomainObjectOrIdTransportAxis", 0) > 0:
        decision = {
            "kind": "SelectDomainObjectIdTransportPolicyInventoryRerun002",
            "reason_token": "DomainObjectIdTransportAxisIsPureTypeTransport",
            "selected_axis": "DomainObjectOrIdTransportAxis",
            "selected_next_card": NEXT_DOMAIN,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedUnclassifiedTransportAxis",
            "selected_axis": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportUnclassifiedEvidenceResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "carrier_type_transport_evidence_inventory_rerun_003": rel(EVIDENCE),
            "unconverted_surface_report": rel(REPORT),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "carrier_type_transport_evidence_inventory_rerun_003_hash": sha256_file(EVIDENCE),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "input_decision": evidence.get("decision"),
        "axis_rows_sample": rows_sample,
        "summary": {
            "unclassified_input_count": len(items),
            "resolved_axis_count": len(axis_counts),
            "axis_counts": dict(sorted(axis_counts.items())),
            "top_return_type_counts": dict(return_type_counts.most_common(30)),
        },
        "selection_rule": {
            "domain_object_id_axis_is_pure_type_transport": True,
            "iterator_or_borrow_axis_not_selected_by_default": True,
            "collection_axis_not_selected_by_count": True,
            "tuple_axis_not_selected_by_count": True,
            "return_type_count_as_proof": False,
            "manual_axis_selection": False,
            "worker_inventory_required_or_waived": True,
        },
        "decision": decision,
        "claims": {
            "carrier_type_transport_evidence_inventory_rerun_003_consumed": 1,
            "unclassified_evidence_resolution_ready": 1,
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
        print("mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
