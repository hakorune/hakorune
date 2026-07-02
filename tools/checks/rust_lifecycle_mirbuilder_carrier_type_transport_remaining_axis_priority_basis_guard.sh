#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_priority_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2079-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisPriorityBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
for key, suffix in {
    "wider_route_selection_basis_008": "source-selfhost-wider-route-selection-basis-008-v0.json",
    "carrier_type_transport_policy_inventory_rerun_003": "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json",
    "carrier_type_transport_evidence_inventory_rerun_003": "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json",
    "carrier_type_transport_unclassified_evidence_resolution_002": "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json",
    "domain_object_id_semantic_resource_domain_declaration_inventory": "mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json",
}.items():
    need(str(inputs.get(key, "")).endswith(suffix), f"input drift: {key}")

previous = fixture.get("previous_state") or {}
need(previous.get("domain_object_id_lane_parked") == 1, "DomainObject/Id not parked")
need(previous.get("domain_object_id_subaxis_selection_eligible") == 0, "DomainObject/Id subaxis eligible drift")
need(previous.get("basis_008_selected_next_card") == token, "BASIS-008 next drift")
need(previous.get("policy_lane_candidates_present") is True, "policy lanes missing")
need(previous.get("unclassified_axis_resolution_present") is True, "unclassified axes missing")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisMechanicalSelectorV1", "selector rule drift")
need(rule.get("basis_selects_concrete_axis") is False, "basis must not select concrete axis")
need(rule.get("rerun_may_select_axis_only_if_exactly_one_proof_tuple_complete") is True, "rerun selection rule drift")
need(rule.get("selection_scope") == "ResolvedNonDomainObjectAxesFromCarrierTypeTransportEvidenceInventoryRequired", "scope drift")
need(rule.get("parent_policy_lanes_deferred_until_unclassified_branch_closed_or_parked") is True, "parent policy deferral drift")
for forbidden in [
    "row_count",
    "owner_name",
    "source_path",
    "route_membership_alone",
    "lexical_order",
    "coverage_percentage",
    "apparent_simplicity",
    "return_type_string_mapping",
    "observed_subaxis_set",
    "hardcoded_carrier_axis_priority",
]:
    need(forbidden in (rule.get("forbidden_priority_sources") or []), f"missing forbidden source: {forbidden}")

parked = fixture.get("parked_axes") or []
need(len(parked) == 1, "parked axis count drift")
need(parked[0].get("axis") == "DomainObjectOrIdTransportAxis", "parked axis drift")
need(parked[0].get("parked") is True, "parked state drift")
need(parked[0].get("selection_eligible") is False, "parked axis must not be eligible")

candidate_axes = fixture.get("candidate_axes") or []
expected_axes = {
    "ProductTupleTransportAxis",
    "CollectionCarrierTransportAxis",
    "IteratorOrBorrowTypeTransportAxis",
    "OpaqueTypeTransportAxis",
    "ScalarKnownTransportAxis",
}
need({row.get("axis") for row in candidate_axes} == expected_axes, "candidate axis set drift")
expected_counts = {
    "ProductTupleTransportAxis": 9,
    "CollectionCarrierTransportAxis": 2,
    "IteratorOrBorrowTypeTransportAxis": 1,
    "OpaqueTypeTransportAxis": 1,
    "ScalarKnownTransportAxis": 1,
}
for row in candidate_axes:
    axis = row.get("axis")
    need(row.get("scope_eligible") is True, f"scope eligibility drift: {axis}")
    need(row.get("diagnostic_count") == expected_counts[axis], f"diagnostic count drift: {axis}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple must be false at basis: {axis}")
    need(row.get("selection_eligible") is False, f"axis must not be selected at basis: {axis}")
    need(row.get("selected_next_card_if_selected"), f"missing next-card pointer: {axis}")

deferred = fixture.get("deferred_parent_policy_lanes") or []
expected_policy = {
    "ResultCarrierPolicyCandidate": 557,
    "OptionCarrierPolicyCandidate": 166,
    "SelfConstructorTransportPolicyCandidate": 56,
    "CollectionCarrierPolicyCandidate": 35,
}
need({row.get("policy_lane") for row in deferred} == set(expected_policy), "deferred policy set drift")
for row in deferred:
    lane = row.get("policy_lane")
    need(row.get("diagnostic_count") == expected_policy[lane], f"deferred count drift: {lane}")
    need(row.get("selection_eligible_in_this_basis") is False, f"deferred lane must not be eligible: {lane}")

summary = fixture.get("summary") or {}
need(summary.get("domain_object_id_lane_parked") == 1, "summary parked drift")
need(summary.get("parked_axis_count") == 1, "summary parked count drift")
need(summary.get("deferred_parent_policy_lane_count") == 4, "summary deferred count drift")
need(summary.get("candidate_axis_count") == 5, "summary candidate count drift")
need(summary.get("basis_selection_eligible_axis_count") == 0, "basis eligible count drift")
need(summary.get("basis_selects_concrete_axis") == 0, "basis selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeRemainingAxisPriorityRerun", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeRemainingAxisPriorityBasisDefined", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "carrier axis must not be selected")
need(decision.get("selected_domain_subaxis") is None, "domain subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "manual_axis_selection",
    "manual_carrier_selection",
    "hardcoded_carrier_axis_priority",
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
need(manifest_row.get("card", "").endswith("2079-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_priority_basis_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-priority-basis")
print("domain_object_id_lane_parked=1")
print("candidate_axis_count=5")
print("deferred_parent_policy_lane_count=4")
print("basis_selects_concrete_axis=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
