#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_missing_projection_policy_cluster_resolution_rerun_005.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2092-MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[3], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005"
next_card = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001"

need(fixture.get("kind") == "MirBuilderMissingProjectionPolicyClusterResolutionRerun005V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("missing_projection_policy_cluster_resolution_v4", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"), "V4 input drift")
need(inputs.get("wider_route_selection_basis_010", "").endswith("source-selfhost-wider-route-selection-basis-010-v0.json"), "BASIS-010 input drift")

exhaustion = fixture.get("type_transport_exhaustion_state") or {}
need(exhaustion.get("TypeTransportMissing_treated_as") == "ParkedExhausted", "TypeTransport state drift")
need(exhaustion.get("type_transport_missing_silently_deleted") is False, "TypeTransport must not be silently deleted")
need(exhaustion.get("basis_010_reason_token") == "CarrierTypeParentPolicyLaneExhaustedReturnToMissingProjectionPolicy", "BASIS-010 reason drift")

inventory = fixture.get("post_type_transport_inventory") or {}
need(inventory.get("input_candidate_count") == 1004, "candidate count drift")
need(inventory.get("input_cluster_count") == 78, "cluster count drift")
need(inventory.get("type_transport_missing_cluster_count_from_v4") == 76, "TypeTransport V4 count drift")
need(inventory.get("type_transport_missing_treated_as_parked_count") == 76, "TypeTransport parked count drift")
need(inventory.get("remaining_blocker_cluster_count") == 5, "remaining cluster count drift")
need(inventory.get("remaining_blocker_candidate_count") == 185, "remaining candidate count drift")
need(inventory.get("remaining_blocker_classes") == ["NoExactOrFixtureMappedOwnerEdge", "MissingShapeSignatureClusterAxis"], "remaining blocker class drift")
need(inventory.get("type_only_cluster_count") == 73, "type-only cluster count drift")
need(inventory.get("type_only_candidate_count") == 819, "type-only candidate count drift")
need(inventory.get("type_only_clusters_are_directly_selectable") is False, "type-only direct selection drift")

remaining = inventory.get("remaining_blocker_clusters") or []
need(len(remaining) == 5, "remaining blocker rows drift")
need(sum(row.get("candidate_count", 0) for row in remaining) == 185, "remaining row sum drift")
for row in remaining:
    need(row.get("owner_edge_confidence") == "None", "remaining owner edge drift")
    need(row.get("shape_signature") == "unknown_shape", "remaining shape drift")
    need(row.get("post_type_blocked_by") == ["MissingShapeSignatureClusterAxis", "NoExactOrFixtureMappedOwnerEdge"], "post-Type blocker drift")

selector = fixture.get("selector_state") or {}
for key in [
    "basis_010_exactly_one_wider_lane_as_projection_policy_proof",
    "type_transport_exhausted_as_projection_policy_proof",
    "type_only_cluster_direct_selection",
    "owner_edge_repair_as_projection_policy_proof",
    "shape_signature_inventory_as_projection_policy_proof",
]:
    need(selector.get(key) == 0, f"selector non-claim drift: {key}")
need(selector.get("post_type_exhaustion_selector_defined") is False, "selector must not be defined here")
need(selector.get("exactly_one_machine_derived_projection_policy_lane") is False, "exactly-one policy lane drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectPostTypeExhaustionSelectionBasis", "decision kind drift")
need(decision.get("reason_token") == "PostTypeTransportExhaustionSelectorBasisRequired", "decision reason drift")
need(decision.get("selected_cluster_id") is None, "cluster must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "new_projection_policy_selected",
    "manual_lane_selection",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "row_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "family_name_as_proof",
    "route_membership_alone_as_proof",
    "historical_preference_as_proof",
    "self_signed_fixture_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows.get(token) or {}
need(row.get("card", "").endswith("2092-MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005.md"), "manifest card drift")
need(row.get("fixture", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_missing_projection_policy_cluster_resolution_rerun_005_guard.sh"), "manifest guard drift")

print("output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-cluster-resolution-rerun-005")
print("remaining_blocker_cluster_count=5")
print("remaining_blocker_candidate_count=185")
print("type_only_cluster_count=73")
print("type_only_candidate_count=819")
print("selected_next_card=" + next_card)
PY
