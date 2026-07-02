#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-009-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_009.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2086-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009.md"
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


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasis009V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("component_evidence_source_discovery_inventory", "").endswith("mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"), "discovery inventory input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_decision") == "SelectWiderRouteSelectionBasis", "previous decision drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeComponentEvidenceSourceAuthority", "previous reason drift")
need(previous.get("component_requirement_count") == 7, "component requirement count drift")
need(previous.get("allowed_source_kind_count") == 5, "allowed source count drift")
need(previous.get("accepted_component_evidence_source_count") == 0, "accepted source drift")
need(previous.get("component_authority_source_count") == 0, "authority source drift")
need(previous.get("component_requirement_with_accepted_source_count") == 0, "requirement source drift")

for key in [
    "stable_component_policy_contract_count",
    "explicit_boundary_declaration_count",
    "stable_cross_lane_handoff_contract_count",
    "collection_overlap_contract_count",
    "typed_direct_closeout_contract_count",
]:
    need(previous.get(key) == 0, f"previous source count drift: {key}")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "PostCarrierTypeRemainingAuthorityExhaustionWiderLaneSelectorV1", "selector drift")
need(rule.get("carrier_type_remaining_lane_must_be_parked_before_wider_selection") is True, "park precondition drift")
need(rule.get("concrete_carrier_type_axis_selection_forbidden") is True, "axis forbidden drift")
need(rule.get("component_specific_card_selection_forbidden") is True, "component forbidden drift")
need(rule.get("direct_parent_policy_candidate_selection_forbidden") is True, "parent policy direct selection drift")
need(rule.get("select_only_if_exactly_one_machine_derived_lane") is True, "exactly-one rule drift")

parked = {row.get("lane_id"): row for row in fixture.get("parked_lanes") or []}
need(parked.get("DomainObjectIdLane", {}).get("parked") is True, "DomainObject lane park drift")
need(parked.get("CarrierTypeRemainingAxisLane", {}).get("parked") is True, "carrier/type lane park drift")
need(parked.get("CarrierTypeRemainingAxisLane", {}).get("park_reason_token") == "NoCarrierTypeComponentEvidenceSourceAuthority", "carrier/type park reason drift")

parent_lanes = fixture.get("deferred_parent_policy_lanes") or []
expected_parent = {
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
}
need({row.get("policy_lane") for row in parent_lanes} == expected_parent, "parent lane set drift")
for row in parent_lanes:
    need(row.get("direct_selection_allowed") is False, f"direct parent selection drift: {row.get('policy_lane')}")

candidate_lanes = fixture.get("candidate_lanes") or []
expected_lanes = {
    "UnconvertedSurfaceReportRerun",
    "NativeOwnerCheckpointRerun",
    "CarrierTypeParentPolicyLanePriority",
    "MissingProjectionPolicyNextLane",
    "BorrowSurfacePolicyLane",
    "GuardConsolidation",
}
need({row.get("lane_id") for row in candidate_lanes} == expected_lanes, "candidate lane set drift")
eligible = [row for row in candidate_lanes if row.get("selection_eligible")]
need(len(eligible) == 1, "eligible lane count drift")
need(eligible[0].get("lane_id") == "CarrierTypeParentPolicyLanePriority", "eligible lane drift")
need(eligible[0].get("selected_next_card_if_eligible") == next_card, "eligible next-card drift")

summary = fixture.get("summary") or {}
need(summary.get("carrier_type_remaining_lane_parked") == 1, "summary park drift")
need(summary.get("component_authority_source_count") == 0, "summary authority drift")
need(summary.get("candidate_lane_count") == 6, "summary lane count drift")
need(summary.get("selection_eligible_lane_count") == 1, "summary eligible drift")
need(summary.get("deferred_parent_policy_lane_count") == 4, "summary parent lane drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeParentPolicyLanePriorityBasis", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeRemainingLaneParkedReturnToParentPolicyLanePriority", "reason drift")
need(decision.get("selected_lane") == "CarrierTypeParentPolicyLanePriority", "selected lane drift")
need(decision.get("selected_next_card") == next_card, "next card drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") is None, "component must not be selected")
need(decision.get("selected_parent_policy_candidate") is None, "parent policy candidate must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "direct_parent_policy_candidate_selection",
    "manual_lane_selection",
    "hardcoded_lane_priority",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "observed_subaxis_set_as_proof",
    "return_type_string_mapping_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2086-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("source-selfhost-wider-route-selection-basis-009-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_source_selfhost_wider_route_selection_basis_009_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-source-selfhost-wider-route-selection-basis-009")
print("carrier_type_remaining_lane_parked=1")
print("selection_eligible_lane_count=1")
print("selected_lane=CarrierTypeParentPolicyLanePriority")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
