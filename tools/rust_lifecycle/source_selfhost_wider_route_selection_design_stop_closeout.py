#!/usr/bin/env python3
"""Close out current Source Selfhost wider route-selection design stop."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-design-stop-closeout-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS_011 = FIXTURES / "source-selfhost-wider-route-selection-basis-011-v0.json"
LOCAL_POLICY = FIXTURES / "source-selfhost-local-candidate-selection-policy-v0.json"
POST_TYPE_RERUN = FIXTURES / "mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"
RERUN_005 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"
V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis_011 = read_json(BASIS_011)
    local_policy = read_json(LOCAL_POLICY)
    post_type = read_json(POST_TYPE_RERUN)
    post_summary = post_type.get("summary") or {}
    basis_summary = basis_011.get("summary") or {}

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionDesignStopCloseoutV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "current_latest_card": "SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001",
            "wider_route_selection_basis_011": rel(BASIS_011),
            "local_candidate_selection_policy": rel(LOCAL_POLICY),
            "missing_projection_policy_post_type_selection_rerun": rel(POST_TYPE_RERUN),
            "missing_projection_policy_rerun_005": rel(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4": rel(V4),
        },
        "provenance": {
            "wider_route_selection_basis_011_hash": sha256_file(BASIS_011),
            "local_candidate_selection_policy_hash": sha256_file(LOCAL_POLICY),
            "missing_projection_policy_post_type_selection_rerun_hash": sha256_file(POST_TYPE_RERUN),
            "missing_projection_policy_rerun_005_hash": sha256_file(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(V4),
        },
        "closeout_rule": {
            "name": "SourceSelfhostWiderRouteSelectionDesignStopCloseoutV1",
            "closeout_is_not_source_selfhost_claim": True,
            "closeout_is_not_hako_adoption": True,
            "closeout_is_not_native_seed_materialization": True,
            "closeout_parks_current_machine_derived_route_tree": True,
            "future_reentry_requires_new_authority_or_stable_input_delta": True,
            "do_not_invent_fresh_executable_owner_from_history": True,
            "future_candidate_selection_uses_local_policy": True,
        },
        "parked_or_exhausted_lanes": [
            {
                "lane_id": "DomainObjectIdLane",
                "parked": True,
                "park_reason_token": "ExplicitSemanticResourceDomainDeclarationSourceMissing",
                "safe_reentry_requires": [
                    "new explicit semantic resource-domain declaration source",
                    "new stable closed-resource manifest",
                    "new non-self-signed return-type taxonomy authority",
                ],
            },
            {
                "lane_id": "CarrierTypeRemainingAxisLane",
                "parked": True,
                "park_reason_token": "NoCarrierTypeComponentEvidenceSourceAuthority",
                "safe_reentry_requires": [
                    "new stable component policy contract",
                    "new explicit boundary declaration",
                    "new stable cross-lane handoff contract",
                    "new collection overlap contract",
                    "new typed direct closeout contract",
                ],
            },
            {
                "lane_id": "CarrierTypeParentPolicyLane",
                "parked": True,
                "park_reason_token": "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority",
                "safe_reentry_requires": [
                    "new current reusable policy contract",
                    "new verifier contract compatibility proof",
                    "new stable parent policy dependency root",
                    "new prior closed policy continuation contract",
                    "new cross-lane policy handoff contract",
                ],
            },
            {
                "lane_id": "MissingProjectionPolicyPostTypeTransportLane",
                "parked": True,
                "park_reason_token": "NoMachineDerivedMissingProjectionPolicyRerun005Lane",
                "remaining_blocker_cluster_count": post_summary.get("remaining_blocker_cluster_count"),
                "remaining_blocker_candidate_count": post_summary.get("remaining_blocker_candidate_count"),
                "type_only_cluster_count": post_summary.get("type_only_cluster_count"),
                "type_only_candidate_count": post_summary.get("type_only_candidate_count"),
                "projection_policy_selected": 0,
                "safe_reentry_requires": [
                    "new residual blocker root authority",
                    "new type-only projection policy selector authority",
                    "new projection descriptor / overlay freshness delta",
                ],
            },
        ],
        "basis_011_candidate_lanes": [
            {
                "lane_id": row.get("lane_id"),
                "selection_eligible": row.get("selection_eligible"),
                "reentry_condition": " / ".join(row.get("required_proof") or []),
            }
            for row in basis_011.get("candidate_lanes") or []
        ],
        "summary": {
            "current_machine_derived_progress_lane_count": 0,
            "parked_or_exhausted_lane_count": 4,
            "basis_011_candidate_lane_count": basis_summary.get("candidate_lane_count"),
            "basis_011_selection_eligible_progress_lane_count": basis_summary.get(
                "selection_eligible_progress_lane_count"
            ),
            "source_selfhost_status": "Stopped",
            "source_selfhost_claim": 0,
        },
        "reentry_policy": {
            "automatic_local_reentry_allowed": True,
            "worker_inventory_first": local_policy.get("policy_rule", {}).get("worker_inventory_first"),
            "external_consultation_only_for_new_authority": local_policy.get("policy_rule", {}).get(
                "external_consultation_only_for_new_authority"
            ),
            "allowed_only_when": [
                "stable input hash delta is detected",
                "new non-self-signed authority source is added",
                "new checker-verified contradiction invalidates closeout",
                "reviewer provides explicit design authority for a new proof axis",
            ],
            "reentry_must_open": [
                "freshness rerun",
                "authority inventory",
                "selector basis",
                "guard consolidation if guard is concrete blocker",
            ],
            "reentry_must_not_open": [
                "direct Source Selfhost claim",
                "direct HakoAdopted decision",
                "direct native seed materialization",
                "direct projection policy selection",
                "manual lane preference",
            ],
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "SourceSelfhostRouteSelectionExhaustedNoMachineDerivedNextLane",
            "selected_next_card": DESIGN_STOP,
            "selected_lane": None,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "source_selfhost_complete": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "projection_policy_selected": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_lane_selection": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "family_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "historical_preference_as_proof": 0,
            "lexical_order_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "self_signed_fixture_as_proof": 0,
            "route_exhaustion_as_source_selfhost_success": 0,
            "route_exhaustion_as_hako_adoption": 0,
            "route_exhaustion_as_native_seed_readiness": 0,
            "route_exhaustion_as_projection_policy_selection": 0,
            "route_exhaustion_as_owner_selection": 0,
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
        print("source-selfhost-wider-route-selection-design-stop-closeout unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
