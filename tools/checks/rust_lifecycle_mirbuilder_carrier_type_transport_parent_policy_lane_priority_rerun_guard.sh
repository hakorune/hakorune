#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_parent_policy_lane_priority_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2088-MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportParentPolicyLanePriorityRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("parent_policy_lane_priority_basis", "").endswith("mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json"), "basis input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_decision") == "SelectCarrierTypeParentPolicyLanePriorityRerun", "basis decision drift")
need(previous.get("basis_reason_token") == "CarrierTypeParentPolicyLanePriorityBasisDefined", "basis reason drift")
need(previous.get("basis_selected_next_card") == token, "basis selected next drift")
need(previous.get("candidate_parent_policy_lane_count") == 4, "previous lane count drift")
need(previous.get("basis_selection_eligible_parent_policy_lane_count") == 0, "previous selection count drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeParentPolicyLaneMechanicalSelectorV1", "selector drift")
need(rule.get("selection_requires_exactly_one_proof_tuple_complete") is True, "exactly-one rule drift")
need(rule.get("historical_policy_contract_is_diagnostic_until_current_compatibility_proven") is True, "historical diagnostic rule drift")
need(rule.get("result_history_as_direct_selection_proof") is False, "result history proof drift")
need(rule.get("row_count_as_proof") is False, "row count proof drift")
need(rule.get("hardcoded_parent_policy_priority") is False, "hardcoded priority drift")

lanes = fixture.get("candidate_parent_policy_lanes") or []
expected_lanes = {
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
}
need({lane.get("policy_lane") for lane in lanes} == expected_lanes, "candidate lane set drift")
for lane in lanes:
    need(lane.get("scope_eligible") is True, f"scope drift: {lane.get('policy_lane')}")
    need(lane.get("guard_clean_authority", {}).get("status") == "Proven", f"guard clean drift: {lane.get('policy_lane')}")
    need(lane.get("evidence_inventory_completeness", {}).get("status") == "Proven", f"evidence drift: {lane.get('policy_lane')}")
    need(lane.get("proof_tuple_complete") is False, f"proof tuple drift: {lane.get('policy_lane')}")
    need(lane.get("selection_eligible") is False, f"selection drift: {lane.get('policy_lane')}")

result = next(lane for lane in lanes if lane.get("policy_lane") == "ResultCarrierPolicyCandidate")
readiness = result.get("current_policy_contract_readiness") or {}
need(readiness.get("status") == "Unproven", "Result readiness must be unproven")
need(readiness.get("historical_contract_present") == 1, "Result historical contract missing")
need(readiness.get("historical_contract_row_count") == 3, "Result historical row count drift")
need(readiness.get("current_candidate_count") == 557, "Result current count drift")
need(readiness.get("current_input_fixture_hash_compatibility") == 0, "Result compatibility drift")
need(readiness.get("supported_policy_lane_matches_current_lane") == 0, "Result lane match drift")
need("ResultHistoryIsNotDirectSelectionProof" in readiness.get("blocked_by", []), "Result history blocker missing")

summary = fixture.get("summary") or {}
for key, expected in {
    "candidate_parent_policy_lane_count": 4,
    "scope_eligible_parent_policy_lane_count": 4,
    "guard_clean_parent_policy_lane_count": 4,
    "evidence_inventory_complete_parent_policy_lane_count": 4,
    "current_policy_contract_ready_count": 0,
    "dependency_root_candidate_count": 0,
    "prior_closed_policy_continuation_candidate_count": 0,
    "proof_tuple_complete_parent_policy_lane_count": 0,
    "selection_eligible_parent_policy_lane_count": 0,
    "historical_result_contract_present": 1,
    "historical_result_contract_as_direct_selection_proof": 0,
}.items():
    need(summary.get(key) == expected, f"summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeParentPolicyLaneMechanicalCandidate", "reason drift")
need(decision.get("selected_parent_policy_candidate") is None, "parent policy must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "direct_parent_policy_candidate_selection",
    "manual_parent_policy_selection",
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
need(manifest_row.get("card", "").endswith("2088-MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_parent_policy_lane_priority_rerun_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun")
print("candidate_parent_policy_lane_count=4")
print("selection_eligible_parent_policy_lane_count=0")
print("reason=NoCarrierTypeParentPolicyLaneMechanicalCandidate")
print("source_selfhost_claim=0")
print("summary=ok")
PY
