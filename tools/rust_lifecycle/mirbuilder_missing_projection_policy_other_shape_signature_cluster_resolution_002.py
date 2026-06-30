#!/usr/bin/env python3
"""Rerun Other shape-signature resolution after landed descriptors."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INVENTORY = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-002"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def shape_rows(inventory: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    rows: dict[str, list[dict[str, Any]]] = {}
    for assignment in inventory.get("assignments", []):
        rows.setdefault(assignment["candidate_shape_signature"], []).append(assignment)
    return rows


def has_any(rows: list[dict[str, Any]], key: str, values: set[str]) -> bool:
    return any(row[key] in values for row in rows)


def completed_other_shape_signatures(family_manifest: dict[str, Any]) -> list[str]:
    shapes: set[str] = set()
    for row in family_manifest.get("rows", []):
        token = row.get("token") or ""
        fixture_path = row.get("fixture") or ""
        if "PROJECTION-POLICY" not in token or not fixture_path:
            continue
        path = ROOT / fixture_path
        if not path.exists() or path == OUTPUT:
            continue
        try:
            fixture = read_json(path)
        except json.JSONDecodeError:
            continue
        input_state = fixture.get("input_state") or {}
        selected_shape = input_state.get("selected_shape_signature")
        claims = fixture.get("claims") or {}
        if (
            isinstance(selected_shape, str)
            and selected_shape.startswith("shape.other_")
            and claims.get("descriptor_selected") == 1
        ):
            shapes.add(selected_shape)
    return sorted(shapes)


def evaluate_shape(shape: str, rows: list[dict[str, Any]], completed_shapes: set[str]) -> dict[str, Any]:
    blocked_by: list[str] = []
    if shape in completed_shapes:
        blocked_by.append("ProjectionPolicyDescriptorAlreadyLanded")
    if has_any(rows, "type_transport_axis", {
        "ResultCarrierNeedsVerifier",
        "MissingTypeTransport",
        "ReturnedIteratorNeedsPolicy",
    }):
        blocked_by.append("TypeTransportOrVerifierGap")
    if has_any(rows, "borrow_axis", {
        "BorrowPolicyNeeded",
        "ReturnedMutableAliasUnknown",
        "NoReturnedBorrowMutableReceiver",
        "NoReturnedBorrowSharedReceiver",
    }):
        blocked_by.append("BorrowOrReceiverPolicyGap")
    if has_any(rows, "return_family", {
        "constructor_self",
        "custom_carrier",
        "iterator",
        "vec",
        "option",
        "result",
    }):
        blocked_by.append("CarrierPolicyGap")

    selection_eligible = not blocked_by
    return {
        "shape_signature": shape,
        "subcluster_count": len(rows),
        "candidate_count": sum(row["candidate_count"] for row in rows),
        "return_families": sorted({row["return_family"] for row in rows}),
        "borrow_axes": sorted({row["borrow_axis"] for row in rows}),
        "type_transport_axes": sorted({row["type_transport_axis"] for row in rows}),
        "selection_eligible": selection_eligible,
        "blocked_by": blocked_by,
        "reason_token": (
            "OtherShapeSignatureClusterEligible"
            if selection_eligible else
            "OtherShapeSignatureClusterNotSelectable"
        ),
    }


def next_card_for_shape(shape: str) -> str:
    name = shape.removeprefix("shape.").upper().replace("_", "-").replace(".", "-")
    return f"MIRBUILDER-{name}-PROJECTION-POLICY-001"


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    family_manifest = read_json(FAMILY_MANIFEST)
    rows_by_shape = shape_rows(inventory)
    completed_shapes = set(completed_other_shape_signatures(family_manifest))
    clusters = [
        evaluate_shape(shape, rows, completed_shapes)
        for shape, rows in sorted(rows_by_shape.items())
    ]
    eligible = [cluster for cluster in clusters if cluster["selection_eligible"]]

    if len(eligible) == 1:
        selected = eligible[0]
        decision = {
            "kind": "SelectOtherShapeSignatureProjectionPolicy",
            "selected_shape_signature": selected["shape_signature"],
            "selected_next_card": next_card_for_shape(selected["shape_signature"]),
            "reason_token": "ExactlyOneUnclosedOtherShapeSignatureClusterEligible",
        }
    elif len(eligible) > 1:
        decision = {
            "kind": "KeepStopped",
            "selected_shape_signature": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "AmbiguousUnclosedOtherShapeSignatureClusters",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_shape_signature": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoUnclosedOtherShapeSignatureClusterEligible",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyOtherShapeSignatureClusterResolutionV2",
        "token": TOKEN,
        "input_state": {
            "other_shape_signature_inventory": rel(INVENTORY),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_shape_signature_count": inventory["summary"]["shape_signature_count"],
            "input_other_owner_cluster_count": inventory["summary"]["input_other_owner_cluster_count"],
            "completed_other_shape_signatures": sorted(completed_shapes),
        },
        "provenance": {
            "other_shape_signature_inventory_hash": sha256_file(INVENTORY),
            "family_manifest_hash": sha256_file(FAMILY_MANIFEST),
        },
        "selection_rule": {
            "exclude_completed_other_shape_descriptors": 1,
            "requires_type_transport_known": 1,
            "requires_no_borrow_or_receiver_policy_gap": 1,
            "requires_no_carrier_policy_gap": 1,
            "cluster_size_as_proof": 0,
        },
        "clusters": clusters,
        "summary": {
            "input_shape_signature_count": inventory["summary"]["shape_signature_count"],
            "input_other_owner_cluster_count": inventory["summary"]["input_other_owner_cluster_count"],
            "completed_shape_signature_count": len(completed_shapes),
            "selection_eligible_shape_count": len(eligible),
            "selected_shape_signature": eligible[0]["shape_signature"] if len(eligible) == 1 else None,
        },
        "decision": decision,
        "claims": {
            "other_shape_signature_inventory_consumed": 1,
            "family_manifest_consumed": 1,
            "completed_other_shape_descriptors_excluded": 1,
            "input_shape_signature_count": inventory["summary"]["shape_signature_count"],
            "input_other_owner_cluster_count": inventory["summary"]["input_other_owner_cluster_count"],
            "shape_clusters_evaluated_by_evidence_quality": 1,
            "cluster_size_as_proof": 0,
            "manual_family_selection": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Other shape rerun fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
