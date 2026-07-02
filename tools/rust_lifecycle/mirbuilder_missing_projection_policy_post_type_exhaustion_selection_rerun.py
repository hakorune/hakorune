#!/usr/bin/env python3
"""Apply MissingProjectionPolicy post-TypeTransport selector."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"

TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-missing-projection-policy-post-type-exhaustion-selection-basis-v0.json"
RERUN_005 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    rerun_005 = read_json(RERUN_005)
    lanes = basis.get("candidate_lanes") or []
    eligible = [lane for lane in lanes if lane.get("selection_eligible") is True]
    summary = basis.get("summary") or {}

    if len(eligible) == 1:
        lane = eligible[0]
        decision = {
            "kind": "SelectPostTypeExhaustionLane",
            "reason_token": "MissingProjectionPolicyPostTypeExhaustionLaneSelected",
            "selected_lane": lane.get("lane_id"),
            "selected_next_card": lane.get("selected_next_card_if_eligible"),
            "selected_projection_policy_cluster": None,
        }
    elif len(eligible) > 1:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleProjectionPolicyClustersAfterTypeTransportExhaustion",
            "selected_lane": None,
            "selected_next_card": DESIGN_STOP,
            "selected_projection_policy_cluster": None,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedMissingProjectionPolicyRerun005Lane",
            "selected_lane": None,
            "selected_next_card": DESIGN_STOP,
            "selected_projection_policy_cluster": None,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyPostTypeExhaustionSelectionRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "post_type_exhaustion_selection_basis": rel(BASIS),
            "missing_projection_policy_rerun_005": rel(RERUN_005),
        },
        "provenance": {
            "post_type_exhaustion_selection_basis_hash": sha256_file(BASIS),
            "missing_projection_policy_rerun_005_hash": sha256_file(RERUN_005),
        },
        "selector_rule": basis.get("selector_rule"),
        "post_type_transport_inventory": rerun_005.get("post_type_transport_inventory"),
        "candidate_lanes": lanes,
        "summary": {
            "candidate_lane_count": len(lanes),
            "selection_eligible_lane_count": len(eligible),
            "remaining_blocker_cluster_count": summary.get("remaining_blocker_cluster_count"),
            "remaining_blocker_candidate_count": summary.get("remaining_blocker_candidate_count"),
            "type_only_cluster_count": summary.get("type_only_cluster_count"),
            "type_only_candidate_count": summary.get("type_only_candidate_count"),
            "new_projection_policy_selected": 0,
        },
        "decision": decision,
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "new_projection_policy_selected": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
            "self_signed_fixture_as_proof": 0,
            "basis_010_exactly_one_wider_lane_as_projection_policy_proof": 0,
            "type_transport_exhausted_as_projection_policy_proof": 0,
            "type_only_cluster_direct_selection": 0,
            "owner_edge_repair_as_projection_policy_proof": 0,
            "shape_signature_inventory_as_projection_policy_proof": 0,
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
        print("mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
