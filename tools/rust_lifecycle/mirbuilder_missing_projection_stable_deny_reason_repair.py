#!/usr/bin/env python3
"""Assign stable deny reasons to MissingProjectionPolicy surfaces.

This repair consumes the owner-edge confidence repair result. It only assigns
medium-grained deny reasons for resolver clustering; it does not define a
projection policy, emit Hako, or select Source Selfhost.
"""

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
OWNER_EDGE_REPAIR = FIXTURES / "mirbuilder-owner-edge-confidence-repair-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-stable-deny-reason-repair-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def stable_reason_for(item: dict[str, Any]) -> str:
    if item.get("owner_edge_confidence") == "FixtureMapped":
        return "UnsupportedDirectShape"
    return "OwnerEdgeConfidenceMissing"


def build_repair() -> dict[str, Any]:
    report = read_json(REPORT)
    owner_edge_repair = read_json(OWNER_EDGE_REPAIR)
    missing_items = [
        item for item in report["items"]
        if item["classification"] == "MissingProjectionPolicy"
    ]

    reason_counts: dict[str, int] = {}
    confidence_counts: dict[str, int] = {}
    for item in missing_items:
        reason = stable_reason_for(item)
        reason_counts[reason] = reason_counts.get(reason, 0) + 1
        confidence = item.get("owner_edge_confidence") or "None"
        confidence_counts[confidence] = confidence_counts.get(confidence, 0) + 1

    rules = [
        {
            "match": {
                "classification": "MissingProjectionPolicy",
                "owner_edge_confidence": "FixtureMapped",
            },
            "stable_deny_reason": "UnsupportedDirectShape",
            "candidate_count": reason_counts.get("UnsupportedDirectShape", 0),
            "selection_eligible": True,
            "reason_token": "FixtureMappedProjectionPolicyStillLacksDirectShapePolicy",
        },
        {
            "match": {
                "classification": "MissingProjectionPolicy",
                "owner_edge_confidence": "None",
            },
            "stable_deny_reason": "OwnerEdgeConfidenceMissing",
            "candidate_count": reason_counts.get("OwnerEdgeConfidenceMissing", 0),
            "selection_eligible": False,
            "reason_token": "OwnerEdgeConfidenceMissingRemainsUnselectable",
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionStableDenyReasonRepairV1",
        "token": "MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001",
        "input_state": {
            "source_report": rel(REPORT),
            "owner_edge_confidence_repair": rel(OWNER_EDGE_REPAIR),
            "input_missing_projection_policy_count": len(missing_items),
            "owner_edge_repair_decision": owner_edge_repair["decision"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "reason_counts_hash": sha256_text(stable_json(reason_counts)),
            "owner_edge_confidence_counts_hash": sha256_text(stable_json(confidence_counts)),
        },
        "rules": rules,
        "summary": {
            "input_candidate_count": len(missing_items),
            "stable_deny_reason_counts_after_repair": dict(sorted(reason_counts.items())),
            "owner_edge_confidence_counts": dict(sorted(confidence_counts.items())),
            "selectable_stable_deny_reason_count": reason_counts.get("UnsupportedDirectShape", 0),
            "unselectable_stable_deny_reason_count": reason_counts.get("OwnerEdgeConfidenceMissing", 0),
        },
        "decision": {
            "kind": "ApplyStableDenyReasonRepair",
            "selected_next_card": "MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001",
            "reason_token": "StableDenyReasonRepairExposesShapeSignatureGap",
        },
        "claims": {
            "input_missing_projection_policy_count": len(missing_items),
            "stable_deny_reason_repair_defined": 1,
            "unsupported_direct_shape_count_after_repair": reason_counts.get("UnsupportedDirectShape", 0),
            "owner_edge_confidence_missing_count_after_repair": reason_counts.get("OwnerEdgeConfidenceMissing", 0),
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "family_name_based_policy": 0,
            "hako_emission": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in repair fixture.")
    args = parser.parse_args()

    output = stable_json(build_repair())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-stable-deny-reason-repair unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
