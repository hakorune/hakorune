#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-011-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_011.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2095-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasis011V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("post_type_exhaustion_selection_rerun", "").endswith("mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"), "post-Type rerun input drift")
need(inputs.get("missing_projection_policy_rerun_005", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"), "rerun 005 input drift")
need(inputs.get("missing_projection_policy_cluster_resolution_v4", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"), "V4 input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_decision") == "KeepStopped", "previous decision drift")
need(previous.get("previous_reason_token") == "NoMachineDerivedMissingProjectionPolicyRerun005Lane", "previous reason drift")
need(previous.get("candidate_lane_count") == 4, "previous candidate count drift")
need(previous.get("selection_eligible_lane_count") == 0, "previous eligible count drift")
need(previous.get("residual_owner_edge_shape_lane_selection_eligible") is False, "residual lane drift")
need(previous.get("type_only_projection_policy_lane_selection_eligible") is False, "type-only lane drift")
need(previous.get("projection_descriptor_overlay_freshness_selection_eligible") is False, "freshness lane drift")
need(previous.get("remaining_blocker_cluster_count") == 5, "remaining cluster drift")
need(previous.get("remaining_blocker_candidate_count") == 185, "remaining row drift")
need(previous.get("type_only_cluster_count") == 73, "type-only cluster drift")
need(previous.get("type_only_candidate_count") == 819, "type-only row drift")

parked = {row.get("lane_id"): row for row in fixture.get("parked_lanes") or []}
lane = parked.get("MissingProjectionPolicyPostTypeTransportLane") or {}
need(lane.get("parked") is True, "post-Type lane must be parked")
need(lane.get("park_reason_token") == "NoMachineDerivedMissingProjectionPolicyRerun005Lane", "park reason drift")
need(lane.get("projection_policy_selected") == 0, "projection policy selection drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "PostMissingProjectionPolicyPostTypeTransportExhaustionWiderSelectorV1", "selector drift")
need(rule.get("park_missing_projection_post_type_lane_before_wider_selection") is True, "park rule drift")
need(rule.get("projection_policy_selection_forbidden") is True, "projection policy rule drift")
need(rule.get("select_only_if_exactly_one_machine_derived_wider_lane") is True, "exactly-one rule drift")
need(rule.get("keep_stopped_when_no_progress_lane_is_eligible") is True, "keep stopped rule drift")

lanes = fixture.get("candidate_lanes") or []
expected_lanes = {
    "NativeOwnerCheckpointRerun",
    "UnconvertedSurfaceReportRerun",
    "BorrowSurfacePolicyLane",
    "GuardConsolidation",
}
need({row.get("lane_id") for row in lanes} == expected_lanes, "candidate lane set drift")
need(not [row for row in lanes if row.get("selection_eligible") is True], "no progress lane may be eligible")

summary = fixture.get("summary") or {}
need(summary.get("missing_projection_post_type_lane_parked") == 1, "summary park drift")
need(summary.get("candidate_lane_count") == 4, "summary lane count drift")
need(summary.get("selection_eligible_progress_lane_count") == 0, "summary eligible drift")
need(summary.get("keep_stopped") == 1, "summary keep stopped drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoMachineDerivedPostMissingProjectionPolicyWiderLane", "reason drift")
need(decision.get("selected_lane") is None, "lane must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")
need(decision.get("selected_projection_policy_cluster") is None, "projection cluster must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "projection_policy_selected",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "row_count_as_proof",
    "cluster_size_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "historical_preference_as_proof",
    "basis_010_exactly_one_wider_lane_as_projection_policy_proof",
    "type_transport_exhausted_as_projection_policy_proof",
    "type_only_cluster_direct_selection",
    "owner_edge_repair_as_projection_policy_proof",
    "shape_signature_inventory_as_projection_policy_proof",
    "residual_blocker_count_as_root_proof",
    "type_only_cluster_count_as_root_proof",
    "freshness_rerun_as_semantic_priority",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows.get(token) or {}
need(row.get("card", "").endswith("2095-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011.md"), "manifest card drift")
need(row.get("fixture", "").endswith("source-selfhost-wider-route-selection-basis-011-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_source_selfhost_wider_route_selection_basis_011_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-source-selfhost-wider-route-selection-basis-011")
print("decision=KeepStopped")
print("reason_token=NoMachineDerivedPostMissingProjectionPolicyWiderLane")
print("missing_projection_post_type_lane_parked=1")
print("selection_eligible_progress_lane_count=0")
PY
