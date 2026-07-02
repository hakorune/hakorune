#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_parent_policy_lane_priority_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2087-MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportParentPolicyLanePriorityBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("wider_route_selection_basis_009", "").endswith("source-selfhost-wider-route-selection-basis-009-v0.json"), "basis 009 input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_009_decision") == "SelectCarrierTypeParentPolicyLanePriorityBasis", "basis 009 decision drift")
need(previous.get("basis_009_reason_token") == "CarrierTypeRemainingLaneParkedReturnToParentPolicyLanePriority", "basis 009 reason drift")
need(previous.get("basis_009_selected_next_card") == token, "basis 009 next drift")
need(previous.get("carrier_type_remaining_lane_parked") == 1, "lane park drift")
need(previous.get("direct_parent_policy_candidate_selection") == 0, "previous direct selection drift")

expected_counts = {
    "ResultCarrierPolicyCandidate": 557,
    "OptionCarrierPolicyCandidate": 166,
    "SelfConstructorTransportPolicyCandidate": 56,
    "CollectionCarrierPolicyCandidate": 35,
}
need(previous.get("policy_lane_candidate_counts") == expected_counts, "policy lane count drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeParentPolicyLaneMechanicalSelectorV1", "selector drift")
need(rule.get("basis_selects_concrete_parent_policy_candidate") is False, "basis selection rule drift")
need(rule.get("rerun_may_select_parent_policy_only_if_exactly_one_proof_tuple_complete") is True, "rerun selection rule drift")
need(rule.get("tie_breaking_forbidden") is True, "tie-break rule drift")
need(rule.get("if_multiple_parent_policy_lanes_keep_stopped") is True, "multiple stop rule drift")

lanes = fixture.get("candidate_parent_policy_lanes") or []
expected_lanes = set(expected_counts)
need({lane.get("policy_lane") for lane in lanes} == expected_lanes, "candidate lane set drift")
for lane in lanes:
    name = lane.get("policy_lane")
    need(lane.get("diagnostic_count") == expected_counts[name], f"diagnostic count drift: {name}")
    need(lane.get("diagnostic_count_as_proof") is False, f"count proof drift: {name}")
    need(lane.get("scope_eligible") is True, f"scope drift: {name}")
    need(lane.get("proof_tuple_complete") is False, f"proof tuple drift: {name}")
    need(lane.get("selection_eligible") is False, f"selection drift: {name}")
    need(lane.get("selected_next_card_if_selected"), f"missing selected-next if selected: {name}")

summary = fixture.get("summary") or {}
need(summary.get("candidate_parent_policy_lane_count") == 4, "summary candidate count drift")
need(summary.get("scope_eligible_parent_policy_lane_count") == 4, "summary scope count drift")
need(summary.get("basis_selection_eligible_parent_policy_lane_count") == 0, "summary eligible drift")
need(summary.get("basis_selects_concrete_parent_policy_candidate") == 0, "summary concrete selection drift")
need(summary.get("direct_parent_policy_candidate_selection") == 0, "summary direct selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeParentPolicyLanePriorityRerun", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeParentPolicyLanePriorityBasisDefined", "reason drift")
need(decision.get("selected_parent_policy_candidate") is None, "parent policy must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "direct_parent_policy_candidate_selection",
    "manual_lane_selection",
    "manual_carrier_selection",
    "hardcoded_parent_policy_priority",
    "row_count_as_proof",
    "return_type_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "historical_preference_as_proof",
    "result_history_as_direct_selection_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2087-MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_parent_policy_lane_priority_basis_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis")
print("candidate_parent_policy_lane_count=4")
print("basis_selection_eligible_parent_policy_lane_count=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
