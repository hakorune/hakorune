#!/usr/bin/env python3
"""Inventory shape-signature candidates for repaired Other owner rows."""

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
RERUN = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def shape_signature(subcluster: dict[str, Any]) -> str:
    ret = subcluster["return_family"]
    borrow = subcluster["borrow_axis"]
    type_axis = subcluster["type_transport_axis"]
    if ret == "result" and borrow == "NoReturnedBorrowMutableReceiver":
        return "shape.other_mutating_result_surface"
    if ret == "result":
        return "shape.other_result_helper_surface"
    if ret == "option":
        return "shape.other_optional_read_surface"
    if ret == "bool":
        return "shape.other_predicate_surface"
    if ret == "unit" and borrow == "NoReturnedBorrowMutableReceiver":
        return "shape.other_mutation_unit_surface"
    if ret == "unit":
        return "shape.other_unit_observer_surface"
    if ret == "constructor_self":
        return "shape.other_constructor_surface"
    if ret == "vec" or type_axis == "KnownVecCarrier":
        return "shape.other_vector_snapshot_surface"
    if ret in {"scalar", "string"}:
        return "shape.other_scalar_string_surface"
    if ret == "iterator" or type_axis == "ReturnedIteratorNeedsPolicy":
        return "shape.other_iterator_borrow_surface"
    return "shape.other_custom_carrier_surface"


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN)
    subclusters = rerun.get("subclusters", [])

    assignments: list[dict[str, Any]] = []
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for subcluster in subclusters:
        shape = shape_signature(subcluster)
        assignment = {
            "subcluster_id": subcluster["subcluster_id"],
            "known_owner_edge": subcluster["known_owner_edge"],
            "candidate_count": subcluster["candidate_count"],
            "prior_shape_signature": subcluster["shape_signature"],
            "candidate_shape_signature": shape,
            "return_family": subcluster["return_family"],
            "borrow_axis": subcluster["borrow_axis"],
            "type_transport_axis": subcluster["type_transport_axis"],
            "reason_token": "OtherShapeSignatureDerivedFromReturnBorrowTypeAxes",
            "selected_as_projection_policy": False,
        }
        assignments.append(assignment)
        grouped[shape].append(assignment)

    shape_signatures = []
    for shape, rows in sorted(grouped.items()):
        shape_signatures.append({
            "shape_signature": shape,
            "subcluster_count": len(rows),
            "candidate_count": sum(row["candidate_count"] for row in rows),
            "reason_token": "OtherShapeSignatureInventoryCandidate",
            "selected": False,
        })

    shape_counts = Counter(row["candidate_shape_signature"] for row in assignments)
    decision = {
        "kind": "SelectOtherShapeSignatureClusterResolution",
        "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001",
        "reason_token": "MultipleOtherShapeSignatureCandidates",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyOtherShapeSignatureInventoryV1",
        "token": TOKEN,
        "input_state": {
            "other_owner_cluster_rerun": rel(RERUN),
            "other_owner_cluster_rerun_decision": rerun.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_other_owner_cluster_count": rerun["summary"]["input_other_owner_cluster_count"],
            "input_subcluster_count": len(subclusters),
        },
        "provenance": {
            "other_owner_cluster_rerun_hash": sha256_file(RERUN),
        },
        "shape_assignment_policy": {
            "policy": "OtherShapeSignatureFromReturnBorrowTypeAxes",
            "semantic_projection_inference": 0,
            "family_name_based_policy": 0,
        },
        "shape_signatures": shape_signatures,
        "assignments": assignments,
        "summary": {
            "input_other_owner_cluster_count": rerun["summary"]["input_other_owner_cluster_count"],
            "input_subcluster_count": len(subclusters),
            "assigned_subcluster_count": len(assignments),
            "assigned_row_count": sum(row["candidate_count"] for row in assignments),
            "shape_signature_count": len(shape_signatures),
            "unknown_shape_count_after_inventory": 0,
            "shape_signature_counts": dict(sorted(shape_counts.items())),
        },
        "decision": decision,
        "claims": {
            "other_owner_cluster_rerun_consumed": 1,
            "input_other_owner_cluster_count": rerun["summary"]["input_other_owner_cluster_count"],
            "all_other_owner_subclusters_assigned_shape_candidate": 1,
            "unknown_shape_count_after_inventory": 0,
            "semantic_projection_inference": 0,
            "family_name_based_policy": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Other shape inventory fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-other-shape-signature-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
