#!/usr/bin/env python3
"""Resolve MissingProjectionPolicy after the native-owner checkpoint."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v2-v0.json"

TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
CHECKPOINT = FIXTURES / "source-selfhost-native-owner-checkpoint-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY_RESOLUTION = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    checkpoint = read_json(CHECKPOINT)
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY_RESOLUTION)
    report_summary = report.get("summary") or {}
    priority_claims = priority.get("claims") or {}

    all_eligible_landed = (
        priority_claims.get("eligible_cluster_count") == 41
        and priority_claims.get("excluded_existing_decision_cluster_count") == 41
        and priority_claims.get("selectable_cluster_count") == 0
    )

    if all_eligible_landed:
        decision = {
            "kind": "SelectProjectionDescriptorCoverageReclassification",
            "reason_token": "ProjectionPolicyClustersAlreadyLandedButReportStillMissing",
            "selected_next_card": "MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MissingProjectionPolicyClusterStateAmbiguous",
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyClusterResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "native_owner_checkpoint": rel(CHECKPOINT),
            "unconverted_surface_report": rel(REPORT),
            "cluster_resolution_v1": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY_RESOLUTION),
        },
        "provenance": {
            "native_owner_checkpoint_hash": sha256_file(CHECKPOINT),
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "cluster_resolution_v1_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY_RESOLUTION),
        },
        "input_checkpoint": checkpoint.get("decision"),
        "report_state": {
            "missing_projection_policy_count": report_summary.get("missing_projection_policy_count"),
            "borrow_policy_needed_count": report_summary.get("borrow_policy_needed_count"),
        },
        "cluster_state": {
            "input_candidate_count": (cluster_resolution.get("summary") or {}).get("input_candidate_count"),
            "selection_eligible_cluster_count": (cluster_resolution.get("summary") or {}).get("selection_eligible_cluster_count"),
            "priority_eligible_cluster_count": priority_claims.get("eligible_cluster_count"),
            "excluded_existing_decision_cluster_count": priority_claims.get("excluded_existing_decision_cluster_count"),
            "selectable_cluster_count": priority_claims.get("selectable_cluster_count"),
        },
        "resolution": {
            "eligible_projection_policy_clusters_already_landed": all_eligible_landed,
            "report_reclassification_required": all_eligible_landed,
            "new_projection_policy_selection_allowed": False,
            "candidate_count_as_proof": False,
        },
        "decision": decision,
        "claims": {
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "candidate_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "new_projection_policy_selected": 0,
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
        print("mirbuilder-missing-projection-policy-cluster-resolution-v2 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
