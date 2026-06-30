#!/usr/bin/env python3
"""Materialize the Other unit observer surface projection-policy descriptor."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
INVENTORY = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json"
RESOLUTION = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-v0.json"
RERUN = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json"
OUTPUT = FIXTURES / "mirbuilder-other-unit-observer-surface-projection-policy-v0.json"
TOKEN = "MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001"
SHAPE = "shape.other_unit_observer_surface"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def report_items_by_source_id(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {item["source_id"]: item for item in report.get("items", [])}


def selected_assignments(inventory: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in inventory.get("assignments", [])
        if item["candidate_shape_signature"] == SHAPE
    ]


def selected_source_ids(assignments: list[dict[str, Any]], rerun: dict[str, Any]) -> list[str]:
    by_subcluster = {item["subcluster_id"]: item for item in rerun.get("subclusters", [])}
    source_ids: list[str] = []
    for assignment in assignments:
        source_ids.extend(by_subcluster[assignment["subcluster_id"]]["source_ids"])
    return sorted(source_ids)


def read_source_marker(item: dict[str, Any]) -> str:
    path = ROOT / item["source_path"]
    line = int(item["line"])
    lines = path.read_text(encoding="utf-8").splitlines()
    start = max(0, line - 1)
    snippet = "\n".join(lines[start:start + 4])
    if item["symbol"] not in snippet:
        raise SystemExit(f"source marker drift for {item['source_id']}")
    return snippet.strip()


def build_policy() -> dict[str, Any]:
    report = read_json(REPORT)
    inventory = read_json(INVENTORY)
    resolution = read_json(RESOLUTION)
    rerun = read_json(RERUN)

    decision = resolution["decision"]
    if decision.get("selected_shape_signature") != SHAPE or decision.get("selected_next_card") != TOKEN:
        raise SystemExit("shape cluster resolution does not select Other unit observer policy")

    assignments = selected_assignments(inventory)
    source_ids = selected_source_ids(assignments, rerun)
    items_by_id = report_items_by_source_id(report)
    surfaces = []
    for source_id in source_ids:
        item = items_by_id[source_id]
        if item.get("return_type") not in {"", None}:
            raise SystemExit(f"unit observer surface returned non-unit: {source_id}")
        if item.get("receiver") not in {"None", None, ""}:
            raise SystemExit(f"unit observer surface has receiver policy gap: {source_id}")
        surfaces.append({
            "source_id": source_id,
            "symbol": item["symbol"],
            "source_path": item["source_path"],
            "line": item["line"],
            "visibility": item["visibility"],
            "receiver": item.get("receiver"),
            "params": item.get("params"),
            "return_type": item.get("return_type") or "",
            "known_owner_edge": next(
                assignment["known_owner_edge"]
                for assignment in assignments
                if source_id in rerun_subcluster_source_ids(assignment["subcluster_id"], rerun)
            ),
            "owner_edge_confidence": "FileScoped",
            "source_marker": read_source_marker(item),
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderOtherUnitObserverSurfaceProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "shape_signature_cluster_resolution": rel(RESOLUTION),
            "shape_signature_inventory": rel(INVENTORY),
            "unconverted_surface_report": rel(REPORT),
            "selected_shape_signature": SHAPE,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "shape_signature_cluster_resolution_hash": sha256_file(RESOLUTION),
            "shape_signature_inventory_hash": sha256_file(INVENTORY),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "selection_axes": {
            "shape_signature": SHAPE,
            "return_family": "unit",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
            "owner_edge_confidence": "FileScoped",
        },
        "source_surfaces": surfaces,
        "unit_observer_descriptor": {
            "descriptor_id": "other_unit_observer_surface_v1",
            "source_extraction": "rust_unit_observer_no_return_no_receiver",
            "return_contract": "unit",
            "mutation_frame": [],
            "returned_borrow": 0,
            "receiver_borrow": 0,
            "type_transport": "KnownUnit",
            "projection_semantics": "observe_or_annotate_without_value_transport",
        },
        "selected_policy": {
            "policy": "OtherUnitObserverSurfaceDescriptor",
            "descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "OtherUnitObserverDescriptorRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectProjectionPolicyDescriptor",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "OtherUnitObserverSurfaceDescriptorMaterialized",
        },
        "claims": {
            "shape_signature_cluster_resolution_consumed": 1,
            "shape_signature_inventory_consumed": 1,
            "unconverted_surface_report_consumed": 1,
            "source_count": len(surfaces),
            "descriptor_selected": 1,
            "hako_projection_selected": 0,
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
        "provenance_role": {
            "tool_role": "FactsAdapterGuardOrchestrator",
            "semantic_projection_inference": 0,
        },
    }


def rerun_subcluster_source_ids(subcluster_id: str, rerun: dict[str, Any]) -> list[str]:
    for subcluster in rerun.get("subclusters", []):
        if subcluster["subcluster_id"] == subcluster_id:
            return subcluster["source_ids"]
    raise SystemExit(f"missing rerun subcluster: {subcluster_id}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-other-unit-observer-surface-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
