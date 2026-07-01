#!/usr/bin/env python3
"""Compute the Source Selfhost native-owner checkpoint."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-native-owner-checkpoint-v0.json"

TOKEN = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def native_owner_rows(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for row in manifest.get("rows") or []:
        token = str(row.get("token") or "")
        if not token.endswith("-HAKO-ADOPTION-DECISION-001"):
            continue
        fixture_path = ROOT / str(row.get("fixture"))
        if not fixture_path.exists():
            continue
        fixture = read_json(fixture_path)
        claims = fixture.get("claims") or {}
        decision = fixture.get("decision") or {}
        post = fixture.get("post_decision_state") or {}
        adopted = claims.get("hako_adopted") == 1 or decision.get("value") == "Adopt"
        native_owner = claims.get("native_hako_source_owner_present") == 1 or post.get("native_edit_authority") == 1
        if not (adopted and native_owner):
            continue
        rows.append(
            {
                "token": token,
                "fixture": row.get("fixture"),
                "owner_edge_id": fixture.get("family_id"),
                "family_scope": (fixture.get("target") or {}).get("family_scope"),
                "rust_role": post.get("rust_role"),
                "source_selfhost_claim": claims.get("source_selfhost_claim", 0),
            }
        )
    return sorted(rows, key=lambda item: item["owner_edge_id"] or "")


def blocker_quality(report: dict[str, Any]) -> dict[str, Any]:
    items = report.get("items") or []
    missing = [item for item in items if item.get("classification") == "MissingProjectionPolicy"]
    borrow = [item for item in items if item.get("classification") == "BorrowSurfaceNeedsPolicy"]
    missing_quality = [
        item for item in missing
        if item.get("owner_edge_confidence") in {"ExactSymbol", "FixtureMapped"}
        and bool(item.get("shape_signature"))
        and item.get("stable_deny_reason") not in {None, "OwnerEdgeConfidenceMissing"}
    ]
    borrow_quality = [
        item for item in borrow
        if item.get("owner_edge_confidence") in {"ExactSymbol", "FixtureMapped"}
    ]

    return {
        "MissingProjectionPolicy": {
            "candidate_count": len(missing),
            "evidence_quality_count": len(missing_quality),
            "selection_eligible": len(missing_quality) > 0,
            "reason_token": "MissingProjectionPolicyHasFixtureMappedKnownShapeEvidence"
            if missing_quality
            else "MissingProjectionPolicyRequiresEvidenceQuality",
            "next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2",
        },
        "BorrowSurfaceNeedsPolicy": {
            "candidate_count": len(borrow),
            "evidence_quality_count": len(borrow_quality),
            "selection_eligible": len(borrow_quality) > 0,
            "reason_token": "BorrowSurfaceNeedsPolicyOwnerConfidenceReady"
            if borrow_quality
            else "BorrowSurfaceNeedsOwnerEdgeConfidenceFirst",
            "next_card": "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-002",
        },
        "RouteRepairNeeded": {
            "candidate_count": 0,
            "evidence_quality_count": 0,
            "selection_eligible": False,
            "reason_token": "NoConcreteRouteRepairInFreshReport",
            "next_card": "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001",
        },
    }


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    manifest = read_json(MANIFEST)
    native_owners = native_owner_rows(manifest)
    blockers = blocker_quality(report)
    missing = blockers["MissingProjectionPolicy"]
    borrow = blockers["BorrowSurfaceNeedsPolicy"]

    if missing["selection_eligible"] and not borrow["selection_eligible"]:
        decision = {
            "kind": "SelectMissingProjectionPolicyClusterResolutionV2",
            "reason_token": "MissingProjectionPolicyEvidenceQualityWinsCheckpoint",
            "selected_blocker_class": "MissingProjectionPolicy",
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUniqueNativeOwnerCheckpointBlockerClass",
            "selected_blocker_class": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostNativeOwnerCheckpointV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "unconverted_surface_report": rel(REPORT),
            "source_selfhost_family_guard_manifest": rel(MANIFEST),
        },
        "provenance": {
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "source_selfhost_family_guard_manifest_hash": sha256_file(MANIFEST),
        },
        "native_owner_map": {
            "native_owner_count": len(native_owners),
            "owners": native_owners,
        },
        "blocker_class_evidence": blockers,
        "selection_rule": {
            "route_repair_precedes_policy_lanes": True,
            "fresh_report_required": True,
            "evidence_quality_precedes_candidate_count": True,
            "missing_projection_requires_fixture_mapped_known_shape": True,
            "borrow_surface_requires_owner_confidence": True,
            "source_selfhost_claim_allowed": False,
            "manual_blocker_class_selection": False,
        },
        "decision": decision,
        "claims": {
            "native_owner_checkpoint": 1,
            "source_selfhost_claim": 0,
            "rust_deletion": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "candidate_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
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
        print("source-selfhost-native-owner-checkpoint unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
