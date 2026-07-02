#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-design-stop-closeout-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_wider_route_selection_design_stop_closeout.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2097-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001.md"
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


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionDesignStopCloseoutV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("current_latest_card") == "SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001", "latest input drift")
need(inputs.get("wider_route_selection_basis_011", "").endswith("source-selfhost-wider-route-selection-basis-011-v0.json"), "BASIS-011 input drift")
need(inputs.get("local_candidate_selection_policy", "").endswith("source-selfhost-local-candidate-selection-policy-v0.json"), "local policy input drift")

rule = fixture.get("closeout_rule") or {}
for key in [
    "closeout_is_not_source_selfhost_claim",
    "closeout_is_not_hako_adoption",
    "closeout_is_not_native_seed_materialization",
    "closeout_parks_current_machine_derived_route_tree",
    "future_reentry_requires_new_authority_or_stable_input_delta",
    "do_not_invent_fresh_executable_owner_from_history",
    "future_candidate_selection_uses_local_policy",
]:
    need(rule.get(key) is True, f"closeout rule drift: {key}")

lanes = {row.get("lane_id"): row for row in fixture.get("parked_or_exhausted_lanes") or []}
expected_lanes = {
    "DomainObjectIdLane",
    "CarrierTypeRemainingAxisLane",
    "CarrierTypeParentPolicyLane",
    "MissingProjectionPolicyPostTypeTransportLane",
}
need(set(lanes) == expected_lanes, "parked lane set drift")
for lane_id, row in lanes.items():
    need(row.get("parked") is True, f"lane must be parked: {lane_id}")
    need(row.get("park_reason_token"), f"park reason missing: {lane_id}")
    need(row.get("safe_reentry_requires"), f"reentry conditions missing: {lane_id}")
missing = lanes["MissingProjectionPolicyPostTypeTransportLane"]
need(missing.get("remaining_blocker_cluster_count") == 5, "remaining cluster drift")
need(missing.get("remaining_blocker_candidate_count") == 185, "remaining row drift")
need(missing.get("type_only_cluster_count") == 73, "type-only cluster drift")
need(missing.get("type_only_candidate_count") == 819, "type-only row drift")
need(missing.get("projection_policy_selected") == 0, "projection policy selection drift")

basis_lanes = fixture.get("basis_011_candidate_lanes") or []
need(len(basis_lanes) == 4, "BASIS-011 candidate lane count drift")
need(not [row for row in basis_lanes if row.get("selection_eligible") is True], "BASIS-011 lane eligibility drift")

summary = fixture.get("summary") or {}
need(summary.get("current_machine_derived_progress_lane_count") == 0, "progress lane count drift")
need(summary.get("parked_or_exhausted_lane_count") == 4, "parked lane count drift")
need(summary.get("basis_011_candidate_lane_count") == 4, "summary basis lane count drift")
need(summary.get("basis_011_selection_eligible_progress_lane_count") == 0, "summary eligibility drift")
need(summary.get("source_selfhost_status") == "Stopped", "Source Selfhost status drift")
need(summary.get("source_selfhost_claim") == 0, "Source Selfhost claim drift")

reentry = fixture.get("reentry_policy") or {}
need(reentry.get("automatic_local_reentry_allowed") is True, "automatic local reentry drift")
need(reentry.get("worker_inventory_first") is True, "worker-first drift")
need(reentry.get("external_consultation_only_for_new_authority") is True, "external gate drift")
for key in [
    "stable input hash delta is detected",
    "new non-self-signed authority source is added",
    "new checker-verified contradiction invalidates closeout",
    "reviewer provides explicit design authority for a new proof axis",
]:
    need(key in (reentry.get("allowed_only_when") or []), f"reentry allow condition missing: {key}")
for key in [
    "direct Source Selfhost claim",
    "direct HakoAdopted decision",
    "direct native seed materialization",
    "direct projection policy selection",
    "manual lane preference",
]:
    need(key in (reentry.get("reentry_must_not_open") or []), f"reentry forbidden missing: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "SourceSelfhostRouteSelectionExhaustedNoMachineDerivedNextLane", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next card drift")
need(decision.get("selected_lane") is None, "lane must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "source_selfhost_complete",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "projection_policy_selected",
    "generated_artifact_as_native_edit_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
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
    "lexical_order_as_proof",
    "apparent_simplicity_as_proof",
    "self_signed_fixture_as_proof",
    "route_exhaustion_as_source_selfhost_success",
    "route_exhaustion_as_hako_adoption",
    "route_exhaustion_as_native_seed_readiness",
    "route_exhaustion_as_projection_policy_selection",
    "route_exhaustion_as_owner_selection",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows.get(token) or {}
need(row.get("card", "").endswith("2097-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001.md"), "manifest card drift")
need(row.get("fixture", "").endswith("source-selfhost-wider-route-selection-design-stop-closeout-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_source_selfhost_wider_route_selection_design_stop_closeout_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-source-selfhost-wider-route-selection-design-stop-closeout")
print("decision=KeepStopped")
print("reason_token=SourceSelfhostRouteSelectionExhaustedNoMachineDerivedNextLane")
print("parked_or_exhausted_lane_count=4")
print("source_selfhost_claim=0")
PY
