#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-010-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_010.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2091-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010.md"
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


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010"
next_card = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasis010V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("carrier_type_parent_policy_evidence_source_discovery_inventory", "").endswith("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory-v0.json"), "parent inventory input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_decision") == "SelectWiderRouteSelectionBasis", "previous decision drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority", "previous reason drift")
need(previous.get("candidate_parent_policy_lane_count") == 4, "candidate count drift")
for key in [
    "accepted_parent_policy_evidence_source_count",
    "parent_policy_authority_source_count",
    "parent_policy_lane_with_accepted_source_count",
    "current_reusable_policy_contract_count",
    "current_verifier_contract_compatibility_count",
    "stable_parent_policy_dependency_root_count",
    "prior_closed_policy_continuation_contract_count",
    "cross_lane_policy_handoff_contract_count",
    "parent_policy_candidate_selection",
    "result_history_as_direct_selection_proof",
]:
    need(previous.get(key) == 0, f"previous zero drift: {key}")

parked = {row.get("lane_id"): row for row in fixture.get("parked_lanes") or []}
for lane in ["DomainObjectIdLane", "CarrierTypeRemainingAxisLane", "CarrierTypeParentPolicyLane"]:
    need(parked.get(lane, {}).get("parked") is True, f"park drift: {lane}")
need(parked["CarrierTypeParentPolicyLane"].get("park_reason_token") == "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority", "parent park reason drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "PostCarrierTypeParentPolicyAuthorityExhaustionWiderLaneSelectorV1", "selector drift")
need(rule.get("park_carrier_type_parent_policy_lane_before_wider_selection") is True, "park rule drift")
need(rule.get("direct_parent_policy_candidate_selection_forbidden") is True, "direct policy rule drift")
need(rule.get("result_history_as_direct_selection_proof") is False, "result history rule drift")
need(rule.get("select_only_if_exactly_one_machine_derived_lane") is True, "exactly-one rule drift")

candidate_lanes = fixture.get("candidate_lanes") or []
expected_lanes = {
    "UnconvertedSurfaceReportRerun",
    "NativeOwnerCheckpointRerun",
    "MissingProjectionPolicyNextLane",
    "BorrowSurfacePolicyLane",
    "GuardConsolidation",
}
need({row.get("lane_id") for row in candidate_lanes} == expected_lanes, "candidate lane set drift")
eligible = [row for row in candidate_lanes if row.get("selection_eligible")]
need(len(eligible) == 1, "eligible lane count drift")
need(eligible[0].get("lane_id") == "MissingProjectionPolicyNextLane", "eligible lane drift")
need(eligible[0].get("selected_next_card_if_eligible") == next_card, "eligible next drift")

blocked = {row.get("lane_id"): row for row in fixture.get("blocked_reopen_lanes") or []}
need(blocked.get("ParentPolicyAuthorityReopen", {}).get("blocked") is True, "authority reopen blocker drift")
need(blocked.get("CurrentResultCompatibility", {}).get("blocked") is True, "Result compatibility blocker drift")
need(blocked.get("CurrentResultCompatibility", {}).get("result_history_as_direct_selection_proof") == 0, "Result history blocker drift")

summary = fixture.get("summary") or {}
need(summary.get("carrier_type_parent_policy_lane_parked") == 1, "summary park drift")
need(summary.get("candidate_lane_count") == 5, "summary candidate count drift")
need(summary.get("selection_eligible_lane_count") == 1, "summary eligible count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMissingProjectionPolicyClusterResolutionRerun", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeParentPolicyLaneExhaustedReturnToMissingProjectionPolicy", "reason drift")
need(decision.get("selected_lane") == "MissingProjectionPolicyNextLane", "selected lane drift")
need(decision.get("selected_next_card") == next_card, "next card drift")
need(decision.get("selected_parent_policy_candidate") is None, "parent policy must not be selected")
need(decision.get("selected_carrier_type_axis") is None, "carrier axis must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "parent_policy_candidate_selection",
    "direct_parent_policy_candidate_selection",
    "result_history_as_direct_selection_proof",
    "manual_lane_selection",
    "hardcoded_lane_priority",
    "hardcoded_parent_policy_priority",
    "row_count_as_proof",
    "return_type_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "observed_subaxis_set_as_proof",
    "historical_preference_as_proof",
    "return_type_string_mapping_as_proof",
    "self_signed_fixture_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2091-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("source-selfhost-wider-route-selection-basis-010-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_source_selfhost_wider_route_selection_basis_010_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-source-selfhost-wider-route-selection-basis-010")
print("carrier_type_parent_policy_lane_parked=1")
print("selection_eligible_lane_count=1")
print("selected_lane=MissingProjectionPolicyNextLane")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
